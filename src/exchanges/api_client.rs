use crate::structs;

#[async_trait::async_trait]
pub trait ApiClient: Send + Sync{
    async fn get_klines(&self, symbol: &String, interval: &String, limit: Option<&String>) -> Vec<structs::Kline>;
    async fn get_symbols(&self) -> Vec<String>;
}
