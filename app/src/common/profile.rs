use super::user::UserState;
use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserProfile {
    pub org_id: String,
    pub user_id: String,
    pub user_name: String,
    pub email: String,
    pub state: UserState,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrgProfile {
    pub org_id: String,
    pub org_name: String,
    pub org_type: OrgType,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum OrgType {
    #[default]
    Free,
}

impl std::fmt::Display for OrgType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrgType::Free => write!(f, "Free"),
        }
    }
}

/// An API which does the following:
/// 1. To check if the session is valid (which performs a DDB lookup for the
///    user record).
/// 2. To return the User record for future use.
///
/// NOTE: The API path _must_ be static to allow us to specify the path in the
/// axum router manually. It also helps prevent issues when we have rolling
/// updates.
#[tracing::instrument(skip_all)]
#[server(ProfileApi, "/api", endpoint = "user_profile")]
pub async fn get_user_profile() -> Result<UserProfile, ServerFnError> {
    Ok(UserProfile {
        org_id: "624614ba-f9f1-4022-8437-43b117d1ceaa".to_string(),
        user_id: "a24614ba-f9f1-4022-8437-43b117d1ceaa".to_string(),
        user_name: "User Name".to_string(),
        email: "email@example.com".to_string(),
        state: crate::common::user::UserState::Active,
    })
}

/// An API which does the following:
/// 1. To check if the session is valid (which performs a DDB lookup for the
///    user record).
/// 2. To return the User record for future use.
///
/// NOTE: The API path _must_ be static to allow us to specify the path in the
/// axum router manually. It also helps prevent issues when we have rolling
/// updates.
#[tracing::instrument(skip_all)]
#[server(OrgProfileApi, "/api", endpoint = "org_profile")]
pub async fn get_org_profile() -> Result<OrgProfile, ServerFnError> {
    Ok(OrgProfile {
        org_id: "624614ba-f9f1-4022-8437-43b117d1ceaa".to_string(),
        org_name: "Org Name".to_string(),
        org_type: OrgType::Free,
    })
}
