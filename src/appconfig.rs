use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub homepage: String,
    pub html_template: String,
    pub list_template: String,
    pub css: String,
    pub pages: Vec<Page>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Page {
    pub name: String,
    kind: PageKind,
    pub source: PathBuf,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
enum PageKind {
    Single,
    List,
}

use crate::render::{render_list, render_page};

impl AppConfig {
    pub fn generate_page(&self, page_name: &str) -> String {
        let page = match self.pages.iter().find(|&p| p.name == page_name) {
            Some(p) => p,
            None => return "Not found".to_string(),
        };

        match page.kind {
            PageKind::Single => render_page(self, page),
            PageKind::List => {
                let contents = fs::read_dir(&page.source).unwrap();

                // Get directory contents, include only files (not directories)
                let paths: Vec<String> = contents
                    .map(|p| p.unwrap().path())
                    .filter(|p| p.is_file())
                    .map(|p| p.file_name().unwrap().to_str().unwrap().to_string())
                    .collect();

                render_list(self, page, &paths)
            }
        }
    }

    pub fn generate_homepage(&self) -> String {
        self.generate_page(&self.homepage)
    }

    pub fn generate_list_page(&self, list_name: &str, page_name: &str) -> String {
        let mut page = match self.pages.iter().find(|&p| p.name == list_name) {
            Some(p) => p.clone(),
            None => return "Not found".to_string(),
        };

        page.source.push(page_name);

        render_page(self, &page)
    }
}
