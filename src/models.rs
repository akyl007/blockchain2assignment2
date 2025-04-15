use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewsArticle {
    pub title: String,
    pub source: String,
    pub published_at: DateTime<Utc>,
    pub description: String,
    pub url: String,
    pub sentiment: Option<f32>, // Для анализа тональности
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewsResponse {
    pub articles: Vec<NewsArticle>,
    pub error: Option<String>,
} 