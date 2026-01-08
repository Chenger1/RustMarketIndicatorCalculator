use crate::structs;

pub trait ApiClient{
    async fn get_klines(&self, symbol: &String, interval: &String, limit: Option<&String>) -> Vec<structs::Kline>;
    async fn get_symbols(&self) -> Vec<structs::Ticker>;
}
