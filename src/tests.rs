#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_loader::HouseRecord;

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

    #[test]
    fn test_correlation_analysis() {
        let data = vec![
            HouseRecord { price: 300_000.0, bedrooms: 3, bathrooms: 2, sqft_living: 1500, sqft_lot: 5000, floors: 1.0 },
            HouseRecord { price: 450_000.0, bedrooms: 4, bathrooms: 3, sqft_living: 2000, sqft_lot:
