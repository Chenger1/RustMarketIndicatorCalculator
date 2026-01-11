use tokio::fs::File;
use tokio::io::AsyncWriteExt;
// use std::io::BufReader;
use crate::storage::storage_trait::Storage;
use crate::structs::Indicator;

pub struct JsonStorage{
    filename: &'static str
}

impl JsonStorage{
    pub fn new(filename: &'static str) -> Self{
        JsonStorage{
            filename: filename
        }
    }
}

impl Storage for JsonStorage{
    // fn get_data(&self) -> Vec<Indicator>{
    //     // Add error handling
    //     let file = File::open(self.filename).unwrap();
    //     let buffer = BufReader::new(file);
    //     serde_json::from_reader(buffer).unwrap()
    // }

    async fn write_data(&mut self, indicators: Vec<Indicator>) {
        let mut file = File::create(self.filename).await.unwrap();
        let json_content = serde_json::to_string_pretty(&indicators).unwrap();
        file.write_all(json_content.as_bytes()).await.unwrap();
        file.flush().await.unwrap();
    }
}