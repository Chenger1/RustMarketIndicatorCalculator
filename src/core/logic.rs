use crate::structs::Kline;

pub struct IndicatorsCalculator{}

impl IndicatorsCalculator{
    pub fn calculate_rsi(arr: &Vec<Kline>) -> f32{
        let mut green = 0.0;
        let mut red = 0.0;
        for kline in arr.iter(){
            let length = (kline.close_price - kline.open_price).abs();
            if kline.close_price > kline.open_price{
                green += length;
            }else{
                red += length;
            }
        }
        let rs = green / red;
        let rsi = 100.0 - 100.0 / (1.0 + rs);
        if rsi.is_nan(){
            return 0.0;
        }
        rsi
    }

    pub fn calculate_price(arr: &Vec<Kline>) -> f32{
        arr.last().unwrap().close_price
    }
}
