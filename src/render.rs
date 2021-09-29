use serde::Serialize;
use std::collections::HashMap;
use tinytemplate::TinyTemplate;

use crate::appconfig::{AppConfig, Page};
use crate::error::GhError;
use pulldown_cmark::{html, Options, Parser};

/// Variables for generating page from a template
#[derive(Debug, Serialize)]
struct PageContext<'a> {
    title: &'a str,
    css: &'a str,
    body: &'a str,
}

/// Variables for generating pages list
#[derive(Debug, Serialize)]
struct ListContext<'a> {
    link: &'a str,
    display: String,
}

/// Renders a markdown file, single or page from a list
pub async fn render_page(app_state: &AppConfig, page: &Page) -> Result<String, GhError> {
    let (html_template, css, page_html) = tokio::try_join!(
        tokio::fs::read_to_string(&app_state.html_template),
        tokio::fs::read_to_string(&app_state.css),
        markdown_to_html(&page.source),
    )?;

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

/// Renders a list of pages (List kind)
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

    // Generate list HTML
    tt.add_template("list", &list_template)?;

    let list_ctx: Vec<ListContext> = paths
        .iter()
        .map(|p| ListContext {
            link: p,
            display: format_page_name(p),
        })
        .collect();

    let mut list_ser = HashMap::new();
    list_ser.insert("list", list_ctx);
    let rendered_list = tt.render("list", &list_ser)?;

    // Generate full page HTML
    let page_ctx = PageContext {
        title: &page.name,
        css: &css,
        body: &rendered_list,
    };

    tt.add_template("list_page", &html_template)?;

    let ret = tt.render("list_page", &page_ctx)?;

    Ok(ret)
}

async fn markdown_to_html(path: &std::path::Path) -> Result<String, std::io::Error> {
    let input = tokio::fs::read_to_string(path).await?;

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(&input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    Ok(html_output)
}

/// Removes all '_' and everything after a '.'
fn format_page_name(page: &str) -> String {
    let mut new = page.replace("_", " ");

    if let Some(index) = new.find('.') {
        new.truncate(index);
    }

    new
}
