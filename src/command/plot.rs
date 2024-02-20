use std::cell::RefCell;
use std::path::Path;

use plotly::Plot;

use crate::{config, plot, util::Result};

pub(crate) fn plot<T: AsRef<Path>>(configuration_yaml_path: T) -> Result<()> {
    let config: config::Root = match config::load_config(&configuration_yaml_path) {
        Ok(config) => config,
        Err(e) => {
            return Err(format!(
                "Error loading configuration {}: {}",
                configuration_yaml_path.as_ref().display(),
                e
            )
            .into());
        }
    };
    let configuration_dir = configuration_yaml_path.as_ref().parent().unwrap();
    let source_path = configuration_dir.join(&config.source);
    let absolute_source_path = match source_path.canonicalize() {
        Ok(path) => path,
        Err(e) => {
            return Err(format!("Error opening database {}: {}", source_path.display(), e).into());
        }
    };
    let conn = match rusqlite::Connection::open(&absolute_source_path) {
        Ok(conn) => conn,
        Err(e) => {
            return Err(format!(
                "Error opening database {}: {}",
                absolute_source_path.display(),
                e
            )
            .into());
        }
    };

    let mut plot = Plot::new();
    if let Some(layout) = &config.layout {
        plot.set_layout(layout.into());
    }

    let color_gen = plot::DefaultColorGenerator::new();
    let mut context = plot::PlotContext {
        conn,
        color_gen: RefCell::new(color_gen),
        plot_config: config::PlotCommon::empty(),
    };

    for plot_common in config.plots {
        context.plot_config = plot_common.plot_common;
        let traces = match &plot_common.plot_variant {
            config::PlotVariant::TimeScatter(plot_variant_config) => {
                println!("Plotting time scatter plot: {:?}", plot_variant_config);
                plot::time_scatter_plot(&context, plot_variant_config)?
            }
            config::PlotVariant::CountScatter(plot_variant_config) => {
                println!("Plotting count scatter plot: {:?}", plot_variant_config);
                plot::count_scatter_plot(&context, plot_variant_config)?
            }
            config::PlotVariant::Overlap(plot_variant_config) => {
                println!("Plotting overlap plot: {:?}", plot_variant_config);
                plot::overlap_plot(&context.conn, &context.plot_config, plot_variant_config)?
            }
        };
        plot.add_traces(traces);
    }

    plot.show();
    Ok(())
}
