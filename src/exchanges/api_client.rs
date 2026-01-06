use crate::structs;

pub trait ApiClient{
    fn get_klines(&self, symbol: &String, interval: &String, limit: Option<&String>) -> Vec<structs::Kline>;
    fn get_symbols(&self) -> Vec<structs::Ticker>;
}

pub enum ExchangeEnum{
    BybitFutures,
    BybitSpot,
    BinanceFutures
}