use crate::{
    common::{
        route_paths::{ABS_APP_PROFILE_PATH, ABS_AUTH_LOGIN_PATH},
        utils::{get_initials, sanitize_id},
    },
    components::{create_click, logout::Logout, ChevronDownIcon},
    router::ReadUserProfile,
};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use leptos_router::location::State;
use leptos_router::NavigateOptions;

#[component]
pub fn ProfileDropdown(action: ServerAction<Logout>) -> impl IntoView {
    let show = RwSignal::new(false);

    let options = vec![
        ("Your Profile", {
            Callback::new(move |_| {
                let navigate = use_navigate();
                navigate(
                    ABS_APP_PROFILE_PATH,
                    NavigateOptions {
                        resolve: false,
                        replace: false,
                        scroll: false,
                        state: State::new(None),
                    },
                );
            })
        }),
        ("Sign Out", {
            Callback::new(move |_| {
                // Async log out
                action.dispatch(Logout {});

                let navigate = use_navigate();
                navigate(
                    ABS_AUTH_LOGIN_PATH,
                    NavigateOptions {
                        resolve: false,
                        replace: false,
                        scroll: true,
                        state: State::new(None),
                    },
                );
            })
        }),
    ];

    view! {
        <div class="hidden lg:block lg:w-px lg:h-6 lg:bg-gray-900/10" aria-hidden="true"></div>
        <div class="relative">
            <ProfileDropdownButton show />
            <Show when=move || show.get()>
                <ProfileDropdownUL options=options.clone() />
            </Show>
        </div>
    }
}

#[component]
fn ProfileDropdownButton(show: RwSignal<bool>) -> impl IntoView {
    let sr_desc = "Open user menu";
    let profile = expect_context::<ReadUserProfile>();
    let username = Signal::derive(move || profile.get().map(|u| u.user_name));
    let initials = Signal::derive(move || username.get().map(|username| get_initials(&username)));
    let button_ref = create_click(move || show.set(false));

    view! {
        <button
            type="button"
            node_ref=button_ref
            class="flex items-center p-1.5 -m-1.5"
            id="user-menu-button"
            aria-expanded="false"
            aria-haspopup="true"
            on:click=move |_| show.update(|u| *u = !*u)
        >
            <span class="sr-only">{sr_desc}</span>
            <span class="inline-flex justify-center items-center w-8 h-8 bg-gray-500 rounded-full">
                <span class="text-sm font-medium leading-none text-white">{initials}</span>
            </span>
            <span class="hidden lg:flex lg:items-center">
                <Show when=move || profile.get().is_some()>
                    <Show when=move || username.get().is_some_and(|name| !name.is_empty())>
                        <span
                            class="ml-4 text-sm font-semibold leading-6 text-gray-900"
                            aria-hidden="true"
                        >
                            {username}
                        </span>
                    </Show>
                    <ChevronDownIcon attr:class="ml-2 size-5 text-gray-400" />
                </Show>
            </span>
        </button>
    }
}

#[component]
fn ProfileDropdownUL(
    /// A list containing, an `id`, a type `T` to return in a callback, and the
    /// callback, `cb` to invoke when a selection is made.
    options: Vec<(&'static str, Callback<()>)>,
) -> impl IntoView {
    view! {
        <ul
            class="overflow-auto absolute right-0 z-10 mt-2 w-max bg-white rounded-md divide-y divide-gray-200 ring-1 ring-black/5 shadow-lg origin-top-right focus:outline-none"
            tabindex="-1"
            role="listbox"
            aria-orientation="vertical"
            aria-activedescendant="listbox-options"
        >
            {options
                .into_iter()
                .map(|(id, cb)| {
                    view! { <ProfileDropdownLI id cb /> }
                })
                .collect_view()}
        </ul>
    }
}

#[component]
fn ProfileDropdownLI(id: &'static str, cb: Callback<()>) -> impl IntoView {
    let entered = RwSignal::new(false);
    let on_mouseenter = move |_| entered.set(true);
    let on_mouseleave = move |_| entered.set(false);
    let li_class = Signal::derive(move || {
        let style = if entered.get() {
            "bg-indigo-600 text-white"
        } else {
            "text-gray-900"
        };
        format!("{style} cursor-pointer select-none p-4 text-sm")
    });
    let on_click = move |_| cb.run(());
    view! {
        <li
            on:mouseenter=on_mouseenter
            on:mouseleave=on_mouseleave
            on:click=on_click
            class=li_class
            id=format!("listbox-item-{}", sanitize_id(id))
            role="listboxitem"
        >
            {id}
        </li>
    }
}
