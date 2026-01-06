use reqwest;
use serde_json::Value;
use crate::exchange::api_client::ApiClient;
use crate::structs as common_structs;

const SYMBOLS: &str = "https://fapi.binance.com/fapi/v1/ticker/24hr";

pub struct BinanceApiClient{
    client: reqwest::blocking::Client,
}

impl BinanceApiClient {
    pub fn new() -> Self{
        Self{
            client: reqwest::blocking::Client::new()
        }
    }
}

impl ApiClient for BinanceApiClient{
    fn get_symbols(&self) -> Vec<common_structs::Ticker>{
        let resp = self.client.get(SYMBOLS).send().unwrap().text().unwrap();
        let v: Value = serde_json::from_str(&resp).unwrap();
        serde_json::from_value(v).clone().unwrap()
    }
}
