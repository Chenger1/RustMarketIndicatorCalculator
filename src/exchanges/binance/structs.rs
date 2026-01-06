use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct BinanceResponseTicker{
    pub symbol: String,
    pub count: f32
}
