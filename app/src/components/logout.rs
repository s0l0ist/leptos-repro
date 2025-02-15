use leptos::prelude::*;

#[tracing::instrument(skip_all)]
#[server(Logout, "/api", endpoint = "auth/logout")]
pub async fn logout() -> Result<(), ServerFnError> {
    Ok(())
}
