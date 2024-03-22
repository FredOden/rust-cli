// Ajoutez ces dépendances dans votre Cargo.toml
// reqwest = "0.11"
// serde = { version = "1.0", features = ["derive"] }
// tokio = { version = "1", features = ["full"] }

use std::collections::HashMap;
use reqwest;
use serde::{Deserialize, Serialize};
use std::error::Error;
// Structure pour désérialiser la réponse de l'API.
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse {
    // Utilisez les champs appropriés selon la réponse de l'API Alpha Vantage.
    // Exemple:
    #[serde(rename = "Time Series (Daily)")]
    time_series_daily: std::collections::HashMap<String, DailyData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DailyData {
    #[serde(rename = "1. open")]
    open : String,
    #[serde(rename = "2. high")]
    high : String,
    #[serde(rename = "3. low")]
    low : String,
    #[serde(rename = "4. close")]
    close : String,
    // Ajoutez d'autres champs selon les données fournies par l'API.
}


// Structure pour gérer la configuration de l'API Alpha Vantage.
pub struct AlphaVantageApi {
    base_url: String,
    api_key: String,
}

impl AlphaVantageApi {
    pub fn new(api_key: String) -> Self {
        AlphaVantageApi {
            base_url: "https://www.alphavantage.co/query?".to_string(),
            api_key,
        }
    }

    // Fonction pour récupérer les données financières d'un symbole boursier spécifique.
    pub async fn get(&self, symbol: &str) -> Result<ApiResponse, Box<dyn Error>> {
        let url = format!("{}function=TIME_SERIES_DAILY&symbol={}&apikey={}", self.base_url, symbol, self.api_key);

        //let response = reqwest::get(&url).await?.json::<ApiResponse>().await?;
        let response = reqwest::get(&url).await?.json::<ApiResponse>().await?;

        Ok(response)
    }
}

/*

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = "votre_clé_api"; // Remplacez par votre clé API Alpha Vantage.
    let symbol = "AAPL"; // Remplacez par le symbole boursier que vous souhaitez interroger.

    let api = AlphaVantageApi::new(api_key.to_string());
    let financial_data = api.get_financial_data(symbol).await?;

    println!("{:?}", financial_data);

    Ok(())
}
*/
