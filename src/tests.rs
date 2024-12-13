mod tests {
    use super::*;
    use crate::data_loader::HouseRecord;
    
    #[test]
    fn test_correlation_analysis() {
        let data = vec![
            HouseRecord { price: 300_000.0, net_sqm: 50.0, ..Default::default() },
            HouseRecord { price: 450_000.0, net_sqm: 75.0, ..Default::default() },
            HouseRecord { price: 500_000.0, net_sqm: 80.0, ..Default::default() },
            HouseRecord { price: 600_000.0, net_sqm: 100.0, ..Default::default() },
            HouseRecord { price: 750_000.0, net_sqm: 120.0, ..Default::default() },
        ];

        // Calling the correlation analysis function
        let correlations = data_analysis::correlation_analysis(&data);

        // Assert that the correlation is within the valid range [-1, 1]
        let correlation = correlations["price_sqm_correlation"];
        assert!(correlation >= -1.0 && correlation <= 1.0, "Correlation is out of range: {}", correlation);
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
}
