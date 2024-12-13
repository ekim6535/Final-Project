// Modules
mod data_loader;
mod data_analysis;
mod visualization;
mod tests;

use data_loader::load_data;
use data_analysis::{calculate_statistics, find_price_trends, correlation_analysis};
use visualization::generate_histogram;
use std::error::Error;


