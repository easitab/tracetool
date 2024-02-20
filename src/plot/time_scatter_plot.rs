use plotly::{Scatter, Trace};

use crate::{config, plot::*, util, util::Result};

pub fn time_scatter_plot(
    context: &PlotContext,
    plot_config: &config::TimeScatterPlot,
) -> Result<Vec<Box<dyn Trace>>> {
    let common_cfg = &context.plot_config;
    let (timestamp, duration) = util::get_samples(
        &context.conn,
        common_cfg.filter.as_ref().and_then(|f| f.start.as_deref()),
        common_cfg.filter.as_ref().and_then(|f| f.end.as_deref()),
        common_cfg.filter.as_ref(),
        &plot_config.column,
        &plot_config.table,
    )?;

    let (timestamp, duration) =
        util::apply_workday_filter(timestamp, duration, common_cfg.filter.as_ref());

    let segments = util::aggregate_and_segment(common_cfg, timestamp, duration);

    let line_color = util::get_line_color(context, common_cfg);
    let mut traces: Vec<Box<dyn Trace>> = Vec::with_capacity(segments.len());
    let mut first = true;
    for (x, y) in segments.iter() {
        let x = util::nanoseconds_epoch_to_plotly_time(x);
        let y = util::nanoseconds_duration_to_unit(y, plot_config.unit);

        let mut trace = Scatter::new(x, y);

        trace = util::apply_common_plot_configuration(trace, common_cfg, line_color);

        trace = trace.show_legend(first);
        first = false;

        traces.push(trace)
    }
    Ok(traces)
}
