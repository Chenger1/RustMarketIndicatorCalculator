
use serde:: {Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Kline{
    pub open_price: f32,
    pub close_price: f32,
    pub volume: f32
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Ticker{
    pub symbol: String,
    pub volume: f32
}

#[derive(Clone)]
pub struct Symbol{
    pub symbol: String,
    pub interval: String
}


#[derive (Serialize, Deserialize)]
pub struct Indicator{
    pub symbol: String,
    pub interval: String,
    pub value: f32
}