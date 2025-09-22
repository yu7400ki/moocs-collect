use crate::domain::models::Credentials;
use crate::error::Result;
use async_trait::async_trait;

/// Authentication service trait
#[async_trait]
pub trait AuthenticationService: Send + Sync {
    /// Login to MOOCs system
    async fn login_moocs(&self, credentials: &Credentials) -> Result<()>;

    /// Login to Google SAML system
    async fn login_google(&self, credentials: &Credentials) -> Result<()>;

    /// Check if logged into MOOCs
    async fn is_logged_in_moocs(&self) -> Result<bool>;

    /// Check if logged into Google
    async fn is_logged_in_google(&self) -> Result<bool>;
}
