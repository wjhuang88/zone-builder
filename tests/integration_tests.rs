use serde::{Deserialize, Serialize};
use std::fs;
use tempfile::TempDir;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Article {
    title: String,
    date: String,
    update: String,
    summary: String,
    path: String,
    #[serde(default)]
    collection: Option<String>,
}

#[derive(Debug)]
struct BlogProcessor {
    blog_dir: String,
}

impl BlogProcessor {
    fn new(blog_dir: String) -> Self {
        Self { blog_dir }
    }

    fn extract_frontmatter(content: &str) -> Option<(Article, &str)> {
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

    fn update_category_meta(
        &self,
        category: &str,
        article: &Article,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let meta_path = std::path::Path::new(&self.blog_dir)
            .join(category)
            .join("meta.json");

        let mut articles: Vec<Article> = if meta_path.exists() {
            let content = fs::read_to_string(&meta_path)?;
            serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
        } else {
            Vec::new()
        };

        if let Some(existing_idx) = articles.iter().position(|a| a.path == article.path) {
            articles[existing_idx] = article.clone();
        } else {
            articles.push(article.clone());
        }

        articles.sort_by(|a, b| b.date.cmp(&a.date));

        let json_content = serde_json::to_string_pretty(&articles)?;
        fs::write(&meta_path, json_content)?;

        Ok(())
    }

    fn update_root_json_files(&self, article: &Article) -> Result<(), Box<dyn std::error::Error>> {
        self.update_root_file("latest.json", article, 10)?;
        self.update_root_file("recommended.json", article, usize::MAX)?;
        self.update_root_file("notebooks.json", article, usize::MAX)?;
        self.update_root_file("index.json", article, usize::MAX)?;

        Ok(())
    }

    fn update_root_file(
        &self,
        filename: &str,
        article: &Article,
        max_count: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = std::path::Path::new(&self.blog_dir).join(filename);

        let mut articles: Vec<Article> = if file_path.exists() {
            let content = fs::read_to_string(&file_path)?;
            serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
        } else {
            Vec::new()
        };

        if let Some(existing_idx) = articles.iter().position(|a| a.path == article.path) {
            articles[existing_idx] = article.clone();
        } else {
            articles.push(article.clone());
        }

        articles.sort_by(|a, b| b.date.cmp(&a.date));

        if max_count != usize::MAX && articles.len() > max_count {
            articles.truncate(max_count);
        }

        let json_content = serde_json::to_string_pretty(&articles)?;
        fs::write(&file_path, json_content)?;

        Ok(())
    }

    fn process_directory(&self) -> Result<(), Box<dyn std::error::Error>> {
        let blog_path = std::path::Path::new(&self.blog_dir);

        for entry in walkdir::WalkDir::new(blog_path) {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
                let content = fs::read_to_string(path)?;
                if let Some((mut article, _)) = Self::extract_frontmatter(&content) {
                    if let Some(relative_path) =
                        path.strip_prefix(blog_path).ok().and_then(|p| p.to_str())
                    {
                        article.path = relative_path.to_string();

                        if article.collection.is_none() {
                            if let Some(parent_dir) = path
                                .parent()
                                .and_then(|p| p.file_name())
                                .and_then(|n| n.to_str())
                            {
                                if parent_dir != "images" {
                                    article.collection = Some(parent_dir.to_string());
                                }
                            }
                        }

                        if let Some(ref category) = article.collection {
                            self.update_category_meta(category, &article)?;
                        }

                        self.update_root_json_files(&article)?;
                    }
                }
            }
        }

        Ok(())
    }
}

#[test]
fn test_extract_frontmatter() {
    let content = r#"+++
title = "Test Article"
date = "2023-01-01"
update = "2023-01-02"
summary = "This is a test article"
path = "test-article.md"
+++

This is the content of the test article."#;

    if let Some((article, content_part)) = BlogProcessor::extract_frontmatter(content) {
        assert_eq!(article.title, "Test Article");
        assert_eq!(article.date, "2023-01-01");
        assert_eq!(article.update, "2023-01-02");
        assert_eq!(article.summary, "This is a test article");
        assert_eq!(article.path, "test-article.md");
        assert!(content_part.contains("This is the content"));
    } else {
        panic!("Failed to extract frontmatter");
    }
}

#[test]
fn test_extract_frontmatter_invalid_format() {
    let content = r#"---
title = "Invalid Format"
date = "2023-01-01"
---
Content"#;

    let result = BlogProcessor::extract_frontmatter(content);
    assert!(result.is_none());
}

#[test]
fn test_update_category_meta() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let blog_path = temp_dir.path().to_str().unwrap().to_string();

    let category_dir = temp_dir.path().join("test-category");
    fs::create_dir(&category_dir).expect("Failed to create category dir");

    let article = Article {
        title: "Test Article".to_string(),
        date: "2023-01-01".to_string(),
        update: "2023-01-02".to_string(),
        summary: "Test summary".to_string(),
        path: "test-article.md".to_string(),
        collection: Some("test-category".to_string()),
    };

    let processor = BlogProcessor::new(blog_path);
    processor
        .update_category_meta("test-category", &article)
        .expect("Failed to update category meta");

    let meta_path = category_dir.join("meta.json");
    assert!(meta_path.exists());

    let content = fs::read_to_string(&meta_path).expect("Failed to read meta.json");
    let articles: Vec<Article> = serde_json::from_str(&content).expect("Failed to parse JSON");
    assert_eq!(articles.len(), 1);
    assert_eq!(articles[0].title, "Test Article");
}

#[test]
fn test_update_root_json_files() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let blog_path = temp_dir.path().to_str().unwrap().to_string();

    let article = Article {
        title: "Test Article".to_string(),
        date: "2023-01-01".to_string(),
        update: "2023-01-02".to_string(),
        summary: "Test summary".to_string(),
        path: "test-category/test-article.md".to_string(),
        collection: Some("test-category".to_string()),
    };

    let processor = BlogProcessor::new(blog_path);
    processor
        .update_root_json_files(&article)
        .expect("Failed to update root JSON files");

    let files_to_check = [
        "latest.json",
        "recommended.json",
        "notebooks.json",
        "index.json",
    ];
    for file_name in &files_to_check {
        let file_path = temp_dir.path().join(file_name);
        assert!(file_path.exists(), "File {} was not created", file_name);

        let content =
            fs::read_to_string(&file_path).expect(&format!("Failed to read {}", file_name));
        let articles: Vec<Article> =
            serde_json::from_str(&content).expect(&format!("Failed to parse {}", file_name));
        assert_eq!(articles.len(), 1);
        assert_eq!(articles[0].title, "Test Article");
    }
}

#[test]
fn test_process_directory() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let blog_path = temp_dir.path().to_str().unwrap().to_string();

    let category_dir = temp_dir.path().join("test-category");
    fs::create_dir(&category_dir).expect("Failed to create category dir");

    let md_content = r#"+++
title = "Test Article"
date = "2023-01-01"
update = "2023-01-02"
summary = "This is a test article"
path = "test-article.md"
+++

This is the content of the test article."#;

    let md_path = category_dir.join("test-article.md");
    fs::write(&md_path, md_content).expect("Failed to write test markdown file");

    let processor = BlogProcessor::new(blog_path);
    processor
        .process_directory()
        .expect("Failed to process directory");

    let meta_path = category_dir.join("meta.json");
    assert!(meta_path.exists());

    let meta_content = fs::read_to_string(&meta_path).expect("Failed to read meta.json");
    let articles: Vec<Article> =
        serde_json::from_str(&meta_content).expect("Failed to parse meta.json");
    assert_eq!(articles.len(), 1);
    assert_eq!(articles[0].title, "Test Article");

    let root_files = [
        "latest.json",
        "recommended.json",
        "notebooks.json",
        "index.json",
    ];
    for file in &root_files {
        let file_path = temp_dir.path().join(file);
        assert!(file_path.exists(), "{} should exist", file);
    }
}
