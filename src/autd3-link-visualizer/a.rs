#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod error {
    use autd3_driver::error::AUTDInternalError;
    use thiserror::Error;
    #[cfg(feature = "plotters")]
    use plotters::drawing::DrawingAreaErrorKind;
    pub enum VisualizerError {
        #[error("Plot range is invalid")]
        InvalidPlotRange,
        #[cfg(feature = "plotters")]
        #[error("{0}")]
        DrawingAreaError(String),
        #[error("Not supported operation")]
        NotSupported,
        #[error("{0}")]
        IoError(std::io::Error),
        #[cfg(feature = "plotters")]
        #[error("{0}")]
        BitMapBackendError(plotters_bitmap::BitMapBackendError),
    }
    #[allow(unused_qualifications)]
    impl std::error::Error for VisualizerError {}
    #[allow(unused_qualifications)]
    impl ::core::fmt::Display for VisualizerError {
        fn fmt(&self, __formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            use thiserror::__private::AsDisplay as _;
            #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
            match self {
                VisualizerError::InvalidPlotRange {} => {
                    __formatter.write_fmt(format_args!("Plot range is invalid"))
                }
                VisualizerError::DrawingAreaError(_0) => {
                    __formatter.write_fmt(format_args!("{0}", _0.as_display()))
                }
                VisualizerError::NotSupported {} => {
                    __formatter.write_fmt(format_args!("Not supported operation"))
                }
                VisualizerError::IoError(_0) => {
                    __formatter.write_fmt(format_args!("{0}", _0.as_display()))
                }
                VisualizerError::BitMapBackendError(_0) => {
                    __formatter.write_fmt(format_args!("{0}", _0.as_display()))
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for VisualizerError {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                VisualizerError::InvalidPlotRange => {
                    ::core::fmt::Formatter::write_str(f, "InvalidPlotRange")
                }
                VisualizerError::DrawingAreaError(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "DrawingAreaError",
                        &__self_0,
                    )
                }
                VisualizerError::NotSupported => {
                    ::core::fmt::Formatter::write_str(f, "NotSupported")
                }
                VisualizerError::IoError(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "IoError",
                        &__self_0,
                    )
                }
                VisualizerError::BitMapBackendError(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BitMapBackendError",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl From<VisualizerError> for AUTDInternalError {
        fn from(val: VisualizerError) -> AUTDInternalError {
            AUTDInternalError::LinkError(val.to_string())
        }
    }
    #[cfg(feature = "plotters")]
    impl<E: std::error::Error + Send + Sync> From<DrawingAreaErrorKind<E>>
    for VisualizerError {
        fn from(value: DrawingAreaErrorKind<E>) -> Self {
            Self::DrawingAreaError(value.to_string())
        }
    }
    #[cfg(feature = "plotters")]
    impl From<plotters_bitmap::BitMapBackendError> for VisualizerError {
        fn from(value: plotters_bitmap::BitMapBackendError) -> Self {
            Self::BitMapBackendError(value)
        }
    }
    impl From<std::io::Error> for VisualizerError {
        fn from(value: std::io::Error) -> Self {
            Self::IoError(value)
        }
    }
}
mod backend {
    mod null {
        use crate::Backend;
        use autd3_driver::defined::float;
        /// Backend with no plotting
        pub struct NullBackend {}
        pub struct NullPlotConfig {}
        impl Backend for NullBackend {
            type PlotConfig = NullPlotConfig;
            fn new() -> Self {
                Self {}
            }
            fn initialize(&mut self) -> Result<(), crate::error::VisualizerError> {
                Ok(())
            }
            fn plot_1d(
                _observe_points: Vec<float>,
                _acoustic_pressures: Vec<autd3_driver::defined::Complex>,
                _resolution: float,
                _x_label: &str,
                _config: Self::PlotConfig,
            ) -> Result<(), crate::error::VisualizerError> {
                Err(crate::error::VisualizerError::NotSupported)
            }
            fn plot_2d(
                _observe_x: Vec<float>,
                _observe_y: Vec<float>,
                _acoustic_pressures: Vec<autd3_driver::defined::Complex>,
                _resolution: float,
                _x_label: &str,
                _y_label: &str,
                _config: Self::PlotConfig,
            ) -> Result<(), crate::error::VisualizerError> {
                Err(crate::error::VisualizerError::NotSupported)
            }
            fn plot_modulation(
                _modulation: Vec<float>,
                _config: Self::PlotConfig,
            ) -> Result<(), crate::error::VisualizerError> {
                Err(crate::error::VisualizerError::NotSupported)
            }
            fn plot_phase<T: autd3_driver::geometry::Transducer>(
                _config: Self::PlotConfig,
                _geometry: &autd3_driver::geometry::Geometry<T>,
                _phases: Vec<float>,
            ) -> Result<(), crate::error::VisualizerError> {
                Err(crate::error::VisualizerError::NotSupported)
            }
        }
    }
    #[cfg(feature = "plotters")]
    mod plotters {
        use std::ffi::OsString;
        use plotters::{coord::Shift, prelude::*};
        use scarlet::colormap::{ColorMap, ListedColorMap};
        use crate::{colormap, error::VisualizerError, Backend};
        use autd3_driver::{autd3_device::AUTD3, defined::float, geometry::Geometry};
        pub struct PlotConfig {
            pub figsize: (u32, u32),
            pub cbar_size: float,
            pub font_size: u32,
            pub label_area_size: u32,
            pub margin: u32,
            pub ticks_step: float,
            pub cmap: ListedColorMap,
            pub fname: OsString,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for PlotConfig {
            #[inline]
            fn clone(&self) -> PlotConfig {
                PlotConfig {
                    figsize: ::core::clone::Clone::clone(&self.figsize),
                    cbar_size: ::core::clone::Clone::clone(&self.cbar_size),
                    font_size: ::core::clone::Clone::clone(&self.font_size),
                    label_area_size: ::core::clone::Clone::clone(&self.label_area_size),
                    margin: ::core::clone::Clone::clone(&self.margin),
                    ticks_step: ::core::clone::Clone::clone(&self.ticks_step),
                    cmap: ::core::clone::Clone::clone(&self.cmap),
                    fname: ::core::clone::Clone::clone(&self.fname),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PlotConfig {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "figsize",
                    "cbar_size",
                    "font_size",
                    "label_area_size",
                    "margin",
                    "ticks_step",
                    "cmap",
                    "fname",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.figsize,
                    &self.cbar_size,
                    &self.font_size,
                    &self.label_area_size,
                    &self.margin,
                    &self.ticks_step,
                    &self.cmap,
                    &&self.fname,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "PlotConfig",
                    names,
                    values,
                )
            }
        }
        impl Default for PlotConfig {
            fn default() -> Self {
                Self {
                    figsize: (960, 640),
                    cbar_size: 0.15,
                    ticks_step: 10.,
                    label_area_size: 80,
                    margin: 10,
                    font_size: 24,
                    cmap: colormap::jet(),
                    fname: OsString::new(),
                }
            }
        }
        /// Backend using [plotters](https://github.com/plotters-rs/plotters)
        pub struct PlottersBackend {}
        impl PlottersBackend {
            fn plot_modulation_impl<B: plotters::backend::DrawingBackend>(
                root: DrawingArea<B, Shift>,
                modulation: Vec<float>,
                config: &PlotConfig,
            ) -> Result<(), crate::error::VisualizerError>
            where
                VisualizerError: From<
                    DrawingAreaErrorKind<
                        <B as plotters::backend::DrawingBackend>::ErrorType,
                    >,
                >,
            {
                root.fill(&WHITE)?;
                let mut chart = ChartBuilder::on(&root)
                    .margin(config.margin)
                    .x_label_area_size(config.label_area_size)
                    .y_label_area_size(config.label_area_size)
                    .build_cartesian_2d::<
                        _,
                        std::ops::Range<float>,
                    >(0..modulation.len(), 0.0..1.0)?;
                chart
                    .configure_mesh()
                    .disable_x_mesh()
                    .disable_y_mesh()
                    .x_label_style(
                        ("sans-serif", config.font_size).into_text_style(&root),
                    )
                    .y_label_style(
                        ("sans-serif", config.font_size).into_text_style(&root),
                    )
                    .x_desc("Index")
                    .y_desc("Modulation")
                    .draw()?;
                chart
                    .draw_series(
                        LineSeries::new(
                            modulation.iter().enumerate().map(|(i, &v)| (i, v)),
                            BLUE.stroke_width(2),
                        ),
                    )?;
                root.present()?;
                Ok(())
            }
            fn plot_1d_impl<B: plotters::backend::DrawingBackend>(
                root: &DrawingArea<B, Shift>,
                observe_points: &[float],
                acoustic_pressures: &[autd3_driver::defined::Complex],
                x_label: &str,
                yrange: (float, float),
                config: &PlotConfig,
            ) -> Result<(), crate::error::VisualizerError>
            where
                VisualizerError: From<
                    DrawingAreaErrorKind<
                        <B as plotters::backend::DrawingBackend>::ErrorType,
                    >,
                >,
            {
                root.fill(&WHITE)?;
                let xrange = observe_points
                    .iter()
                    .fold(
                        (float::MAX, float::MIN),
                        |acc, &x| { (acc.0.min(x), acc.1.max(x)) },
                    );
                let x_labels = ((xrange.1 - xrange.0).floor() / config.ticks_step)
                    as usize + 1;
                let mut chart = ChartBuilder::on(root)
                    .margin(config.margin)
                    .x_label_area_size(config.label_area_size)
                    .y_label_area_size(config.label_area_size)
                    .build_cartesian_2d(xrange.0..xrange.1, yrange.0..yrange.1)?;
                chart
                    .configure_mesh()
                    .disable_x_mesh()
                    .disable_y_mesh()
                    .x_labels(x_labels)
                    .x_label_style(
                        ("sans-serif", config.font_size).into_text_style(root),
                    )
                    .y_label_style(
                        ("sans-serif", config.font_size).into_text_style(root),
                    )
                    .x_desc(x_label)
                    .y_desc("Amplitude [-]")
                    .draw()?;
                chart
                    .draw_series(
                        LineSeries::new(
                            observe_points
                                .iter()
                                .zip(acoustic_pressures.iter())
                                .map(|(&x, v)| (x, v.norm())),
                            BLUE.stroke_width(2),
                        ),
                    )?;
                root.present()?;
                Ok(())
            }
            #[allow(clippy::too_many_arguments)]
            fn plot_2d_impl<B: plotters::backend::DrawingBackend>(
                root: &DrawingArea<B, Shift>,
                observe_points_x: &[float],
                observe_points_y: &[float],
                acoustic_pressures: &[autd3_driver::defined::Complex],
                x_label: &str,
                y_label: &str,
                zrange: (float, float),
                resolution: float,
                config: &PlotConfig,
            ) -> Result<(), crate::error::VisualizerError>
            where
                VisualizerError: From<
                    DrawingAreaErrorKind<
                        <B as plotters::backend::DrawingBackend>::ErrorType,
                    >,
                >,
            {
                root.fill(&WHITE)?;
                let main_area_size_x = (config.figsize.0 as float
                    * (1.0 - config.cbar_size)) as u32;
                let (main_area, cbar_area) = root.split_horizontally(main_area_size_x);
                let color_map_size = 1000;
                let cmap: Vec<scarlet::color::RGBColor> = config
                    .cmap
                    .transform(
                        (0..=color_map_size).map(|x| x as f64 / color_map_size as f64),
                    );
                {
                    let xrange = observe_points_x
                        .iter()
                        .fold(
                            (float::MAX, float::MIN),
                            |acc, &x| { (acc.0.min(x), acc.1.max(x)) },
                        );
                    let yrange = observe_points_y
                        .iter()
                        .fold(
                            (float::MAX, float::MIN),
                            |acc, &x| { (acc.0.min(x), acc.1.max(x)) },
                        );
                    let plot_range_x = xrange.1 - xrange.0;
                    let plot_range_y = yrange.1 - yrange.0;
                    let x_labels = (plot_range_x.floor() / config.ticks_step) as usize
                        + 1;
                    let y_labels = (plot_range_y.floor() / config.ticks_step) as usize
                        + 1;
                    let available_size_x = main_area_size_x - config.label_area_size
                        - config.margin;
                    let available_size_y = config.figsize.1 - config.label_area_size
                        - config.margin * 2;
                    let px_per_ps = (available_size_x as float / plot_range_x)
                        .min(available_size_y as float / plot_range_y);
                    let plot_size_x = (plot_range_x * px_per_ps) as u32;
                    let plot_size_y = (plot_range_y * px_per_ps) as u32;
                    let left_margin = config.margin
                        + (main_area_size_x - plot_size_x - config.label_area_size
                            - config.margin)
                            .max(0) / 2;
                    let right_margin = config.margin
                        + (main_area_size_x - plot_size_x - config.label_area_size
                            - config.margin)
                            .max(0) / 2;
                    let top_margin = config.margin
                        + (config.figsize.1 - plot_size_y - config.label_area_size
                            - config.margin)
                            .max(0) / 2;
                    let bottom_margin = config.margin
                        + (config.figsize.1 - plot_size_y - config.label_area_size
                            - config.margin)
                            .max(0) / 2;
                    let mut chart = ChartBuilder::on(&main_area)
                        .margin_left(left_margin)
                        .margin_top(top_margin)
                        .margin_bottom(bottom_margin)
                        .margin_right(right_margin)
                        .x_label_area_size(config.label_area_size)
                        .y_label_area_size(config.label_area_size)
                        .build_cartesian_2d(xrange.0..xrange.1, yrange.0..yrange.1)?;
                    chart
                        .configure_mesh()
                        .x_labels(x_labels)
                        .y_labels(y_labels)
                        .disable_x_mesh()
                        .disable_y_mesh()
                        .label_style(("sans-serif", config.font_size))
                        .x_desc(x_label)
                        .y_desc(y_label)
                        .draw()?;
                    chart
                        .draw_series(
                            ::itertools::Itertools::cartesian_product(
                                    ::itertools::__std_iter::IntoIterator::into_iter(
                                        observe_points_y,
                                    ),
                                    ::itertools::__std_iter::IntoIterator::into_iter(
                                        observe_points_x,
                                    ),
                                )
                                .zip(acoustic_pressures.iter())
                                .map(|((&y, &x), c)| {
                                    #[allow(clippy::unnecessary_cast)]
                                    let c: scarlet::color::RGBColor = config
                                        .cmap
                                        .transform_single(
                                            ((c.norm() - zrange.0) / (zrange.1 - zrange.0)) as f64,
                                        );
                                    Rectangle::new(
                                        [(x, y), (x + resolution, y + resolution)],
                                        RGBAColor(c.int_r(), c.int_g(), c.int_b(), 1.0).filled(),
                                    )
                                }),
                        )?;
                }
                {
                    let mut chart = ChartBuilder::on(&cbar_area)
                        .margin_left(config.margin)
                        .margin_top(config.margin)
                        .margin_bottom(config.margin + config.label_area_size)
                        .margin_right(config.margin)
                        .y_label_area_size(config.label_area_size)
                        .set_label_area_size(LabelAreaPosition::Left, 0)
                        .set_label_area_size(LabelAreaPosition::Right, 80)
                        .build_cartesian_2d(0i32..1i32, 0i32..color_map_size)?;
                    chart
                        .configure_mesh()
                        .disable_x_axis()
                        .disable_x_mesh()
                        .disable_y_mesh()
                        .axis_style(BLACK.stroke_width(1))
                        .label_style(("sans-serif", config.font_size))
                        .y_label_formatter(
                            &(|&v| {
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "{0:.2}",
                                            zrange.0
                                                + (zrange.1 - zrange.0) * v as float
                                                    / color_map_size as float,
                                        ),
                                    );
                                    res
                                }
                            }),
                        )
                        .y_desc("Amplitude [-]")
                        .draw()?;
                    chart
                        .draw_series(
                            cmap
                                .iter()
                                .enumerate()
                                .map(|(i, c)| {
                                    Rectangle::new(
                                        [(0, i as i32), (1, i as i32 + 1)],
                                        RGBAColor(c.int_r(), c.int_g(), c.int_b(), 1.0).filled(),
                                    )
                                }),
                        )?;
                    chart
                        .draw_series([
                            Rectangle::new(
                                [(0, 0), (1, color_map_size + 1)],
                                BLACK.stroke_width(1),
                            ),
                        ])?;
                }
                root.present()?;
                Ok(())
            }
            fn plot_phase_impl<
                T: autd3_driver::geometry::Transducer,
                B: plotters::backend::DrawingBackend,
            >(
                root: DrawingArea<B, Shift>,
                config: &PlotConfig,
                geometry: &Geometry<T>,
                phases: Vec<float>,
            ) -> Result<(), crate::error::VisualizerError>
            where
                VisualizerError: From<
                    DrawingAreaErrorKind<
                        <B as plotters::backend::DrawingBackend>::ErrorType,
                    >,
                >,
            {
                root.fill(&WHITE)?;
                let main_area_size_x = (config.figsize.0 as float
                    * (1.0 - config.cbar_size)) as u32;
                let (main_area, cbar_area) = root.split_horizontally(main_area_size_x);
                let color_map_size = 1000;
                let cmap: Vec<scarlet::color::RGBColor> = config
                    .cmap
                    .transform(
                        (0..=color_map_size).map(|x| x as f64 / color_map_size as f64),
                    );
                {
                    let p = geometry
                        .iter()
                        .flat_map(|dev| {
                            dev.iter().map(|t| (t.position().x, t.position().y))
                        })
                        .collect::<Vec<_>>();
                    let min_x = p.iter().fold(float::MAX, |acc, &(x, _)| acc.min(x))
                        - AUTD3::TRANS_SPACING / 2.0;
                    let min_y = p.iter().fold(float::MAX, |acc, &(_, y)| acc.min(y))
                        - AUTD3::TRANS_SPACING / 2.0;
                    let max_x = p.iter().fold(float::MIN, |acc, &(x, _)| acc.max(x))
                        + AUTD3::TRANS_SPACING / 2.0;
                    let max_y = p.iter().fold(float::MIN, |acc, &(_, y)| acc.max(y))
                        + AUTD3::TRANS_SPACING / 2.0;
                    let plot_range_x = max_x - min_x;
                    let plot_range_y = max_y - min_y;
                    let available_size_x = main_area_size_x - config.label_area_size
                        - config.margin;
                    let available_size_y = config.figsize.1 - config.label_area_size
                        - config.margin * 2;
                    let px_per_ps = (available_size_x as float / plot_range_x)
                        .min(available_size_y as float / plot_range_y);
                    let plot_size_x = (plot_range_x * px_per_ps) as u32;
                    let plot_size_y = (plot_range_y * px_per_ps) as u32;
                    let left_margin = config.margin
                        + (main_area_size_x - plot_size_x - config.label_area_size
                            - config.margin)
                            .max(0) / 2;
                    let right_margin = config.margin
                        + (main_area_size_x - plot_size_x - config.label_area_size
                            - config.margin)
                            .max(0) / 2;
                    let top_margin = config.margin
                        + (config.figsize.1 - plot_size_y - config.label_area_size
                            - config.margin)
                            .max(0) / 2;
                    let bottom_margin = config.margin
                        + (config.figsize.1 - plot_size_y - config.label_area_size
                            - config.margin)
                            .max(0) / 2;
                    let mut scatter_ctx = ChartBuilder::on(&main_area)
                        .margin_left(left_margin)
                        .margin_right(right_margin)
                        .margin_top(top_margin)
                        .margin_bottom(bottom_margin)
                        .x_label_area_size(config.label_area_size)
                        .y_label_area_size(config.label_area_size)
                        .build_cartesian_2d(min_x..max_x, min_y..max_y)?;
                    scatter_ctx
                        .configure_mesh()
                        .disable_x_mesh()
                        .disable_y_mesh()
                        .x_label_formatter(
                            &(|v| {
                                let res = ::alloc::fmt::format(format_args!("{0:.1}", v));
                                res
                            }),
                        )
                        .y_label_formatter(
                            &(|v| {
                                let res = ::alloc::fmt::format(format_args!("{0:.1}", v));
                                res
                            }),
                        )
                        .x_label_style(
                            ("sans-serif", config.font_size).into_text_style(&main_area),
                        )
                        .y_label_style(
                            ("sans-serif", config.font_size).into_text_style(&main_area),
                        )
                        .x_desc("x [mm]")
                        .y_desc("y [mm]")
                        .draw()?;
                    scatter_ctx
                        .draw_series(
                            p
                                .iter()
                                .zip(phases.iter())
                                .map(|(&(x, y), &p)| {
                                    let v = (p / (2.0 * autd3_driver::defined::PI)) % 1.;
                                    let c = cmap[((v * color_map_size as float) as usize)
                                        .clamp(0, cmap.len() - 1)];
                                    Circle::new(
                                        (x, y),
                                        AUTD3::TRANS_SPACING * px_per_ps / 2.0,
                                        RGBColor(c.int_r(), c.int_g(), c.int_b())
                                            .filled()
                                            .stroke_width(0),
                                    )
                                }),
                        )?;
                }
                {
                    let mut chart = ChartBuilder::on(&cbar_area)
                        .margin_left(config.margin)
                        .margin_top(config.margin)
                        .margin_bottom(config.margin + config.label_area_size)
                        .margin_right(config.margin)
                        .y_label_area_size(config.label_area_size)
                        .set_label_area_size(LabelAreaPosition::Left, 0)
                        .set_label_area_size(LabelAreaPosition::Right, 80)
                        .build_cartesian_2d(0i32..1i32, 0i32..color_map_size)?;
                    chart
                        .configure_mesh()
                        .disable_x_axis()
                        .y_labels(3)
                        .disable_x_mesh()
                        .disable_y_mesh()
                        .axis_style(BLACK.stroke_width(1))
                        .label_style(("sans-serif", config.font_size))
                        .y_label_formatter(
                            &(|&v| {
                                if v == 0 {
                                    "0".to_owned()
                                } else if v == color_map_size / 2 {
                                    "π".to_owned()
                                } else {
                                    "2π".to_owned()
                                }
                            }),
                        )
                        .draw()?;
                    chart
                        .draw_series(
                            cmap
                                .iter()
                                .enumerate()
                                .map(|(i, c)| {
                                    Rectangle::new(
                                        [(0, i as i32), (1, i as i32 + 1)],
                                        RGBAColor(c.int_r(), c.int_g(), c.int_b(), 1.0).filled(),
                                    )
                                }),
                        )?;
                    chart
                        .draw_series([
                            Rectangle::new(
                                [(0, 0), (1, color_map_size + 1)],
                                BLACK.stroke_width(1),
                            ),
                        ])?;
                }
                root.present()?;
                Ok(())
            }
        }
        impl Backend for PlottersBackend {
            type PlotConfig = PlotConfig;
            fn new() -> Self {
                Self {}
            }
            fn initialize(&mut self) -> Result<(), crate::error::VisualizerError> {
                Ok(())
            }
            fn plot_1d(
                observe_points: Vec<float>,
                acoustic_pressures: Vec<autd3_driver::defined::Complex>,
                _resolution: float,
                x_label: &str,
                config: Self::PlotConfig,
            ) -> Result<(), crate::error::VisualizerError> {
                let path = std::path::Path::new(&config.fname);
                if !path.parent().map_or(true, |p| p.exists()) {
                    std::fs::create_dir_all(path.parent().unwrap())?;
                }
                let yrange = acoustic_pressures
                    .iter()
                    .fold(
                        (float::MAX, float::MIN),
                        |acc, &x| { (acc.0.min(x.norm()), acc.1.max(x.norm())) },
                    );
                if path.extension().map_or(false, |e| e == "svg") {
                    Self::plot_1d_impl(
                        &SVGBackend::new(&config.fname, config.figsize)
                            .into_drawing_area(),
                        &observe_points,
                        &acoustic_pressures,
                        x_label,
                        yrange,
                        &config,
                    )
                } else {
                    Self::plot_1d_impl(
                        &BitMapBackend::new(&config.fname, config.figsize)
                            .into_drawing_area(),
                        &observe_points,
                        &acoustic_pressures,
                        x_label,
                        yrange,
                        &config,
                    )
                }
            }
            fn plot_2d(
                observe_x: Vec<float>,
                observe_y: Vec<float>,
                acoustic_pressures: Vec<autd3_driver::defined::Complex>,
                resolution: float,
                x_label: &str,
                y_label: &str,
                config: Self::PlotConfig,
            ) -> Result<(), crate::error::VisualizerError> {
                let path = std::path::Path::new(&config.fname);
                if !path.parent().map_or(true, |p| p.exists()) {
                    std::fs::create_dir_all(path.parent().unwrap())?;
                }
                let zrange = acoustic_pressures
                    .iter()
                    .fold(
                        (float::MAX, float::MIN),
                        |acc, &x| { (acc.0.min(x.norm()), acc.1.max(x.norm())) },
                    );
                if path.extension().map_or(false, |e| e == "svg") {
                    Self::plot_2d_impl(
                        &SVGBackend::new(&config.fname, config.figsize)
                            .into_drawing_area(),
                        &observe_x,
                        &observe_y,
                        &acoustic_pressures,
                        x_label,
                        y_label,
                        zrange,
                        resolution,
                        &config,
                    )
                } else {
                    Self::plot_2d_impl(
                        &BitMapBackend::new(&config.fname, config.figsize)
                            .into_drawing_area(),
                        &observe_x,
                        &observe_y,
                        &acoustic_pressures,
                        x_label,
                        y_label,
                        zrange,
                        resolution,
                        &config,
                    )
                }
            }
            fn plot_modulation(
                modulation: Vec<float>,
                config: Self::PlotConfig,
            ) -> Result<(), crate::error::VisualizerError> {
                let path = std::path::Path::new(&config.fname);
                if !path.parent().map_or(true, |p| p.exists()) {
                    std::fs::create_dir_all(path.parent().unwrap())?;
                }
                if path.extension().map_or(false, |e| e == "svg") {
                    Self::plot_modulation_impl(
                        SVGBackend::new(&config.fname, config.figsize)
                            .into_drawing_area(),
                        modulation,
                        &config,
                    )
                } else {
                    Self::plot_modulation_impl(
                        BitMapBackend::new(&config.fname, config.figsize)
                            .into_drawing_area(),
                        modulation,
                        &config,
                    )
                }
            }
            fn plot_phase<T: autd3_driver::geometry::Transducer>(
                config: Self::PlotConfig,
                geometry: &autd3_driver::geometry::Geometry<T>,
                phases: Vec<float>,
            ) -> Result<(), crate::error::VisualizerError> {
                let path = std::path::Path::new(&config.fname);
                if !path.parent().map_or(true, |p| p.exists()) {
                    std::fs::create_dir_all(path.parent().unwrap())?;
                }
                if path.extension().map_or(false, |e| e == "svg") {
                    Self::plot_phase_impl(
                        SVGBackend::new(&config.fname, config.figsize)
                            .into_drawing_area(),
                        &config,
                        geometry,
                        phases,
                    )
                } else {
                    Self::plot_phase_impl(
                        BitMapBackend::new(&config.fname, config.figsize)
                            .into_drawing_area(),
                        &config,
                        geometry,
                        phases,
                    )
                }
            }
        }
    }
    use crate::error::VisualizerError;
    use autd3_driver::{
        defined::{float, Complex},
        geometry::{Geometry, Transducer},
    };
    /// Plotting backend
    pub trait Backend: Send + Sync {
        type PlotConfig;
        fn new() -> Self;
        fn initialize(&mut self) -> Result<(), VisualizerError>;
        fn plot_1d(
            observe_points: Vec<float>,
            acoustic_pressures: Vec<Complex>,
            resolution: float,
            x_label: &str,
            config: Self::PlotConfig,
        ) -> Result<(), VisualizerError>;
        #[allow(clippy::too_many_arguments)]
        fn plot_2d(
            observe_x: Vec<float>,
            observe_y: Vec<float>,
            acoustic_pressures: Vec<Complex>,
            resolution: float,
            x_label: &str,
            y_label: &str,
            config: Self::PlotConfig,
        ) -> Result<(), VisualizerError>;
        fn plot_modulation(
            modulation: Vec<float>,
            config: Self::PlotConfig,
        ) -> Result<(), VisualizerError>;
        fn plot_phase<T: Transducer>(
            config: Self::PlotConfig,
            geometry: &Geometry<T>,
            phases: Vec<float>,
        ) -> Result<(), VisualizerError>;
    }
    #[cfg(feature = "plotters")]
    pub use self::plotters::*;
    pub use null::*;
}
use autd3_derive::LinkSync;
pub use backend::*;
#[cfg(feature = "plotters")]
pub mod colormap {
    use scarlet::colormap::ListedColorMap;
    /// Colormap of jet in matplotlib
    pub fn jet() -> ListedColorMap {
        ListedColorMap::new(
            [
                [0.0, 0.0, 0.5],
                [0.0, 0.0, 0.517825311942959],
                [0.0, 0.0, 0.535650623885918],
                [0.0, 0.0, 0.553475935828877],
                [0.0, 0.0, 0.571301247771836],
                [0.0, 0.0, 0.589126559714795],
                [0.0, 0.0, 0.606951871657754],
                [0.0, 0.0, 0.624777183600713],
                [0.0, 0.0, 0.642602495543672],
                [0.0, 0.0, 0.660427807486631],
                [0.0, 0.0, 0.67825311942959],
                [0.0, 0.0, 0.696078431372549],
                [0.0, 0.0, 0.713903743315508],
                [0.0, 0.0, 0.731729055258467],
                [0.0, 0.0, 0.749554367201426],
                [0.0, 0.0, 0.767379679144385],
                [0.0, 0.0, 0.785204991087344],
                [0.0, 0.0, 0.803030303030303],
                [0.0, 0.0, 0.820855614973262],
                [0.0, 0.0, 0.838680926916221],
                [0.0, 0.0, 0.85650623885918],
                [0.0, 0.0, 0.874331550802139],
                [0.0, 0.0, 0.892156862745098],
                [0.0, 0.0, 0.909982174688057],
                [0.0, 0.0, 0.927807486631016],
                [0.0, 0.0, 0.945632798573975],
                [0.0, 0.0, 0.963458110516934],
                [0.0, 0.0, 0.981283422459893],
                [0.0, 0.0, 0.999108734402852],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.00196078431372549, 1.0],
                [0.0, 0.0176470588235293, 1.0],
                [0.0, 0.03333333333333333, 1.0],
                [0.0, 0.049019607843137254, 1.0],
                [0.0, 0.06470588235294118, 1.0],
                [0.0, 0.08039215686274499, 1.0],
                [0.0, 0.09607843137254903, 1.0],
                [0.0, 0.11176470588235295, 1.0],
                [0.0, 0.12745098039215685, 1.0],
                [0.0, 0.14313725490196066, 1.0],
                [0.0, 0.1588235294117647, 1.0],
                [0.0, 0.17450980392156862, 1.0],
                [0.0, 0.19019607843137254, 1.0],
                [0.0, 0.20588235294117635, 1.0],
                [0.0, 0.22156862745098038, 1.0],
                [0.0, 0.2372549019607843, 1.0],
                [0.0, 0.2529411764705882, 1.0],
                [0.0, 0.26862745098039204, 1.0],
                [0.0, 0.28431372549019607, 1.0],
                [0.0, 0.3, 1.0],
                [0.0, 0.3156862745098039, 1.0],
                [0.0, 0.3313725490196077, 1.0],
                [0.0, 0.34705882352941175, 1.0],
                [0.0, 0.3627450980392157, 1.0],
                [0.0, 0.3784313725490196, 1.0],
                [0.0, 0.3941176470588234, 1.0],
                [0.0, 0.40980392156862744, 1.0],
                [0.0, 0.42549019607843136, 1.0],
                [0.0, 0.4411764705882353, 1.0],
                [0.0, 0.4568627450980391, 1.0],
                [0.0, 0.4725490196078431, 1.0],
                [0.0, 0.48823529411764705, 1.0],
                [0.0, 0.503921568627451, 1.0],
                [0.0, 0.5196078431372549, 1.0],
                [0.0, 0.5352941176470586, 1.0],
                [0.0, 0.5509803921568628, 1.0],
                [0.0, 0.5666666666666667, 1.0],
                [0.0, 0.5823529411764706, 1.0],
                [0.0, 0.5980392156862745, 1.0],
                [0.0, 0.6137254901960785, 1.0],
                [0.0, 0.6294117647058823, 1.0],
                [0.0, 0.6450980392156863, 1.0],
                [0.0, 0.66078431372549, 1.0],
                [0.0, 0.6764705882352942, 1.0],
                [0.0, 0.692156862745098, 1.0],
                [0.0, 0.707843137254902, 1.0],
                [0.0, 0.7235294117647059, 1.0],
                [0.0, 0.7392156862745098, 1.0],
                [0.0, 0.7549019607843137, 1.0],
                [0.0, 0.7705882352941177, 1.0],
                [0.0, 0.7862745098039213, 1.0],
                [0.0, 0.8019607843137255, 1.0],
                [0.0, 0.8176470588235294, 1.0],
                [0.0, 0.8333333333333334, 1.0],
                [0.0, 0.8490196078431372, 1.0],
                [0.0, 0.8647058823529412, 0.9962049335863378],
                [0.0, 0.8803921568627451, 0.9835547122074637],
                [0.0, 0.8960784313725491, 0.9709044908285895],
                [0.009487666034155417, 0.9117647058823527, 0.9582542694497156],
                [0.022137887413029723, 0.9274509803921569, 0.9456040480708413],
                [0.03478810879190385, 0.9431372549019608, 0.9329538266919671],
                [0.04743833017077798, 0.9588235294117647, 0.920303605313093],
                [0.06008855154965211, 0.9745098039215686, 0.9076533839342189],
                [0.07273877292852624, 0.9901960784313726, 0.8950031625553447],
                [0.08538899430740036, 1.0, 0.8823529411764706],
                [0.0980392156862745, 1.0, 0.8697027197975965],
                [0.11068943706514844, 1.0, 0.8570524984187226],
                [0.12333965844402275, 1.0, 0.8444022770398483],
                [0.13598987982289687, 1.0, 0.8317520556609741],
                [0.148640101201771, 1.0, 0.8191018342820999],
                [0.16129032258064513, 1.0, 0.8064516129032259],
                [0.17394054395951927, 1.0, 0.7938013915243517],
                [0.1865907653383934, 1.0, 0.7811511701454776],
                [0.19924098671726753, 1.0, 0.7685009487666035],
                [0.21189120809614148, 1.0, 0.7558507273877295],
                [0.2245414294750158, 1.0, 0.7432005060088551],
                [0.2371916508538899, 1.0, 0.7305502846299811],
                [0.24984187223276405, 1.0, 0.717900063251107],
                [0.26249209361163817, 1.0, 0.7052498418722328],
                [0.2751423149905123, 1.0, 0.6925996204933587],
                [0.2877925363693864, 1.0, 0.6799493991144845],
                [0.30044275774826057, 1.0, 0.6672991777356103],
                [0.3130929791271345, 1.0, 0.6546489563567364],
                [0.3257432005060088, 1.0, 0.6419987349778622],
                [0.3383934218848829, 1.0, 0.629348513598988],
                [0.3510436432637571, 1.0, 0.6166982922201139],
                [0.3636938646426312, 1.0, 0.6040480708412397],
                [0.3763440860215053, 1.0, 0.5913978494623656],
                [0.38899430740037944, 1.0, 0.5787476280834916],
                [0.4016445287792536, 1.0, 0.5660974067046174],
                [0.4142947501581275, 1.0, 0.5534471853257434],
                [0.42694497153700184, 1.0, 0.540796963946869],
                [0.43959519291587595, 1.0, 0.5281467425679949],
                [0.45224541429475007, 1.0, 0.5154965211891208],
                [0.46489563567362424, 1.0, 0.5028462998102468],
                [0.47754585705249836, 1.0, 0.4901960784313726],
                [0.4901960784313725, 1.0, 0.4775458570524984],
                [0.5028462998102466, 1.0, 0.46489563567362435],
                [0.5154965211891207, 1.0, 0.4522454142947502],
                [0.5281467425679949, 1.0, 0.439595192915876],
                [0.5407969639468686, 1.0, 0.4269449715370023],
                [0.5534471853257431, 1.0, 0.4142947501581278],
                [0.5660974067046173, 1.0, 0.4016445287792536],
                [0.5787476280834913, 1.0, 0.38899430740037955],
                [0.5913978494623655, 1.0, 0.3763440860215054],
                [0.6040480708412397, 1.0, 0.3636938646426312],
                [0.6166982922201137, 1.0, 0.35104364326375714],
                [0.6293485135989879, 1.0, 0.338393421884883],
                [0.641998734977862, 1.0, 0.3257432005060089],
                [0.6546489563567361, 1.0, 0.31309297912713474],
                [0.6672991777356103, 1.0, 0.30044275774826057],
                [0.6799493991144844, 1.0, 0.2877925363693865],
                [0.6925996204933585, 1.0, 0.27514231499051234],
                [0.7052498418722326, 1.0, 0.26249209361163817],
                [0.7179000632511068, 1.0, 0.2498418722327641],
                [0.730550284629981, 1.0, 0.23719165085388993],
                [0.7432005060088547, 1.0, 0.2245414294750162],
                [0.7558507273877292, 1.0, 0.2118912080961417],
                [0.7685009487666034, 1.0, 0.19924098671726753],
                [0.7811511701454774, 1.0, 0.18659076533839347],
                [0.7938013915243516, 1.0, 0.1739405439595193],
                [0.8064516129032256, 1.0, 0.16129032258064513],
                [0.8191018342820998, 1.0, 0.14864010120177107],
                [0.831752055660974, 1.0, 0.1359898798228969],
                [0.844402277039848, 1.0, 0.12333965844402273],
                [0.8570524984187222, 1.0, 0.11068943706514867],
                [0.8697027197975963, 1.0, 0.0980392156862745],
                [0.8823529411764705, 1.0, 0.08538899430740043],
                [0.8950031625553446, 1.0, 0.07273877292852626],
                [0.9076533839342187, 1.0, 0.06008855154965209],
                [0.9203036053130929, 1.0, 0.04743833017077803],
                [0.932953826691967, 1.0, 0.03478810879190386],
                [0.9456040480708408, 0.9883805374001459, 0.022137887413030133],
                [0.9582542694497153, 0.973856209150327, 0.009487666034155628],
                [0.9709044908285893, 0.9593318809005086, 0.0],
                [0.9835547122074635, 0.9448075526506902, 0.0],
                [0.9962049335863377, 0.9302832244008717, 0.0],
                [1.0, 0.9157588961510532, 0.0],
                [1.0, 0.9012345679012348, 0.0],
                [1.0, 0.8867102396514164, 0.0],
                [1.0, 0.872185911401598, 0.0],
                [1.0, 0.8576615831517794, 0.0],
                [1.0, 0.843137254901961, 0.0],
                [1.0, 0.8286129266521426, 0.0],
                [1.0, 0.8140885984023241, 0.0],
                [1.0, 0.7995642701525056, 0.0],
                [1.0, 0.7850399419026872, 0.0],
                [1.0, 0.7705156136528688, 0.0],
                [1.0, 0.7559912854030507, 0.0],
                [1.0, 0.741466957153232, 0.0],
                [1.0, 0.7269426289034134, 0.0],
                [1.0, 0.712418300653595, 0.0],
                [1.0, 0.6978939724037765, 0.0],
                [1.0, 0.6833696441539581, 0.0],
                [1.0, 0.6688453159041396, 0.0],
                [1.0, 0.6543209876543212, 0.0],
                [1.0, 0.6397966594045028, 0.0],
                [1.0, 0.6252723311546844, 0.0],
                [1.0, 0.6107480029048659, 0.0],
                [1.0, 0.5962236746550474, 0.0],
                [1.0, 0.5816993464052289, 0.0],
                [1.0, 0.5671750181554105, 0.0],
                [1.0, 0.5526506899055921, 0.0],
                [1.0, 0.5381263616557737, 0.0],
                [1.0, 0.5236020334059556, 0.0],
                [1.0, 0.5090777051561368, 0.0],
                [1.0, 0.4945533769063183, 0.0],
                [1.0, 0.48002904865649987, 0.0],
                [1.0, 0.46550472040668145, 0.0],
                [1.0, 0.4509803921568629, 0.0],
                [1.0, 0.4364560639070445, 0.0],
                [1.0, 0.4219317356572261, 0.0],
                [1.0, 0.40740740740740755, 0.0],
                [1.0, 0.39288307915758913, 0.0],
                [1.0, 0.3783587509077707, 0.0],
                [1.0, 0.3638344226579523, 0.0],
                [1.0, 0.34931009440813376, 0.0],
                [1.0, 0.33478576615831535, 0.0],
                [1.0, 0.3202614379084969, 0.0],
                [1.0, 0.3057371096586785, 0.0],
                [1.0, 0.2912127814088604, 0.0],
                [1.0, 0.27668845315904156, 0.0],
                [1.0, 0.26216412490922314, 0.0],
                [1.0, 0.24763979665940472, 0.0],
                [1.0, 0.2331154684095862, 0.0],
                [1.0, 0.21859114015976777, 0.0],
                [1.0, 0.20406681190994935, 0.0],
                [1.0, 0.18954248366013093, 0.0],
                [1.0, 0.1750181554103124, 0.0],
                [1.0, 0.16049382716049398, 0.0],
                [1.0, 0.14596949891067557, 0.0],
                [1.0, 0.13144517066085715, 0.0],
                [1.0, 0.11692084241103862, 0.0],
                [1.0, 0.1023965141612202, 0.0],
                [1.0, 0.08787218591140178, 0.0],
                [0.9991087344028523, 0.07334785766158336, 0.0],
                [0.9812834224598939, 0.058823529411765274, 0.0],
                [0.9634581105169343, 0.04429920116194641, 0.0],
                [0.9456327985739753, 0.029774872912127992, 0.0],
                [0.9278074866310163, 0.015250544662309573, 0.0],
                [0.9099821746880573, 0.0007262164124910431, 0.0],
                [0.8921568627450983, 0.0, 0.0],
                [0.8743315508021392, 0.0, 0.0],
                [0.8565062388591802, 0.0, 0.0],
                [0.8386809269162212, 0.0, 0.0],
                [0.8208556149732622, 0.0, 0.0],
                [0.8030303030303032, 0.0, 0.0],
                [0.7852049910873442, 0.0, 0.0],
                [0.7673796791443852, 0.0, 0.0],
                [0.7495543672014262, 0.0, 0.0],
                [0.7317290552584672, 0.0, 0.0],
                [0.7139037433155082, 0.0, 0.0],
                [0.6960784313725497, 0.0, 0.0],
                [0.6782531194295901, 0.0, 0.0],
                [0.6604278074866311, 0.0, 0.0],
                [0.6426024955436721, 0.0, 0.0],
                [0.6247771836007131, 0.0, 0.0],
                [0.606951871657754, 0.0, 0.0],
                [0.589126559714795, 0.0, 0.0],
                [0.571301247771836, 0.0, 0.0],
                [0.553475935828877, 0.0, 0.0],
                [0.535650623885918, 0.0, 0.0],
                [0.517825311942959, 0.0, 0.0],
                [0.5, 0.0, 0.0],
            ]
                .into_iter(),
        )
    }
}
use std::{marker::PhantomData, time::Duration};
use autd3_driver::{
    acoustics::{
        directivity::{Directivity, Sphere},
        propagate,
    },
    cpu::{RxMessage, TxDatagram},
    defined::{float, Complex, PI},
    error::AUTDInternalError, geometry::{Geometry, Transducer, Vector3},
    link::{Link, LinkBuilder},
};
use autd3_firmware_emulator::CPUEmulator;
#[cfg(feature = "plotters")]
pub use scarlet::colormap::ListedColorMap;
use error::VisualizerError;
/// Link to monitoring the status of AUTD and acoustic field
pub struct Visualizer<D, B>
where
    D: Directivity,
    B: Backend,
{
    is_open: bool,
    timeout: Duration,
    cpus: Vec<CPUEmulator>,
    _d: PhantomData<D>,
    _b: PhantomData<B>,
}
#[cfg(feature = "sync")]
pub struct VisualizerSync<D, B>
where
    D: Directivity,
    B: Backend,
{
    inner: Visualizer<D, B>,
    runtime: tokio::runtime::Runtime,
}
#[cfg(feature = "sync")]
pub struct VisualizerSyncBuilder<D, B>
where
    D: Directivity,
    B: Backend,
{
    inner: VisualizerBuilder<D, B>,
    runtime: tokio::runtime::Runtime,
}
#[cfg(feature = "sync")]
impl<D, B> autd3_driver::link::LinkSync for VisualizerSync<D, B>
where
    D: Directivity,
    B: Backend,
{
    fn close(&mut self) -> Result<(), autd3_driver::error::AUTDInternalError> {
        self.runtime.block_on(self.inner.close())
    }
    fn send(
        &mut self,
        tx: &autd3_driver::cpu::TxDatagram,
    ) -> Result<bool, autd3_driver::error::AUTDInternalError> {
        self.runtime.block_on(self.inner.send(tx))
    }
    fn receive(
        &mut self,
        rx: &mut [autd3_driver::cpu::RxMessage],
    ) -> Result<bool, autd3_driver::error::AUTDInternalError> {
        self.runtime.block_on(self.inner.receive(rx))
    }
    fn is_open(&self) -> bool {
        self.inner.is_open()
    }
    fn timeout(&self) -> std::time::Duration {
        self.inner.timeout()
    }
    fn send_receive(
        &mut self,
        tx: &autd3_driver::cpu::TxDatagram,
        rx: &mut [autd3_driver::cpu::RxMessage],
        timeout: Option<std::time::Duration>,
    ) -> Result<bool, autd3_driver::error::AUTDInternalError> {
        self.runtime.block_on(self.inner.send_receive(tx, rx, timeout))
    }
    fn wait_msg_processed(
        &mut self,
        tx: &autd3_driver::cpu::TxDatagram,
        rx: &mut [autd3_driver::cpu::RxMessage],
        timeout: std::time::Duration,
    ) -> Result<bool, autd3_driver::error::AUTDInternalError> {
        self.runtime.block_on(self.inner.wait_msg_processed(tx, rx, timeout))
    }
}
#[cfg(feature = "sync")]
impl<D, B, T: autd3_driver::geometry::Transducer> autd3_driver::link::LinkSyncBuilder<T>
for VisualizerSyncBuilder<D, B>
where
    D: Directivity,
    B: Backend,
{
    type L = VisualizerSync<D, B>;
    fn open(
        self,
        geometry: &autd3_driver::geometry::Geometry<T>,
    ) -> Result<Self::L, autd3_driver::error::AUTDInternalError> {
        let Self { inner, runtime } = self;
        let inner = runtime.block_on(inner.open(geometry))?;
        Ok(Self::L { inner, runtime })
    }
}
#[cfg(feature = "sync")]
impl<D, B> VisualizerBuilder<D, B>
where
    D: Directivity,
    B: Backend,
{
    pub fn blocking(self) -> VisualizerSyncBuilder {
        VisualizerSyncBuilder {
            inner: self,
            runtime: tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
        }
    }
}
pub struct VisualizerBuilder<D, B>
where
    D: Directivity,
    B: Backend,
{
    backend: B,
    timeout: Duration,
    _d: PhantomData<D>,
}
impl<T: Transducer, D: Directivity, B: Backend> LinkBuilder<T>
for VisualizerBuilder<D, B> {
    type L = Visualizer<D, B>;
    #[allow(unused_mut)]
    #[allow(
        clippy::async_yields_async,
        clippy::diverging_sub_expression,
        clippy::let_unit_value,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn open<'life0, 'async_trait>(
        self,
        geometry: &'life0 Geometry<T>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = Result<Self::L, AUTDInternalError>,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                Result<Self::L, AUTDInternalError>,
            > {
                return __ret;
            }
            let mut __self = self;
            let __ret: Result<Self::L, AUTDInternalError> = {
                let VisualizerBuilder { mut backend, timeout, .. } = __self;
                backend.initialize()?;
                Ok(Self::L {
                    is_open: true,
                    timeout,
                    cpus: geometry
                        .iter()
                        .enumerate()
                        .map(|(i, dev)| {
                            let mut cpu = CPUEmulator::new(i, dev.num_transducers());
                            cpu.init();
                            cpu
                        })
                        .collect(),
                    _d: PhantomData,
                    _b: PhantomData,
                })
            };
            #[allow(unreachable_code)] __ret
        })
    }
}
pub struct PlotRange {
    pub x_range: std::ops::Range<float>,
    pub y_range: std::ops::Range<float>,
    pub z_range: std::ops::Range<float>,
    pub resolution: float,
}
#[automatically_derived]
impl ::core::clone::Clone for PlotRange {
    #[inline]
    fn clone(&self) -> PlotRange {
        PlotRange {
            x_range: ::core::clone::Clone::clone(&self.x_range),
            y_range: ::core::clone::Clone::clone(&self.y_range),
            z_range: ::core::clone::Clone::clone(&self.z_range),
            resolution: ::core::clone::Clone::clone(&self.resolution),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for PlotRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "PlotRange",
            "x_range",
            &self.x_range,
            "y_range",
            &self.y_range,
            "z_range",
            &self.z_range,
            "resolution",
            &&self.resolution,
        )
    }
}
impl PlotRange {
    fn n(range: &std::ops::Range<float>, resolution: float) -> usize {
        ((range.end - range.start) / resolution).floor() as usize + 1
    }
    fn nx(&self) -> usize {
        Self::n(&self.x_range, self.resolution)
    }
    fn ny(&self) -> usize {
        Self::n(&self.y_range, self.resolution)
    }
    fn nz(&self) -> usize {
        Self::n(&self.z_range, self.resolution)
    }
    fn is_1d(&self) -> bool {
        match (self.nx(), self.ny(), self.nz()) {
            (_, 1, 1) | (1, _, 1) | (1, 1, _) => true,
            _ => false,
        }
    }
    fn is_2d(&self) -> bool {
        if self.is_1d() {
            return false;
        }
        match (self.nx(), self.ny(), self.nz()) {
            (1, _, _) | (_, 1, _) | (_, _, 1) => true,
            _ => false,
        }
    }
    fn observe(n: usize, start: float, resolution: float) -> Vec<float> {
        (0..n).map(|i| start + resolution * i as float).collect()
    }
    fn observe_x(&self) -> Vec<float> {
        Self::observe(self.nx(), self.x_range.start, self.resolution)
    }
    fn observe_y(&self) -> Vec<float> {
        Self::observe(self.ny(), self.y_range.start, self.resolution)
    }
    fn observe_z(&self) -> Vec<float> {
        Self::observe(self.nz(), self.z_range.start, self.resolution)
    }
    fn observe_points(&self) -> Vec<Vector3> {
        match (self.nx(), self.ny(), self.nz()) {
            (_, 1, 1) => {
                self.observe_x()
                    .iter()
                    .map(|&x| Vector3::new(x, self.y_range.start, self.z_range.start))
                    .collect()
            }
            (1, _, 1) => {
                self.observe_y()
                    .iter()
                    .map(|&y| Vector3::new(self.x_range.start, y, self.z_range.start))
                    .collect()
            }
            (1, 1, _) => {
                self.observe_z()
                    .iter()
                    .map(|&z| Vector3::new(self.x_range.start, self.y_range.start, z))
                    .collect()
            }
            (_, _, 1) => {
                ::itertools::Itertools::cartesian_product(
                        ::itertools::__std_iter::IntoIterator::into_iter(
                            self.observe_y(),
                        ),
                        ::itertools::__std_iter::IntoIterator::into_iter(
                            self.observe_x(),
                        ),
                    )
                    .map(|(y, x)| Vector3::new(x, y, self.z_range.start))
                    .collect()
            }
            (_, 1, _) => {
                ::itertools::Itertools::cartesian_product(
                        ::itertools::__std_iter::IntoIterator::into_iter(
                            self.observe_x(),
                        ),
                        ::itertools::__std_iter::IntoIterator::into_iter(
                            self.observe_z(),
                        ),
                    )
                    .map(|(x, z)| Vector3::new(x, self.y_range.start, z))
                    .collect()
            }
            (1, _, _) => {
                ::itertools::Itertools::cartesian_product(
                        ::itertools::__std_iter::IntoIterator::into_iter(
                            self.observe_z(),
                        ),
                        ::itertools::__std_iter::IntoIterator::into_iter(
                            self.observe_y(),
                        ),
                    )
                    .map(|(z, y)| Vector3::new(self.x_range.start, y, z))
                    .collect()
            }
            (_, _, _) => {
                ::itertools::cons_tuples(
                        ::itertools::Itertools::cartesian_product(
                            ::itertools::__std_iter::IntoIterator::into_iter(
                                ::itertools::Itertools::cartesian_product(
                                    ::itertools::__std_iter::IntoIterator::into_iter(
                                        self.observe_z(),
                                    ),
                                    ::itertools::__std_iter::IntoIterator::into_iter(
                                        self.observe_y(),
                                    ),
                                ),
                            ),
                            ::itertools::__std_iter::IntoIterator::into_iter(
                                self.observe_x(),
                            ),
                        ),
                    )
                    .map(|(z, y, x)| Vector3::new(x, y, z))
                    .collect()
            }
        }
    }
}
impl Visualizer<Sphere, PlottersBackend> {
    pub fn builder() -> VisualizerBuilder<Sphere, PlottersBackend> {
        VisualizerBuilder {
            backend: PlottersBackend::new(),
            timeout: Duration::ZERO,
            _d: PhantomData,
        }
    }
}
#[cfg(feature = "plotters")]
impl Visualizer<Sphere, PlottersBackend> {
    /// Constructor with Plotters backend
    pub fn plotters() -> VisualizerBuilder<Sphere, PlottersBackend> {
        Self::builder()
    }
}
impl Visualizer<Sphere, NullBackend> {
    /// Constructor with Null backend
    pub fn null() -> VisualizerBuilder<Sphere, NullBackend> {
        VisualizerBuilder {
            backend: NullBackend::new(),
            timeout: Duration::ZERO,
            _d: PhantomData,
        }
    }
}
impl<D: Directivity, B: Backend> VisualizerBuilder<D, B> {
    /// Set directivity
    pub fn with_directivity<U: Directivity>(self) -> VisualizerBuilder<U, B> {
        VisualizerBuilder {
            backend: self.backend,
            timeout: self.timeout,
            _d: PhantomData,
        }
    }
    /// Set backend
    pub fn with_backend<U: Backend>(self) -> VisualizerBuilder<D, U> {
        VisualizerBuilder {
            backend: U::new(),
            timeout: self.timeout,
            _d: PhantomData,
        }
    }
}
impl<D: Directivity, B: Backend> Visualizer<D, B> {
    /// Get phases of transducers
    ///
    /// # Arguments
    ///
    /// * `idx` - Index of STM. If you use Gain, this value should be 0.
    ///
    pub fn phases_of(&self, idx: usize) -> Vec<float> {
        self.cpus
            .iter()
            .flat_map(|cpu| {
                cpu.fpga()
                    .duties_and_phases(idx)
                    .iter()
                    .zip(cpu.fpga().cycles().iter())
                    .map(|(&d, &c)| 2. * PI * d.1 as float / c as float)
                    .collect::<Vec<_>>()
            })
            .collect()
    }
    /// Get duty ratios of transducers
    ///
    /// # Arguments
    ///
    /// * `idx` - Index of STM. If you use Gain, this value should be 0.
    ///
    pub fn duties_of(&self, idx: usize) -> Vec<float> {
        self.cpus
            .iter()
            .flat_map(|cpu| {
                cpu.fpga()
                    .duties_and_phases(idx)
                    .iter()
                    .zip(cpu.fpga().cycles().iter())
                    .map(|(&d, &c)| (PI * d.0 as float / c as float).sin())
                    .collect::<Vec<_>>()
            })
            .collect()
    }
    /// Get phases of transducers
    pub fn phases(&self) -> Vec<float> {
        self.phases_of(0)
    }
    /// Get duty ratios of transducers
    pub fn duties(&self) -> Vec<float> {
        self.duties_of(0)
    }
    /// Get raw modulation data
    pub fn modulation_raw(&self) -> Vec<float> {
        self.cpus[0].fpga().modulation().iter().map(|&x| x as float / 255.).collect()
    }
    /// Get modulation data
    pub fn modulation(&self) -> Vec<float> {
        self.modulation_raw().iter().map(|&x| (0.5 * PI * x).sin()).collect()
    }
    /// Calculate acoustic field at specified points
    ///
    /// # Arguments
    ///
    /// * `observe_points` - Observe points iterator
    /// * `geometry` - Geometry
    ///
    pub fn calc_field<'a, T: Transducer, I: IntoIterator<Item = &'a Vector3>>(
        &self,
        observe_points: I,
        geometry: &Geometry<T>,
    ) -> Vec<Complex> {
        self.calc_field_of::<T, I>(observe_points, geometry, 0)
    }
    /// Calculate acoustic field at specified points
    ///
    /// # Arguments
    ///
    /// * `observe_points` - Observe points iterator
    /// * `geometry` - Geometry
    /// * `idx` - Index of STM. If you use Gain, this value should be 0.
    ///
    pub fn calc_field_of<'a, T: Transducer, I: IntoIterator<Item = &'a Vector3>>(
        &self,
        observe_points: I,
        geometry: &Geometry<T>,
        idx: usize,
    ) -> Vec<Complex> {
        observe_points
            .into_iter()
            .map(|target| {
                self.cpus
                    .iter()
                    .enumerate()
                    .fold(
                        Complex::new(0., 0.),
                        |acc, (i, cpu)| {
                            let sound_speed = geometry[i].sound_speed;
                            let drives = cpu.fpga().duties_and_phases(idx);
                            acc
                                + geometry[i]
                                    .iter()
                                    .zip(drives.iter())
                                    .fold(
                                        Complex::new(0., 0.),
                                        |acc, (t, d)| {
                                            let amp = (PI * d.0 as float / t.cycle() as float).sin();
                                            let phase = 2. * PI * d.1 as float / t.cycle() as float;
                                            acc
                                                + propagate::<D, T>(t, 0.0, sound_speed, target)
                                                    * Complex::from_polar(amp, phase)
                                        },
                                    )
                        },
                    )
            })
            .collect()
    }
    /// Plot acoustic field
    ///
    /// # Arguments
    ///
    /// * `range` - Plot range
    /// * `config` - Plot configuration
    /// * `geometry` - Geometry
    ///
    pub fn plot_field<T: Transducer>(
        &self,
        config: B::PlotConfig,
        range: PlotRange,
        geometry: &Geometry<T>,
    ) -> Result<(), VisualizerError> {
        self.plot_field_of(config, range, geometry, 0)
    }
    /// Plot acoustic field
    ///
    /// # Arguments
    ///
    /// * `config` - Plot configuration
    /// * `range` - Plot range
    /// * `geometry` - Geometry
    /// * `idx` - Index of STM. If you use Gain, this value should be 0.
    ///
    pub fn plot_field_of<T: Transducer>(
        &self,
        config: B::PlotConfig,
        range: PlotRange,
        geometry: &Geometry<T>,
        idx: usize,
    ) -> Result<(), VisualizerError> {
        let observe_points = range.observe_points();
        let acoustic_pressures = self.calc_field_of(&observe_points, geometry, idx);
        if range.is_1d() {
            let (observe, label) = match (range.nx(), range.ny(), range.nz()) {
                (_, 1, 1) => (range.observe_x(), "x [mm]"),
                (1, _, 1) => (range.observe_y(), "y [mm]"),
                (1, 1, _) => (range.observe_z(), "z [mm]"),
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            };
            B::plot_1d(observe, acoustic_pressures, range.resolution, label, config)
        } else if range.is_2d() {
            let (observe_x, x_label) = match (range.nx(), range.ny(), range.nz()) {
                (_, _, 1) => (range.observe_x(), "x [mm]"),
                (1, _, _) => (range.observe_y(), "y [mm]"),
                (_, 1, _) => (range.observe_z(), "z [mm]"),
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            };
            let (observe_y, y_label) = match (range.nx(), range.ny(), range.nz()) {
                (_, _, 1) => (range.observe_y(), "y [mm]"),
                (1, _, _) => (range.observe_z(), "z [mm]"),
                (_, 1, _) => (range.observe_x(), "x [mm]"),
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            };
            B::plot_2d(
                observe_x,
                observe_y,
                acoustic_pressures,
                range.resolution,
                x_label,
                y_label,
                config,
            )
        } else {
            Err(VisualizerError::InvalidPlotRange)
        }
    }
    /// Plot transducers with phases
    ///
    /// # Arguments
    ///
    /// * `config` - Plot configuration
    /// * `geometry` - Geometry
    ///
    pub fn plot_phase<T: Transducer>(
        &self,
        config: B::PlotConfig,
        geometry: &Geometry<T>,
    ) -> Result<(), VisualizerError> {
        self.plot_phase_of(config, geometry, 0)
    }
    /// Plot transducers with phases
    ///
    /// # Arguments
    ///
    /// * `config` - Plot configuration
    /// * `geometry` - Geometry
    /// * `idx` - Index of STM. If you use Gain, this value should be 0.
    ///
    pub fn plot_phase_of<T: Transducer>(
        &self,
        config: B::PlotConfig,
        geometry: &Geometry<T>,
        idx: usize,
    ) -> Result<(), VisualizerError> {
        let phases = self.phases_of(idx);
        B::plot_phase(config, geometry, phases)
    }
    /// Plot modulation data
    pub fn plot_modulation(&self, config: B::PlotConfig) -> Result<(), VisualizerError> {
        B::plot_modulation(self.modulation(), config)?;
        Ok(())
    }
    /// Plot raw modulation data
    pub fn plot_modulation_raw(
        &self,
        config: B::PlotConfig,
    ) -> Result<(), VisualizerError> {
        B::plot_modulation(self.modulation_raw(), config)?;
        Ok(())
    }
}
impl<D: Directivity, B: Backend> Link for Visualizer<D, B> {
    #[allow(
        clippy::async_yields_async,
        clippy::diverging_sub_expression,
        clippy::let_unit_value,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn close<'life0, 'async_trait>(
        &'life0 mut self,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = Result<(), AUTDInternalError>,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                Result<(), AUTDInternalError>,
            > {
                return __ret;
            }
            let mut __self = self;
            let __ret: Result<(), AUTDInternalError> = {
                if !__self.is_open {
                    return Ok(());
                }
                __self.is_open = false;
                Ok(())
            };
            #[allow(unreachable_code)] __ret
        })
    }
    #[allow(
        clippy::async_yields_async,
        clippy::diverging_sub_expression,
        clippy::let_unit_value,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn send<'life0, 'life1, 'async_trait>(
        &'life0 mut self,
        tx: &'life1 TxDatagram,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = Result<bool, AUTDInternalError>,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                Result<bool, AUTDInternalError>,
            > {
                return __ret;
            }
            let mut __self = self;
            let __ret: Result<bool, AUTDInternalError> = {
                if !__self.is_open {
                    return Ok(false);
                }
                __self
                    .cpus
                    .iter_mut()
                    .for_each(|cpu| {
                        cpu.send(tx);
                    });
                Ok(true)
            };
            #[allow(unreachable_code)] __ret
        })
    }
    #[allow(
        clippy::async_yields_async,
        clippy::diverging_sub_expression,
        clippy::let_unit_value,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn receive<'life0, 'life1, 'async_trait>(
        &'life0 mut self,
        rx: &'life1 mut [RxMessage],
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = Result<bool, AUTDInternalError>,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                Result<bool, AUTDInternalError>,
            > {
                return __ret;
            }
            let mut __self = self;
            let __ret: Result<bool, AUTDInternalError> = {
                if !__self.is_open {
                    return Ok(false);
                }
                __self
                    .cpus
                    .iter_mut()
                    .for_each(|cpu| {
                        rx[cpu.idx()].ack = cpu.ack();
                        rx[cpu.idx()].data = cpu.rx_data();
                    });
                Ok(true)
            };
            #[allow(unreachable_code)] __ret
        })
    }
    fn is_open(&self) -> bool {
        self.is_open
    }
    fn timeout(&self) -> Duration {
        self.timeout
    }
}
