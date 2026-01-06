use reqwest;
use serde_json::Value;
use std::collections::BTreeMap;
use crate::exchanges::api_client::ApiClient;
use crate::exchanges::binance::structs::BinanceResponseTicker;
use crate::structs as common_structs;

mod structs;

const KLINE: &str = "https://fapi.binance.com/fapi/v1/klines?";
const SYMBOLS: &str = "https://fapi.binance.com/fapi/v1/ticker/24hr?";

pub struct BinanceFuturesApiClient{
    client: reqwest::blocking::Client,
}

impl BinanceFuturesApiClient {
    pub fn new() -> Self{
        Self{
            client: reqwest::blocking::Client::new()
        }
    }

    fn build_request(&self, url: &str, parameters: BTreeMap<&str, &String>) -> String{
        let mut url = String::from(url);
        for (key, value) in parameters{
            url.push_str(format!("{key}={value}&").as_ref());
        }
        url
    }

    fn get_interval(&self, interval: &String) -> String{
        match interval.clone().as_str(){
            "1" => "1m".to_string(),
            "5" => "5m".to_string(),
            "15" => "15m".to_string(),
            "30" => "30m".to_string(),
            "60" => "1h".to_string(),
            _ => panic!("Unknown interval")
        }
    }
}

impl ApiClient for BinanceFuturesApiClient{
    fn get_klines(&self, symbol: &String, interval: &String, limit: Option<&String>) -> Vec<common_structs::Kline> {
        let interval = self.get_interval(interval);
        let mut parameters = BTreeMap::from([
            ("symbol", symbol),
            ("interval", &interval),
        ]);
        if let Some(lt) = limit{
            parameters.insert("limit", lt);
        }
        
        let url = self.build_request(KLINE, parameters);
        let resp = self.client.get(url).send().unwrap().text().unwrap();
        let v: Vec<Value> = serde_json::from_str(&resp).unwrap();
        v.into_iter().map(|kline| common_structs::Kline{
            open_price: kline[1].as_str().unwrap().parse::<f32>().unwrap(),
            close_price: kline[4].as_str().unwrap().parse::<f32>().unwrap(),
            volume: kline[5].as_str().unwrap().parse::<f32>().unwrap(),
        }).collect()
    }
    
    fn get_symbols(&self) -> Vec<common_structs::Ticker>{
        let resp = self.client.get(SYMBOLS).send().unwrap().text().unwrap();
        let v: Value = serde_json::from_str(&resp).unwrap();
        let response: Vec<BinanceResponseTicker> = serde_json::from_value(v.clone()).unwrap();
        response.into_iter().map(|ticker| common_structs::Ticker{
            symbol: ticker.symbol,
            volume: ticker.count
        }).collect()
    }
}
