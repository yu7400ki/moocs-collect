use regex::Regex;
use reqwest::{Client, Response};
use scraper::{node::Element, Html};

#[derive(Debug, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

async fn login(
    client: &Client,
    credentials: &Credentials,
    form: &Element,
) -> Result<Response, reqwest::Error> {
    let action = form.attr("action").unwrap();

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

fn extract_signin_form(document: &Html) -> Option<Element> {
    document
        .select(&scraper::Selector::parse("form.form-signin").unwrap())
        .next()
        .and_then(|form| Some(form.value().clone()))
}

pub async fn check_logged_in_moocs(client: &Client) -> Result<bool, reqwest::Error> {
    let url = "https://moocs.iniad.org/account";
    let response = client.get(url).send().await?;
    Ok(response.url().path() == "/account")
}

pub async fn check_logged_in_google(client: &Client) -> Result<bool, reqwest::Error> {
    let url = "https://myaccount.google.com";
    let response = client.get(url).send().await?;
    Ok(response.url().domain() == Some("myaccount.google.com"))
}

pub async fn login_moocs(
    client: &Client,
    credentials: &Credentials,
) -> Result<bool, reqwest::Error> {
    let login_url = "https://moocs.iniad.org/auth/iniad";
    let response = client.get(login_url).send().await?;
    let body = response.text().await?;
    let document = Html::parse_document(&body);
    let form = extract_signin_form(&document);
    if let Some(form) = form {
        login(client, credentials, &form).await?;
    }
    check_logged_in_moocs(client).await
}

pub async fn login_google(
    client: &Client,
    credentials: &Credentials,
) -> Result<bool, reqwest::Error> {
    let login_url = "https://accounts.google.com/samlredirect?domain=iniad.org";
    let response = client.get(login_url).send().await?;
    let body = response.text().await?;
    let mut document = Html::parse_document(&body);
    let form = extract_signin_form(&document);
    if let Some(form) = form {
        let response = login(client, credentials, &form).await?;
        let body = response.text().await?;
        document = Html::parse_document(&body);
    }

    let form = document
        .select(&scraper::Selector::parse("form[name='saml-post-binding']").unwrap())
        .next()
        .unwrap();
    let action = form.attr("action").unwrap();
    let saml_response = form
        .select(&scraper::Selector::parse("input[name='SAMLResponse']").unwrap())
        .next()
        .and_then(|input| Some(input.value().attr("value").unwrap()))
        .unwrap();
    let relay_state = form
        .select(&scraper::Selector::parse("input[name='RelayState']").unwrap())
        .next()
        .and_then(|input| Some(input.value().attr("value").unwrap()))
        .unwrap();
    let response = client
        .post(action)
        .form(&[("SAMLResponse", saml_response), ("RelayState", relay_state)])
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

    check_logged_in_google(client).await
}
