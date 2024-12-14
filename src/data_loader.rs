use csv::Reader;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize, Clone)]
pub struct HouseRecord {
    pub bedroom_count: u32,     // Number of bedrooms
    pub net_sqm: f64,           // Total usable interior space (in square meters)
    pub center_distance: f64,   // Distance from the city center (in kilometers)
    pub metro_distance: f64,    // Distance from the nearest metro or bus station (in kilometers)
    pub floor: i32,             // Specific floor (-1 for basement, 0 for ground floor, etc.)
    pub age: u32,               // Age of the property (in years)
    pub price: f64,             // Price of the property
}

pub fn load_data(file_path: &str) -> Result<Vec<HouseRecord>, Box<dyn Error>> {
    let mut reader = Reader::from_path(file_path)?;
    let mut records = Vec::new();

    for result in reader.deserialize() {
        let record: HouseRecord = result?;
        records.push(record);
    }

    Ok(records)
}
