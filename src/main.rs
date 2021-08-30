use axum::{extract, handler::get, response::Html, AddExtensionLayer, Router};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tinytemplate::TinyTemplate;

// TODO: split into files
// TODO: add error handling
// TODO: add logging

#[derive(Debug, Deserialize, Serialize)]
struct AppConfig {
    homepage: String,
    html_template: String,
    list_template: String,
    css: String,
    pages: Vec<Page>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Page {
    name: String,
    kind: PageKind,
    source: PathBuf,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
enum PageKind {
    Single,
    List,
}

#[derive(Debug, Serialize)]
struct PageContext<'a> {
    title: &'a str,
    css: &'a str,
    body: &'a str,
}

#[tokio::main]
async fn main() {
    let read_conf: AppConfig = toml::from_str(&fs::read_to_string("Config.toml").unwrap()).unwrap();

    let shared_state = Arc::new(read_conf);

    // Application routes + middleware
    let app = Router::new()
        .route("/", get(get_home))
        .route("/:page", get(get_page))
        .route("/:list/:page", get(get_list_page))
        .layer(AddExtensionLayer::new(shared_state));

    // Run the app
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

impl AppConfig {
    fn render_page(&self, page_name: &str) -> String {
        let html_template = fs::read_to_string(&self.html_template).unwrap();

        let page = self.pages.iter().find(|&p| p.name == page_name);
        let page = match page {
            Some(p) => p,
            None => return "Not found".to_string(),
        };

        match page.kind {
            PageKind::Single => {
                let page_html = markdown::file_to_html(std::path::Path::new(&page.source)).unwrap();
                let ctx = PageContext {
                    title: &page.name,
                    css: &self.css,
                    body: &page_html,
                };

                let mut tt = TinyTemplate::new();
                tt.set_default_formatter(&tinytemplate::format_unescaped);
                tt.add_template("normal_page", &html_template).unwrap();

                tt.render("normal_page", &ctx).unwrap()
            }
            PageKind::List => {
                let contents = fs::read_dir(&page.source).unwrap();

                // Get directory contents, include only files (not directories)
                let paths: Vec<String> = contents
                    .map(|p| p.unwrap().path())
                    .filter(|p| p.is_file())
                    .map(|p| p.file_name().unwrap().to_str().unwrap().to_string())
                    .collect();

                let list_template = fs::read_to_string(&self.list_template).unwrap();

                let mut tt = TinyTemplate::new();
                tt.set_default_formatter(&tinytemplate::format_unescaped);

                tt.add_template("list", &list_template).unwrap();

                let mut list_ser = HashMap::new();
                list_ser.insert("list", paths);

                let rendered_list = tt.render("list", &list_ser).unwrap();

                let ctx = PageContext {
                    title: &page.name,
                    css: &self.css,
                    body: &rendered_list,
                };

                tt.add_template("list_page", &html_template).unwrap();

                tt.render("list_page", &ctx).unwrap()
            }
        }
    }

    fn render_homepage(&self) -> String {
        self.render_page(&self.homepage)
    }

    fn render_list_page(&self, list_name: &str, page_name: &str) -> String {
        // TODO: this should be extracted to function
        let html_template = fs::read_to_string(&self.html_template).unwrap();

        let page = self.pages.iter().find(|&p| p.name == list_name);
        let mut page = match page {
            Some(p) => p.clone(),
            None => return "Not found".to_string(),
        };

        page.source.push(page_name);

        let page_html = markdown::file_to_html(std::path::Path::new(&page.source)).unwrap();
        let ctx = PageContext {
            title: &page.name,
            css: &self.css,
            body: &page_html,
        };

        let mut tt = TinyTemplate::new();
        tt.set_default_formatter(&tinytemplate::format_unescaped);
        tt.add_template("normal_page", &html_template).unwrap();

        tt.render("normal_page", &ctx).unwrap()
    }
}

async fn get_home(state: extract::Extension<Arc<AppConfig>>) -> Html<String> {
    let state: Arc<AppConfig> = state.0;
    Html(state.render_homepage())
}

async fn get_page(
    extract::Path(page): extract::Path<String>,
    state: extract::Extension<Arc<AppConfig>>,
) -> Html<String> {
    let state: Arc<AppConfig> = state.0;

    Html(state.render_page(&page))
}

async fn get_list_page(
    extract::Path((list, page)): extract::Path<(String, String)>,
    state: extract::Extension<Arc<AppConfig>>,
) -> Html<String> {
    let state: Arc<AppConfig> = state.0;

    Html(state.render_list_page(&list, &page))
}
