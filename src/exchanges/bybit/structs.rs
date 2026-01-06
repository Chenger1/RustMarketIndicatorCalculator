use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct BybitKlineResponse{
    pub start_time: String,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub close_price: String,
    pub volume: String,
    pub turnover: String
}


#[derive(Serialize, Deserialize, Clone)]
pub struct BybitResponseTicker{
    pub symbol: String,
    pub volume24h: String
}
