use crate::domain::models::Credentials;
use crate::domain::repository::AuthenticationRepository;
use crate::domain::service::AuthenticationService;
use crate::error::Result;
use async_trait::async_trait;
use std::sync::Arc;

/// Authentication service implementation
pub struct AuthenticationServiceImpl {
    auth_repository: Arc<dyn AuthenticationRepository>,
}

impl AuthenticationServiceImpl {
    /// Create a new authentication service
    pub fn new(auth_repository: Arc<dyn AuthenticationRepository>) -> Self {
        Self { auth_repository }
    }
}

#[async_trait]
impl AuthenticationService for AuthenticationServiceImpl {
    /// Login to MOOCs system
    async fn login_moocs(&self, credentials: &Credentials) -> Result<()> {
        self.auth_repository.login_moocs(credentials).await
    }

    /// Login to Google SAML system
    async fn login_google(&self, credentials: &Credentials) -> Result<()> {
        self.auth_repository.login_google(credentials).await
    }

    /// Check if logged into MOOCs
    async fn is_logged_in_moocs(&self) -> Result<bool> {
        self.auth_repository.is_logged_in_moocs().await
    }

    /// Check if logged into Google
    async fn is_logged_in_google(&self) -> Result<bool> {
        self.auth_repository.is_logged_in_google().await
    }
}
