/*
 * File: gain.rs
 * Project: stm
 * Created Date: 04/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    common::SamplingConfiguration, datagram::*, defined::float, error::AUTDInternalError,
    operation::GainSTMMode,
};

use super::STMProps;

/// GainSTM is an STM for moving [Gain].
///
/// The sampling timing is determined by hardware, thus the sampling time is precise.
///
/// GainSTM has following restrictions:
/// - The maximum number of sampling [Gain] is 2048.
/// - The sampling frequency is [crate::FPGA_CLK_FREQ]/N, where `N` is a 32-bit unsigned integer and must be at least [crate::SAMPLING_FREQ_DIV_MIN]
///
pub struct GainSTM<G: Gain> {
    gains: Vec<G>,
    mode: GainSTMMode,
    props: STMProps,
}

impl<G: Gain> GainSTM<G> {
    /// constructor
    ///
    /// # Arguments
    ///
    /// * `freq` - Frequency of STM. The frequency closest to `freq` from the possible frequencies is set.
    ///
    pub fn new(freq: float) -> Self {
        Self::from_props_mode(STMProps::new(freq), GainSTMMode::PhaseIntensityFull)
    }

    /// constructor
    ///
    /// # Arguments
    ///
    /// * `period` - Period. The period closest to `period` from the possible periods is set.
    ///
    pub fn from_period(period: std::time::Duration) -> Self {
        Self::from_props_mode(
            STMProps::from_period(period),
            GainSTMMode::PhaseIntensityFull,
        )
    }

    /// constructor
    ///
    /// # Arguments
    ///
    /// * `freq_div` - Sampling frequency division of STM. The sampling frequency is [crate::FPGA_CLK_FREQ]/`freq_div`.
    ///
    pub fn from_sampling_config(config: SamplingConfiguration) -> Self {
        Self::from_props_mode(
            STMProps::from_sampling_config(config),
            GainSTMMode::PhaseIntensityFull,
        )
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
        self.props.freq(self.gains.len())
    }

    pub fn period(&self) -> std::time::Duration {
        self.props.period(self.gains.len())
    }

    pub fn sampling_config(&self) -> SamplingConfiguration {
        self.props.sampling_config(self.gains.len()).unwrap()
    }

    /// Set the mode of GainSTM
    pub fn with_mode(self, mode: GainSTMMode) -> Self {
        Self { mode, ..self }
    }

    pub fn mode(&self) -> GainSTMMode {
        self.mode
    }

    /// Add a [Gain] to GainSTM
    pub fn add_gain(mut self, gain: G) -> Result<Self, AUTDInternalError> {
        self.gains.push(gain);
        self.props.sampling_config(self.gains.len())?;
        Ok(self)
    }

    /// Add boxed [Gain]s from iterator to GainSTM
    pub fn add_gains_from_iter<I: IntoIterator<Item = G>>(
        mut self,
        iter: I,
    ) -> Result<Self, AUTDInternalError> {
        self.gains.extend(iter);
        self.props.sampling_config(self.gains.len())?;
        Ok(self)
    }

    /// constructor
    ///
    /// # Arguments
    ///
    /// * `props` - STMProps
    /// * `mode` - GainSTMMode
    pub fn from_props_mode(props: STMProps, mode: GainSTMMode) -> Self {
        Self {
            gains: Vec::new(),
            mode,
            props,
        }
    }

    /// Get [Gain]s
    pub fn gains(&self) -> &[G] {
        &self.gains
    }

    /// Clear current [Gain]s
    ///
    /// # Returns
    /// removed [Gain]s
    pub fn clear(&mut self) -> Vec<G> {
        std::mem::take(&mut self.gains)
    }
}

impl GainSTM<Box<dyn Gain>> {
    /// get Gain of specified index
    ///
    /// # Arguments
    ///
    /// * `idx` - index
    ///
    /// # Returns
    ///
    /// * Gain of specified index if the type is matched, otherwise None
    ///
    pub fn get<G: Gain + 'static>(&self, idx: usize) -> Option<&G> {
        if idx >= self.gains.len() {
            return None;
        }
        self.gains[idx].as_ref().as_any().downcast_ref::<G>()
    }
}

impl<G: Gain> Datagram for GainSTM<G> {
    type O1 = crate::operation::GainSTMOp<G>;
    type O2 = crate::operation::NullOp;

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        let freq_div = self
            .props
            .sampling_config(self.gains.len())?
            .frequency_division();
        let Self {
            gains, mode, props, ..
        } = self;
        Ok((
            Self::O1::new(gains, mode, freq_div, props.start_idx, props.finish_idx),
            Self::O2::default(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    use crate::{
        common::Drive,
        datagram::{Gain, GainAsAny},
        derive::prelude::Geometry,
        operation::{tests::NullGain, GainSTMOp, NullOp},
    };

    #[test]
    fn new() {
        let stm = GainSTM::<NullGain>::new(1.)
            .add_gains_from_iter((0..10).map(|_| NullGain {}))
            .unwrap();

        assert_eq!(stm.frequency(), 1.);
        assert_eq!(stm.sampling_config().frequency(), 1. * 10.);
    }

    #[test]
    fn from_period() {
        let stm = GainSTM::<NullGain>::from_period(std::time::Duration::from_micros(250))
            .add_gains_from_iter((0..10).map(|_| NullGain {}))
            .unwrap();

        assert_eq!(stm.period(), std::time::Duration::from_micros(250));
        assert_eq!(
            stm.sampling_config().period(),
            std::time::Duration::from_micros(25)
        );
    }

    #[test]
    fn from_sampling_config() {
        let stm = GainSTM::<NullGain>::from_sampling_config(
            SamplingConfiguration::from_period(std::time::Duration::from_micros(25)).unwrap(),
        )
        .add_gains_from_iter((0..10).map(|_| NullGain {}))
        .unwrap();

        assert_eq!(stm.period(), std::time::Duration::from_micros(250));
        assert_eq!(
            stm.sampling_config().period(),
            std::time::Duration::from_micros(25)
        );
    }

    #[test]
    fn with_mode() {
        let stm = GainSTM::<NullGain>::new(1.0);
        assert_eq!(stm.mode(), GainSTMMode::PhaseIntensityFull);

        let stm = stm.with_mode(GainSTMMode::PhaseFull);
        assert_eq!(stm.mode(), GainSTMMode::PhaseFull);

        let stm = stm.with_mode(GainSTMMode::PhaseHalf);
        assert_eq!(stm.mode(), GainSTMMode::PhaseHalf);

        let stm = stm.with_mode(GainSTMMode::PhaseIntensityFull);
        assert_eq!(stm.mode(), GainSTMMode::PhaseIntensityFull);
    }

    struct NullGain2 {}

    impl GainAsAny for NullGain2 {
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    impl Gain for NullGain2 {
        #[cfg_attr(coverage_nightly, coverage(off))]
        fn calc(
            &self,
            _: &Geometry,
            _: GainFilter,
        ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
            unimplemented!()
        }
    }

    #[test]
    fn test_get() {
        let stm = GainSTM::<Box<dyn Gain>>::new(1.0)
            .add_gain(Box::new(NullGain {}))
            .unwrap()
            .add_gain(Box::new(NullGain2 {}))
            .unwrap();

        assert!(stm.get::<NullGain>(0).is_some());
        assert!(stm.get::<NullGain2>(0).is_none());

        assert!(stm.get::<NullGain2>(1).is_some());
        assert!(stm.get::<NullGain>(1).is_none());

        assert!(stm.get::<NullGain>(2).is_none());
        assert!(stm.get::<NullGain2>(2).is_none());
    }

    #[test]
    fn test_clear() {
        let mut stm = GainSTM::<Box<dyn Gain>>::new(1.0)
            .add_gain(Box::new(NullGain {}))
            .unwrap()
            .add_gain(Box::new(NullGain2 {}))
            .unwrap();

        let gains = stm.clear();

        assert_eq!(stm.gains().len(), 0);
        assert_eq!(gains.len(), 2);
    }

    #[test]
    fn start_idx() {
        let stm = GainSTM::<Box<dyn Gain>>::new(1.);
        assert_eq!(stm.start_idx(), None);

        let stm = GainSTM::<Box<dyn Gain>>::new(1.).with_start_idx(Some(0));
        assert_eq!(stm.start_idx(), Some(0));

        let stm = GainSTM::<Box<dyn Gain>>::new(1.).with_start_idx(None);
        assert_eq!(stm.start_idx(), None);
    }

    #[test]
    fn finish_idx() {
        let stm = GainSTM::<Box<dyn Gain>>::new(1.);
        assert_eq!(stm.finish_idx(), None);

        let stm = GainSTM::<Box<dyn Gain>>::new(1.).with_finish_idx(Some(0));
        assert_eq!(stm.finish_idx(), Some(0));

        let stm = GainSTM::<Box<dyn Gain>>::new(1.).with_finish_idx(None);
        assert_eq!(stm.finish_idx(), None);
    }

    #[test]
    fn gain_stm_operation() {
        let stm = GainSTM::<Box<dyn Gain>>::new(1.)
            .add_gain(Box::new(NullGain {}))
            .unwrap()
            .add_gain(Box::new(NullGain2 {}))
            .unwrap();

        let r = stm.operation();
        assert!(r.is_ok());
        let _: (GainSTMOp<Box<dyn Gain>>, NullOp) = r.unwrap();
    }
}
