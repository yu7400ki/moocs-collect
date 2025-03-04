use crate::utils::extract_element_attribute;
use regex::Regex;
use reqwest::{Client, Response};
use scraper::{ElementRef, Html};

#[derive(Debug, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

async fn login(
    client: &Client,
    credentials: &Credentials,
    action: &str,
) -> reqwest::Result<Response> {
    let response = client
        .post(action)
        .form(&[
            ("username", &credentials.username),
            ("password", &credentials.password),
        ])
        .send()
        .await?;

    Ok(response)
}

fn extract_form_action(document: &ElementRef, query: &str) -> anyhow::Result<String> {
    extract_element_attribute(document, query, "action")
}

fn extract_input_value(document: &ElementRef, query: &str) -> anyhow::Result<String> {
    extract_element_attribute(document, query, "value")
}

pub async fn check_logged_in_moocs(client: &Client) -> anyhow::Result<bool> {
    let url = "https://moocs.iniad.org/account";
    let response = client.get(url).send().await?;
    let success = response.url().path() == "/account";
    Ok(success)
}

pub async fn check_logged_in_google(client: &Client) -> anyhow::Result<bool> {
    let url = "https://myaccount.google.com";
    let response = client.get(url).send().await?;
    let success = response.url().domain() == Some("myaccount.google.com");
    Ok(success)
}

pub async fn login_moocs(client: &Client, credentials: &Credentials) -> anyhow::Result<bool> {
    let login_url = "https://moocs.iniad.org/auth/iniad";
    let response = client.get(login_url).send().await?;
    let body = response.text().await?;
    let document = Html::parse_document(&body);
    let action = extract_form_action(&document.root_element(), "form.form-signin");
    if action.is_ok() {
        let action = action?;
        login(client, credentials, &action).await?;
    }
    Ok(check_logged_in_moocs(client).await?)
}

pub async fn login_google(client: &Client, credentials: &Credentials) -> anyhow::Result<bool> {
    let login_url = "https://accounts.google.com/samlredirect?domain=iniad.org";
    let response = client.get(login_url).send().await?;
    let body = response.text().await?;
    let mut document = Html::parse_document(&body);
    let action = extract_form_action(&document.root_element(), "form.form-signin");
    if action.is_ok() {
        let action = action?;
        let response = login(client, credentials, &action).await?;
        let body = response.text().await?;
        document = Html::parse_document(&body);
    }

    let (action, saml_response, relay_state) = {
        let root_element = document.root_element();
        (
            extract_form_action(&root_element, "form[name='saml-post-binding']")?,
            extract_input_value(&root_element, "input[name='SAMLResponse']")?,
            extract_input_value(&root_element, "input[name='RelayState']")?,
        )
    };
    let response = client
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
            extract_form_action(&root_element, "form[name='hiddenpost']")?,
            extract_input_value(&root_element, "input[name='RelayState']")?,
            extract_input_value(&root_element, "input[name='SAMLResponse']")?,
            extract_input_value(&root_element, "input[name='trampoline']")?,
        )
    };
    let response = client
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
    let response = client.get(href.replace("&amp;", "&")).send().await?;

    let body = response.text().await?;
    let regex = Regex::new(r#"<meta\s+http-equiv="refresh"\s+content=".*\s+url=(.*?)">"#).unwrap();
    let url = regex.captures(&body).unwrap().get(1).unwrap().as_str();
    client.get(url.replace("&amp;", "&")).send().await?;

    Ok(check_logged_in_google(client).await?)
}
