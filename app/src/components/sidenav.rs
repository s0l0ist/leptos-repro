use crate::{
    common::route_paths::{ABS_APP_GRAPHS_PATH, ABS_APP_SETTINGS_PATH, ABS_APP_USERS_PATH},
    components::{Cog6ToothIcon, UsersIcon},
    router::RouteLink,
};
use leptos::prelude::*;
use leptos_router::hooks::use_location;

#[component]
pub fn MenuItems(sidebar_visible: RwSignal<bool>) -> impl IntoView {
    let company_name = "BugRepro";
    let on_click = move |_| sidebar_visible.set(false);
    // Only show in local development for now.
    let feature_flag = move || true;

    view! {
        <div class="flex items-center h-16 shrink-0">
            <RouteLink path=ABS_APP_GRAPHS_PATH class="inline-block" on:click=on_click>
                <span class="sr-only">{company_name}</span>
            </RouteLink>

        </div>
        <nav class="flex flex-col flex-1">
            <ul role="list" class="flex flex-col flex-1 gap-y-7">
                <li>
                    <ul role="list" class="-mx-2 space-y-1">
                        <Show when=feature_flag>
                            <li>
                                <UsersNavItem sidebar_visible />
                            </li>
                        </Show>
                    </ul>
                </li>

                <li class="mt-auto">
                    <SettingsNavItem sidebar_visible />
                </li>
            </ul>
        </nav>
    }
}

#[component]
fn UsersNavItem(sidebar_visible: RwSignal<bool>) -> impl IntoView {
    let name = "Team";
    let abs_path = ABS_APP_USERS_PATH;
    view! {
        <SideNavItem name abs_path sidebar_visible>
            <UsersIcon />
        </SideNavItem>
    }
}

#[component]
fn SettingsNavItem(sidebar_visible: RwSignal<bool>) -> impl IntoView {
    let name = "Settings";
    let abs_path = ABS_APP_SETTINGS_PATH;
    view! {
        <SideNavItem name abs_path sidebar_visible>
            <Cog6ToothIcon attr:class="w-6 h-6 shrink-0" />
        </SideNavItem>
    }
}

#[component]
fn SideNavItem(
    name: &'static str,
    abs_path: &'static str,
    sidebar_visible: RwSignal<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let location = use_location();
    let class = Signal::derive(move || {
        let path = location.pathname.get();
        if path == abs_path {
            "group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6 text-white bg-gray-800"
        } else {
            "group flex gap-x-3 rounded-md p-2 text-sm font-semibold leading-6 text-gray-400 hover:text-white hover:bg-gray-800"
        }
    });

    let on_click = move |_| sidebar_visible.set(false);

    view! {
        <RouteLink path=abs_path on:click=on_click class=class>
            {children()}
            {name}
        </RouteLink>
    }
}
