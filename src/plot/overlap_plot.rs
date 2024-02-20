use plotly::common::{ColorScale, ColorScaleElement};
use plotly::{HeatMap, Trace};
use rusqlite::Connection;

use crate::util::ViewDurationVsOverlap;
use crate::{config, util, util::Result};

pub fn overlap_plot(
    conn: &Connection,
    common_plot_config: &config::PlotCommon,
    plot_config: &config::OverlapPlot,
) -> Result<Vec<Box<dyn Trace>>> {
    let ViewDurationVsOverlap {
        wallclock_time,
        overlap,
    } = util::get_overlap_samples_for_view(
        conn,
        plot_config.view_id,
        common_plot_config
            .filter
            .as_ref()
            .and_then(|f| f.start.as_deref()),
        common_plot_config
            .filter
            .as_ref()
            .and_then(|f| f.end.as_deref()),
        None,
    )?;

    let overlap = util::overlap_to_percent(&wallclock_time, &overlap);

    let wallclock_time = util::nanoseconds_duration_to_seconds(&wallclock_time);

    let x_bins = plot_config.x_bins.unwrap_or(256);
    let y_bins = plot_config.y_bins.unwrap_or(256);

    let (x_labels, y_labels, z) = make_2d_histogram(&wallclock_time, &overlap, x_bins, y_bins);

    // Compute logarithm of z to emphasize small values
    let z = z
        .iter()
        .map(|row| {
            row.iter()
                .map(|x| (*x as f64 + 1.0).ln())
                .collect::<Vec<f64>>()
        })
        .collect::<Vec<Vec<f64>>>();

    let custom_color_scale = ColorScale::Vector(vec![
        ColorScaleElement(0.0, "black".to_string()),
        ColorScaleElement(1e-6, "black".to_string()),
        ColorScaleElement(1e-5, "white".to_string()),
        ColorScaleElement(0.5, "orange".to_string()),
        ColorScaleElement(1.0, "red".to_string()),
    ]);

    Ok(vec![
        HeatMap::new(x_labels, y_labels, z).color_scale(custom_color_scale)
    ])
}

fn make_2d_histogram(
    x: &[f64],
    y: &[f64],
    x_bins: u32,
    y_bins: u32,
) -> (Vec<f64>, Vec<f64>, Vec<Vec<i64>>) {
    if x.len() != y.len() {
        panic!("x and y must have the same length");
    }
    if x.is_empty() {
        return (vec![], vec![], vec![]);
    }

    let x_min = *(x.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap());
    let x_max = *(x.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap());
    eprintln!("x_min: {}, x_max: {}", x_min, x_max);
    let y_min = *(y.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap());
    let y_max = *(y.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap());
    eprintln!("y_min: {}, y_max: {}", y_min, y_max);

    let x_bin_size = (x_max - x_min) / x_bins as f64;
    let y_bin_size = (y_max - y_min) / y_bins as f64;

    let mut bins = vec![vec![0; y_bins as usize]; x_bins as usize];

    for i in 0..x.len() {
        let x_bin = ((x[i] - x_min) / x_bin_size) as usize;
        let y_bin = ((y[i] - y_min) / y_bin_size) as usize;

        let x_bin_clamped = x_bin.min(x_bins as usize - 1);
        let y_bin_clamped = y_bin.min(y_bins as usize - 1);
        bins[x_bin_clamped][y_bin_clamped] += 1;
    }

    let x_labels = (0..x_bins as u64)
        .map(|i| (x_min + i as f64) * x_bin_size)
        .collect();
    let y_labels = (0..y_bins as u64)
        .map(|i| (y_min + i as f64) * y_bin_size)
        .collect();

    (x_labels, y_labels, bins)
}
