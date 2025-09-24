use crate::cache::Cache;
use crate::domain::{models::Credentials, repository::AuthenticationRepository};
use crate::error::{CollectError, Result};
use crate::utils::extract_element_attribute;
use async_trait::async_trait;
use regex::Regex;
use reqwest::{Client, Response};
use scraper::Html;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AuthCacheKey {
    MoocsAuth,
    GoogleAuth,
}

pub struct AuthenticationRepositoryImpl {
    client: Arc<Client>,
    auth_cache: Cache<AuthCacheKey, bool>,
}

impl AuthenticationRepositoryImpl {
    pub fn new(client: Arc<Client>) -> Self {
        Self {
            client,
            auth_cache: Cache::new(Duration::from_secs(30 * 60)), // 30分のキャッシュ有効期限
        }
    }

    fn get_cached_moocs_auth(&self) -> Option<bool> {
        self.auth_cache.get(&AuthCacheKey::MoocsAuth)
    }

    fn get_cached_google_auth(&self) -> Option<bool> {
        self.auth_cache.get(&AuthCacheKey::GoogleAuth)
    }

    fn set_moocs_authenticated(&self, is_authenticated: bool) {
        self.auth_cache
            .insert(AuthCacheKey::MoocsAuth, is_authenticated);
    }

    fn set_google_authenticated(&self, is_authenticated: bool) {
        self.auth_cache
            .insert(AuthCacheKey::GoogleAuth, is_authenticated);
    }

    async fn check_moocs_login_status(&self) -> Result<bool> {
        let url = "https://moocs.iniad.org/account";
        let response = self.client.get(url).send().await?;
        let success = response.url().path() == "/account";
        Ok(success)
    }

    async fn check_google_login_status(&self) -> Result<bool> {
        let url = "https://myaccount.google.com";
        let response = self.client.get(url).send().await?;
        let success = response.url().domain() == Some("myaccount.google.com");
        Ok(success)
    }

    async fn login_with_form(&self, action: &str, credentials: &Credentials) -> Result<Response> {
        let response = self
            .client
            .post(action)
            .form(&[
                ("username", &credentials.username),
                ("password", &credentials.password),
            ])
            .send()
            .await?;

        Ok(response)
    }
}

#[async_trait]
impl AuthenticationRepository for AuthenticationRepositoryImpl {
    async fn login_moocs(&self, credentials: &Credentials) -> Result<()> {
        let login_url = "https://moocs.iniad.org/auth/iniad";
        let response = self.client.get(login_url).send().await?;
        let response_url = response.url().to_string();
        if response_url == "https://moocs.iniad.org/courses" {
            return Ok(());
        }
        let body = response.text().await?;
        let document = Html::parse_document(&body);
        let action =
            extract_element_attribute(&document.root_element(), "form.form-signin", "action")?;
        self.login_with_form(&action, credentials).await?;
        if self.is_logged_in_moocs().await? {
            Ok(())
        } else {
            Err(CollectError::Authentication {
                reason: "Invalid username or password.".into(),
            })
        }
    }

    async fn login_google(&self, credentials: &Credentials) -> Result<()> {
        let login_url = "https://accounts.google.com/samlredirect?domain=iniad.org";
        let response = self.client.get(login_url).send().await?;
        let body = response.text().await?;
        let mut document = Html::parse_document(&body);
        let action =
            { extract_element_attribute(&document.root_element(), "form.form-signin", "action") };
        if action.is_ok() {
            let action = action?;
            let response = self.login_with_form(&action, credentials).await?;
            let body = response.text().await?;
            let error_message = "Invalid username or password.";
            if body.contains(error_message) {
                return Err(CollectError::Authentication {
                    reason: error_message.to_string(),
                });
            }
            document = Html::parse_document(&body);
            let root_element = document.root_element();
            extract_element_attribute(&root_element, "form[name='saml-post-binding']", "action")?;
        }

        let (action, saml_response, relay_state) = {
            let root_element = document.root_element();
            (
                extract_element_attribute(
                    &root_element,
                    "form[name='saml-post-binding']",
                    "action",
                )?,
                extract_element_attribute(&root_element, "input[name='SAMLResponse']", "value")?,
                extract_element_attribute(&root_element, "input[name='RelayState']", "value")?,
            )
        };
        let response = self
            .client
            .post(&action)
            .form(&[
                ("SAMLResponse", &saml_response),
                ("RelayState", &relay_state),
            ])
            .send()
            .await?;

        let body = response.text().await?;
        let document = Html::parse_document(&body);
        let (action, relay_state, saml_response, trampoline) = {
            let root_element = document.root_element();
            (
                extract_element_attribute(&root_element, "form[name='hiddenpost']", "action")?,
                extract_element_attribute(&root_element, "input[name='RelayState']", "value")?,
                extract_element_attribute(&root_element, "input[name='SAMLResponse']", "value")?,
                extract_element_attribute(&root_element, "input[name='trampoline']", "value")?,
            )
        };
        let response = self
            .client
            .post(&action)
            .form(&[
                ("RelayState", &relay_state),
                ("SAMLResponse", &saml_response),
                ("trampoline", &trampoline),
            ])
            .send()
            .await?;

        let body = response.text().await?;
        let regex = Regex::new(r#"<a\s+(?:[^>]*?\s+)?href="([^"]*)""#).unwrap();
        let href = regex.captures(&body).unwrap().get(1).unwrap().as_str();
        let response = self.client.get(href.replace("&amp;", "&")).send().await?;

        let body = response.text().await?;
        let regex =
            Regex::new(r#"<meta\s+http-equiv="refresh"\s+content=".*\s+url=(.*?)">"#).unwrap();
        let url = regex.captures(&body).unwrap().get(1).unwrap().as_str();
        self.client.get(url.replace("&amp;", "&")).send().await?;

        if self.is_logged_in_google().await? {
            Ok(())
        } else {
            Err(CollectError::Authentication {
                reason: "Invalid username or password.".into(),
            })
        }
    }

    async fn is_logged_in_moocs(&self) -> Result<bool> {
        // キャッシュされた認証状態をチェック
        if let Some(cached_auth) = self.get_cached_moocs_auth() {
            return Ok(cached_auth);
        }

        // キャッシュが無効な場合、実際のログイン状態をチェック
        let is_logged_in = self.check_moocs_login_status().await?;

        // 結果をキャッシュ（タイムスタンプ付き）
        self.set_moocs_authenticated(is_logged_in);

        Ok(is_logged_in)
    }

    async fn is_logged_in_google(&self) -> Result<bool> {
        // キャッシュされた認証状態をチェック
        if let Some(cached_auth) = self.get_cached_google_auth() {
            return Ok(cached_auth);
        }

        // キャッシュが無効な場合、実際のログイン状態をチェック
        let is_logged_in = self.check_google_login_status().await?;

        // 結果をキャッシュ（タイムスタンプ付き）
        self.set_google_authenticated(is_logged_in);

        Ok(is_logged_in)
    }
}
