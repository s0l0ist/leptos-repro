use serde::Deserialize;

/// These are the credentials submitted from the frontend. They currently only
/// support email/password; however, they will need to support other forms of
/// authentication to handle MFA, SSO, etc.
#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}
