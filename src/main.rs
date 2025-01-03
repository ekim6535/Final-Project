mod data_loader;
mod data_analysis;
mod visualization;
mod tests;

use data_loader::load_data;
pub(crate) use data_analysis::{calculate_statistics, find_price_trends, correlation_analysis};
use visualization::generate_histogram;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Load the dataset
    let dataset_path = "data/house.csv"; // Updated CSV file name
    let data = load_data(dataset_path)?;

    // Perform analysis
    let statistics = calculate_statistics(&data);
    println!("Statistics: {:?}", statistics);

    let trends = find_price_trends(&data);
    println!("Price Trends: {:?}", trends);

    let correlations = correlation_analysis(&data);
    println!("Correlations: {:?}", correlations);

    // Generate visualizations
    generate_histogram(
        &data,
        "Price",
        |record| record.price,
        "output/price_histogram.png",
    )?;

    Ok(())
}
