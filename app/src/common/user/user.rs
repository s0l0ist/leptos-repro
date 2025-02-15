use super::{OnboardMethod, UserState};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub org_id: String,
    pub user_id: String,
    pub user_name: String,
    pub email: String,
    pub onboard_method: OnboardMethod,
    pub state: UserState,
    pub created_at: i64,
    pub updated_at: i64,
}
