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
