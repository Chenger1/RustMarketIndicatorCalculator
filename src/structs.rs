use serde:: {Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Kline{
    pub open_price: f32,
    pub close_price: f32,
    pub volume: f32
}


pub struct Indicator{
    pub symbol_id: i32,
    // pub symbol: String,
    // pub interval: String,
    pub value: f32,
}