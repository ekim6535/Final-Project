use crate::data_loader::HouseRecord; // To access the `HouseRecord` struct
use std::collections::HashMap; // For returning analysis results

pub fn calculate_statistics(data: &[HouseRecord]) -> HashMap<&'static str, f64> {
    let total_houses = data.len() as f64;

    let avg_price = data.iter().map(|r| r.price).sum::<f64>() / total_houses;
    let avg_bedrooms = data.iter().map(|r| r.bedroom_count).sum::<u32>() as f64 / total_houses;
    let avg_net_sqm = data.iter().map(|r| r.net_sqm).sum::<f64>() / total_houses;

    HashMap::from([
        ("average_price", avg_price),
        ("average_bedrooms", avg_bedrooms),
        ("average_net_sqm", avg_net_sqm),
    ])
}

pub fn find_price_trends(data: &[HouseRecord]) -> HashMap<&'static str, f64> {
    let avg_price_per_sqm = data
        .iter()
        .map(|r| r.price / r.net_sqm)
        .sum::<f64>()
        / data.len() as f64;

    HashMap::from([("average_price_per_sqm", avg_price_per_sqm)])
}

pub fn correlation_analysis(data: &[HouseRecord]) -> HashMap<&'static str, f64> {
    let total_houses = data.len() as f64;

    let avg_price = data.iter().map(|r| r.price).sum::<f64>() / total_houses;
    let avg_net_sqm = data.iter().map(|r| r.net_sqm).sum::<f64>() / total_houses;

    let covariance = data
        .iter()
        .map(|r| (r.price - avg_price) * (r.net_sqm - avg_net_sqm))
        .sum::<f64>()
        / total_houses;

    let price_variance = data
        .iter()
        .map(|r| (r.price - avg_price).powi(2))
        .sum::<f64>()
        / total_houses;

    let net_sqm_variance = data
        .iter()
        .map(|r| (r.net_sqm - avg_net_sqm).powi(2))
        .sum::<f64>()
        / total_houses;

    let correlation = covariance / (price_variance.sqrt() * net_sqm_variance.sqrt());

    HashMap::from([("price_net_sqm_correlation", correlation)])
}
