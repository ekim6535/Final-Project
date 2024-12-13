pub mod data_loader {
    use csv::Reader;
    use serde::Deserialize;
    use std::error::Error;

    #[derive(Debug, Deserialize)]
    pub struct HouseRecord {
        pub bedroom_count: u32,     // Number of bedrooms
        pub net_sqm: f64,           // Usable interior space in square meters
        pub center_distance: f64,   // Distance to central city/downtown
        pub metro_distance: f64,    // Distance to nearest subway or bus stop
        pub floor: u32,             // Floor number
        pub age: u32,               // Age of the property
        pub price: f64, 
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
}
