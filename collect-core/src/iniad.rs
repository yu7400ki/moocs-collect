use regex::Regex;
use reqwest::{Client, Response};
use scraper::Html;

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

fn extract_element_attribute(document: &Html, query: &str, attribute: &str) -> Option<String> {
    document
        .select(&scraper::Selector::parse(query).unwrap())
        .next()
        .and_then(|element| Some(element.value().clone()))
        .and_then(|element| element.attr(attribute).map(|value| value.to_string()))
}

fn extract_form_action(document: &Html, query: &str) -> Option<String> {
    extract_element_attribute(document, query, "action")
}

fn extract_input_value(document: &Html, query: &str) -> Option<String> {
    extract_element_attribute(document, query, "value")
}

fn logged_in(success: bool) -> anyhow::Result<()> {
    match success {
        true => Ok(()),
        false => Err(anyhow::anyhow!("Not logged in")),
    }
}

pub async fn check_logged_in_moocs(client: &Client) -> reqwest::Result<anyhow::Result<()>> {
    let url = "https://moocs.iniad.org/account";
    let response = client.get(url).send().await?;
    let success = response.url().path() == "/account";
    Ok(logged_in(success))
}

pub async fn check_logged_in_google(client: &Client) -> reqwest::Result<anyhow::Result<()>> {
    let url = "https://myaccount.google.com";
    let response = client.get(url).send().await?;
    let success = response.url().domain() == Some("myaccount.google.com");
    Ok(logged_in(success))
}

pub async fn login_moocs(client: &Client, credentials: &Credentials) -> reqwest::Result<bool> {
    let login_url = "https://moocs.iniad.org/auth/iniad";
    let response = client.get(login_url).send().await?;
    let body = response.text().await?;
    let document = Html::parse_document(&body);
    let action = extract_form_action(&document, "form.form-signin");
    if let Some(action) = action {
        login(client, credentials, &action).await?;
    }
    Ok(check_logged_in_moocs(client).await?.is_ok())
}

pub async fn login_google(client: &Client, credentials: &Credentials) -> reqwest::Result<bool> {
    let login_url = "https://accounts.google.com/samlredirect?domain=iniad.org";
    let response = client.get(login_url).send().await?;
    let body = response.text().await?;
    let mut document = Html::parse_document(&body);
    let action = extract_form_action(&document, "form.form-signin");
    if let Some(action) = action {
        let response = login(client, credentials, &action).await?;
        let body = response.text().await?;
        document = Html::parse_document(&body);
    }

    let action = extract_form_action(&document, "form[name='saml-post-binding']");
    let saml_response = extract_input_value(&document, "input[name='SAMLResponse']");
    let relay_state = extract_input_value(&document, "input[name='RelayState']");
    let response = if let (Some(action), Some(saml_response), Some(relay_state)) =
        (action, saml_response, relay_state)
    {
        client
            .post(&action)
            .form(&[
                ("SAMLResponse", &saml_response),
                ("RelayState", &relay_state),
            ])
            .send()
            .await?
    } else {
        return Ok(false);
    };

    let body = response.text().await?;
    let document = Html::parse_document(&body);
    let action = extract_form_action(&document, "form[name='hiddenpost']");
    let relay_state = extract_input_value(&document, "input[name='RelayState']");
    let saml_response = extract_input_value(&document, "input[name='SAMLResponse']");
    let trampoline = extract_input_value(&document, "input[name='trampoline']");
    let response = if let (Some(action), Some(relay_state), Some(saml_response), Some(trampoline)) =
        (action, relay_state, saml_response, trampoline)
    {
        client
            .post(&action)
            .form(&[
                ("RelayState", &relay_state),
                ("SAMLResponse", &saml_response),
                ("trampoline", &trampoline),
            ])
            .send()
            .await?
    } else {
        return Ok(false);
    };

    let body = response.text().await?;
    let regex = Regex::new(r#"<a\s+(?:[^>]*?\s+)?href="([^"]*)""#).unwrap();
    let href = regex.captures(&body).unwrap().get(1).unwrap().as_str();
    let response = client.get(href.replace("&amp;", "&")).send().await?;

    let body = response.text().await?;
    let regex = Regex::new(r#"<meta\s+http-equiv="refresh"\s+content=".*\s+url=(.*?)">"#).unwrap();
    let url = regex.captures(&body).unwrap().get(1).unwrap().as_str();
    client.get(url.replace("&amp;", "&")).send().await?;

    Ok(check_logged_in_google(client).await?.is_ok())
}
