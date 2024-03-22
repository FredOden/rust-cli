// Dépendances externes à ajouter dans Cargo.toml
// reqwest = "0.11"
// serde = { version = "1.0", features = ["derive"] }
// tokio = { version = "1", features = ["full"] }

use reqwest;
use serde::{Deserialize, Serialize};
use std::error::Error;

// Structure pour désérialiser la réponse de l'API.
#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    data: Vec<StockData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StockData {
    symbol: String,
    name: String,
    close: f64,
    // Ajoutez d'autres champs selon les données fournies par l'API.
}

// Structure pour gérer la configuration de l'API Marketstack.
pub struct MarketstackApi {
    base_url: String,
    api_key: String,
}

impl MarketstackApi {
    pub fn new(api_key: String) -> Self {
        MarketstackApi {
            base_url: "http://api.marketstack.com/v1/".to_string(),
            api_key,
        }
    }

    // Fonction pour récupérer les données financières d'un symbole boursier spécifique.
    pub async fn get_financial_data(&self, symbol: &str) -> Result<StockData, Box<dyn Error>> {
        let url = format!("{}eod?access_key={}&symbols={}", self.base_url, self.api_key, symbol);

        let response = reqwest::get(&url).await?.json::<ApiResponse>().await?;

        // Supposons que l'API renvoie toujours au moins un élément.
        Ok(response.data.into_iter().next().unwrap())
    }
}

/*
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = "votre_clé_api"; // Remplacez par votre clé API Marketstack.
    let symbol = "AAPL"; // Remplacez par le symbole boursier que vous souhaitez interroger.

    let api = MarketstackApi::new(api_key.to_string());
    let financial_data = api.get_financial_data(symbol).await?;

    println!("{:?}", financial_data);

    Ok(())
}
*/

