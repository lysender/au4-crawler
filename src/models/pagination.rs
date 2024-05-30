use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PaginationMeta {
    pub page: u32,
    pub per_page: u32,
    pub total_records: u32,
    pub total_pages: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PaginationResult<T> {
    pub meta: PaginationMeta,
    pub data: Vec<T>,
}
