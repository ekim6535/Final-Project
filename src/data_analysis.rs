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
    pub fn find_price_trends(data: &[HouseRecord]) -> HashMap<&'static str, f64> {
        let avg_price_per_sqm = data
            .iter()
            .map(|r| r.price / r.net_sqm)
            .sum::<f64>()
            / data.len() as f64;

        HashMap::from([
            ("average_price_per_sqm", avg_price_per_sqm),
        ])
    }
    pub fn correlation_analysis(data: &[HouseRecord]) -> HashMap<&'static str, f64> {
        let total_houses = data.len() as f64;
        let avg_price = data.iter().map(|r| r.price).sum::<f64>() / total_houses;
        let avg_sqm = data.iter().map(|r| r.net_sqm).sum::<f64>() / total_houses;

        let covariance = data.iter()
            .map(|r| (r.price - avg_price) * (r.net_sqm - avg_sqm))
            .sum::<f64>()
            / total_houses;

        let price_variance = data.iter()
            .map(|r| (r.price - avg_price).powi(2))
            .sum::<f64>()
            / total_houses;

        let sqm_variance = data.iter()
            .map(|r| (r.net_sqm - avg_sqm).powi(2))
            .sum::<f64>()
            / total_houses;

        let correlation = covariance / (price_variance.sqrt() * sqm_variance.sqrt());

        HashMap::from([
            ("price_sqm_correlation", correlation),
        ])
    }
}

#[test]
fn test_calculate_statistics() {
    let data = vec![
        HouseRecord { price: 300_000.0, bedrooms: 3, bathrooms: 2, sqft_living: 1500, sqft_lot: 5000, floors: 1.0 },
        HouseRecord { price: 450_000.0, bedrooms: 4, bathrooms: 3, sqft_living: 2000, sqft_lot: 6000, floors: 2.0 },
    ];

    let stats = data_analysis::calculate_statistics(&data);
    assert!(stats["average_price"] > 0.0);
}

#[test]
fn test_find_price_trends() {
    let data = vec![
        HouseRecord { price: 300_000.0, bedrooms: 3, bathrooms: 2, sqft_living: 1500, sqft_lot: 5000, floors: 1.0 },
        HouseRecord { price: 450_000.0, bedrooms: 4, bathrooms: 3, sqft_living: 2000, sqft_lot: 6000, floors: 2.0 },
    ];

    let trends = data_analysis::find_price_trends(&data);
    assert!(trends["average_price_per_sqft"] > 0.0);
}

