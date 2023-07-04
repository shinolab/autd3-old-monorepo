/*
 * File: focus.rs
 * Project: stm
 * Created Date: 05/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{datagram::Datagram, error::AUTDInternalError, geometry::*};

use autd3_driver::float;

use super::STMProps;

#[derive(Clone, Debug, Copy)]
pub struct ControlPoint {
    point: Vector3,
    shift: u8,
}

impl ControlPoint {
    pub fn new(point: Vector3) -> Self {
        Self { point, shift: 0 }
    }

    pub fn with_shift(point: Vector3, shift: u8) -> Self {
        Self { point, shift }
    }

    pub fn point(&self) -> &Vector3 {
        &self.point
    }

    pub fn shift(&self) -> u8 {
        self.shift
    }
}

impl From<Vector3> for ControlPoint {
    fn from(point: Vector3) -> Self {
        Self::new(point)
    }
}

impl From<(Vector3, u8)> for ControlPoint {
    fn from((point, shift): (Vector3, u8)) -> Self {
        Self::with_shift(point, shift)
    }
}

impl From<&Vector3> for ControlPoint {
    fn from(point: &Vector3) -> Self {
        Self::new(*point)
    }
}

impl From<&(Vector3, u8)> for ControlPoint {
    fn from((point, shift): &(Vector3, u8)) -> Self {
        Self::with_shift(*point, *shift)
    }
}

#[derive(Clone)]
pub struct FocusSTM {
    control_points: Vec<ControlPoint>,
    props: STMProps,
}

impl FocusSTM {
    pub fn add_focus<C: Into<ControlPoint>>(mut self, point: C) -> Self {
        self.control_points.push(point.into());
        self
    }

    pub fn add_foci_from_iter<C: Into<ControlPoint>, T: IntoIterator<Item = C>>(
        mut self,
        iter: T,
    ) -> Self {
        self.control_points
            .extend(iter.into_iter().map(|c| c.into()));
        self
    }

    pub fn control_points(&self) -> &[ControlPoint] {
        &self.control_points
    }

    pub fn with_props(props: STMProps) -> Self {
        Self {
            control_points: Vec::new(),
            props,
        }
    }
}

impl<T: Transducer> Datagram<T> for FocusSTM {
    type H = autd3_driver::NullHeader;
    type B = autd3_driver::FocusSTM;

    fn operation(&self, geometry: &Geometry<T>) -> Result<(Self::H, Self::B), AUTDInternalError> {
        let points = geometry
            .device_map()
            .iter()
            .scan(0, |state, tr_num| {
                let r = Some(*state);
                *state += tr_num;
                r
            })
            .map(|origin_idx| {
                let tr = &geometry[origin_idx];
                let origin = tr.position();
                let trans_inv =
                    Matrix3::from_columns(&[tr.x_direction(), tr.y_direction(), tr.z_direction()])
                        .transpose();
                self.control_points
                    .iter()
                    .map(|p| {
                        let lp = trans_inv * (p.point() - origin);
                        autd3_driver::STMFocus::new(lp.x, lp.y, lp.z, p.shift())
                    })
                    .collect()
            })
            .collect();
        let tr_num_min = geometry.device_map().iter().min().unwrap();

        let props = autd3_driver::FocusSTMProps {
            freq_div: self.sampling_frequency_division(),
            sound_speed: geometry.sound_speed,
            start_idx: self.props.start_idx,
            finish_idx: self.props.finish_idx,
        };
        Ok((Self::H::default(), Self::B::new(points, *tr_num_min, props)))
    }
}

impl FocusSTM {
    pub fn new(freq: float) -> Self {
        Self {
            control_points: vec![],
            props: STMProps::new(freq),
        }
    }

    pub fn with_sampling_frequency_division(freq_div: u32) -> Self {
        Self {
            control_points: vec![],
            props: STMProps::with_sampling_frequency_division(freq_div),
        }
    }

    pub fn with_sampling_frequency(freq: float) -> Self {
        Self {
            control_points: vec![],
            props: STMProps::with_sampling_frequency(freq),
        }
    }

    pub fn with_start_idx(self, idx: Option<u16>) -> Self {
        Self {
            props: self.props.with_start_idx(idx),
            ..self
        }
    }

    pub fn with_finish_idx(self, idx: Option<u16>) -> Self {
        Self {
            props: self.props.with_finish_idx(idx),
            ..self
        }
    }

    pub fn start_idx(&self) -> Option<u16> {
        self.props.start_idx()
    }

    pub fn finish_idx(&self) -> Option<u16> {
        self.props.finish_idx()
    }

    pub fn size(&self) -> usize {
        self.control_points.len()
    }

    pub fn freq(&self) -> float {
        self.props.freq(self.size())
    }

    pub fn sampling_frequency(&self) -> float {
        self.props.sampling_frequency(self.size())
    }

    pub fn sampling_frequency_division(&self) -> u32 {
        self.props.sampling_frequency_division(self.size())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use autd3_driver::FPGA_SUB_CLK_FREQ;

    #[test]
    fn freq() {
        let stm = FocusSTM::new(1.0);
        assert_eq!(stm.freq(), 1.0);

        let stm = FocusSTM::with_sampling_frequency_division(512)
            .add_foci_from_iter((0..10).map(|_| (Vector3::zeros(), 0)));
        assert_approx_eq!(stm.freq(), FPGA_SUB_CLK_FREQ as float / 512. / 10.);

        let stm = FocusSTM::with_sampling_frequency(40e3)
            .add_foci_from_iter((0..10).map(|_| (Vector3::zeros(), 0)));
        assert_approx_eq!(stm.freq(), 40e3 / 10.);
    }

    #[test]
    fn sampling_frequency_division() {
        let stm = FocusSTM::with_sampling_frequency_division(512);
        assert_eq!(stm.sampling_frequency_division(), 512);

        let stm = FocusSTM::new(1.0).add_foci_from_iter((0..10).map(|_| (Vector3::zeros(), 0)));
        assert_eq!(
            stm.sampling_frequency_division(),
            (FPGA_SUB_CLK_FREQ as float / 10.) as u32
        );

        let stm = FocusSTM::with_sampling_frequency(40e3);
        assert_eq!(
            stm.sampling_frequency_division(),
            (FPGA_SUB_CLK_FREQ as float / 40e3) as u32
        );
    }

    #[test]
    fn sampling_frequency() {
        let stm = FocusSTM::with_sampling_frequency(40e3);
        assert_eq!(stm.sampling_frequency(), 40e3);

        let stm = FocusSTM::with_sampling_frequency_division(512);
        assert_approx_eq!(stm.sampling_frequency(), FPGA_SUB_CLK_FREQ as float / 512.);

        let stm = FocusSTM::new(1.0).add_foci_from_iter((0..10).map(|_| (Vector3::zeros(), 0)));
        assert_approx_eq!(stm.sampling_frequency(), 1. * 10.);
    }
}
