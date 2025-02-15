#![forbid(unsafe_code)]
#![recursion_limit = "512"]

// The only public resource should be the common directory
pub mod common;

mod components;
mod pages;
mod router;

use components::Metadata;
use leptos::prelude::*;
use leptos_meta::*;
use router::AppRouter;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en-US">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options=options.clone() />
                <MetaTags />
                <HashedStylesheet id="leptos" options />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
#[must_use]
pub fn App() -> impl IntoView {
    view! {
        <Metadata />
        <AppRouter />
    }
}
