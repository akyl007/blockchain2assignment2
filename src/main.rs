mod models;
mod api;
mod web;

use std::sync::Arc;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let crypto_news_api_key = env::var("CRYPTO_NEWS_API_KEY")
        .expect("CRYPTO_NEWS_API_KEY не установлен в .env файле");
    
    let coin_gecko_api_key = env::var("COIN_GECKO_API_KEY")
        .expect("COIN_GECKO_API_KEY не установлен в .env файле");

    let fetcher = Arc::new(api::NewsFetcher::new(
        crypto_news_api_key,
        coin_gecko_api_key,
    ));

    web::start_server(fetcher).await;
} 