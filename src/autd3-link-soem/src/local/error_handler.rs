/*
 * File: error_handler.rs
 * Project: ecat_thread
 * Created Date: 03/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::fmt::Write as _;

use crate::local::soem_bindings::*;

use autd3_core::spdlog::prelude::*;

pub struct EcatErrorHandler<F: Fn(&str)> {
    pub logger: Logger,
    pub on_lost: Option<F>,
}

impl<F: Fn(&str)> EcatErrorHandler<F> {
    pub fn handle(&self) -> bool {
        unsafe {
            ec_group[0].docheckstate = 0;
            ec_readstate();
            let mut msg = String::new();
            ec_slave
                .iter_mut()
                .enumerate()
                .skip(1)
                .take(ec_slavecount as usize)
                .for_each(|(i, slave)| {
                    if slave.state != ec_state_EC_STATE_OPERATIONAL as u16 {
                        ec_group[0].docheckstate = 1;
                        if slave.state
                            == ec_state_EC_STATE_SAFE_OP as u16 + ec_state_EC_STATE_ERROR as u16
                        {
                            warn!(
                                logger: self.logger,
                                "slave {} is in SAFE_OP + ERROR, attempting ack",
                            i);
                            slave.state =
                                ec_state_EC_STATE_SAFE_OP as u16 + ec_state_EC_STATE_ACK as u16;
                            ec_writestate(i as _);
                        } else if slave.state == ec_state_EC_STATE_SAFE_OP as u16 {
                            warn!(
                                logger: self.logger,
                                "slave {} is in SAFE_OP, change to OPERATIONAL",
                            i);
                            slave.state = ec_state_EC_STATE_OPERATIONAL as _;
                            ec_writestate(i as _);
                        } else if slave.state > ec_state_EC_STATE_NONE as _ {
                            if ec_reconfig_slave(i as _, 500) != 0 {
                                slave.islost = 0;
                                info!(logger: self.logger,"slave {} reconfigured", i);
                            }
                        } else if slave.islost == 0 {
                            ec_statecheck(
                                i as _,
                                ec_state_EC_STATE_OPERATIONAL as _,
                                EC_TIMEOUTRET as _,
                            );
                            if slave.state == ec_state_EC_STATE_NONE as u16 {
                                slave.islost = 1;
                                writeln!(msg, "slave {i} lost").unwrap();
                                error!(logger: self.logger, "slave {} lost", i);
                            }
                        }
                    }
                    if slave.islost != 0 {
                        if slave.state == ec_state_EC_STATE_NONE as u16 {
                            if ec_recover_slave(i as _, 500) != 0 {
                                slave.islost = 0;
                                info!(logger: self.logger, "slave {} recovered", i);
                            }
                        } else {
                            slave.islost = 0;
                            info!(logger: self.logger, "slave {} found", i);
                        }
                    }
                });

            if ec_group[0].docheckstate == 0 {
                return true;
            }

            if ec_slave
                .iter()
                .skip(1)
                .take(ec_slavecount as usize)
                .any(|slave| slave.islost != 0)
            {
                if let Some(f) = &self.on_lost {
                    f(&msg);
                }
                return false;
            }
            true
        }
    }
}
