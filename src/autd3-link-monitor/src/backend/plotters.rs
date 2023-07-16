/*
 * File: plotters.rs
 * Project: backend
 * Created Date: 16/07/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 16/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use scarlet::colormap::ColorMap;

use plotters::prelude::*;

use crate::{Backend, PlotConfig};

use autd3_core::autd3_device::TRANS_SPACING;

pub struct PlottersBackend {}

impl Backend for PlottersBackend {
    type PlotConfig = PlotConfig;

    fn new() -> Self {
        Self {}
    }

    fn initialize(&mut self) -> Result<(), crate::error::MonitorError> {
        Ok(())
    }

    fn plot_1d(
        observe_points: Vec<f64>,
        acoustic_pressures: Vec<autd3_core::acoustics::Complex>,
        resolution: f64,
        x_label: &str,
        config: Self::PlotConfig,
    ) -> Result<(), crate::error::MonitorError> {
        todo!()
    }

    fn plot_2d(
        observe_x: Vec<f64>,
        observe_y: Vec<f64>,
        acoustic_pressures: Vec<autd3_core::acoustics::Complex>,
        resolution: f64,
        x_label: &str,
        y_label: &str,
        config: Self::PlotConfig,
    ) -> Result<(), crate::error::MonitorError> {
        todo!()
    }

    fn plot_modulation(
        modulation: Vec<f64>,
        config: Self::PlotConfig,
    ) -> Result<(), crate::error::MonitorError> {
        todo!()
    }

    fn plot_phase<T: autd3_core::geometry::Transducer>(
        config: Self::PlotConfig,
        geometry: &autd3_core::geometry::Geometry<T>,
        phases: Vec<f64>,
    ) -> Result<(), crate::error::MonitorError> {
        let main_area_size_x = (config.figsize.0 as f64 * (1.0 - config.cbar_size)) as u32;

        let root = BitMapBackend::new(&config.fname, config.figsize).into_drawing_area();
        root.fill(&WHITE)?;

        let (main_area, cbar_area) = root.split_horizontally(main_area_size_x);

        let color_map_size = 1000;
        let cmap: Vec<scarlet::color::RGBColor> = config
            .cmap
            .transform((0..=color_map_size).map(|x| x as f64 / color_map_size as f64));

        {
            let p = geometry
                .transducers()
                .map(|t| (t.position().x, t.position().y))
                .collect::<Vec<_>>();

            let min_x =
                p.iter().fold(std::f64::MAX, |acc, &(x, _)| acc.min(x)) - TRANS_SPACING / 2.0;
            let min_y =
                p.iter().fold(std::f64::MAX, |acc, &(_, y)| acc.min(y)) - TRANS_SPACING / 2.0;
            let max_x =
                p.iter().fold(std::f64::MIN, |acc, &(x, _)| acc.max(x)) + TRANS_SPACING / 2.0;
            let max_y =
                p.iter().fold(std::f64::MIN, |acc, &(_, y)| acc.max(y)) + TRANS_SPACING / 2.0;

            let plot_range_x = max_x - min_x;
            let plot_range_y = max_y - min_y;

            let available_size_x = main_area_size_x - config.label_area_size - config.margin;
            let available_size_y = config.figsize.1 - config.label_area_size - config.margin * 2;

            let px_per_ps = (available_size_x as f64 / plot_range_x)
                .min(available_size_y as f64 / plot_range_y);

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
                .draw()?;

            scatter_ctx.draw_series(p.iter().zip(phases.iter()).map(|(&(x, y), &p)| {
                let v = (p / (2.0 * std::f64::consts::PI)) % 1.;
                let c = cmap[((v * color_map_size as f64) as usize).clamp(0, cmap.len() - 1)];
                Circle::new(
                    (x, y),
                    TRANS_SPACING * px_per_ps / 2.0,
                    RGBColor(c.int_r(), c.int_g(), c.int_b())
                        .filled()
                        .stroke_width(0),
                )
            }))?;
        }

        {
            let mut chart = ChartBuilder::on(&cbar_area)
                .margin_left(config.margin)
                .margin_top(50)
                .margin_bottom(50)
                .margin_right(config.margin)
                .y_label_area_size(config.label_area_size)
                .set_label_area_size(LabelAreaPosition::Left, 0)
                .set_label_area_size(LabelAreaPosition::Right, 30)
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

    fn animate_1d(
        observe_points: Vec<f64>,
        acoustic_pressures: Vec<Vec<autd3_core::acoustics::Complex>>,
        resolution: f64,
        x_label: &str,
        config: Self::PlotConfig,
    ) -> Result<(), crate::error::MonitorError> {
        todo!()
    }

    fn animate_2d(
        observe_x: Vec<f64>,
        observe_y: Vec<f64>,
        acoustic_pressures: Vec<Vec<autd3_core::acoustics::Complex>>,
        resolution: f64,
        x_label: &str,
        y_label: &str,
        config: Self::PlotConfig,
    ) -> Result<(), crate::error::MonitorError> {
        todo!()
    }
}
