use serde::Serialize;
use std::collections::HashMap;
use tinytemplate::TinyTemplate;

use crate::appconfig::{AppConfig, Page};
use crate::error::GhError;

#[derive(Debug, Serialize)]
struct PageContext<'a> {
    title: &'a str,
    css: &'a str,
    body: &'a str,
}

pub async fn render_page(app_state: &AppConfig, page: &Page) -> Result<String, GhError> {
    let (html_template, css) = tokio::try_join!(
        tokio::fs::read_to_string(&app_state.html_template),
        tokio::fs::read_to_string(&app_state.css)
    )?;

    let page_html = markdown::file_to_html(std::path::Path::new(&page.source))
        .map_err(|_| GhError::PageNotFound())?;

    let ctx = PageContext {
        title: &page.name,
        css: &css,
        body: &page_html,
    };

    let mut tt = TinyTemplate::new();
    tt.set_default_formatter(&tinytemplate::format_unescaped);
    tt.add_template("normal_page", &html_template)?;

    let ret = tt.render("normal_page", &ctx)?;

    Ok(ret)
}

pub async fn render_list(
    app_state: &AppConfig,
    page: &Page,
    paths: &[String],
) -> Result<String, GhError> {
    let (list_template, html_template, css) = tokio::try_join!(
        tokio::fs::read_to_string(&app_state.list_template),
        tokio::fs::read_to_string(&app_state.html_template),
        tokio::fs::read_to_string(&app_state.css)
    )?;

    let mut tt = TinyTemplate::new();

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

    Ok(ret)
}
