use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use tinytemplate::TinyTemplate;

use crate::appconfig::{AppConfig, Page};
use crate::error::GhError;

#[derive(Debug, Serialize)]
struct PageContext<'a> {
    title: &'a str,
    css: &'a str,
    body: &'a str,
}

#[tracing::instrument]
pub fn render_page(app_state: &AppConfig, page: &Page) -> Result<String, GhError> {
    tracing::event!(tracing::Level::TRACE, "Start");
    let mut tt = TinyTemplate::new();

    let html_template = fs::read_to_string(&app_state.html_template)?;
    let page_html = markdown::file_to_html(std::path::Path::new(&page.source))
        .map_err(|_| GhError::PageNotFound())?;
    let css = fs::read_to_string(&app_state.css)?;

    let ctx = PageContext {
        title: &page.name,
        css: &css,
        body: &page_html,
    };

    tt.set_default_formatter(&tinytemplate::format_unescaped);
    tt.add_template("normal_page", &html_template)?;

    let ret = tt.render("normal_page", &ctx)?;

    tracing::event!(tracing::Level::TRACE, "Stop");

    Ok(ret)
}

#[tracing::instrument]
pub fn render_list(
    app_state: &AppConfig,
    page: &Page,
    paths: &[String],
) -> Result<String, GhError> {
    tracing::event!(tracing::Level::TRACE, "Start");
    let mut tt = TinyTemplate::new();

    let list_template = fs::read_to_string(&app_state.list_template)?;
    let html_template = fs::read_to_string(&app_state.html_template)?;
    let css = fs::read_to_string(&app_state.css)?;

    tt.set_default_formatter(&tinytemplate::format_unescaped);

    tt.add_template("list", &list_template)?;

    let mut list_ser = HashMap::new();
    list_ser.insert("list", paths);

    let rendered_list = tt.render("list", &list_ser)?;

    let ctx = PageContext {
        title: &page.name,
        css: &css,
        body: &rendered_list,
    };

    tt.add_template("list_page", &html_template)?;

    let ret = tt.render("list_page", &ctx)?;

    tracing::event!(tracing::Level::TRACE, "Stop");

    Ok(ret)
}
