use crate::data_loader::HouseRecord;
use std::collections::HashMap;
pub mod data_analysis {
    pub fn calculate_statistics(data: &[HouseRecord]) -> HashMap<&'static str, f64> {
        let total_houses = data.len() as f64;

        let avg_price = data.iter().map(|r| r.price).sum::<f64>() / total_houses;
        let avg_bedrooms = data.iter().map(|r| r.bedroom_count).sum::<u32>() as f64 / total_houses;
        let avg_age = data.iter().map(|r| r.age).sum::<u32>() as f64 / total_houses;

        HashMap::from([
            ("average_price", avg_price),
            ("average_bedrooms", avg_bedrooms),
            ("average_age", avg_age),
        ])
    }
