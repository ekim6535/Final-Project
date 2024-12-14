use plotters::prelude::*;
use std::error::Error;

pub fn generate_histogram<T, F>(
    data: &[T],
    label: &str,
    value_fn: F,
    output_path: &str,
) -> Result<(), Box<dyn Error>>
where
    T: Clone,
    F: Fn(&T) -> f64,
{
    let values: Vec<f64> = data.iter().map(|item| value_fn(item)).collect();

    // Create bins for the histogram (adjust bin size as needed)
    let min_value = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_value = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let bin_count = 10; // Number of bins
    let bin_width = (max_value - min_value) / bin_count as f64;

    let mut bins = vec![0; bin_count];
    for &value in &values {
        let bin_index = ((value - min_value) / bin_width).floor() as usize;
        if bin_index < bin_count {
            bins[bin_index] += 1;
        }
    }

    // Set up the drawing area
    let root = BitMapBackend::new(output_path, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(label, ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..bin_count, 0..*bins.iter().max().unwrap_or(&0))?;

    chart.configure_mesh().draw()?;

    // Plot the histogram
    chart.draw_series(bins.iter().enumerate().map(|(i, &count)| {
        let x0 = i;
        let x1 = i + 1;
        Rectangle::new([(x0, 0), (x1, count)], RED.mix(0.5).filled())
    }))?;

    Ok(())
}
