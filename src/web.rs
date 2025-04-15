use warp::{Filter, Rejection, Reply};
use crate::models::NewsResponse;
use crate::api::NewsFetcher;
use std::sync::Arc;

pub async fn start_server(fetcher: Arc<NewsFetcher>) {
    let fetcher = warp::any().map(move || fetcher.clone());

    let search = warp::path!("search" / String)
        .and(fetcher.clone())
        .and_then(handle_search);

    let index = warp::path::end()
        .and(warp::get())
        .map(|| {
            warp::reply::html(include_str!("../templates/index.html"))
        });

    let routes = index.or(search);

    println!("Сервер запущен на http://localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn handle_search(
    crypto_symbol: String,
    fetcher: Arc<NewsFetcher>,
) -> Result<impl Reply, Rejection> {
    match fetcher.fetch_news(&crypto_symbol).await {
        Ok(response) => Ok(warp::reply::json(&response)),
        Err(e) => Ok(warp::reply::json(&NewsResponse {
            articles: Vec::new(),
            error: Some(e.to_string()),
        })),
    }
} 