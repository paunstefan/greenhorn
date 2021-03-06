use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::error::GhError;

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub homepage: String,
    pub html_template: PathBuf,
    pub list_template: PathBuf,
    pub media_dir: PathBuf,
    pub css: PathBuf,
    pub pages: Vec<Page>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Page {
    pub name: String,
    pub kind: PageKind,
    pub source: PathBuf,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum PageKind {
    Single,
    List,
}

use crate::render::{render_list, render_page};

impl AppConfig {
    pub fn new(path: PathBuf) -> Self {
        let config: AppConfig =
            toml::from_str(&fs::read_to_string(path).unwrap()).expect("Couldn't read config");

        if !&config.html_template.exists()
            || !&config.list_template.exists()
            || !&config.css.exists()
            || !&config.media_dir.exists()
        {
            panic!("Template files do not exist.")
        }

        config.pages.iter().for_each(|p| {
            if !&p.source.exists() {
                panic!("Page source \"{}\" does not exist", &p.source.display())
            }
        });

        if !config
            .pages
            .iter()
            .map(|p| &p.name)
            .any(|n| n == &config.homepage)
        {
            panic!("Homepage not part of declared pages")
        }

        config
    }

    /// Generates a first level page
    /// Can be a Single page or a List page
    pub async fn generate_page(&self, page_name: &str) -> Result<String, GhError> {
        // First check if page exists
        let page = match self.pages.iter().find(|&p| p.name == page_name) {
            Some(p) => p,
            None => return Err(GhError::PageNotFound()),
        };

        match page.kind {
            PageKind::Single => render_page(self, page).await,
            PageKind::List => {
                let contents = fs::read_dir(&page.source)?;

                // Get directory contents to list, include only files (not directories)
                let paths: Vec<String> = contents
                    .map(|p| p.unwrap().path())
                    .filter(|p| p.is_file())
                    .map(|p| p.file_name().unwrap().to_str().unwrap().to_string())
                    .collect();

                render_list(self, page, &paths).await
            }
        }
    }

    pub async fn generate_homepage(&self) -> Result<String, GhError> {
        self.generate_page(&self.homepage).await
    }

    /// Generates a 2nd level page from a list
    pub async fn generate_list_page(
        &self,
        list_name: &str,
        page_name: &str,
    ) -> Result<String, GhError> {
        // Check if page exists
        let mut page = match self.pages.iter().find(|&p| p.name == list_name) {
            Some(p) => p.clone(),
            None => return Err(GhError::PageNotFound()),
        };

        page.source.push(page_name);

        render_page(self, &page).await
    }
}
