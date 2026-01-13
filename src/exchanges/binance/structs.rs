use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct BinanceResponseTicker{
    pub symbol: String,
    pub count: f32
}
