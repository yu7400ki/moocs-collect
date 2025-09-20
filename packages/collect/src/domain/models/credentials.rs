/// User credentials for authentication
#[derive(Debug, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

impl Credentials {
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credentials() {
        let creds = Credentials::new("user", "pass");
        assert_eq!(creds.username, "user");
        assert_eq!(creds.password, "pass");
    }
}
