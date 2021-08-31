use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use tinytemplate::TinyTemplate;

use crate::appconfig::{AppConfig, Page};

#[derive(Debug, Serialize)]
struct PageContext<'a> {
    title: &'a str,
    css: &'a str,
    body: &'a str,
}

pub fn render_page(app_state: &AppConfig, page: &Page) -> String {
    let mut tt = TinyTemplate::new();

    let html_template = fs::read_to_string(&app_state.html_template).unwrap();
    let page_html = markdown::file_to_html(std::path::Path::new(&page.source)).unwrap();
    let css = fs::read_to_string(&app_state.css).unwrap();

    let ctx = PageContext {
        title: &page.name,
        css: &css,
        body: &page_html,
    };

    tt.set_default_formatter(&tinytemplate::format_unescaped);
    tt.add_template("normal_page", &html_template).unwrap();

    tt.render("normal_page", &ctx).unwrap()
}

pub fn render_list(app_state: &AppConfig, page: &Page, paths: &[String]) -> String {
    let mut tt = TinyTemplate::new();

    let list_template = fs::read_to_string(&app_state.list_template).unwrap();
    let html_template = fs::read_to_string(&app_state.html_template).unwrap();
    let css = fs::read_to_string(&app_state.css).unwrap();

    tt.set_default_formatter(&tinytemplate::format_unescaped);

    tt.add_template("list", &list_template).unwrap();

    let mut list_ser = HashMap::new();
    list_ser.insert("list", paths);

    let rendered_list = tt.render("list", &list_ser).unwrap();

    let ctx = PageContext {
        title: &page.name,
        css: &css,
        body: &rendered_list,
    };

    tt.add_template("list_page", &html_template).unwrap();

    tt.render("list_page", &ctx).unwrap()
}
