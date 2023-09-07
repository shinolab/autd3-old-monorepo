/*
 * File: gain.rs
 * Project: stm
 * Created Date: 04/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    datagram::*, defined::float, error::AUTDInternalError, geometry::*, operation::GainSTMMode,
};

use super::STMProps;

/// GainSTM is an STM for moving [Gain].
///
/// The sampling timing is determined by hardware, thus the sampling time is precise.
///
/// GainSTM has following restrictions:
/// - The maximum number of sampling [Gain] is 2048 (Legacy mode) or 1024 (Advanced/AdvancedPhase mode).
/// - The sampling frequency is [crate::FPGA_SUB_CLK_FREQ]/N, where `N` is a 32-bit unsigned integer and must be at least [crate::SAMPLING_FREQ_DIV_MIN]
///
pub struct GainSTM<T: Transducer, G: Gain<T>> {
    gains: Vec<G>,
    mode: GainSTMMode,
    props: STMProps,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Transducer, G: Gain<T>> Clone for GainSTM<T, G>
where
    G: Clone,
{
    fn clone(&self) -> Self {
        Self {
            gains: self.gains.clone(),
            mode: self.mode,
            props: self.props,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Transducer, G: Gain<T>> GainSTM<T, G> {
    /// constructor
    ///
    /// # Arguments
    ///
    /// * `freq` - Frequency of STM. The frequency closest to `freq` from the possible frequencies is set.
    ///
    pub fn new(freq: float) -> Self {
        Self {
            gains: vec![],
            mode: GainSTMMode::PhaseDutyFull,
            props: STMProps::new(freq),
            _phantom: std::marker::PhantomData,
        }
    }

    /// constructor
    ///
    /// # Arguments
    ///
    /// * `period` - Period. The period closest to `period` from the possible periods is set.
    ///
    pub fn with_period(period: std::time::Duration) -> Self {
        Self::new(1000000000. / period.as_nanos() as float)
    }

    /// constructor
    ///
    /// # Arguments
    ///
    /// * `freq_div` - Sampling frequency division of STM. The sampling frequency is [crate::FPGA_SUB_CLK_FREQ]/`freq_div`.
    ///
    pub fn with_sampling_frequency_division(freq_div: u32) -> Self {
        Self {
            gains: vec![],
            mode: GainSTMMode::PhaseDutyFull,
            props: STMProps::with_sampling_frequency_division(freq_div),
            _phantom: std::marker::PhantomData,
        }
    }

    /// constructor
    ///
    /// # Arguments
    ///
    /// * `freq` - Sampling frequency of STM. The sampling frequency closest to `freq` from the possible sampling frequencies is set.
    ///
    pub fn with_sampling_frequency(freq: float) -> Self {
        Self {
            gains: vec![],
            mode: GainSTMMode::PhaseDutyFull,
            props: STMProps::with_sampling_frequency(freq),
            _phantom: std::marker::PhantomData,
        }
    }

    /// constructor
    ///
    /// # Arguments
    ///
    /// * `period` - Sampling period. The sampling period closest to `period` from the possible sampling periods is set.
    ///
    pub fn with_sampling_period(period: std::time::Duration) -> Self {
        Self {
            gains: vec![],
            mode: GainSTMMode::PhaseDutyFull,
            props: STMProps::with_sampling_period(period),
            _phantom: std::marker::PhantomData,
        }
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

    #[doc(hidden)]
    /// This is used only for internal.
    pub fn size(&self) -> usize {
        self.gains.len()
    }

    pub fn freq(&self) -> float {
        self.props.freq(self.size())
    }

    pub fn period(&self) -> std::time::Duration {
        self.props.period(self.size())
    }

    pub fn sampling_frequency(&self) -> float {
        self.props.sampling_frequency(self.size())
    }

    pub fn sampling_frequency_division(&self) -> u32 {
        self.props.sampling_frequency_division(self.size())
    }

    pub fn sampling_period(&self) -> std::time::Duration {
        self.props.sampling_period(self.size())
    }

    /// Set the mode of GainSTM
    pub fn with_mode(self, mode: GainSTMMode) -> Self {
        Self { mode, ..self }
    }

    pub fn mode(&self) -> GainSTMMode {
        self.mode
    }

    /// Add a [Gain] to GainSTM
    pub fn add_gain(mut self, gain: G) -> Self {
        self.gains.push(gain);
        self
    }

    /// Add boxed [Gain]s from iterator to GainSTM
    pub fn add_gains_from_iter<I: IntoIterator<Item = G>>(mut self, iter: I) -> Self {
        self.gains.extend(iter);
        self
    }

    #[doc(hidden)]
    /// This is used only for capi.
    pub fn with_props(props: STMProps) -> Self {
        Self {
            gains: Vec::new(),
            mode: GainSTMMode::PhaseDutyFull,
            props,
            _phantom: std::marker::PhantomData,
        }
    }

    #[doc(hidden)]
    /// This is used only for capi.
    pub fn with_props_mode(props: STMProps, mode: GainSTMMode) -> Self {
        Self {
            gains: Vec::new(),
            mode,
            props,
            _phantom: std::marker::PhantomData,
        }
    }

    #[doc(hidden)]
    /// This is used only for capi.
    pub fn gains(&self) -> &[G] {
        &self.gains
    }

    #[doc(hidden)]
    /// This is used only for capi.
    pub fn take_gains(&mut self) -> Vec<G> {
        std::mem::take(&mut self.gains)
    }
}

impl<T: Transducer + 'static> GainSTM<T, Box<dyn Gain<T>>> {
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
    pub fn get<G: Gain<T> + 'static>(&self, idx: usize) -> Option<&G> {
        if idx >= self.gains.len() {
            return None;
        }
        self.gains[idx].as_ref().as_any().downcast_ref::<G>()
    }
}

impl<T: Transducer, G: Gain<T>> Datagram<T> for GainSTM<T, G>
where
    crate::operation::GainSTMOp<T, G>: crate::operation::Operation<T>,
{
    type O1 = crate::operation::GainSTMOp<T, G>;
    type O2 = crate::operation::NullOp;

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        let freq_div = self.sampling_frequency_division();
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
        datagram::{Gain, GainAsAny},
        defined::Drive,
        fpga::FPGA_SUB_CLK_FREQ,
    };

    use assert_approx_eq::assert_approx_eq;

    struct NullGain {}

    impl GainAsAny for NullGain {
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    impl<T: Transducer> Gain<T> for NullGain {
        fn calc(
            &self,
            _: &[&Device<T>],
            _: GainFilter,
        ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
            unimplemented!()
        }
    }

    #[test]
    fn freq() {
        let stm = GainSTM::<LegacyTransducer, NullGain>::new(1.0);
        assert_eq!(stm.freq(), 1.0);

        let stm = GainSTM::<LegacyTransducer, _>::with_sampling_frequency_division(512)
            .add_gains_from_iter((0..10).map(|_| NullGain {}));
        assert_approx_eq!(stm.freq(), FPGA_SUB_CLK_FREQ as float / 512. / 10.);

        let stm = GainSTM::<LegacyTransducer, _>::with_sampling_frequency(40e3)
            .add_gains_from_iter((0..10).map(|_| NullGain {}));
        assert_approx_eq!(stm.freq(), 40e3 / 10.);
    }

    #[test]
    fn period() {
        let stm =
            GainSTM::<LegacyTransducer, NullGain>::with_period(std::time::Duration::from_millis(1));
        assert_eq!(stm.period(), std::time::Duration::from_millis(1));

        let stm = GainSTM::<LegacyTransducer, _>::with_sampling_period(
            std::time::Duration::from_millis(1),
        )
        .add_gains_from_iter((0..10).map(|_| NullGain {}));
        assert_eq!(stm.period(), std::time::Duration::from_millis(10));
    }

    #[test]
    fn sampling_frequency_division() {
        let stm = GainSTM::<LegacyTransducer, NullGain>::with_sampling_frequency_division(512);
        assert_eq!(stm.sampling_frequency_division(), 512);

        let stm = GainSTM::<LegacyTransducer, _>::new(1.0)
            .add_gains_from_iter((0..10).map(|_| NullGain {}));
        assert_eq!(
            stm.sampling_frequency_division(),
            (FPGA_SUB_CLK_FREQ as float / 10.) as u32
        );

        let stm = GainSTM::<LegacyTransducer, NullGain>::with_sampling_frequency(40e3);
        assert_eq!(
            stm.sampling_frequency_division(),
            (FPGA_SUB_CLK_FREQ as float / 40e3) as u32
        );
    }

    #[test]
    fn sampling_frequency() {
        let stm = GainSTM::<LegacyTransducer, NullGain>::with_sampling_frequency(40e3);
        assert_eq!(stm.sampling_frequency(), 40e3);

        let stm = GainSTM::<LegacyTransducer, NullGain>::with_sampling_frequency_division(512);
        assert_approx_eq!(stm.sampling_frequency(), FPGA_SUB_CLK_FREQ as float / 512.);

        let stm = GainSTM::<LegacyTransducer, _>::new(1.0)
            .add_gains_from_iter((0..10).map(|_| NullGain {}));
        assert_approx_eq!(stm.sampling_frequency(), 1. * 10.);
    }

    #[test]
    fn sampling_period() {
        let stm = GainSTM::<LegacyTransducer, NullGain>::with_sampling_period(
            std::time::Duration::from_millis(1),
        );
        assert_eq!(stm.sampling_period(), std::time::Duration::from_millis(1));

        let stm = GainSTM::<LegacyTransducer, _>::with_period(std::time::Duration::from_millis(10))
            .add_gains_from_iter((0..10).map(|_| NullGain {}));
        assert_eq!(stm.sampling_period(), std::time::Duration::from_millis(1));
    }

    struct NullGain2 {}

    impl GainAsAny for NullGain2 {
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    impl<T: Transducer> Gain<T> for NullGain2 {
        fn calc(
            &self,
            _: &[&Device<T>],
            _: GainFilter,
        ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
            unimplemented!()
        }
    }

    #[test]
    fn test_get() {
        let stm = GainSTM::<LegacyTransducer, Box<dyn Gain<LegacyTransducer>>>::new(1.0)
            .add_gain(Box::new(NullGain {}))
            .add_gain(Box::new(NullGain2 {}));

        assert!(stm.get::<NullGain>(0).is_some());
        assert!(stm.get::<NullGain2>(0).is_none());

        assert!(stm.get::<NullGain2>(1).is_some());
        assert!(stm.get::<NullGain>(1).is_none());

        assert!(stm.get::<NullGain>(2).is_none());
        assert!(stm.get::<NullGain2>(2).is_none());
    }
}
