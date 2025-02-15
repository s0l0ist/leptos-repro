use crate::fileserv::file_and_error_handler;
use app::{App, shell};
use axum::{
    Router,
    body::Body,
    extract::{FromRef, Request, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use core::net::SocketAddr;
use leptos::prelude::*;
use leptos_axum::{
    AxumRouteListing, LeptosRoutes, generate_route_list, handle_server_fns_with_context,
};
use tower::ServiceBuilder;
use tower_http::{catch_panic::CatchPanicLayer, compression::CompressionLayer, trace::TraceLayer};

/// This takes advantage of Axum's `SubStates` feature by deriving `FromRef`. This
/// is the only way to have more than one item in Axum's State. Leptos requires
/// you to have leptosOptions in your State struct for the leptos route handlers
///
/// NOTE: The only shared state should be related to connections and other
/// metadata such as configurations. We cannot use a local state because we're
/// running a distributed system.
#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    /// The leptos options used in the application
    pub leptos_options: LeptosOptions,

    /// The list of available leptos routes
    pub routes: Vec<AxumRouteListing>,
}

pub async fn build_service() -> (Router, SocketAddr) {
    let conf = get_configuration(Some("Cargo.toml")).unwrap();
    let addr = conf.leptos_options.site_addr;

    let routes = generate_route_list(App);

    let app_state = AppState {
        leptos_options: conf.leptos_options,
        routes: routes.clone(),
    };

    // build our application with a route
    let router = Router::new()
        .route(
            "/api/*fn_name",
            get(server_fn_handler).post(server_fn_handler),
        )
        .route("/api/auth/*fn_name", post(server_fn_handler))
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        .with_state(app_state)
        .layer(CompressionLayer::new())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CatchPanicLayer::new()),
        )
        .route("/api/health", get(health_check));
    (router, addr)
}

#[tracing::instrument(skip_all)]
async fn server_fn_handler(
    State(app_state): State<AppState>,
    request: Request<Body>,
) -> impl IntoResponse {
    handle_server_fns_with_context(
        move || {
            provide_context(app_state.clone());
        },
        request,
    )
    .await
    .into_response()
}

#[tracing::instrument(skip_all)]
async fn leptos_routes_handler(
    state: State<AppState>,
    request: Request<Body>,
) -> axum::response::Response {
    let handler = {
        let State(app_state) = state.clone();
        let app_state = app_state.clone();
        let leptos_options = app_state.leptos_options.clone();
        leptos_axum::render_route_with_context(
            app_state.routes.clone(),
            move || {
                provide_context(app_state.clone());
            },
            move || shell(leptos_options.clone()),
        )
    };

    handler(state, request).await.into_response()
}

/// A simple endpoint to determine if the server is running
pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
