use reqwest;
use serde_json::Value;
use std::collections::BTreeMap;
use crate::structs as common_structs;
use crate::exchanges::api_client::ApiClient;
use structs::{BybitKlineResponse, BybitResponseTicker};

const KLINE: &str = "https://api.bybit.com/v5/market/kline?";
const SYMBOLS: &str = "https://api.bybit.com/v5/market/tickers?";

mod structs;

pub struct BybitApiClient{
    client: reqwest::blocking::Client,
    category: String
}

impl BybitApiClient{
    pub fn new(category: String) -> Self{
        Self{
            client: reqwest::blocking::Client::new(),
            category: category
        }
    }

    fn build_request(&self, url: &str, parameters: BTreeMap<&str, &String>) -> String{
        let mut url = String::from(url);
        for (key, value) in parameters{
            url.push_str(format!("{key}={value}&").as_ref());
        }
        url
    }
}

impl ApiClient for BybitApiClient{    
    fn get_klines(&self, symbol: &String, interval: &String, limit: Option<&String>) -> Vec<common_structs::Kline>{
        let mut parameters = BTreeMap::from([
            ("category", &self.category),
            ("symbol", symbol),
            ("interval", interval),
        ]);
        if let Some(lt) = limit{
            parameters.insert("limit", lt);
        }

        let url = self.build_request(KLINE, parameters);
        let resp = self.client.get(url).send().unwrap().text().unwrap();
        let v: Value = serde_json::from_str(&resp).unwrap();
        let kline_response: Vec<BybitKlineResponse> = serde_json::from_value(v["result"]["list"].clone()).unwrap();
        kline_response.into_iter().map(|kline| common_structs::Kline{
            close_price: kline.close_price.parse().unwrap(),
            open_price: kline.open_price.parse().unwrap(),
            volume: kline.volume.parse().unwrap()
        }).collect()
    }

    fn get_symbols(&self) -> Vec<common_structs::Ticker>{
        let parameters = BTreeMap::from([
            ("category", &self.category)
        ]);
        let url = self.build_request(SYMBOLS, parameters);
        let resp = self.client.get(url).send().unwrap().text().unwrap();
        let v: Value = serde_json::from_str(&resp).unwrap();
        let response: Vec<BybitResponseTicker> = serde_json::from_value(v["result"]["list"].clone()).unwrap();
        response.into_iter().map(|ticker| common_structs::Ticker{
            symbol: ticker.symbol,
            volume: ticker.volume24h.parse().unwrap()
        }).collect()
    }
}