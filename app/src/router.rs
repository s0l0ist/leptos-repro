use crate::{
    common::{
        error_template::{AppError, ErrorTemplate},
        profile::{UserProfile, get_user_profile},
        route_paths::*,
        utils::make_url,
    },
    components::*,
    pages::*,
};
use leptos::either::Either;
use leptos::tachys::html::class::IntoClass;
use leptos::{prelude::*, reactive::wrappers::write::SignalSetter};
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::*;

#[derive(Default, Clone, Debug)]
struct GlobalState {
    pub user_profile: Option<UserProfile>,
}

// Create aliases for `use_context` or `expect_context`.
pub type ReadUserProfile = Signal<Option<UserProfile>>;
pub type WriteUserProfile = SignalSetter<Option<UserProfile>>;

#[component(transparent)]
pub fn AppRouter() -> impl IntoView {
    let state = RwSignal::new(GlobalState::default());
    let (user_profile, set_user_profile) = create_slice(
        state,
        |state| state.user_profile.clone(),
        |state, new_profile| state.user_profile = new_profile,
    );

    provide_context(user_profile);
    provide_context(set_user_profile);

    // Create our server actions related to authentication. We also combine any
    // action that can update the user's profile because the authentication
    // control is gated on the user's profile data and keeps everything related
    // in a single place.
    let logout = ServerAction::<Logout>::new();
    let login = ServerAction::<Login>::new();

    let _ = Effect::watch(
        move || logout.value().get(),
        move |res, _, _| {
            if let Some(res) = res {
                match res {
                    Ok(_) => set_user_profile.set(None),
                    Err(_) => set_user_profile.set(None),
                }
            };
        },
        false,
    );
    let _ = Effect::watch(
        move || login.value().get(),
        move |res, _, _| {
            if let Some(res) = res {
                match res {
                    Ok(res) => set_user_profile.set(Some(res.profile.clone())),
                    Err(_) => set_user_profile.set(None),
                }
            };
        },
        false,
    );

    view! {
        <Router>
            <Routes fallback=|| {
                let mut outside_errors = Errors::default();
                outside_errors.insert_with_default_key(AppError::NotFound);
                view! { <ErrorTemplate outside_errors /> }
            }>
                <PublicRoutesWithMainHeaderAndFooter />
                <PublicAuthRoutes login />
                <PrivateRoutes logout set_user_profile />
            </Routes>
        </Router>
    }
}

#[component(transparent)]
fn PublicRoutesWithMainHeaderAndFooter() -> impl MatchNestedRoutes + Clone {
    view! {
        // Define the section that requires headers/footers
        <ParentRoute path=path!("/") view=WithHeaderFooter>
            <Route
                path=path!("")
                view=|| {
                    let title = "Bug Reproduction";
                    let desc = "A minimal reproduction";
                    let url = make_url("", "");
                    view! {
                        <TitleAndDescription title desc />
                        <Link rel="canonical" href=url />
                        <Landing />
                    }
                }
            />

        </ParentRoute>
    }
    .into_inner()
}

#[component(transparent)]
fn PublicAuthRoutes(login: ServerAction<Login>) -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/auth") view=Outlet>

            <Route
                path=path!("login")
                view=move || {
                    let title = "Log In";
                    let desc = "Log into your account.";
                    let url = make_url(ABS_AUTH_LOGIN_PATH, "");
                    view! {
                        <TitleAndDescription title desc />
                        <Link rel="canonical" href=url />
                        <LoginView action=login />
                    }
                }
            />

        </ParentRoute>
    }
    .into_inner()
}

#[component(transparent)]
fn PrivateRoutes(
    logout: ServerAction<Logout>,
    set_user_profile: SignalSetter<Option<UserProfile>>,
) -> impl MatchNestedRoutes + Clone {
    // Define routes where the user must be logged in.
    view! {
        <Route
            path=path!("/app")
            view=move || {
                view! {
                    <LoggedIn
                        set_user_profile
                        fallback=|| {
                            view! {
                                <TitleAndDescription title="Unauthorized" desc="Unauthorized" />
                                <main class="flex relative justify-center items-center h-screen isolate">
                                    <Page401 />
                                </main>
                            }
                        }
                    >
                        <WithSideNav action=logout>
                            <p>"test"</p>
                        </WithSideNav>
                    </LoggedIn>
                }
            }
        ></Route>
    }.into_inner()
}

#[component]
pub fn LoggedIn<F, IV>(
    set_user_profile: WriteUserProfile,
    fallback: F,
    children: ChildrenFn,
) -> impl IntoView
where
    F: Fn() -> IV + Send + Sync + 'static,
    IV: IntoView + 'static,
{
    // By reading the users profile here, we can avoid the `Page401` component
    // from rendering briefly after a user logs into the application. It happens
    // so quickly, that the rendering isn't shown; however, the effect to update
    // the URL query occurs and the `next` parameter gets populated.
    let user_profile = expect_context::<ReadUserProfile>();

    // While the server already has the user's profile data from the session, we
    // need to rehydrate the client with that information. Therefore, we use a
    // resource to load it. This means it will run once again on the server, but
    // it is very fast since it just accesses the struct field.
    let resource_user_profile = Resource::new(|| (), |_| async { get_user_profile().await });

    let is_authed = Signal::derive(move || match resource_user_profile.get() {
        Some(res) => match res {
            Ok(user_profile) => {
                set_user_profile.set(Some(user_profile));
                true
            }
            _ => {
                set_user_profile.set(None);
                false
            }
        },
        _ => false,
    });

    // let fallback = StoredValue::new(fallback);
    // let children = StoredValue::new(children);

    view! {
        <Suspense fallback=AppLoader>
            <Show
                when=move || { is_authed.get() || user_profile.get().is_some() }
                fallback=move || fallback()
            >
                {children()}
            </Show>
        </Suspense>
    }
}

#[component]
pub fn WithSideNav(action: ServerAction<Logout>, children: ChildrenFn) -> impl IntoView {
    let sidebar_visible = RwSignal::new(false);
    let set_sidebar_close = move |_| sidebar_visible.set(false);
    let set_sidebar_open = move |_| sidebar_visible.set(true);

    view! {
        <div class="flex flex-col h-screen">
            <Show when=move || sidebar_visible.get() fallback=move || view! {}>
                <p>"Sidenav"</p>
            </Show>
            // <!-- Off-canvas menu for mobile, show/hide based on off-canvas menu state. -->
            <div class="relative z-50 lg:hidden" role="dialog" aria-modal="true">
                <div class="fixed inset-0 bg-gray-900/80" aria-hidden="true"></div>
                <div class="flex fixed inset-0">
                    <div class="flex relative flex-1 mr-16 w-full max-w-xs">
                        <div class="flex absolute top-0 left-full justify-center pt-5 w-16">
                            <button on:click=set_sidebar_close type="button" class="p-2.5 -m-2.5">
                                <span class="sr-only">Close sidebar</span>
                                <XMarkIcon attr:class="w-6 h-6 text-white" />
                            </button>
                        </div>

                        // <!-- Sidebar component -->
                        <div class="flex overflow-y-auto flex-col gap-y-5 px-6 pb-4 bg-gray-900 ring-1 grow ring-white/10">
                            <MenuItems sidebar_visible />
                        </div>
                    </div>
                </div>
            </div>
            // </Show>

            // <!-- Static sidebar for desktop -->
            <div class="hidden lg:flex lg:fixed lg:inset-y-0 lg:z-50 lg:flex-col lg:w-72">
                // <!-- Sidebar component -->
                <div class="flex overflow-y-auto flex-col gap-y-5 px-6 pb-4 bg-gray-900 grow">
                    <MenuItems sidebar_visible />
                </div>
            </div>
            <div class="flex flex-col flex-grow lg:pl-72">
                <div class="flex sticky top-0 z-40 gap-x-4 items-center px-4 h-16 bg-white border-b border-gray-200 sm:gap-x-6 sm:px-6 lg:px-8 shadow-xs shrink-0">
                    <button
                        on:click=set_sidebar_open
                        type="button"
                        class="p-2.5 -m-2.5 text-gray-700 lg:hidden"
                    >
                        <span class="sr-only">Open sidebar</span>
                        <Bars3Icon />
                    </button>

                    // <!-- Separator -->
                    <div class="w-px h-6 lg:hidden bg-gray-900/10" aria-hidden="true"></div>

                    <div class="flex flex-1 gap-x-4 justify-end self-stretch lg:gap-x-6">
                        <div class="flex gap-x-4 items-center lg:gap-x-6">
                            <ProfileDropdown action />
                        </div>
                    </div>
                </div>

                <div class="flex flex-col flex-grow">{children()}</div>
            </div>
        </div>
    }
}

#[component]
pub fn AppLoader() -> impl IntoView {
    view! {
        <main class="flex flex relative flex-col justify-center items-center w-full min-h-screen bg-white isolate">
            <div class="inline-flex flex-grow justify-center items-center text-sm font-semibold text-indigo-500 rounded-md">
                <LoaderIcon attr:class="w-14 h-14 animate-spin" />
            </div>
        </main>
    }
}

#[component]
fn WithHeaderFooter() -> impl IntoView {
    view! {
        <div class="flex flex-col min-h-screen">
            <Header />
            <div class="flex-grow">
                <Outlet />
            </div>
            <Footer />
        </div>
    }
}

#[component]
pub fn RouteLink<T, C>(
    path: T,
    #[prop(optional)] class: Option<C>,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(default = false)] open_new_tab: bool,
) -> impl IntoView
where
    T: ToHref + Send + Sync + 'static,
    C: IntoClass + 'static,
    Option<C>: leptos::attr::AttributeValue,
{
    if open_new_tab {
        Either::Left(view! {
            <A href=path attr:class=class target="_blank">
                {children.map(|c| c())}
            </A>
        })
    } else {
        Either::Right(view! {
            <A href=path attr:class=class>
                {children.map(|c| c())}
            </A>
        })
    }
}
