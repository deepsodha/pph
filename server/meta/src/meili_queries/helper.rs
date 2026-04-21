use actix_web::{web, HttpResponse};
use common::dto::search_query::SearchQuery;
use dotenv::var;

use meilisearch_sdk::SearchResults;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct SerializableSearchResults<T> {
    hits: Vec<T>,
    offset: usize,
    limit: usize,
    query: Option<String>,
}

impl<T> From<SearchResults<T>> for SerializableSearchResults<T> {
    fn from(results: SearchResults<T>) -> Self {
        let hits = results
            .hits
            .into_iter()
            .map(|search_result| search_result.result)
            .collect();

        SerializableSearchResults {
            hits,
            offset: results.offset.unwrap_or(0),
            limit: results.limit.unwrap_or(0),
            query: Some(results.query),
        }
    }
}

pub async fn search<'a, T>(index_name: &'a str, query: web::Json<SearchQuery>) -> HttpResponse
where
    T: serde::Serialize + Clone + for<'de> Deserialize<'de> + 'a + 'static,
{
    let api_key = var("MEILI_MASTER_KEY").expect("MEILI_MASTER_KEY must be set");
    let client = meilisearch_sdk::Client::new("http://localhost:7700", Some(api_key));
    let max_limit = std::usize::MAX;
    let results: SearchResults<T> = match client
        .index(index_name)
        .search()
        .with_query(&query.query)
        .with_limit(max_limit)
        .execute()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            log::error!("Meilisearch query failed for index '{}': {}", index_name, e);
            return HttpResponse::ServiceUnavailable().body("Search service unavailable");
        }
    };

    let serializable_results: SerializableSearchResults<T> = results.into();

    match serde_json::to_string(&serializable_results) {
        Ok(json) => HttpResponse::Ok().body(json),
        Err(e) => {
            log::error!("Failed to serialize search results: {}", e);
            HttpResponse::InternalServerError().body("Failed to serialize results")
        }
    }
}
