/*
 * File: error.rs
 * Project: cpu
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{defined::float, fpga::*, operation::GainSTMMode};

#[derive(Debug, PartialEq)]
pub enum AUTDInternalError {
    ModulationSizeOutOfRange(usize),
    ModFreqDivOutOfRange(u32),
    STMStartIndexOutOfRange,
    STMFinishIndexOutOfRange,
    FocusSTMPointSizeOutOfRange(usize),
    FocusSTMFreqDivOutOfRange(u32),
    FocusSTMPointOutOfRange(float, float, float),
    GainSTMLegacySizeOutOfRange(usize),
    GainSTMSizeOutOfRange(usize),
    GainSTMFreqDivOutOfRange(u32),
    CycleOutOfRange(u16),
    GainSTMModeNotSupported(GainSTMMode),
    ModulationError(String),
    GainError(String),
    LinkError(String),
    NotSupported(String),
    LinkClosed,
    TimerCreationFailed,
    TimerDeleteFailed,

    #[cfg(target_os = "windows")]
    WindowsError(windows::core::Error),
}

impl std::fmt::Display for AUTDInternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AUTDInternalError::ModulationSizeOutOfRange(size) => {
                write!(
                    f,
                    "Modulation buffer size ({}) is out of range ([{}, {}])",
                    size, MOD_BUF_SIZE_MIN, MOD_BUF_SIZE_MAX
                )
            }
            AUTDInternalError::ModFreqDivOutOfRange(div) => {
                write!(
                    f,
                    "Frequency division ({}) is out of range ([{}, {}])",
                    div, SAMPLING_FREQ_DIV_MIN, SAMPLING_FREQ_DIV_MAX
                )
            }
            AUTDInternalError::STMStartIndexOutOfRange => {
                write!(f, "STM index is out of range")
            }
            AUTDInternalError::STMFinishIndexOutOfRange => {
                write!(f, "STM finish is out of range")
            }
            AUTDInternalError::FocusSTMPointSizeOutOfRange(size) => {
                write!(
                    f,
                    "FocusSTM size ({}) is out of range ([{}, {}])",
                    size, STM_BUF_SIZE_MIN, FOCUS_STM_BUF_SIZE_MAX
                )
            }
            AUTDInternalError::FocusSTMFreqDivOutOfRange(div) => {
                write!(
                    f,
                    "Frequency division ({}) is out of range ([{}, {}])",
                    div, SAMPLING_FREQ_DIV_MIN, SAMPLING_FREQ_DIV_MAX
                )
            }
            AUTDInternalError::FocusSTMPointOutOfRange(x, y, z) => {
                write!(
                    f,
                    "Point ({}, {}, {}) is out of range. Each parameter must be in [{}, {}].",
                    x,
                    y,
                    z,
                    FOCUS_STM_FIXED_NUM_UNIT * FOCUS_STM_FIXED_NUM_LOWER as float,
                    FOCUS_STM_FIXED_NUM_UNIT * FOCUS_STM_FIXED_NUM_UPPER as float
                )
            }
            AUTDInternalError::GainSTMLegacySizeOutOfRange(size) => {
                write!(
                    f,
                    "GainSTM size ({}) is out of range ([{}, {}])",
                    size, STM_BUF_SIZE_MIN, GAIN_STM_LEGACY_BUF_SIZE_MAX
                )
            }
            AUTDInternalError::GainSTMSizeOutOfRange(size) => {
                write!(
                    f,
                    "GainSTM size ({}) is out of range ([{}, {}])",
                    size, STM_BUF_SIZE_MIN, GAIN_STM_BUF_SIZE_MAX
                )
            }
            AUTDInternalError::GainSTMFreqDivOutOfRange(div) => {
                write!(
                    f,
                    "Frequency division ({}) is out of range ([{}, {}])",
                    div, SAMPLING_FREQ_DIV_MIN, SAMPLING_FREQ_DIV_MAX
                )
            }
            AUTDInternalError::CycleOutOfRange(cycle) => {
                write!(
                    f,
                    "Cycle ({}) is out of range ([{}, {}])",
                    cycle, MIN_CYCLE, MAX_CYCLE
                )
            }
            AUTDInternalError::GainSTMModeNotSupported(mode) => {
                write!(f, "GainSTMMode ({:?}) is not supported", mode)
            }
            AUTDInternalError::ModulationError(message)
            | AUTDInternalError::GainError(message)
            | AUTDInternalError::LinkError(message)
            | AUTDInternalError::NotSupported(message) => {
                write!(f, "{}", message)
            }
            AUTDInternalError::LinkClosed => {
                write!(f, "Link is closed")
            }
            AUTDInternalError::TimerCreationFailed => {
                write!(f, "Failed to create timer")
            }
            AUTDInternalError::TimerDeleteFailed => {
                write!(f, "Failed to delete timer")
            }
            #[cfg(target_os = "windows")]
            AUTDInternalError::WindowsError(error) => {
                write!(f, "{}", error)
            }
        }
    }
}

impl std::error::Error for AUTDInternalError {
    #[cfg(target_os = "windows")]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AUTDInternalError::WindowsError(e) => Some(e),
            _ => None,
        }
    }

    #[cfg(not(target_os = "windows"))]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

// #[derive(Error, Debug, PartialEq)]
// pub enum AUTDInternalError {
//     #[error(
//         "Modulation buffer size ({0}) is out of range ([{}, {}])",
//         MOD_BUF_SIZE_MIN,
//         MOD_BUF_SIZE_MAX
//     )]
//     ModulationSizeOutOfRange(usize),
//     #[error(
//         "Frequency division ({0}) is out of range ([{}, {}])",
//         SAMPLING_FREQ_DIV_MIN,
//         SAMPLING_FREQ_DIV_MAX
//     )]
//     ModFreqDivOutOfRange(u32),
//     #[error("STM index is out of range")]
//     STMStartIndexOutOfRange,
//     #[error("STM finish is out of range")]
//     STMFinishIndexOutOfRange,
//     #[error(
//         "FocusSTM size ({0}) is out of range ([{}, {}])",
//         STM_BUF_SIZE_MIN,
//         FOCUS_STM_BUF_SIZE_MAX
//     )]
//     FocusSTMPointSizeOutOfRange(usize),
//     #[error(
//         "Frequency division ({0}) is out of range ([{}, {}])",
//         SAMPLING_FREQ_DIV_MIN,
//         SAMPLING_FREQ_DIV_MAX
//     )]
//     FocusSTMFreqDivOutOfRange(u32),
//     #[error("Point ({0}, {1}, {2}) is out of range. Each parameter must be in [{}, {}].", FOCUS_STM_FIXED_NUM_UNIT * FOCUS_STM_FIXED_NUM_LOWER as float, FOCUS_STM_FIXED_NUM_UNIT * FOCUS_STM_FIXED_NUM_UPPER as float)]
//     FocusSTMPointOutOfRange(float, float, float),
//     #[error(
//         "GainSTM size ({0}) is out of range ([{}, {}])",
//         STM_BUF_SIZE_MIN,
//         GAIN_STM_LEGACY_BUF_SIZE_MAX
//     )]
//     GainSTMLegacySizeOutOfRange(usize),
//     #[error(
//         "GainSTM size ({0}) is out of range ([{}, {}])",
//         STM_BUF_SIZE_MIN,
//         GAIN_STM_BUF_SIZE_MAX
//     )]
//     GainSTMSizeOutOfRange(usize),
//     #[error(
//         "Frequency division ({0}) is out of range ([{}, {}])",
//         SAMPLING_FREQ_DIV_MIN,
//         SAMPLING_FREQ_DIV_MAX
//     )]
//     GainSTMFreqDivOutOfRange(u32),
//     #[error("Cycle ({0}) is out of range ([{}, {}])", MIN_CYCLE, MAX_CYCLE)]
//     CycleOutOfRange(u16),

//     #[error("GainSTMMode ({0:?}) is not supported")]
//     GainSTMModeNotSupported(GainSTMMode),

//     #[error("{0}")]
//     ModulationError(String),
//     #[error("{0}")]
//     GainError(String),
//     #[error("{0}")]
//     LinkError(String),

//     #[error("{0}")]
//     NotSupported(String),

//     #[error("Link is closed")]
//     LinkClosed,

//     #[error("Failed to create timer")]
//     TimerCreationFailed(),
//     #[error("Failed to delete timer")]
//     TimerDeleteFailed(),

//     #[cfg(target_os = "windows")]
//     #[error("{0}")]
//     WindowsError(windows::core::Error),
// }

#[cfg(target_os = "windows")]
impl From<windows::core::Error> for AUTDInternalError {
    fn from(e: windows::core::Error) -> Self {
        AUTDInternalError::WindowsError(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn modulation_size_out_of_range() {
        let err = AUTDInternalError::ModulationSizeOutOfRange(1);
        assert!(err.source().is_none());
        assert_eq!(
            format!("{}", err),
            "Modulation buffer size (1) is out of range ([2, 65536])"
        );
        assert_eq!(format!("{:?}", err), "ModulationSizeOutOfRange(1)");

        let err = AUTDInternalError::ModulationSizeOutOfRange(1);
        assert_eq!(err, AUTDInternalError::ModulationSizeOutOfRange(1));
        assert_ne!(err, AUTDInternalError::ModulationSizeOutOfRange(2));
    }

    #[test]
    fn mod_freq_div_out_of_range() {
        let err = AUTDInternalError::ModFreqDivOutOfRange(1);
        assert!(err.source().is_none());
        assert_eq!(
            format!("{}", err),
            "Frequency division (1) is out of range ([512, 536870911])"
        );
        assert_eq!(format!("{:?}", err), "ModFreqDivOutOfRange(1)");

        let err = AUTDInternalError::ModFreqDivOutOfRange(1);
        assert_eq!(err, AUTDInternalError::ModFreqDivOutOfRange(1));
        assert_ne!(err, AUTDInternalError::ModFreqDivOutOfRange(2));
    }

    #[test]
    fn stm_start_index_out_of_range() {
        let err = AUTDInternalError::STMStartIndexOutOfRange;
        assert!(err.source().is_none());
        assert_eq!(format!("{}", err), "STM index is out of range");
        assert_eq!(format!("{:?}", err), "STMStartIndexOutOfRange");

        let err = AUTDInternalError::STMStartIndexOutOfRange;
        assert_eq!(err, AUTDInternalError::STMStartIndexOutOfRange);
    }

    #[test]
    fn stm_finish_index_out_of_range() {
        let err = AUTDInternalError::STMFinishIndexOutOfRange;
        assert!(err.source().is_none());
        assert_eq!(format!("{}", err), "STM finish is out of range");
        assert_eq!(format!("{:?}", err), "STMFinishIndexOutOfRange");

        let err = AUTDInternalError::STMFinishIndexOutOfRange;
        assert_eq!(err, AUTDInternalError::STMFinishIndexOutOfRange);
    }

    #[test]
    fn focus_stm_point_size_out_of_range() {
        let err = AUTDInternalError::FocusSTMPointSizeOutOfRange(1);
        assert!(err.source().is_none());
        assert_eq!(
            format!("{}", err),
            "FocusSTM size (1) is out of range ([2, 65536])"
        );
        assert_eq!(format!("{:?}", err), "FocusSTMPointSizeOutOfRange(1)");

        let err = AUTDInternalError::FocusSTMPointSizeOutOfRange(1);
        assert_eq!(err, AUTDInternalError::FocusSTMPointSizeOutOfRange(1));
        assert_ne!(err, AUTDInternalError::FocusSTMPointSizeOutOfRange(2));
    }

    #[test]
    fn focus_stm_freq_div_out_of_range() {
        let err = AUTDInternalError::FocusSTMFreqDivOutOfRange(1);
        assert!(err.source().is_none());
        assert_eq!(
            format!("{}", err),
            "Frequency division (1) is out of range ([512, 536870911])"
        );
        assert_eq!(format!("{:?}", err), "FocusSTMFreqDivOutOfRange(1)");

        let err = AUTDInternalError::FocusSTMFreqDivOutOfRange(1);
        assert_eq!(err, AUTDInternalError::FocusSTMFreqDivOutOfRange(1));
        assert_ne!(err, AUTDInternalError::FocusSTMFreqDivOutOfRange(2));
    }

    #[test]
    fn focus_stm_point_out_of_range() {
        let err = AUTDInternalError::FocusSTMPointOutOfRange(1.0, 2.0, 3.0);
        assert!(err.source().is_none());
        assert_eq!(
            format!("{}", err),
            "Point (1, 2, 3) is out of range. Each parameter must be in [-3276.8, 3276.775]."
        );
        assert_eq!(
            format!("{:?}", err),
            "FocusSTMPointOutOfRange(1.0, 2.0, 3.0)"
        );

        let err = AUTDInternalError::FocusSTMPointOutOfRange(1.0, 2.0, 3.0);
        assert_eq!(
            err,
            AUTDInternalError::FocusSTMPointOutOfRange(1.0, 2.0, 3.0)
        );
        assert_ne!(
            err,
            AUTDInternalError::FocusSTMPointOutOfRange(1.0, 2.0, 4.0)
        );
    }

    #[test]
    fn gain_stm_legacy_size_out_of_range() {
        let err = AUTDInternalError::GainSTMLegacySizeOutOfRange(1);
        assert!(err.source().is_none());
        assert_eq!(
            format!("{}", err),
            "GainSTM size (1) is out of range ([2, 2048])"
        );
        assert_eq!(format!("{:?}", err), "GainSTMLegacySizeOutOfRange(1)");

        let err = AUTDInternalError::GainSTMLegacySizeOutOfRange(1);
        assert_eq!(err, AUTDInternalError::GainSTMLegacySizeOutOfRange(1));
        assert_ne!(err, AUTDInternalError::GainSTMLegacySizeOutOfRange(2));
    }

    #[test]
    fn gain_stm_size_out_of_range() {
        let err = AUTDInternalError::GainSTMSizeOutOfRange(1);
        assert!(err.source().is_none());
        assert_eq!(
            format!("{}", err),
            "GainSTM size (1) is out of range ([2, 1024])"
        );
        assert_eq!(format!("{:?}", err), "GainSTMSizeOutOfRange(1)");

        let err = AUTDInternalError::GainSTMSizeOutOfRange(1);
        assert_eq!(err, AUTDInternalError::GainSTMSizeOutOfRange(1));
        assert_ne!(err, AUTDInternalError::GainSTMSizeOutOfRange(2));
    }

    #[test]
    fn gain_stm_freq_div_out_of_range() {
        let err = AUTDInternalError::GainSTMFreqDivOutOfRange(1);
        assert!(err.source().is_none());
        assert_eq!(
            format!("{}", err),
            "Frequency division (1) is out of range ([512, 536870911])"
        );
        assert_eq!(format!("{:?}", err), "GainSTMFreqDivOutOfRange(1)");

        let err = AUTDInternalError::GainSTMFreqDivOutOfRange(1);
        assert_eq!(err, AUTDInternalError::GainSTMFreqDivOutOfRange(1));
        assert_ne!(err, AUTDInternalError::GainSTMFreqDivOutOfRange(2));
    }

    #[test]
    fn cycle_out_of_range() {
        let err = AUTDInternalError::CycleOutOfRange(1);
        assert!(err.source().is_none());
        assert_eq!(format!("{}", err), "Cycle (1) is out of range ([2, 8191])");
        assert_eq!(format!("{:?}", err), "CycleOutOfRange(1)");

        let err = AUTDInternalError::CycleOutOfRange(1);
        assert_eq!(err, AUTDInternalError::CycleOutOfRange(1));
        assert_ne!(err, AUTDInternalError::CycleOutOfRange(2));
    }

    #[test]
    fn gain_stm_mode_not_supported() {
        let err = AUTDInternalError::GainSTMModeNotSupported(GainSTMMode::PhaseDutyFull);
        assert!(err.source().is_none());
        assert_eq!(
            format!("{}", err),
            "GainSTMMode (PhaseDutyFull) is not supported"
        );
        assert_eq!(
            format!("{:?}", err),
            "GainSTMModeNotSupported(PhaseDutyFull)"
        );

        let err = AUTDInternalError::GainSTMModeNotSupported(GainSTMMode::PhaseDutyFull);
        assert_eq!(
            err,
            AUTDInternalError::GainSTMModeNotSupported(GainSTMMode::PhaseDutyFull)
        );
        assert_ne!(
            err,
            AUTDInternalError::GainSTMModeNotSupported(GainSTMMode::PhaseFull)
        );
    }

    #[test]
    fn modulation_error() {
        let err = AUTDInternalError::ModulationError("error".to_string());
        assert!(err.source().is_none());
        assert_eq!(format!("{}", err), "error");
        assert_eq!(format!("{:?}", err), "ModulationError(\"error\")");

        let err = AUTDInternalError::ModulationError("error".to_string());
        assert_eq!(err, AUTDInternalError::ModulationError("error".to_string()));
        assert_ne!(
            err,
            AUTDInternalError::ModulationError("error2".to_string())
        );
    }

    #[test]
    fn gain_error() {
        let err = AUTDInternalError::GainError("error".to_string());
        assert!(err.source().is_none());
        assert_eq!(format!("{}", err), "error");
        assert_eq!(format!("{:?}", err), "GainError(\"error\")");

        let err = AUTDInternalError::GainError("error".to_string());
        assert_eq!(err, AUTDInternalError::GainError("error".to_string()));
        assert_ne!(err, AUTDInternalError::GainError("error2".to_string()));
    }

    #[test]
    fn link_error() {
        let err = AUTDInternalError::LinkError("error".to_string());
        assert!(err.source().is_none());
        assert_eq!(format!("{}", err), "error");
        assert_eq!(format!("{:?}", err), "LinkError(\"error\")");

        let err = AUTDInternalError::LinkError("error".to_string());
        assert_eq!(err, AUTDInternalError::LinkError("error".to_string()));
        assert_ne!(err, AUTDInternalError::LinkError("error2".to_string()));
    }

    #[test]
    fn not_supported() {
        let err = AUTDInternalError::NotSupported("error".to_string());
        assert!(err.source().is_none());
        assert_eq!(format!("{}", err), "error");
        assert_eq!(format!("{:?}", err), "NotSupported(\"error\")");

        let err = AUTDInternalError::NotSupported("error".to_string());
        assert_eq!(err, AUTDInternalError::NotSupported("error".to_string()));
        assert_ne!(err, AUTDInternalError::NotSupported("error2".to_string()));
    }

    #[test]
    fn link_closed() {
        let err = AUTDInternalError::LinkClosed;
        assert!(err.source().is_none());
        assert_eq!(format!("{}", err), "Link is closed");
        assert_eq!(format!("{:?}", err), "LinkClosed");

        let err = AUTDInternalError::LinkClosed;
        assert_eq!(err, AUTDInternalError::LinkClosed);
    }

    #[test]
    fn timer_creation_failed() {
        let err = AUTDInternalError::TimerCreationFailed;
        assert!(err.source().is_none());
        assert_eq!(format!("{}", err), "Failed to create timer");
        assert_eq!(format!("{:?}", err), "TimerCreationFailed");

        let err = AUTDInternalError::TimerCreationFailed;
        assert_eq!(err, AUTDInternalError::TimerCreationFailed);
    }

    #[test]
    fn timer_delete_failed() {
        let err = AUTDInternalError::TimerDeleteFailed;
        assert!(err.source().is_none());
        assert_eq!(format!("{}", err), "Failed to delete timer");
        assert_eq!(format!("{:?}", err), "TimerDeleteFailed");

        let err = AUTDInternalError::TimerDeleteFailed;
        assert_eq!(err, AUTDInternalError::TimerDeleteFailed);
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn windows_error() {
        let win_err =
            windows::core::Error::new(windows::core::HRESULT(1), windows::core::HSTRING::new());
        let err = AUTDInternalError::from(win_err.clone());
        assert_eq!(
            err.source()
                .and_then(|e| e.downcast_ref::<windows::core::Error>()),
            Some(&win_err)
        );
        assert_eq!(format!("{}", err), format!("{}", win_err));
        assert_eq!(format!("{:?}", err), format!("WindowsError({:?})", win_err));

        let err = AUTDInternalError::from(win_err.clone());
        assert_eq!(err, AUTDInternalError::from(win_err));
        assert_ne!(
            err,
            AUTDInternalError::from(windows::core::Error::new(
                windows::core::HRESULT(2),
                windows::core::HSTRING::new(),
            ))
        );
    }
}
