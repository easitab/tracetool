use std::cell::RefCell;

use crate::config;

/// Context for the plot subcommand.
pub struct PlotContext {
    /// The SQLite database connection.
    pub conn: rusqlite::Connection,
    /// Color generator for the plots.
    pub color_gen: RefCell<DefaultColorGenerator>,
    /// The plot configuration.
    pub plot_config: config::PlotCommon,
}

lazy_static! {
    /// The default colors to use for plots. These are the same as the default
    /// colors used by plotly.
    static ref DEFAULT_COLORS: [&'static str; 10] = [
        "#1f77b4", "#ff7f0e", "#2ca02c", "#d62728", "#9467bd", "#8c564b", "#e377c2", "#7f7f7f",
        "#bcbd22", "#17becf",
    ];
}

/// A color generator that cycles through a set of default colors. This is used
/// to assign colors to different traces in a plot. The reason we don't use the
/// default color assignment in plotly is that we want the ability to segment a
/// plot into multiple traces when the data is not contiguous, in order to avoid
/// connecting the segments with a spurious line. Using the default color
/// assignment would result in the multiple segments having different colors
/// making them seem unrelated. With this we can give each segment in a single
/// data set the same color.
pub struct DefaultColorGenerator {
    index: i8,
}

impl DefaultColorGenerator {
    pub(crate) fn new() -> Self {
        Self { index: 0 }
    }

    /// Get the next color in the sequence.
    pub(crate) fn next(&mut self) -> &'static str {
        let res = DEFAULT_COLORS[self.index as usize];
        self.index = (self.index + 1) % DEFAULT_COLORS.len() as i8;
        res
    }
}
