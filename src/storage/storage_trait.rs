use crate::structs::Indicator;

pub trait Storage{
    async fn write_data(&mut self, indicators: Vec<Indicator>);
}
