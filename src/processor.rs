use crate::models::*;
use crate::Article;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct BlogProcessor {
    pub blog_dir: String,
}

impl BlogProcessor {
    pub fn new(blog_dir: String) -> Self {
        Self { blog_dir }
    }

    pub fn update_category_meta(
        &self,
        category: &str,
        article: &Article,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let meta_path = Path::new(&self.blog_dir).join(category).join("meta.json");

        let mut articles: Vec<Article> = if meta_path.exists() {
            let content = fs::read_to_string(&meta_path)?;
            serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
        } else {
            Vec::new()
        };

        let article_filename = Path::new(&article.path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(&article.path);

        if let Some(existing_idx) = articles.iter().position(|a| {
            let existing_filename = Path::new(&a.path)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or(&a.path);
            existing_filename == article_filename
        }) {
            articles[existing_idx] = article.clone();
        } else {
            articles.push(article.clone());
        }

        articles.sort_by(|a, b| b.date.cmp(&a.date));

        let json_content = serde_json::to_string_pretty(&articles)?;
        fs::write(&meta_path, json_content)?;

        Ok(())
    }

    pub fn update_root_json_files(
        &self,
        article: &Article,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.update_root_file("latest.json", article, 5)?;
        self.update_root_file("recommended.json", article, 5)?;
        self.update_notebooks_json()?;
        self.update_index_json(article)?;

        Ok(())
    }

    fn update_root_file(
        &self,
        filename: &str,
        article: &Article,
        max_count: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = Path::new(&self.blog_dir).join(filename);

        let mut articles: Vec<Article> = if file_path.exists() {
            let content = fs::read_to_string(&file_path)?;
            serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
        } else {
            Vec::new()
        };

        let article_filename = Path::new(&article.path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(&article.path);

        if let Some(existing_idx) = articles.iter().position(|a| {
            let existing_filename = Path::new(&a.path)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or(&a.path);
            existing_filename == article_filename
        }) {
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

    fn update_index_json(&self, article: &Article) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = Path::new(&self.blog_dir).join("index.json");

        let mut index_data: IndexJson = if file_path.exists() {
            let content = fs::read_to_string(&file_path)?;
            serde_json::from_str(&content)?
        } else {
            IndexJson {
                meta: MetaInfo {
                    title: "Gerald's Blog".to_string(),
                },
                list: Vec::new(),
            }
        };

        let article_filename = Path::new(&article.path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(&article.path);

        if let Some(existing_idx) = index_data.list.iter().position(|a| {
            let existing_filename = Path::new(&a.path)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or(&a.path);
            existing_filename == article_filename
        }) {
            index_data.list[existing_idx] = article.clone();
        } else {
            index_data.list.push(article.clone());
        }

        index_data.list.sort_by(|a, b| b.date.cmp(&a.date));

        let json_content = serde_json::to_string_pretty(&index_data)?;
        fs::write(&file_path, json_content)?;

        Ok(())
    }

    fn update_notebooks_json(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = Path::new(&self.blog_dir).join("notebooks.json");

        let mut notebooks = Vec::new();
        let blog_path = Path::new(&self.blog_dir);

        for entry in WalkDir::new(blog_path)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_dir() {
                let dir_name = entry.file_name().to_string_lossy();
                if dir_name != "images" {
                    let dir_path = entry.path();
                    let has_md_files = dir_path
                        .read_dir()
                        .map(|entries| {
                            entries.filter_map(|e| e.ok()).any(|entry| {
                                entry
                                    .path()
                                    .extension()
                                    .map(|ext| ext == "md")
                                    .unwrap_or(false)
                            })
                        })
                        .unwrap_or(false);

                    if has_md_files {
                        let notebook = NotebookEntry {
                            id: notebooks.len(),
                            title: capitalize_first(dir_name.as_ref()),
                            remark: format!("{} articles", capitalize_first(dir_name.as_ref())),
                            dir: dir_name.to_string(),
                        };
                        notebooks.push(notebook);
                    }
                }
            }
        }

        notebooks.sort_by(|a, b| a.id.cmp(&b.id));

        let json_content = serde_json::to_string_pretty(&notebooks)?;
        fs::write(&file_path, json_content)?;

        Ok(())
    }

    pub fn process_directory(&self) -> Result<(), Box<dyn std::error::Error>> {
        let blog_path = Path::new(&self.blog_dir);
        let mut all_articles = Vec::new();

        for entry in WalkDir::new(blog_path) {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
                println!("Processing file: {:?}", path);

                let content = fs::read_to_string(path)?;
                if let Some((mut article, _)) = Article::extract_frontmatter(&content) {
                    if let Some(_relative_path) =
                        path.strip_prefix(blog_path).ok().and_then(|p| p.to_str())
                    {
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

                        println!("  - Title: {}", article.title);
                        println!("  - Date: {}", article.date);
                        println!("  - Path: {}", article.path);

                        let article_filename = Path::new(&article.path)
                            .file_name()
                            .and_then(|name| name.to_str())
                            .unwrap_or(&article.path);

                        if !all_articles.iter().any(|a: &Article| {
                            let existing_filename = Path::new(&a.path)
                                .file_name()
                                .and_then(|name| name.to_str())
                                .unwrap_or(&a.path);
                            existing_filename == article_filename
                        }) {
                            all_articles.push(article.clone());
                        }
                    }
                }
            }
        }

        for article in &all_articles {
            if let Some(category) = &article.collection {
                self.update_category_meta(category, &article)?;
            }
            self.update_root_json_files(&article)?;
        }

        Ok(())
    }
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
