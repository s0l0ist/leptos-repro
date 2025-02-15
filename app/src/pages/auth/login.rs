use crate::{
    common::{profile::UserProfile, route_paths::ABS_ROOT_PATH},
    components::{BackgroundPattern, CompanyLogoIcon, SubmitButton},
    router::RouteLink,
};
use leptos::prelude::*;
use leptos_router::hooks::use_query_map;
use serde::{Deserialize, Serialize};

#[component]
pub fn LoginView(action: ServerAction<Login>) -> impl IntoView {
    let log_in_to_your_account = "Log in to your account";
    let login = "Log in";
    let company_name = "BugRepro";
    let email_address = "Email address";
    let password = "Password";
    let pending_text = "Logging in...";

    let pending = action.pending();

    let is_err = Signal::derive(move || {
        action.value().get().is_some_and(|result| result.is_err()) && !pending.get()
    });
    let err_string = Signal::derive(move || {
        action
            .value()
            .get()
            .map(|res| match res {
                Ok(_) => String::new(),
                Err(e) => e.to_string(),
            })
            .unwrap_or_default()
    });

    let query = use_query_map();
    let next = move || query.get().get("next");

    view! {
        <main class="relative px-6 pt-14 lg:px-8 isolate">
            <BackgroundPattern />
            <div class="flex flex-col justify-center py-12 min-h-full sm:px-6 lg:px-8">
                <div class="sm:mx-auto sm:w-full sm:max-w-md">
                    <div class="flex justify-center">
                        <RouteLink path=ABS_ROOT_PATH class="inline-block">
                            <span class="sr-only">{company_name}</span>
                            <CompanyLogoIcon attr:class="h-16 w-auto text-indigo-600" />
                        </RouteLink>
                    </div>
                    <h1 class="mt-10 text-2xl font-bold tracking-tight leading-9 text-center text-gray-900">
                        {log_in_to_your_account}
                    </h1>
                </div>

                <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
                    <ActionForm attr:class="space-y-6" action=action>
                        <input id="next" name="next" type="hidden" value=next />
                        <div>
                            <label
                                for="email"
                                class="block text-sm font-medium leading-6 text-gray-900"
                            >
                                {email_address}
                            </label>
                            <div class="mt-2">
                                <input
                                    id="email"
                                    name="email"
                                    type="email"
                                    autocomplete="email"
                                    required
                                    class="block py-1.5 px-3 w-full rounded-md border-0 ring-1 ring-inset ring-gray-300 sm:text-sm sm:leading-6 focus:ring-2 focus:ring-inset focus:ring-indigo-600 shadow-xs placeholder:text-gray-400"
                                />
                            </div>
                        </div>
                        <div>
                            <div class="flex justify-between items-center">
                                <label
                                    for="password"
                                    class="block text-sm font-medium leading-6 text-gray-900"
                                >
                                    {password}
                                </label>

                            </div>
                            <div class="mt-2">
                                <input
                                    id="password"
                                    name="password"
                                    type="password"
                                    autocomplete="current-password"
                                    required
                                    class="block py-1.5 px-3 w-full rounded-md border-0 ring-1 ring-inset ring-gray-300 sm:text-sm sm:leading-6 focus:ring-2 focus:ring-inset focus:ring-indigo-600 shadow-xs placeholder:text-gray-400"
                                />
                            </div>
                        </div>
                        <div>
                            <SubmitButton button_text=login pending_text pending />
                        </div>
                    </ActionForm>
                </div>

                <Show when=move || is_err.get()>
                    <p class="mt-10 text-sm text-center text-red-500">{err_string}</p>
                </Show>
            </div>
        </main>
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub profile: UserProfile,
    next: Option<String>,
}

/// An API to authenticate credentials.
///
/// NOTE: The API path _must_ be static to allow us to specify the path in the
/// axum router manually. It also helps prevent issues when we have rolling
/// updates.
#[tracing::instrument(skip_all)]
#[server(Login, "/api", endpoint = "auth/login")]
pub async fn login(
    email: String,
    password: String,
    next: Option<String>,
) -> Result<LoginResponse, ServerFnError> {
    let _ = password;
    let _ = next;

    let url = "/app";
    leptos_axum::redirect(url);

    Ok(LoginResponse {
        profile: UserProfile {
            org_id: "624614ba-f9f1-4022-8437-43b117d1ceaa".to_string(),
            user_id: "a24614ba-f9f1-4022-8437-43b117d1ceaa".to_string(),
            user_name: "User Name".to_string(),
            email: email,
            state: crate::common::user::UserState::Active,
        },
        next: Some(url.to_string()),
    })
}
