use plotly::{Scatter, Trace};

use crate::{config, plot::*, util, util::Result};

pub fn count_scatter_plot(
    context: &PlotContext,
    plot_config: &config::CountScatterPlot,
) -> Result<Vec<Box<dyn Trace>>> {
    let common_cfg = &context.plot_config;
    let (timestamp, count) = util::get_count_samples(&context.conn, common_cfg, plot_config)?;
    let segments = util::aggregate_and_segment(common_cfg, timestamp, count);

    let line_color = util::get_line_color(context, common_cfg);
    let mut traces: Vec<Box<dyn Trace>> = Vec::with_capacity(segments.len());
    let mut first = true;
    for (x, y) in segments {
        let x = util::nanoseconds_epoch_to_plotly_time(&x);
        let mut trace = Scatter::new(x, y);
        trace = util::apply_common_plot_configuration(trace, common_cfg, line_color);
        trace = trace.show_legend(first);
        first = false;

        traces.push(trace)
    }

    Ok(traces)
}
