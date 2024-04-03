use sea_orm::{ConnectionTrait, Paginator, SelectorTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Paginate {
    pub page: u64,
    pub size: u64,
}

impl Default for Paginate {
    fn default() -> Self {
        Self { page: 1, size: 10 }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationResponse<T> {
    pub items: Vec<T>,
    // next_page_url: Option<String>,
    // previous_page_url: Option<String>,
    pub record_count: u64,
    pub page_count: u64,
    pub current_page: u64,
}

pub async fn get_paginated_result<'db, C, S, T>(
    paginator: Paginator<'db, C, S>,
    current_page: u64,
) -> PaginationResponse<T>
where
    C: ConnectionTrait,
    S: SelectorTrait<Item = T> + 'db,
{
    let record_count = paginator.num_items().await.unwrap();
    let page_count = paginator.num_pages().await.unwrap();

    let items = paginator
        .fetch_page(current_page - 1)
        .await
        .unwrap_or_else(|_| Vec::new());

    PaginationResponse {
        items,
        record_count,
        page_count,
        current_page,
    }
}
