/*
 * File: plotters.rs
 * Project: backend
 * Created Date: 16/07/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 27/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{ffi::OsString, io::Write};

use scarlet::colormap::{ColorMap, ListedColorMap};

use plotters::{coord::Shift, prelude::*};

use crate::{colormap, error::MonitorError, Backend, Config};

use autd3_core::{autd3_device::AUTD3, float};

#[derive(Clone, Debug)]
pub struct PlotConfig {
    pub figsize: (u32, u32),
    pub cbar_size: float,
    pub fontsize: u32,
    pub label_area_size: u32,
    pub margin: u32,
    pub font_size: u32,
    pub ticks_step: float,
    pub cmap: ListedColorMap,
    pub fname: OsString,
    pub interval: u32,
    pub print_progress: bool,
}

impl Default for PlotConfig {
    fn default() -> Self {
        Self {
            figsize: (960, 640),
            cbar_size: 0.15,
            fontsize: 12,
            ticks_step: 10.,
            label_area_size: 80,
            margin: 10,
            font_size: 24,
            cmap: colormap::jet(),
            fname: OsString::new(),
            interval: 100,
            print_progress: false,
        }
    }
}

impl Config for PlotConfig {
    fn print_progress(&self) -> bool {
        self.print_progress
    }
}

/// Backend using [plotters](https://github.com/plotters-rs/plotters)
pub struct PlottersBackend {}

impl PlottersBackend {
    fn plot_modulation_impl<B: plotters::backend::DrawingBackend>(
        root: DrawingArea<B, Shift>,
        modulation: Vec<float>,
        config: &PlotConfig,
    ) -> Result<(), crate::error::MonitorError>
    where
        MonitorError:
            From<DrawingAreaErrorKind<<B as plotters::backend::DrawingBackend>::ErrorType>>,
    {
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .margin(config.margin)
            .x_label_area_size(config.label_area_size)
            .y_label_area_size(config.label_area_size)
            .build_cartesian_2d(0..modulation.len(), 0.0..1.0)?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .x_label_style(("sans-serif", config.font_size).into_text_style(&root))
            .y_label_style(("sans-serif", config.font_size).into_text_style(&root))
            .x_desc("Index")
            .y_desc("Modulation")
            .draw()?;

        chart.draw_series(LineSeries::new(
            modulation.iter().enumerate().map(|(i, &v)| (i, v)),
            BLUE.stroke_width(2),
        ))?;

        root.present()?;

        Ok(())
    }

    fn plot_1d_impl<B: plotters::backend::DrawingBackend>(
        root: &DrawingArea<B, Shift>,
        observe_points: &[float],
        acoustic_pressures: &[autd3_core::acoustics::Complex],
        x_label: &str,
        yrange: (float, float),
        config: &PlotConfig,
    ) -> Result<(), crate::error::MonitorError>
    where
        MonitorError:
            From<DrawingAreaErrorKind<<B as plotters::backend::DrawingBackend>::ErrorType>>,
    {
        root.fill(&WHITE)?;

        let xrange = observe_points
            .iter()
            .fold((float::MAX, float::MIN), |acc, &x| {
                (acc.0.min(x), acc.1.max(x))
            });

        let x_labels = ((xrange.1 - xrange.0).floor() / config.ticks_step) as usize + 1;

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
            .x_label_style(("sans-serif", config.font_size).into_text_style(root))
            .y_label_style(("sans-serif", config.font_size).into_text_style(root))
            .x_desc(x_label)
            .y_desc("Amplitude [-]")
            .draw()?;

        chart.draw_series(LineSeries::new(
            observe_points
                .iter()
                .zip(acoustic_pressures.iter())
                .map(|(&x, v)| (x, v.norm())),
            BLUE.stroke_width(2),
        ))?;

        root.present()?;

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn plot_2d_impl<B: plotters::backend::DrawingBackend>(
        root: &DrawingArea<B, Shift>,
        observe_points_x: &[float],
        observe_points_y: &[float],
        acoustic_pressures: &[autd3_core::acoustics::Complex],
        x_label: &str,
        y_label: &str,
        zrange: (float, float),
        resolution: float,
        config: &PlotConfig,
    ) -> Result<(), crate::error::MonitorError>
    where
        MonitorError:
            From<DrawingAreaErrorKind<<B as plotters::backend::DrawingBackend>::ErrorType>>,
    {
        root.fill(&WHITE)?;

        let main_area_size_x = (config.figsize.0 as float * (1.0 - config.cbar_size)) as u32;

        let (main_area, cbar_area) = root.split_horizontally(main_area_size_x);

        let color_map_size = 1000;
        let cmap: Vec<scarlet::color::RGBColor> = config
            .cmap
            .transform((0..=color_map_size).map(|x| x as float / color_map_size as float));

        {
            let xrange = observe_points_x
                .iter()
                .fold((float::MAX, float::MIN), |acc, &x| {
                    (acc.0.min(x), acc.1.max(x))
                });
            let yrange = observe_points_y
                .iter()
                .fold((float::MAX, float::MIN), |acc, &x| {
                    (acc.0.min(x), acc.1.max(x))
                });

            let plot_range_x = xrange.1 - xrange.0;
            let plot_range_y = yrange.1 - yrange.0;

            let x_labels = (plot_range_x.floor() / config.ticks_step) as usize + 1;
            let y_labels = (plot_range_y.floor() / config.ticks_step) as usize + 1;

            let available_size_x = main_area_size_x - config.label_area_size - config.margin;
            let available_size_y = config.figsize.1 - config.label_area_size - config.margin * 2;

            let px_per_ps = (available_size_x as float / plot_range_x)
                .min(available_size_y as float / plot_range_y);

            let plot_size_x = (plot_range_x * px_per_ps) as u32;
            let plot_size_y = (plot_range_y * px_per_ps) as u32;

            let left_margin = config.margin
                + (main_area_size_x - plot_size_x - config.label_area_size - config.margin).max(0)
                    / 2;
            let right_margin = config.margin
                + (main_area_size_x - plot_size_x - config.label_area_size - config.margin).max(0)
                    / 2;
            let top_margin = config.margin
                + (config.figsize.1 - plot_size_y - config.label_area_size - config.margin).max(0)
                    / 2;
            let bottom_margin = config.margin
                + (config.figsize.1 - plot_size_y - config.label_area_size - config.margin).max(0)
                    / 2;

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

            chart.draw_series(
                itertools::iproduct!(observe_points_y, observe_points_x)
                    .zip(acoustic_pressures.iter())
                    .map(|((&y, &x), c)| {
                        let c: scarlet::color::RGBColor = config
                            .cmap
                            .transform_single((c.norm() - zrange.0) / (zrange.1 - zrange.0));
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
                .y_label_formatter(&|&v| {
                    format!(
                        "{:.2}",
                        zrange.0 + (zrange.1 - zrange.0) * v as float / color_map_size as float
                    )
                })
                .y_desc("Amplitude [-]")
                .draw()?;

            chart.draw_series(cmap.iter().enumerate().map(|(i, c)| {
                Rectangle::new(
                    [(0, i as i32), (1, i as i32 + 1)],
                    RGBAColor(c.int_r(), c.int_g(), c.int_b(), 1.0).filled(),
                )
            }))?;

            chart.draw_series([Rectangle::new(
                [(0, 0), (1, color_map_size + 1)],
                BLACK.stroke_width(1),
            )])?;
        }

        root.present()?;

        Ok(())
    }

    fn plot_phase_impl<T: autd3_core::geometry::Transducer, B: plotters::backend::DrawingBackend>(
        root: DrawingArea<B, Shift>,
        config: &PlotConfig,
        geometry: &autd3_core::geometry::Geometry<T>,
        phases: Vec<float>,
    ) -> Result<(), crate::error::MonitorError>
    where
        MonitorError:
            From<DrawingAreaErrorKind<<B as plotters::backend::DrawingBackend>::ErrorType>>,
    {
        root.fill(&WHITE)?;

        let main_area_size_x = (config.figsize.0 as float * (1.0 - config.cbar_size)) as u32;

        let (main_area, cbar_area) = root.split_horizontally(main_area_size_x);

        let color_map_size = 1000;
        let cmap: Vec<scarlet::color::RGBColor> = config
            .cmap
            .transform((0..=color_map_size).map(|x| x as float / color_map_size as float));

        {
            let p = geometry
                .transducers()
                .map(|t| (t.position().x, t.position().y))
                .collect::<Vec<_>>();

            let min_x =
                p.iter().fold(float::MAX, |acc, &(x, _)| acc.min(x)) - AUTD3::TRANS_SPACING / 2.0;
            let min_y =
                p.iter().fold(float::MAX, |acc, &(_, y)| acc.min(y)) - AUTD3::TRANS_SPACING / 2.0;
            let max_x =
                p.iter().fold(float::MIN, |acc, &(x, _)| acc.max(x)) + AUTD3::TRANS_SPACING / 2.0;
            let max_y =
                p.iter().fold(float::MIN, |acc, &(_, y)| acc.max(y)) + AUTD3::TRANS_SPACING / 2.0;

            let plot_range_x = max_x - min_x;
            let plot_range_y = max_y - min_y;

            let available_size_x = main_area_size_x - config.label_area_size - config.margin;
            let available_size_y = config.figsize.1 - config.label_area_size - config.margin * 2;

            let px_per_ps = (available_size_x as float / plot_range_x)
                .min(available_size_y as float / plot_range_y);

            let plot_size_x = (plot_range_x * px_per_ps) as u32;
            let plot_size_y = (plot_range_y * px_per_ps) as u32;

            let left_margin = config.margin
                + (main_area_size_x - plot_size_x - config.label_area_size - config.margin).max(0)
                    / 2;
            let right_margin = config.margin
                + (main_area_size_x - plot_size_x - config.label_area_size - config.margin).max(0)
                    / 2;
            let top_margin = config.margin
                + (config.figsize.1 - plot_size_y - config.label_area_size - config.margin).max(0)
                    / 2;
            let bottom_margin = config.margin
                + (config.figsize.1 - plot_size_y - config.label_area_size - config.margin).max(0)
                    / 2;

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
                .x_label_formatter(&|v| format!("{:.1}", v))
                .y_label_formatter(&|v| format!("{:.1}", v))
                .x_label_style(("sans-serif", config.font_size).into_text_style(&main_area))
                .y_label_style(("sans-serif", config.font_size).into_text_style(&main_area))
                .x_desc("x [mm]")
                .y_desc("y [mm]")
                .draw()?;

            scatter_ctx.draw_series(p.iter().zip(phases.iter()).map(|(&(x, y), &p)| {
                let v = (p / (2.0 * autd3_core::PI)) % 1.;
                let c = cmap[((v * color_map_size as float) as usize).clamp(0, cmap.len() - 1)];
                Circle::new(
                    (x, y),
                    AUTD3::TRANS_SPACING * px_per_ps / 2.0,
                    RGBColor(c.int_r(), c.int_g(), c.int_b())
                        .filled()
                        .stroke_width(0),
                )
            }))?;
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
                .y_label_formatter(&|&v| {
                    if v == 0 {
                        "0".to_owned()
                    } else if v == color_map_size / 2 {
                        "π".to_owned()
                    } else {
                        "2π".to_owned()
                    }
                })
                .draw()?;

            chart.draw_series(cmap.iter().enumerate().map(|(i, c)| {
                Rectangle::new(
                    [(0, i as i32), (1, i as i32 + 1)],
                    RGBAColor(c.int_r(), c.int_g(), c.int_b(), 1.0).filled(),
                )
            }))?;

            chart.draw_series([Rectangle::new(
                [(0, 0), (1, color_map_size + 1)],
                BLACK.stroke_width(1),
            )])?;
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

    fn initialize(&mut self) -> Result<(), crate::error::MonitorError> {
        Ok(())
    }

    fn plot_1d(
        observe_points: Vec<float>,
        acoustic_pressures: Vec<autd3_core::acoustics::Complex>,
        _resolution: float,
        x_label: &str,
        config: Self::PlotConfig,
    ) -> Result<(), crate::error::MonitorError> {
        let path = std::path::Path::new(&config.fname);
        if !path.parent().map_or(true, |p| p.exists()) {
            std::fs::create_dir_all(path.parent().unwrap())?;
        }

        let yrange = acoustic_pressures
            .iter()
            .fold((float::MAX, float::MIN), |acc, &x| {
                (acc.0.min(x.norm()), acc.1.max(x.norm()))
            });

        if path.extension().map_or(false, |e| e == "svg") {
            Self::plot_1d_impl(
                &SVGBackend::new(&config.fname, config.figsize).into_drawing_area(),
                &observe_points,
                &acoustic_pressures,
                x_label,
                yrange,
                &config,
            )
        } else {
            Self::plot_1d_impl(
                &BitMapBackend::new(&config.fname, config.figsize).into_drawing_area(),
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
        acoustic_pressures: Vec<autd3_core::acoustics::Complex>,
        resolution: float,
        x_label: &str,
        y_label: &str,
        config: Self::PlotConfig,
    ) -> Result<(), crate::error::MonitorError> {
        let path = std::path::Path::new(&config.fname);
        if !path.parent().map_or(true, |p| p.exists()) {
            std::fs::create_dir_all(path.parent().unwrap())?;
        }

        let zrange = acoustic_pressures
            .iter()
            .fold((float::MAX, float::MIN), |acc, &x| {
                (acc.0.min(x.norm()), acc.1.max(x.norm()))
            });

        if path.extension().map_or(false, |e| e == "svg") {
            Self::plot_2d_impl(
                &SVGBackend::new(&config.fname, config.figsize).into_drawing_area(),
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
                &BitMapBackend::new(&config.fname, config.figsize).into_drawing_area(),
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
    ) -> Result<(), crate::error::MonitorError> {
        let path = std::path::Path::new(&config.fname);
        if !path.parent().map_or(true, |p| p.exists()) {
            std::fs::create_dir_all(path.parent().unwrap())?;
        }

        if path.extension().map_or(false, |e| e == "svg") {
            Self::plot_modulation_impl(
                SVGBackend::new(&config.fname, config.figsize).into_drawing_area(),
                modulation,
                &config,
            )
        } else {
            Self::plot_modulation_impl(
                BitMapBackend::new(&config.fname, config.figsize).into_drawing_area(),
                modulation,
                &config,
            )
        }
    }

    fn plot_phase<T: autd3_core::geometry::Transducer>(
        config: Self::PlotConfig,
        geometry: &autd3_core::geometry::Geometry<T>,
        phases: Vec<float>,
    ) -> Result<(), crate::error::MonitorError> {
        let path = std::path::Path::new(&config.fname);
        if !path.parent().map_or(true, |p| p.exists()) {
            std::fs::create_dir_all(path.parent().unwrap())?;
        }

        if path.extension().map_or(false, |e| e == "svg") {
            Self::plot_phase_impl(
                SVGBackend::new(&config.fname, config.figsize).into_drawing_area(),
                &config,
                geometry,
                phases,
            )
        } else {
            Self::plot_phase_impl(
                BitMapBackend::new(&config.fname, config.figsize).into_drawing_area(),
                &config,
                geometry,
                phases,
            )
        }
    }

    fn animate_1d(
        observe_points: Vec<float>,
        acoustic_pressures: Vec<Vec<autd3_core::acoustics::Complex>>,
        _resolution: float,
        x_label: &str,
        config: Self::PlotConfig,
    ) -> Result<(), crate::error::MonitorError> {
        let path = std::path::Path::new(&config.fname);
        if !path.parent().map_or(true, |p| p.exists()) {
            std::fs::create_dir_all(path.parent().unwrap())?;
        }

        if !path.extension().map_or(false, |e| e == "gif") {
            return Err(crate::error::MonitorError::NotSupported);
        }

        let yrange = acoustic_pressures
            .iter()
            .fold((float::MAX, float::MIN), |acc, x| {
                x.iter()
                    .fold(acc, |acc, &x| (acc.0.min(x.norm()), acc.1.max(x.norm())))
            });

        let root =
            BitMapBackend::gif(&config.fname, config.figsize, config.interval)?.into_drawing_area();

        for (i, ap) in acoustic_pressures.iter().enumerate() {
            if config.print_progress() {
                let percent = 100.0 * (i + 1) as float / acoustic_pressures.len() as float;
                print!("\r");
                print!(
                    "Plotted: [{:30}] {}/{} ({}%)",
                    "=".repeat((percent / (100.0 / 30.0)) as usize),
                    i + 1,
                    acoustic_pressures.len(),
                    percent as usize
                );
                std::io::stdout().flush().unwrap();
            }
            Self::plot_1d_impl(&root, &observe_points, ap, x_label, yrange, &config)?;
        }

        if config.print_progress() {
            println!();
        }

        Ok(())
    }

    fn animate_2d(
        observe_x: Vec<float>,
        observe_y: Vec<float>,
        acoustic_pressures: Vec<Vec<autd3_core::acoustics::Complex>>,
        resolution: float,
        x_label: &str,
        y_label: &str,
        config: Self::PlotConfig,
    ) -> Result<(), crate::error::MonitorError> {
        let path = std::path::Path::new(&config.fname);
        if !path.parent().map_or(true, |p| p.exists()) {
            std::fs::create_dir_all(path.parent().unwrap())?;
        }

        if !path.extension().map_or(false, |e| e == "gif") {
            return Err(crate::error::MonitorError::NotSupported);
        }

        let zrange = acoustic_pressures
            .iter()
            .fold((float::MAX, float::MIN), |acc, x| {
                x.iter()
                    .fold(acc, |acc, &x| (acc.0.min(x.norm()), acc.1.max(x.norm())))
            });

        let root =
            BitMapBackend::gif(&config.fname, config.figsize, config.interval)?.into_drawing_area();

        for (i, ap) in acoustic_pressures.iter().enumerate() {
            if config.print_progress() {
                let percent = 100.0 * (i + 1) as float / acoustic_pressures.len() as float;
                print!("\r");
                print!(
                    "Plotted: [{:30}] {}/{} ({}%)",
                    "=".repeat((percent / (100.0 / 30.0)) as usize),
                    i + 1,
                    acoustic_pressures.len(),
                    percent as usize
                );
                std::io::stdout().flush().unwrap();
            }
            Self::plot_2d_impl(
                &root, &observe_x, &observe_y, ap, x_label, y_label, zrange, resolution, &config,
            )?;
        }

        if config.print_progress() {
            println!();
        }

        Ok(())
    }
}
