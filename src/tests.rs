#[cfg(test)]
mod tests {
    use crate::data_loader::{HouseRecord, load_data};
    use crate::data_analysis::{calculate_statistics, find_price_trends, correlation_analysis};

    #[test]
    fn test_calculate_statistics() {
        let data = vec![
            HouseRecord {
                bedroom_count: 3,
                net_sqm: 100.0,
                center_distance: 5.0,
                metro_distance: 2.0,
                floor: 2,
                age: 10,
                price: 300_000.0,
            },
            HouseRecord {
                bedroom_count: 4,
                net_sqm: 150.0,
                center_distance: 3.0,
                metro_distance: 1.5,
                floor: 1,
                age: 5,
                price: 450_000.0,
            },
        ];

        let stats = calculate_statistics(&data);
        assert!(stats["average_price"] > 0.0);
    }

    #[test]
    fn test_find_price_trends() {
        let data = vec![
            HouseRecord {
                bedroom_count: 3,
                net_sqm: 100.0,
                center_distance: 5.0,
                metro_distance: 2.0,
                floor: 2,
                age: 10,
                price: 300_000.0,
            },
            HouseRecord {
                bedroom_count: 4,
                net_sqm: 150.0,
                center_distance: 3.0,
                metro_distance: 1.5,
                floor: 1,
                age: 5,
                price: 450_000.0,
            },
        ];

        let trends = find_price_trends(&data);
        assert!(trends["average_price_per_sqm"] > 0.0);
    }

    #[test]
    fn test_correlation_analysis() {
        let data = vec![
            HouseRecord {
                bedroom_count: 3,
                net_sqm: 100.0,
                center_distance: 5.0,
                metro_distance: 2.0,
                floor: 2,
                age: 10,
                price: 300_000.0,
            },
            HouseRecord {
                bedroom_count: 4,
                net_sqm: 150.0,
                center_distance: 3.0,
                metro_distance: 1.5,
                floor: 1,
                age: 5,
                price: 450_000.0,
            },
        ];

        let correlations = correlation_analysis(&data);
        assert!(correlations["price_net_sqm_correlation"] >= -1.0);
    }
}
