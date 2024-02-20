use std::path::Path;

use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize};

use crate::util::Result;

/**
 * The goal for most YAML configuration is really to just be a passthrough so
 * people can use all the features of Plotly and rely on the Plotly
 * documentation for configuration options. This is why we use serde to
 * deserialize the Plotly. Unfortunately, the Plotly implement Serialize (for
 * converting to JSON before sending to the Plotly JS code), but not Deserialize
 * which would allow us to read configuration from Yaml files. So, to enable
 * this we have to duplicate all of the Plotly configuration structs here and
 * derive Deserialize for them.
 *
 * For the structs that we duplicate from Yaml (like Title, Axis, and Line), we
 * also implement From<T> for the Plotly equivalent so that we can convert
 * from our struct to the Plotly struct almost seamlessly.
 *
 * The reference documentation is at https://plotly.com/javascript/reference/.
 * The plotly crate doesn't implement every option given there, and we don't add
 * every option that the crate has. They can be added as needed.
 */

/**
 * The root configuration struct for the YAML configuration file. This is the
 * top level struct that we will deserialize the YAML file into.
 */
#[derive(Debug, Deserialize)]
pub struct Root {
    pub source: String,
    pub layout: Option<Layout>,
    pub plots: Vec<PlotConfig>,
}

#[derive(Debug, Deserialize)]
pub struct Layout {
    pub title: Option<Title>,
    #[serde(rename = "showlegend")]
    show_legend: Option<bool>,
    pub width: Option<i32>,
    pub height: Option<i32>,

    #[serde(rename = "xaxis")]
    x_axis: Option<Axis>,
    #[serde(rename = "yaxis")]
    y_axis: Option<Axis>,
    #[serde(rename = "zaxis")]
    z_axis: Option<Axis>,
    #[serde(rename = "xaxis2")]
    x_axis2: Option<Axis>,
    #[serde(rename = "yaxis2")]
    y_axis2: Option<Axis>,
    #[serde(rename = "zaxis2")]
    z_axis2: Option<Axis>,
    #[serde(rename = "xaxis3")]
    x_axis3: Option<Axis>,
    #[serde(rename = "yaxis3")]
    y_axis3: Option<Axis>,
    #[serde(rename = "zaxis3")]
    z_axis3: Option<Axis>,
    #[serde(rename = "xaxis4")]
    x_axis4: Option<Axis>,
    #[serde(rename = "yaxis4")]
    y_axis4: Option<Axis>,
    #[serde(rename = "zaxis4")]
    z_axis4: Option<Axis>,
    #[serde(rename = "xaxis5")]
    x_axis5: Option<Axis>,
    #[serde(rename = "yaxis5")]
    y_axis5: Option<Axis>,
    #[serde(rename = "zaxis5")]
    z_axis5: Option<Axis>,
    #[serde(rename = "xaxis6")]
    x_axis6: Option<Axis>,
    #[serde(rename = "yaxis6")]
    y_axis6: Option<Axis>,
    #[serde(rename = "zaxis6")]
    z_axis6: Option<Axis>,
    #[serde(rename = "xaxis7")]
    x_axis7: Option<Axis>,
    #[serde(rename = "yaxis7")]
    y_axis7: Option<Axis>,
    #[serde(rename = "zaxis7")]
    z_axis7: Option<Axis>,
    #[serde(rename = "xaxis8")]
    x_axis8: Option<Axis>,
    #[serde(rename = "yaxis8")]
    y_axis8: Option<Axis>,
    #[serde(rename = "zaxis8")]
    z_axis8: Option<Axis>,
}

#[derive(Debug, Deserialize)]
pub struct Axis {
    visible: Option<bool>,
    //color: Option<Box<dyn Color>>,
    title: Option<Title>,
    r#type: Option<AxisType>,
    #[serde(rename = "autorange")]
    auto_range: Option<bool>,
    #[serde(rename = "rangemode")]
    range_mode: Option<RangeMode>,
    //range: Option<NumOrStringCollection>,
    //#[serde(rename = "fixedrange")]
    //fixed_range: Option<bool>,
    //constrain: Option<AxisConstrain>,
    //#[serde(rename = "constraintoward")]
    //constrain_toward: Option<ConstrainDirection>,
    //#[serde(rename = "tickmode")]
    //tick_mode: Option<TickMode>,
    //#[serde(rename = "nticks")]
    //n_ticks: Option<usize>,
    tick0: Option<f64>,
    dtick: Option<f64>,

    //matches: Option<String>,

    //#[serde(rename = "tickvals")]
    //tick_values: Option<Vec<f64>>,
    //#[serde(rename = "ticktext")]
    //tick_text: Option<Vec<String>>,
    //ticks: Option<TicksDirection>,
    //#[serde(rename = "tickson")]
    //ticks_on: Option<TicksPosition>,
    //mirror: Option<bool>,
    //#[serde(rename = "ticklen")]
    //tick_length: Option<usize>,
    //#[serde(rename = "tickwidth")]
    //tick_width: Option<usize>,
    //#[serde(rename = "tickcolor")]
    //tick_color: Option<Box<dyn Color>>,
    //#[serde(rename = "showticklabels")]
    //show_tick_labels: Option<bool>,
    //#[serde(rename = "automargin")]
    //auto_margin: Option<bool>,
    //#[serde(rename = "showspikes")]
    //show_spikes: Option<bool>,
    //#[serde(rename = "spikecolor")]
    //spike_color: Option<Box<dyn Color>>,
    //#[serde(rename = "spikethickness")]
    //spike_thickness: Option<usize>,
    //#[serde(rename = "spikedash")]
    //spike_dash: Option<DashType>,
    //#[serde(rename = "spikemode")]
    //spike_mode: Option<SpikeMode>,
    //#[serde(rename = "spikesnap")]
    //spike_snap: Option<SpikeSnap>,
    //#[serde(rename = "tickfont")]
    //tick_font: Option<Font>,
    //#[serde(rename = "tickangle")]
    //tick_angle: Option<f64>,
    //#[serde(rename = "tickprefix")]
    //tick_prefix: Option<String>,
    //#[serde(rename = "showtickprefix")]
    //show_tick_prefix: Option<ArrayShow>,
    //#[serde(rename = "ticksuffix")]
    //tick_suffix: Option<String>,
    //#[serde(rename = "showticksuffix")]
    //show_tick_suffix: Option<ArrayShow>,
    //#[serde(rename = "showexponent")]
    //show_exponent: Option<ArrayShow>,
    //#[serde(rename = "exponentformat")]
    //exponent_format: Option<ExponentFormat>,
    //#[serde(rename = "separatethousands")]
    //separate_thousands: Option<bool>,
    //#[serde(rename = "tickformat")]
    //tick_format: Option<String>,
    //#[serde(rename = "tickformatstops")]
    //tick_format_stops: Option<Vec<TickFormatStop>>,
    //#[serde(rename = "hoverformat")]
    //hover_format: Option<String>,
    //#[serde(rename = "showline")]
    //show_line: Option<bool>,
    //#[serde(rename = "linecolor")]
    //line_color: Option<Box<dyn Color>>,
    //#[serde(rename = "linewidth")]
    //line_width: Option<usize>,
    #[serde(rename = "showgrid")]
    show_grid: Option<bool>,
    //#[serde(rename = "gridcolor")]
    //grid_color: Option<Box<dyn Color>>,
    //#[serde(rename = "gridwidth")]
    //grid_width: Option<usize>,
    //#[serde(rename = "zeroline")]
    //zero_line: Option<bool>,
    //#[serde(rename = "zerolinecolor")]
    //zero_line_color: Option<Box<dyn Color>>,
    //#[serde(rename = "zerolinewidth")]
    //zero_line_width: Option<usize>,
    //#[serde(rename = "showdividers")]
    //show_dividers: Option<bool>,
    //#[serde(rename = "dividercolor")]
    //divider_color: Option<Box<dyn Color>>,
    //#[serde(rename = "dividerwidth")]
    //divider_width: Option<usize>,
    //anchor: Option<String>,
    side: Option<AxisSide>,
    overlaying: Option<String>,
    //#[field_setter(skip)]
    //domain: Option<Vec<f64>>,
    //position: Option<f64>,
    //#[serde(rename = "rangeslider")]
    //range_slider: Option<RangeSlider>,
    //#[serde(rename = "rangeselector")]
    //range_selector: Option<RangeSelector>,
    //calendar: Option<Calendar>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum RangeMode {
    Normal,
    ToZero,
    NonNegative,
}

#[derive(Clone, Debug, Default)]
pub struct Title {
    text: String,
    /*font: Option<Font>,
    side: Option<Side>,
    #[serde(rename = "xref")]
    x_ref: Option<Reference>,
    #[serde(rename = "yref")]
    y_ref: Option<Reference>,
    x: Option<f64>,
    y: Option<f64>,
    #[serde(rename = "xanchor")]
    x_anchor: Option<Anchor>,
    #[serde(rename = "yanchor")]
    y_anchor: Option<Anchor>,
    pad: Option<Pad>,*/
}

impl<'de> Deserialize<'de> for Title {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TitleVisitor;

        impl<'de> serde::de::Visitor<'de> for TitleVisitor {
            type Value = Title;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a title string or a title struct")
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: std::error::Error,
            {
                // The plotly documentation says that "Before the existence of
                // `title.text`, the title's contents used to be defined as the
                // `title` attribute itself. This behavior has been deprecated."
                //
                // However, it's very convenient for our use case to set the
                // title directly as a string, so we'll allow it.
                Ok(Title {
                    text: value.to_string(),
                })
            }

            fn visit_map<A>(self, mut map: A) -> std::result::Result<Title, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut text = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "text" => {
                            text = Some(map.next_value()?);
                        }
                        _ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }

                let text = text.ok_or_else(|| serde::de::Error::missing_field("text"))?;
                Ok(Title { text })
            }
        }

        deserializer.deserialize_any(TitleVisitor)
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AxisType {
    #[serde(rename = "-")]
    Default,
    Linear,
    Log,
    Date,
    Category,
    MultiCategory,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AxisSide {
    Top,
    Bottom,
    Left,
    Right,
}

impl From<&AxisSide> for plotly::common::AxisSide {
    fn from(side: &AxisSide) -> Self {
        match side {
            AxisSide::Top => plotly::common::AxisSide::Top,
            AxisSide::Bottom => plotly::common::AxisSide::Bottom,
            AxisSide::Left => plotly::common::AxisSide::Left,
            AxisSide::Right => plotly::common::AxisSide::Right,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PlotConfig {
    #[serde(flatten)]
    pub plot_common: PlotCommon,
    #[serde(flatten)]
    pub plot_variant: PlotVariant,
}

#[derive(Debug, Deserialize)]
pub struct PlotCommon {
    pub name: String,
    pub filter: Option<Filter>,
    pub aggregation: Option<Aggregation>,
    pub visible: Option<Visible>,
    //pub show_legend: Option<bool>,
    //pub legend_group: Option<String>,
    //pub legend_group_title: Option<LegendGroupTitle>,
    //pub opacity: Option<f64>,
    pub mode: Option<Mode>,

    //pub ids: Option<Vec<String>>,
    //pub x: Option<Vec<X>>,
    //pub x0: Option<NumOrString>,
    //pub dx: Option<f64>,
    //pub y: Option<Vec<Y>>,
    //pub y0: Option<NumOrString>,
    //pub dy: Option<f64>,
    //pub text: Option<Dim<String>>,
    //#[serde(rename = "textposition")]
    //pub text_position: Option<Dim<Position>>,
    //#[serde(rename = "texttemplate")]
    //pub text_template: Option<Dim<String>>,
    //#[serde(rename = "hovertext")]
    //pub hover_text: Option<Dim<String>>,
    //#[serde(rename = "hoverinfo")]
    //hover_info: Option<HoverInfo>,
    //#[serde(rename = "hovertemplate")]
    //hover_template: Option<Dim<String>>,
    //meta: Option<NumOrString>,
    //#[serde(rename = "customdata")]
    //pub custom_data: Option<NumOrStringCollection>,
    pub line: Option<Line>,

    #[serde(rename = "xaxis")]
    pub x_axis: Option<String>,
    #[serde(rename = "yaxis")]
    pub y_axis: Option<String>,
}

impl PlotCommon {
    pub fn empty() -> Self {
        Self {
            name: "".to_string(),
            visible: None,
            line: None,
            x_axis: None,
            y_axis: None,
            mode: None,
            filter: None,
            aggregation: None,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "plot", rename_all = "snake_case")]
pub enum PlotVariant {
    TimeScatter(TimeScatterPlot),
    CountScatter(CountScatterPlot),
    Overlap(OverlapPlot),
}

#[derive(Debug, Deserialize)]
pub struct TimeScatterPlot {
    pub table: String,
    pub column: String,
    pub unit: Option<TimeUnit>,
}

#[derive(Debug, Deserialize)]
pub struct CountScatterPlot {
    pub table: String,
    pub column: String,
}

#[derive(Debug, Deserialize)]
pub struct OverlapPlot {
    pub view_id: i32,
    pub x_bins: Option<u32>,
    pub y_bins: Option<u32>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Line {
    pub width: Option<f64>,
    pub shape: Option<LineShape>,
    pub smoothing: Option<f64>,
    pub dash: Option<DashType>,
    pub simplify: Option<bool>,
    pub color: Option<String>,
    pub cauto: Option<bool>,
    pub cmin: Option<f64>,
    pub cmax: Option<f64>,
    pub cmid: Option<f64>,
    #[serde(rename = "colorscale")]
    pub color_scale: Option<ColorScale>,
    #[serde(rename = "autocolorscale")]
    pub auto_color_scale: Option<bool>,
    #[serde(rename = "reversescale")]
    pub reverse_scale: Option<bool>,
    #[serde(rename = "outliercolor")]
    pub outlier_color: Option<String>,
    #[serde(rename = "outlierwidth")]
    pub outlier_width: Option<usize>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Filter {
    pub start: Option<String>,
    pub end: Option<String>,
    #[serde(rename = "where")]
    pub sql_where: Option<String>,
    #[serde(rename = "workhours")]
    pub work_hours: Option<bool>,
}

// TODO write custom serializer for this so we can accept booleans
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Visible {
    True,
    False,
    #[serde(rename = "legendonly")]
    LegendOnly,
}

impl From<&Visible> for plotly::common::Visible {
    fn from(visible: &Visible) -> Self {
        match visible {
            Visible::True => plotly::common::Visible::True,
            Visible::False => plotly::common::Visible::False,
            Visible::LegendOnly => plotly::common::Visible::LegendOnly,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LineShape {
    Linear,
    Spline,
    Hv,
    Vh,
    Hvh,
    Vhv,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DashType {
    Solid,
    Dot,
    Dash,
    LongDash,
    DashDot,
    LongDashDot,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ColorScale {
    Palette(ColorScalePalette),
    Vector(Vec<ColorScaleElement>),
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum ColorScalePalette {
    Greys,
    YlGnBu,
    Greens,
    YlOrRd,
    Bluered,
    RdBu,
    Reds,
    Blues,
    Picnic,
    Rainbow,
    Portland,
    Jet,
    Hot,
    Blackbody,
    Earth,
    Electric,
    Viridis,
    Cividis,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Lines,
    Markers,
    Text,
    #[serde(rename = "lines+markers")]
    LinesMarkers,
    #[serde(rename = "lines+text")]
    LinesText,
    #[serde(rename = "markers+text")]
    MarkersText,
    #[serde(rename = "lines+markers+text")]
    LinesMarkersText,
    None,
}

impl From<&Mode> for plotly::common::Mode {
    fn from(mode: &Mode) -> Self {
        match mode {
            Mode::Lines => plotly::common::Mode::Lines,
            Mode::Markers => plotly::common::Mode::Markers,
            Mode::Text => plotly::common::Mode::Text,
            Mode::LinesMarkers => plotly::common::Mode::LinesMarkers,
            Mode::LinesText => plotly::common::Mode::LinesText,
            Mode::MarkersText => plotly::common::Mode::MarkersText,
            Mode::LinesMarkersText => plotly::common::Mode::LinesMarkersText,
            Mode::None => plotly::common::Mode::None,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ColorScaleElement(pub f64, pub String);

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TimeUnit {
    #[serde(rename = "Y")]
    Years,
    #[serde(rename = "M")]
    Months,
    #[serde(rename = "W")]
    Weeks,
    #[serde(rename = "D")]
    Days,
    #[serde(rename = "h")]
    Hours,
    #[serde(rename = "m")]
    Minutes,
    #[serde(rename = "s")]
    Seconds,
    #[serde(rename = "ms")]
    Milliseconds,
    #[serde(rename = "us")]
    Microseconds,
    #[serde(rename = "ns")]
    Nanoseconds,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AggregationMode {
    #[serde(rename = "mean")]
    Mean,
    #[serde(rename = "min")]
    Min,
    #[serde(rename = "q1")]
    Q1,
    #[serde(rename = "median")]
    Median,
    #[serde(rename = "q3")]
    Q3,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "count")]
    Count,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct TimePeriod {
    pub quantity: u64,
    pub unit: TimeUnit,
}

lazy_static! {
    static ref TIME_PERIOD_REGEX: Regex = Regex::new(r"^\s*(\d+)\s*([A-Za-z]+)\s*$").unwrap();
}

impl<'de> Deserialize<'de> for TimePeriod {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let captures = match TIME_PERIOD_REGEX.captures(&s) {
            Some(captures) => captures,
            None => return Err(serde::de::Error::custom("Invalid time period")),
        };

        let quantity = match captures.get(1).unwrap().as_str().parse::<u64>() {
            Ok(quantity) => quantity,
            Err(_) => {
                return Err(serde::de::Error::custom(
                    "Invalid time period quantity (maximum value is 2**64-1)",
                ))
            }
        };

        let unit_str = captures.get(2).unwrap().as_str();
        let unit = match serde_yaml::from_str::<TimeUnit>(unit_str) {
            Ok(unit) => unit,
            Err(_) => return Err(serde::de::Error::custom("Invalid time period unit")),
        };
        Ok(TimePeriod { quantity, unit })
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Aggregation {
    pub mode: AggregationMode,
    pub size: TimePeriod,
    #[serde(rename = "mincount")]
    pub min_count: Option<usize>,
}

pub fn load_config<T: AsRef<Path>>(path: T) -> Result<Root> {
    let config = std::fs::read_to_string(path)?;
    let config: Root = serde_yaml::from_str(&config)?;
    Ok(config)
}

impl From<&Layout> for plotly::layout::Layout {
    fn from(layout: &Layout) -> plotly::layout::Layout {
        let mut playout = plotly::layout::Layout::new();
        if let Some(ref title) = &layout.title {
            playout = playout.title(title.into());
        }
        if let Some(show_legend) = layout.show_legend {
            playout = playout.show_legend(show_legend);
        }
        if let Some(width) = layout.width {
            assert!(width > 0);
            playout = playout.width(width as usize);
        }
        if let Some(height) = layout.height {
            assert!(height > 0);
            playout = playout.height(height as usize);
        }

        #[allow(clippy::type_complexity)]
        let axes: &[(
            &Option<Axis>,
            fn(plotly::Layout, plotly::layout::Axis) -> plotly::Layout,
        )] = &[
            (&layout.x_axis, |l, a| l.x_axis(a)),
            (&layout.y_axis, |l, a| l.y_axis(a)),
            (&layout.z_axis, |l, a| l.z_axis(a)),
            (&layout.x_axis2, |l, a| l.x_axis2(a)),
            (&layout.y_axis2, |l, a| l.y_axis2(a)),
            (&layout.z_axis2, |l, a| l.z_axis2(a)),
            (&layout.x_axis3, |l, a| l.x_axis3(a)),
            (&layout.y_axis3, |l, a| l.y_axis3(a)),
            (&layout.z_axis3, |l, a| l.z_axis3(a)),
            (&layout.x_axis4, |l, a| l.x_axis4(a)),
            (&layout.y_axis4, |l, a| l.y_axis4(a)),
            (&layout.z_axis4, |l, a| l.z_axis4(a)),
            (&layout.x_axis5, |l, a| l.x_axis5(a)),
            (&layout.y_axis5, |l, a| l.y_axis5(a)),
            (&layout.z_axis5, |l, a| l.z_axis5(a)),
            (&layout.x_axis6, |l, a| l.x_axis6(a)),
            (&layout.y_axis6, |l, a| l.y_axis6(a)),
            (&layout.z_axis6, |l, a| l.z_axis6(a)),
            (&layout.x_axis7, |l, a| l.x_axis7(a)),
            (&layout.y_axis7, |l, a| l.y_axis7(a)),
            (&layout.z_axis7, |l, a| l.z_axis7(a)),
            (&layout.x_axis8, |l, a| l.x_axis8(a)),
            (&layout.y_axis8, |l, a| l.y_axis8(a)),
            (&layout.z_axis8, |l, a| l.z_axis8(a)),
        ];

        for (axis, set_axis_fn) in axes.iter() {
            if let Some(a) = axis {
                playout = set_axis_fn(playout, a.into());
            }
        }

        playout
    }
}

impl From<&Axis> for plotly::layout::Axis {
    fn from(axis: &Axis) -> plotly::layout::Axis {
        let mut plotly_axis = plotly::layout::Axis::new();
        if let Some(visible) = &axis.visible {
            plotly_axis = plotly_axis.visible(*visible);
        }
        if let Some(title) = &axis.title {
            plotly_axis = plotly_axis.title(title.into());
        }
        if let Some(type_) = &axis.r#type {
            plotly_axis = plotly_axis.type_(type_.into());
        }
        if let Some(auto_range) = &axis.auto_range {
            plotly_axis = plotly_axis.auto_range(*auto_range);
        }
        if let Some(range_mode) = &axis.range_mode {
            plotly_axis = plotly_axis.range_mode(range_mode.into());
        }
        if let Some(dtick) = &axis.dtick {
            plotly_axis = plotly_axis.dtick(*dtick);
        }
        if let Some(tick0) = &axis.tick0 {
            plotly_axis = plotly_axis.tick0(*tick0);
        }
        if let Some(show_grid) = &axis.show_grid {
            plotly_axis = plotly_axis.show_grid(*show_grid);
        }
        if let Some(side) = &axis.side {
            plotly_axis = plotly_axis.side(side.into());
        }
        if let Some(overlaying) = &axis.overlaying {
            plotly_axis = plotly_axis.overlaying(overlaying);
        }
        plotly_axis
    }
}

impl From<&Title> for plotly::common::Title {
    fn from(title: &Title) -> plotly::common::Title {
        plotly::common::Title::new(&title.text)
    }
}

impl From<&AxisType> for plotly::layout::AxisType {
    fn from(axis_type: &AxisType) -> plotly::layout::AxisType {
        match axis_type {
            AxisType::Default => plotly::layout::AxisType::Default,
            AxisType::Linear => plotly::layout::AxisType::Linear,
            AxisType::Log => plotly::layout::AxisType::Log,
            AxisType::Date => plotly::layout::AxisType::Date,
            AxisType::Category => plotly::layout::AxisType::Category,
            AxisType::MultiCategory => plotly::layout::AxisType::MultiCategory,
        }
    }
}

impl From<&RangeMode> for plotly::layout::RangeMode {
    fn from(range_mode: &RangeMode) -> plotly::layout::RangeMode {
        match range_mode {
            RangeMode::Normal => plotly::layout::RangeMode::Normal,
            RangeMode::ToZero => plotly::layout::RangeMode::ToZero,
            RangeMode::NonNegative => plotly::layout::RangeMode::NonNegative,
        }
    }
}

impl From<&Line> for plotly::common::Line {
    fn from(line: &Line) -> plotly::common::Line {
        let mut plotly_line = plotly::common::Line::new();
        if let Some(width) = &line.width {
            plotly_line = plotly_line.width(*width);
        }
        if let Some(shape) = &line.shape {
            plotly_line = plotly_line.shape(shape.into());
        }
        if let Some(smoothing) = &line.smoothing {
            plotly_line = plotly_line.smoothing(*smoothing);
        }
        if let Some(dash) = &line.dash {
            plotly_line = plotly_line.dash(dash.into());
        }
        if let Some(simplify) = &line.simplify {
            plotly_line = plotly_line.simplify(*simplify);
        }
        if let Some(color) = &line.color {
            plotly_line = plotly_line.color(color.to_owned());
        }
        if let Some(cauto) = &line.cauto {
            plotly_line = plotly_line.cauto(*cauto);
        }
        if let Some(cmin) = &line.cmin {
            plotly_line = plotly_line.cmin(*cmin);
        }
        if let Some(cmax) = &line.cmax {
            plotly_line = plotly_line.cmax(*cmax);
        }
        if let Some(cmid) = &line.cmid {
            plotly_line = plotly_line.cmid(*cmid);
        }
        if let Some(color_scale) = &line.color_scale {
            plotly_line = plotly_line.color_scale(color_scale.into());
        }
        if let Some(reverse_scale) = &line.reverse_scale {
            plotly_line = plotly_line.reverse_scale(*reverse_scale);
        }
        if let Some(outlier_color) = &line.outlier_color {
            plotly_line = plotly_line.outlier_color(outlier_color.to_owned());
        }
        if let Some(outlier_width) = &line.outlier_width {
            plotly_line = plotly_line.outlier_width(*outlier_width);
        }
        plotly_line
    }
}

impl From<&LineShape> for plotly::common::LineShape {
    fn from(line_shape: &LineShape) -> plotly::common::LineShape {
        match line_shape {
            LineShape::Linear => plotly::common::LineShape::Linear,
            LineShape::Spline => plotly::common::LineShape::Spline,
            LineShape::Hv => plotly::common::LineShape::Hv,
            LineShape::Vh => plotly::common::LineShape::Vh,
            LineShape::Hvh => plotly::common::LineShape::Hvh,
            LineShape::Vhv => plotly::common::LineShape::Vhv,
        }
    }
}

impl From<&DashType> for plotly::common::DashType {
    fn from(dash_type: &DashType) -> plotly::common::DashType {
        match dash_type {
            DashType::Solid => plotly::common::DashType::Solid,
            DashType::Dot => plotly::common::DashType::Dot,
            DashType::Dash => plotly::common::DashType::Dash,
            DashType::LongDash => plotly::common::DashType::LongDash,
            DashType::DashDot => plotly::common::DashType::DashDot,
            DashType::LongDashDot => plotly::common::DashType::LongDashDot,
        }
    }
}

impl From<&ColorScale> for plotly::common::ColorScale {
    fn from(color_scale: &ColorScale) -> plotly::common::ColorScale {
        match color_scale {
            ColorScale::Palette(p) => plotly::common::ColorScale::Palette(p.into()),
            ColorScale::Vector(v) => {
                plotly::common::ColorScale::Vector(v.iter().map(|cse| cse.into()).collect())
            }
        }
    }
}

impl From<&ColorScalePalette> for plotly::common::ColorScalePalette {
    fn from(color_scale_palette: &ColorScalePalette) -> plotly::common::ColorScalePalette {
        match color_scale_palette {
            ColorScalePalette::Greys => plotly::common::ColorScalePalette::Greys,
            ColorScalePalette::YlGnBu => plotly::common::ColorScalePalette::YlGnBu,
            ColorScalePalette::Greens => plotly::common::ColorScalePalette::Greens,
            ColorScalePalette::YlOrRd => plotly::common::ColorScalePalette::YlOrRd,
            ColorScalePalette::Bluered => plotly::common::ColorScalePalette::Bluered,
            ColorScalePalette::RdBu => plotly::common::ColorScalePalette::RdBu,
            ColorScalePalette::Reds => plotly::common::ColorScalePalette::Reds,
            ColorScalePalette::Blues => plotly::common::ColorScalePalette::Blues,
            ColorScalePalette::Picnic => plotly::common::ColorScalePalette::Picnic,
            ColorScalePalette::Rainbow => plotly::common::ColorScalePalette::Rainbow,
            ColorScalePalette::Portland => plotly::common::ColorScalePalette::Portland,
            ColorScalePalette::Jet => plotly::common::ColorScalePalette::Jet,
            ColorScalePalette::Hot => plotly::common::ColorScalePalette::Hot,
            ColorScalePalette::Blackbody => plotly::common::ColorScalePalette::Blackbody,
            ColorScalePalette::Earth => plotly::common::ColorScalePalette::Earth,
            ColorScalePalette::Electric => plotly::common::ColorScalePalette::Electric,
            ColorScalePalette::Viridis => plotly::common::ColorScalePalette::Viridis,
            ColorScalePalette::Cividis => plotly::common::ColorScalePalette::Cividis,
        }
    }
}

impl From<&ColorScaleElement> for plotly::common::ColorScaleElement {
    fn from(color_scale_element: &ColorScaleElement) -> plotly::common::ColorScaleElement {
        plotly::common::ColorScaleElement(color_scale_element.0, color_scale_element.1.clone())
    }
}
