//! Radio Browser integration for ttspotify-rs
//! Supports direct online radio station searching via radio-browser.info API.

use anyhow::{anyhow, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct StationInfo {
    pub name: String,
    pub url_resolved: String,
    pub countrycode: Option<String>,
    pub bitrate: Option<u32>,
}

/// Cari stasiun radio berdasarkan nama menggunakan Radio Browser API
pub async fn search_station(query: &str) -> Result<StationInfo> {
    let clean_query = query.trim();
    if clean_query.is_empty() {
        return Err(anyhow!("Nama stasiun radio tidak boleh kosong"));
    }

    let endpoint = format!(
        "https://de1.api.radio-browser.info/json/stations/byname/{}",
        clean_query
    );

    let client = reqwest::Client::builder()
        .user_agent("TeamTalkRadioBot/1.0")
        .timeout(std::time::Duration::from_secs(8))
        .build()?;

    let resp = client
        .get(&endpoint)
        .query(&[("limit", "1")])
        .send()
        .await?;

    let stations: Vec<StationInfo> = resp.json().await?;

    stations
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("Stasiun radio '{}' tidak ditemukan", clean_query))
}
