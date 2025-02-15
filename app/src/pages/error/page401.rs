use crate::{common::route_paths::ABS_AUTH_LOGIN_PATH, router::RouteLink};
use leptos::prelude::*;
use leptos_router::hooks::use_location;
use urlencoding::encode;

#[component]
pub fn Page401(#[prop(optional, default = None)] message: Option<String>) -> impl IntoView {
    let error_desc = "Unauthorized";
    let sorry = "Sorry, you're not authorized to view this page.";
    let log_in = "Log in";

    let location = use_location();

    let next = format!(
        "{}{}",
        location.pathname.get_untracked(),
        location.query.get_untracked().to_query_string()
    );
    let path = format!("{}?next={}", ABS_AUTH_LOGIN_PATH, encode(&next));

    view! {
        <div class="grid place-items-center py-24 px-6 min-h-full bg-white sm:py-32 lg:px-8">
            <div class="text-center">
                <p class="text-base font-semibold text-indigo-600">401</p>
                <h1 class="mt-4 text-3xl font-bold tracking-tight text-gray-900 sm:text-5xl">
                    {error_desc}
                </h1>
                <p class="mt-6 text-base leading-7 text-gray-600">
                    {if message.is_some() { message.unwrap() } else { sorry.to_string() }}
                </p>
                <div class="flex gap-x-6 justify-center items-center mt-10">
                    <RouteLink
                        path
                        class="py-2.5 px-3.5 text-sm font-semibold text-white bg-indigo-600 rounded-md shadow-xs hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                    >
                        {log_in}
                    </RouteLink>
                </div>
            </div>
        </div>
    }
}
