use crate::common::route_paths::{ABS_AUTH_LOGIN_PATH, ABS_ROOT_PATH};
use crate::components::{Bars3Icon, XMarkIcon};
use crate::{components::CompanyLogoIcon, router::RouteLink};
use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    let menu_visible = RwSignal::new(false);
    let set_menu_close = move |_| menu_visible.set(false);
    let set_menu_open = move |_| menu_visible.set(true);

    let company_name = "BugRepro";
    let home = "Home";
    let login = "Log In";
    let open_menu = "Open main menu";
    let close_menu = "Close menu";

    view! {
        <header class="absolute inset-x-0 top-0 z-50">
            <nav
                class="flex gap-x-6 justify-between items-center py-6 pr-10 pl-8 mx-auto max-w-7xl"
                aria-label="Global"
            >
                <div class="flex sm:flex-1">
                    <RouteLink path=ABS_ROOT_PATH on:click=set_menu_close class="p-1.5 -m-1.5">
                        <span class="sr-only">{company_name}</span>
                        <CompanyLogoIcon attr:class="h-14 w-auto text-indigo-600" />
                    </RouteLink>
                </div>
                <div class="hidden sm:flex sm:gap-x-12">
                    <RouteLink
                        path=ABS_ROOT_PATH
                        on:click=set_menu_close
                        class="text-sm font-semibold leading-6 text-gray-900"
                    >
                        {home}
                    </RouteLink>
                </div>

                <div class="flex flex-1 gap-x-6 justify-end items-center">
                    <RouteLink
                        path=ABS_AUTH_LOGIN_PATH
                        on:click=set_menu_close
                        class="py-2 px-3 text-sm font-semibold text-white bg-indigo-600 rounded-md hover:bg-indigo-500 shadow-xs focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                    >
                        {login}
                    </RouteLink>

                </div>
                <div class="flex sm:hidden">
                    <button
                        type="button"
                        on:click=set_menu_open
                        class="inline-flex justify-center items-center p-2.5 -m-2.5 text-gray-700 rounded-md"
                    >
                        <span class="sr-only">{open_menu}</span>
                        <Bars3Icon />
                    </button>
                </div>
            </nav>
            <Show when=move || menu_visible.get()>
                <div class="sm:hidden" role="dialog" aria-modal="true">
                    <div class="overflow-y-auto fixed inset-y-0 right-0 z-10 py-6 pr-10 pl-8 w-full bg-white sm:max-w-sm sm:ring-1 sm:ring-gray-900/10">
                        <div class="flex gap-x-6 items-center">
                            <RouteLink
                                path=ABS_ROOT_PATH
                                on:click=set_menu_close
                                class="p-1.5 -m-1.5"
                            >
                                <span class="sr-only">{company_name}</span>
                                <CompanyLogoIcon attr:class="h-14 w-auto text-indigo-600" />
                            </RouteLink>
                            <button
                                type="button"
                                on:click=set_menu_close
                                class="p-2.5 -m-2.5 text-gray-700 rounded-md"
                            >
                                <span class="sr-only">{close_menu}</span>
                                <XMarkIcon attr:class="size-6" />
                            </button>
                        </div>
                        <div class="flow-root mt-6">
                            <div class="-my-6 divide-y divide-gray-500/10">
                                <div class="py-6 space-y-2">
                                    <RouteLink
                                        path=ABS_AUTH_LOGIN_PATH
                                        on:click=set_menu_close
                                        class="block py-2.5 px-3 -mx-3 text-base font-semibold leading-7 text-gray-900 rounded-lg hover:bg-gray-50"
                                    >
                                        {login}
                                    </RouteLink>
                                </div>
                                <div class="py-6">
                                    <RouteLink
                                        path=ABS_ROOT_PATH
                                        on:click=set_menu_close
                                        class="block py-2 px-3 -mx-3 text-base font-semibold leading-7 text-gray-900 rounded-lg hover:bg-gray-50"
                                    >
                                        {home}
                                    </RouteLink>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </Show>
        </header>
    }
}
