use axum::{extract, handler::get, response::Html, AddExtensionLayer, Router};
use std::fs;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

// TODO: add error handlings
// TODO: add logging
// TODO: add env vars handling (for tracing and address)

mod appconfig;
mod render;

use appconfig::AppConfig;

#[tokio::main]
async fn main() {
    // temporary, will be changed in the future
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "greenhorn=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();

    let read_conf: AppConfig = toml::from_str(&fs::read_to_string("Config.toml").unwrap()).unwrap();

    let shared_state = Arc::new(read_conf);

    // Application routes + middleware
    let app = Router::new()
        .route("/", get(get_home))
        .route("/:page", get(get_page))
        .route("/:list/:page", get(get_list_page))
        .layer(AddExtensionLayer::new(shared_state))
        .layer(TraceLayer::new_for_http());

    // Run the app
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_home(state: extract::Extension<Arc<AppConfig>>) -> Html<String> {
    let state: Arc<AppConfig> = state.0;
    Html(state.generate_homepage())
}

async fn get_page(
    extract::Path(page): extract::Path<String>,
    state: extract::Extension<Arc<AppConfig>>,
) -> Html<String> {
    let state: Arc<AppConfig> = state.0;

    Html(state.generate_page(&page))
}

async fn get_list_page(
    extract::Path((list, page)): extract::Path<(String, String)>,
    state: extract::Extension<Arc<AppConfig>>,
) -> Html<String> {
    let state: Arc<AppConfig> = state.0;

    Html(state.generate_list_page(&list, &page))
}
