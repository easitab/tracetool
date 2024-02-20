use std::collections::HashMap;
use std::path::Path;

use nalgebra::{DMatrix, DVector, SymmetricEigen};
use ndarray::Array2;

use crate::util::ViewDurationVsOverlap;
use crate::{util, util::Result};

struct ViewDependencyInfo {
    view_id: i32,
    samples: usize,
    statistics: util::Statistics<f64>,
    variance_ratio: f64,
    #[allow(dead_code)]
    basis_vectors: DMatrix<f64>,
}

pub(crate) fn compute_overlap_pca<P: AsRef<Path>>(
    database_path: P,
    start: Option<&str>,
    end: Option<&str>,
) -> Result<()> {
    let conn = rusqlite::Connection::open(database_path)?;

    let by_view_id: HashMap<i32, ViewDurationVsOverlap> =
        util::get_overlap_samples(&conn, start, end, None)?;

    let mut view_info_rows: Vec<ViewDependencyInfo> = Vec::with_capacity(by_view_id.len());

    for (
        view_id,
        ViewDurationVsOverlap {
            wallclock_time,
            overlap,
        },
    ) in &by_view_id
    {
        let overlap = util::overlap_to_percent(wallclock_time, overlap);
        let rows = wallclock_time.len();
        if rows < 20 {
            continue;
        }
        let wallclock_time: Vec<f64> = wallclock_time.iter().map(|v| *v as f64).collect();

        let statistics = {
            let mut sorted_wallclock_time = wallclock_time.clone();
            sorted_wallclock_time.sort_by(|a, b| a.partial_cmp(b).unwrap());
            util::get_statistics(&sorted_wallclock_time)
        };

        let (basis_vectors, variances) = find_shape(wallclock_time, overlap)?;
        let total_variance: f64 = util::kahan_sum(&variances);
        let max_variance = variances.max();

        view_info_rows.push(ViewDependencyInfo {
            view_id: *view_id,
            samples: rows,
            statistics,
            variance_ratio: max_variance / total_variance,
            basis_vectors,
        });
    }

    view_info_rows.sort_by(|a, b| a.variance_ratio.total_cmp(&b.variance_ratio));

    let mut csv_writer = csv::WriterBuilder::new()
        .has_headers(true)
        .from_writer(std::io::stdout());

    csv_writer.write_record(["view ID", "sample count", "Q3 (ms)", "variance ratio"])?;
    for row in view_info_rows.iter() {
        csv_writer.write_record(&[
            row.view_id.to_string(),
            row.samples.to_string(),
            (row.statistics.q3 / 1_000_000.0).to_string(),
            row.variance_ratio.to_string(),
        ])?;
    }
    Ok(())
}

/// Determine the "shape" of the data by finding its natural axes (eigenvectors) and their elongations (eigenvalues).
///
/// This function is based on the principal component analysis (PCA) technique, which helps identify patterns
/// in the data by transforming it to a new coordinate system. The new axes (eigenvectors) are chosen to maximize
/// the variance of the data, and the elongations (eigenvalues) represent the amount of variance along each axis.
/// By examining the eigenvectors and eigenvalues, one can understand the "shape" of the data, i.e., its
/// orientation and elongation in the original coordinate system.
///
/// In simpler terms, this function helps identify the main directions where the data spreads out the most, as if
/// trying to fit an ellipse to the data points. The eigenvectors represent the directions of the ellipse's axes,
/// and the eigenvalues indicate how much the ellipse is stretched along each axis.
///
/// # Arguments
/// * `x_values` - A vector containing the x values of the dataset.
/// * `y_values` - A vector containing the y values of the dataset.
///
/// # Returns
/// A tuple containing the eigenvectors as a DMatrix<f64> and the eigenvalues as a DVector<f64>.
/// The eigenvectors are the basis vectors of the data, and the eigenvalues are the data variance
/// along each axis.
fn find_shape(x_values: Vec<f64>, y_values: Vec<f64>) -> Result<(DMatrix<f64>, DVector<f64>)> {
    assert_eq!(x_values.len(), y_values.len());

    let n = x_values.len();
    assert_ne!(n, 0);

    // Combine and center the data
    let data = Array2::from_shape_vec(
        (n, 2),
        x_values.into_iter().chain(y_values.into_iter()).collect(),
    )
    .unwrap();
    let mean = data.mean_axis(ndarray::Axis(0)).unwrap();
    let data_centered = data - &mean;
    let data_matrix = DMatrix::from_row_slice(n, 2, data_centered.as_slice().unwrap());

    // Find the principal components by calculating the covariance matrix and its eigenvalues and eigenvectors
    let covariance_matrix = data_matrix.transpose() * data_matrix / (n as f64 - 1.0);
    let SymmetricEigen {
        eigenvectors,
        eigenvalues,
    } = SymmetricEigen::new(covariance_matrix);

    Ok((eigenvectors, eigenvalues))
}
