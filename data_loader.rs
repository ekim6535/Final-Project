pub mod data_loader {
    use csv::Reader;
    use serde::Deserialize;
    use std::error::Error;

    #[derive(Debug, Deserialize)]
    pub struct HouseRecord {
        pub price: f64,
        pub bedrooms: u32,
        pub bathrooms: u32,
        pub sqft_living: u32,
        pub sqft_lot: u32,
        pub floors: u32,
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
