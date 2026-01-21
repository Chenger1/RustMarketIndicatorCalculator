pub mod sql;

use crate::entity::{symbols};
use crate::structs::Indicator;

pub trait Storage{
    async fn save_indicators(&self, indicators: Vec<Indicator>);
    async fn create_exchange(&self, title: &String) -> i32;
    async fn create_symbols(&self, symbols: Vec<String>, exchange_id: i32);

    async fn get_symbols(&self, exchange_id: i32) -> Vec<symbols::Model>;
}
