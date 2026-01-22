use serde:: {Serialize, Deserialize};
use crate::entity::indicators::IndicatorType;

#[derive(Serialize, Deserialize)]
pub struct Kline{
    pub open_price: f32,
    pub close_price: f32,
    pub volume: f32
}


pub struct Indicator{
    pub symbol_id: i32,
    // pub interval: String,
    pub value: f32,
    pub indicator_type: IndicatorType
}