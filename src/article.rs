use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Article {
    pub title: String,
    pub date: String,
    pub update: String,
    pub summary: String,
    pub path: String,
    #[serde(default)]
    pub collection: Option<String>,
}

impl Article {
    pub fn extract_frontmatter(content: &str) -> Option<(Article, &str)> {
        if content.starts_with("+++") {
            let mut parts = content.split("\n+++\n");
            let frontmatter_part = parts.next()?;
            let content_part = parts.next().unwrap_or("");

            let frontmatter_content = frontmatter_part.trim_start_matches("+++");

            match toml::from_str::<Article>(frontmatter_content) {
                Ok(article) => Some((article, content_part)),
                Err(_) => None,
            }
        } else {
            None
        }
    }
}
