use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct IndexJson {
    pub meta: MetaInfo,
    pub list: Vec<crate::Article>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MetaInfo {
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NotebookEntry {
    pub id: usize,
    pub title: String,
    pub subtitle: String,
    pub remark: String,
    pub path: String,
}
