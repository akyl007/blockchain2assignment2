use crate::models::{NewsArticle, NewsResponse};
use reqwest::Client;
use std::error::Error;
use chrono::{DateTime, Utc, NaiveDateTime};

pub struct NewsFetcher {
    client: Client,
    crypto_news_api_key: String,
    coin_gecko_api_key: String,
}

impl NewsFetcher {
    pub fn new(crypto_news_api_key: String, coin_gecko_api_key: String) -> Self {
        Self {
            client: Client::new(),
            crypto_news_api_key,
            coin_gecko_api_key,
        }
    }

    pub async fn fetch_news(&self, crypto_symbol: &str) -> Result<NewsResponse, Box<dyn Error>> {
        let mut articles = Vec::new();
        
        // CryptoNews API
        if let Ok(crypto_news) = self.fetch_crypto_news(crypto_symbol).await {
            articles.extend(crypto_news);
        }
        
        // CoinGecko API
        if let Ok(coin_gecko_news) = self.fetch_coin_gecko_news(crypto_symbol).await {
            articles.extend(coin_gecko_news);
        }

        Ok(NewsResponse {
            articles,
            error: None,
        })
    }
    
    async fn fetch_crypto_news(&self, crypto_symbol: &str) -> Result<Vec<NewsArticle>, Box<dyn Error>> {
        let url = format!(
            "https://min-api.cryptocompare.com/data/v2/news/?lang=EN&categories={}",
            crypto_symbol
        );
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Apikey {}", self.crypto_news_api_key))
            .send()
            .await?;
        
        let json: serde_json::Value = response.json().await?;
        
        let mut articles = Vec::new();
        if let Some(data) = json.get("Data") {
            if let Some(items) = data.as_array() {
                for item in items {
                    let timestamp = item["published_on"].as_i64().unwrap_or(0);
                    let naive_datetime = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap_or_default();
                    let datetime = DateTime::<Utc>::from_utc(naive_datetime, Utc);
                    
                    articles.push(NewsArticle {
                        title: item["title"].as_str().unwrap_or("").to_string(),
                        source: item["source"].as_str().unwrap_or("").to_string(),
                        published_at: datetime,
                        description: item["body"].as_str().unwrap_or("").to_string(),
                        url: item["url"].as_str().unwrap_or("").to_string(),
                        sentiment: None,
                    });
                }
            }
        }
        
        Ok(articles)
    }

    async fn fetch_coin_gecko_news(&self, crypto_symbol: &str) -> Result<Vec<NewsArticle>, Box<dyn Error>> {
        let url = format!(
            "https://api.coingecko.com/api/v3/coins/{}/status_updates",
            crypto_symbol.to_lowercase()
        );
        
        let response = self.client
            .get(&url)
            .header("x-cg-pro-api-key", &self.coin_gecko_api_key)
            .send()
            .await?;
        
        let json: serde_json::Value = response.json().await?;
        
        let mut articles = Vec::new();
        if let Some(statuses) = json.get("status_updates") {
            if let Some(items) = statuses.as_array() {
                for item in items {
                    let timestamp = item["created_at"].as_str()
                        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|| Utc::now());
                    
                    articles.push(NewsArticle {
                        title: item["description"].as_str().unwrap_or("").to_string(),
                        source: "CoinGecko".to_string(),
                        published_at: timestamp,
                        description: item["description"].as_str().unwrap_or("").to_string(),
                        url: format!("https://www.coingecko.com/en/coins/{}", crypto_symbol.to_lowercase()),
                        sentiment: None,
                    });
                }
            }
        }
        
        Ok(articles)
    }
} 