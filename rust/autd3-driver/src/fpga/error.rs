/*
 * File: error.rs
 * Project: fpga
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use thiserror::Error;

#[derive(Error, Debug)]
pub enum FPGAError {
    #[error(
        "Modulation sampling frequency minimum is {}, but {0} is specified",
        crate::fpga::MOD_SAMPLING_FREQ_DIV_MIN
    )]
    ModFreqDivOutOfRange(u32),
    #[error(
        "STM sampling frequency minimum is {}, but {0} is specified",
        crate::fpga::STM_SAMPLING_FREQ_DIV_MIN
    )]
    STMFreqDivOutOfRange(u32),
    #[error(
        "Silencer cycle minimum is {}, but {0} is specified",
        crate::fpga::SILENCER_CYCLE_MIN
    )]
    SilencerCycleOutOfRange(u16),
    #[error(
        "Modulation buffer maximum is {}, but {0} are to be sent",
        crate::MOD_BUF_SIZE_MAX
    )]
    ModulationOutOfBuffer(usize),
    #[error(
        "PointSTM buffer maximum is {}, but {0} are to be sent",
        crate::POINT_STM_BUF_SIZE_MAX
    )]
    PointSTMOutOfBuffer(usize),
    #[error("GainSTM buffer maximum is {1}, but {0} are to be sent")]
    GainSTMOutOfBuffer(usize, usize),
}
