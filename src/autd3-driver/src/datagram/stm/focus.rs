/*
 * File: focus.rs
 * Project: stm
 * Created Date: 04/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 14/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    common::SamplingConfiguration, datagram::Datagram, defined::float, error::AUTDInternalError,
    operation::ControlPoint,
};

use super::STMProps;

/// FocusSTM is an STM for moving a single focal point.
///
/// The sampling timing is determined by hardware, thus the sampling time is precise.
///
/// FocusSTM has following restrictions:
/// - The maximum number of sampling points is 65536.
/// - The sampling frequency is [crate::FPGA_CLK_FREQ]/N, where `N` is a 32-bit unsigned integer and must be at least [crate::SAMPLING_FREQ_DIV_MIN]
///
pub struct FocusSTM {
    control_points: Vec<ControlPoint>,
    props: STMProps,
}

impl FocusSTM {
    /// constructor
    ///
    /// # Arguments
    ///
    /// * `freq` - Frequency of STM. The frequency closest to `freq` from the possible frequencies is set.
    ///
    pub fn new(freq: float) -> Self {
        Self::new_with_props(STMProps::new(freq))
    }

    /// constructor
    ///
    /// # Arguments
    ///
    /// * `period` - Period. The period closest to `period` from the possible periods is set.
    ///
    pub fn new_with_period(period: std::time::Duration) -> Self {
        Self::new_with_props(STMProps::new_with_period(period))
    }

    /// constructor
    ///
    /// # Arguments
    ///
    /// * `config` - Sampling configuration
    ///
    pub fn new_with_sampling_config(config: SamplingConfiguration) -> Self {
        Self::new_with_props(STMProps::new_with_sampling_config(config))
    }

    /// constructor
    ///
    /// # Arguments
    ///
    /// * `props` - STMProps
    pub fn new_with_props(props: STMProps) -> Self {
        Self {
            control_points: Vec::new(),
            props,
        }
    }

    /// Add [ControlPoint] to FocusSTM
    pub fn add_focus<C: Into<ControlPoint>>(mut self, point: C) -> Result<Self, AUTDInternalError> {
        self.control_points.push(point.into());
        self.props.sampling_config(self.control_points.len())?;
        Ok(self)
    }

    /// Add [ControlPoint]s to FocusSTM
    pub fn add_foci_from_iter<C: Into<ControlPoint>, T: IntoIterator<Item = C>>(
        mut self,
        iter: T,
    ) -> Result<Self, AUTDInternalError> {
        self.control_points
            .extend(iter.into_iter().map(|c| c.into()));
        self.props.sampling_config(self.control_points.len())?;
        Ok(self)
    }

    /// Clear current [ControlPoint]s
    ///
    /// # Returns
    /// removed [ControlPoint]s
    pub fn clear(&mut self) -> Vec<ControlPoint> {
        std::mem::take(&mut self.control_points)
    }

    /// Get [ControlPoint]s
    pub fn foci(&self) -> &[ControlPoint] {
        &self.control_points
    }

    /// Set the start index of STM
    pub fn with_start_idx(self, idx: Option<u16>) -> Self {
        Self {
            props: self.props.with_start_idx(idx),
            ..self
        }
    }

    /// Set the finish index of STM
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

    pub fn frequency(&self) -> float {
        self.props.freq(self.control_points.len())
    }

    pub fn period(&self) -> std::time::Duration {
        self.props.period(self.control_points.len())
    }

    pub fn sampling_config(&self) -> SamplingConfiguration {
        self.props
            .sampling_config(self.control_points.len())
            .unwrap()
    }
}

impl Datagram for FocusSTM {
    type O1 = crate::operation::FocusSTMOp;
    type O2 = crate::operation::NullOp;

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        let freq_div = self
            .props
            .sampling_config(self.control_points.len())?
            .frequency_division();
        let start_idx = self.props.start_idx;
        let finish_idx = self.props.finish_idx;
        Ok((
            Self::O1::new(self.control_points, freq_div, start_idx, finish_idx),
            Self::O2::default(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::Vector3,
        operation::{FocusSTMOp, NullOp},
    };

    #[test]
    fn freq() {
        let stm = FocusSTM::new(1.0);
        assert_eq!(stm.frequency(), 1.0);
    }

    #[test]
    fn start_idx() {
        let stm = FocusSTM::new(1.);
        assert_eq!(stm.start_idx(), None);

        let stm = FocusSTM::new(1.).with_start_idx(Some(0));
        assert_eq!(stm.start_idx(), Some(0));

        let stm = FocusSTM::new(1.).with_start_idx(None);
        assert_eq!(stm.start_idx(), None);
    }

    #[test]
    fn finish_idx() {
        let stm = FocusSTM::new(1.);
        assert_eq!(stm.finish_idx(), None);

        let stm = FocusSTM::new(1.).with_finish_idx(Some(0));
        assert_eq!(stm.finish_idx(), Some(0));

        let stm = FocusSTM::new(1.).with_finish_idx(None);
        assert_eq!(stm.finish_idx(), None);
    }

    #[test]
    fn add_focus() {
        let stm = FocusSTM::new(1.0)
            .add_focus(Vector3::new(1., 2., 3.))
            .unwrap()
            .add_focus((Vector3::new(4., 5., 6.), 1))
            .unwrap()
            .add_focus(ControlPoint::new(Vector3::new(7., 8., 9.)).with_shift(2))
            .unwrap();

        assert_eq!(stm.foci().len(), 3);

        assert_eq!(stm.foci()[0].point(), &Vector3::new(1., 2., 3.));
        assert_eq!(stm.foci()[0].shift(), 0);

        assert_eq!(stm.foci()[1].point(), &Vector3::new(4., 5., 6.));
        assert_eq!(stm.foci()[1].shift(), 1);

        assert_eq!(stm.foci()[2].point(), &Vector3::new(7., 8., 9.));
        assert_eq!(stm.foci()[2].shift(), 2);
    }

    #[test]
    fn add_foci() {
        let stm = FocusSTM::new(1.0)
            .add_focus(Vector3::new(1., 2., 3.))
            .unwrap()
            .add_focus((Vector3::new(4., 5., 6.), 1))
            .unwrap()
            .add_focus(ControlPoint::new(Vector3::new(7., 8., 9.)).with_shift(2))
            .unwrap();

        assert_eq!(stm.foci().len(), 3);

        assert_eq!(stm.foci()[0].point(), &Vector3::new(1., 2., 3.));
        assert_eq!(stm.foci()[0].shift(), 0);

        assert_eq!(stm.foci()[1].point(), &Vector3::new(4., 5., 6.));
        assert_eq!(stm.foci()[1].shift(), 1);

        assert_eq!(stm.foci()[2].point(), &Vector3::new(7., 8., 9.));
        assert_eq!(stm.foci()[2].shift(), 2);
    }

    #[test]
    fn clear() {
        let mut stm = FocusSTM::new(1.0)
            .add_focus(Vector3::new(1., 2., 3.))
            .unwrap()
            .add_focus((Vector3::new(4., 5., 6.), 1))
            .unwrap()
            .add_focus(ControlPoint::new(Vector3::new(7., 8., 9.)).with_shift(2))
            .unwrap();

        let foci = stm.clear();

        assert_eq!(stm.foci().len(), 0);

        assert_eq!(foci.len(), 3);

        assert_eq!(foci[0].point(), &Vector3::new(1., 2., 3.));
        assert_eq!(foci[0].shift(), 0);
        assert_eq!(foci[1].point(), &Vector3::new(4., 5., 6.));
        assert_eq!(foci[1].shift(), 1);
        assert_eq!(foci[2].point(), &Vector3::new(7., 8., 9.));
        assert_eq!(foci[2].shift(), 2);
    }

    #[test]
    fn focu_stm_operation() {
        let stm = FocusSTM::new(1.0)
            .add_focus(Vector3::new(1., 2., 3.))
            .unwrap()
            .add_focus((Vector3::new(4., 5., 6.), 1))
            .unwrap()
            .add_focus(ControlPoint::new(Vector3::new(7., 8., 9.)).with_shift(2))
            .unwrap();

        let r = <FocusSTM as Datagram>::operation(stm);
        assert!(r.is_ok());
        let _: (FocusSTMOp, NullOp) = r.unwrap();
    }
}
