use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum UserState {
    /// The user is active.
    #[default]
    Active,
    /// The user has not verified their email. Users who create new accounts
    /// have this status set by default.
    Unactivated,
    /// The user account is deactivated. This state is used when an org admin
    /// explicitly sets the user to this state or via SAML/OIDC (+ SCIM)
    /// integrations that may also use this state.
    Deactivated,
    /// The user's account is locked for security reasons (i.e. too many
    /// password attempts have been made). This state is set by the system.
    Locked,
}

impl fmt::Display for UserState {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserState::Active => write!(fmt, "Active"),
            UserState::Unactivated => write!(fmt, "Unactivated"),
            UserState::Deactivated => write!(fmt, "Deactivated"),
            UserState::Locked => write!(fmt, "Locked"),
        }
    }
}
