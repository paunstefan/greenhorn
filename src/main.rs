//! Personal page application

use axum::{
    extract, handler::get, http::StatusCode, response::Html, service, AddExtensionLayer, Router,
};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use structopt::StructOpt;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{filter, prelude::*};

#[derive(StructOpt, Debug)]
#[structopt(name = "greenhorn")]
struct GhOptions {
    /// Server address and port
    #[structopt(short, long)]
    address: String,

    /// Configuration file (TOML)
    #[structopt(parse(from_os_str), short, long)]
    config_file: PathBuf,
}

use greenhorn::appconfig::AppConfig;
use greenhorn::error::GhError;

#[tokio::main]
async fn main() {
    let options = GhOptions::from_args();

    let filter = filter::Targets::new()
        // Enable the `INFO` level for anything in `my_crate`
        .with_target("greenhorn", tracing::Level::DEBUG)
        .with_target("axum", tracing::Level::DEBUG)
        .with_target("tower_http", tracing::Level::WARN);

    let registry = tracing_subscriber::registry();
    match tracing_journald::layer() {
        Ok(subscriber) => {
            registry.with(subscriber.with_filter(filter)).init();
        }
        Err(e) => {
            registry.init();
            tracing::error!("Couldn't connect to journald: {}", e);
        }
    }

    let read_conf = AppConfig::new(options.config_file);

    let shared_state = Arc::new(read_conf);

    // Application routes + middleware
    let app = Router::new()
        .route("/", get(get_home))
        .route("/:page", get(get_page))
        .route("/:list/:page", get(get_list_page))
        .nest(
            "/static",
            service::get(ServeDir::new(&shared_state.media_dir)).handle_error(
                |error: std::io::Error| {
                    Ok::<_, Infallible>((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    ))
                },
            ),
        )
        .layer(AddExtensionLayer::new(shared_state))
        .layer(TraceLayer::new_for_http().on_body_chunk(()).on_eos(()));

    // Run the app
    let addr: SocketAddr = options.address.parse().expect("Address not valid");
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_home(state: extract::Extension<Arc<AppConfig>>) -> Result<Html<String>, GhError> {
    let state: Arc<AppConfig> = state.0;
    let res = state.generate_homepage().await?;
    tracing::info!("Getting home");

    Ok(Html(res))
}

async fn get_page(
    extract::Path(page): extract::Path<String>,
    state: extract::Extension<Arc<AppConfig>>,
) -> Result<Html<String>, GhError> {
    let state: Arc<AppConfig> = state.0;
    tracing::info!("Generating page: {}", &page);

    let res = state.generate_page(&page).await?;

    Ok(Html(res))
}

async fn get_list_page(
    extract::Path((list, page)): extract::Path<(String, String)>,
    state: extract::Extension<Arc<AppConfig>>,
) -> Result<Html<String>, GhError> {
    let state: Arc<AppConfig> = state.0;
    tracing::info!("Generating list page: {}/{}", &list, &page);

    let res = state.generate_list_page(&list, &page).await?;

    Ok(Html(res))
}
