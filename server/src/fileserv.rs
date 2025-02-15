use crate::service::AppState;
use app::shell;
use axum::{
    body::Body,
    extract::State,
    http::{Request, Response, StatusCode, Uri},
    response::{IntoResponse, Response as AxumResponse},
};
use http::{HeaderValue, header::CACHE_CONTROL};
use leptos::prelude::*;
use tower::ServiceExt;
use tower_http::services::ServeDir;

pub async fn file_and_error_handler(
    uri: Uri,
    State(app_state): State<AppState>,
    request: Request<Body>,
) -> AxumResponse {
    let root = &app_state.leptos_options.site_root;
    let mut res = get_static_file(&uri, root).await.unwrap();

    if res.status() == StatusCode::OK {
        // These routes are configured in the leptos config in Cargo.toml.
        if uri.path().starts_with("/pkg/") || uri.path().starts_with("/static/") {
            res.headers_mut().insert(
                CACHE_CONTROL,
                HeaderValue::from_static("public, max-age=31536000, immutable"),
            );
        }
        res.into_response()
    } else {
        let handler = {
            let leptos_options = app_state.leptos_options.clone();
            leptos_axum::render_app_to_stream_with_context(
                move || {
                    // provide_context(auth_session.clone());
                    provide_context(app_state.clone());
                },
                move || shell(leptos_options.clone()),
            )
        };

        handler(request).await.into_response()
    }
}

async fn get_static_file(uri: &Uri, root: &str) -> Result<Response<Body>, (StatusCode, String)> {
    let req = Request::builder().uri(uri).body(Body::empty()).unwrap();
    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // This path is relative to the cargo root
    match ServeDir::new(root).oneshot(req).await {
        Ok(res) => Ok(res.into_response()),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {err}"),
        )),
    }
}
