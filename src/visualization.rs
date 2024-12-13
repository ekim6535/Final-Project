pub mod visualization {
    use crate::data_loader::data_loader::HouseRecord;
    use plotters::prelude::*;
    use std::error::Error;
    
    pub fn generate_histogram<F>(
        data: &[HouseRecord],
        field_name: &str,
        field_extractor: F,
        output_path: &str,
    ) -> Result<(), Box<dyn Error>>
    where
        F: Fn(&HouseRecord) -> f64,
    {
        let values: Vec<f64> = data.iter().map(&field_extractor).collect();

        let root = BitMapBackend::new(output_path, (640, 480)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption(format!("Histogram of {}", field_name), ("sans-serif", 30))
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(
                *values.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()..
                *values.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap(),
                0..values.len() / 10,
            )?;

        chart.configure_mesh().draw()?;

        chart.draw_series(
            Histogram::vertical(&chart)
                .style(RED.mix(0.5).filled())
                .data(values.iter().map(|&v| (v, 1))),
        )?;

        Ok(())
    }
}
