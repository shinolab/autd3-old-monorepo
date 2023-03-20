/*
 * File: error_handler.rs
 * Project: ecat_thread
 * Created Date: 03/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/03/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::fmt::Write as _;

use crate::native_methods::*;

pub struct EcatErrorHandler<F: Fn(&str)> {
    pub error_handle: Option<F>,
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
                .take(ec_slavecount as usize + 1)
                .skip(1)
                .for_each(|(i, slave)| {
                    if slave.state != ec_state_EC_STATE_OPERATIONAL as _ {
                        ec_group[0].docheckstate = 1;
                        if slave.state
                            == ec_state_EC_STATE_SAFE_OP as u16 + ec_state_EC_STATE_ERROR as u16
                        {
                            writeln!(
                                msg,
                                "ERROR : slave {i} is in SAFE_OP + ERROR, attempting ack",
                            )
                            .unwrap();
                            slave.state =
                                ec_state_EC_STATE_SAFE_OP as u16 + ec_state_EC_STATE_ACK as u16;
                            ec_writestate(i as _);
                        } else if slave.state == ec_state_EC_STATE_SAFE_OP as _ {
                            writeln!(
                                msg,
                                "ERROR : slave {i} is in SAFE_OP, change to OPERATIONAL",
                            )
                            .unwrap();
                            slave.state = ec_state_EC_STATE_OPERATIONAL as _;
                            ec_writestate(i as _);
                        } else if slave.state > ec_state_EC_STATE_NONE as _ {
                            if ec_reconfig_slave(i as _, 500) != 0 {
                                slave.islost = 0;
                                writeln!(msg, "MESSAGE : slave {i} reconfigured").unwrap();
                            }
                        } else if slave.islost == 0 {
                            ec_statecheck(
                                i as _,
                                ec_state_EC_STATE_OPERATIONAL as _,
                                EC_TIMEOUTRET as _,
                            );
                            if slave.state == ec_state_EC_STATE_NONE as _ {
                                slave.islost = 1;
                                writeln!(msg, "ERROR : slave {i} lost").unwrap();
                            }
                        }
                    }
                    if slave.islost != 0 {
                        if slave.state == ec_state_EC_STATE_NONE as _ {
                            if ec_recover_slave(i as _, 500) != 0 {
                                slave.islost = 0;
                                writeln!(msg, "MESSAGE : slave {i} recovered").unwrap();
                            }
                        } else {
                            slave.islost = 0;
                            writeln!(msg, "MESSAGE : slave {i} found").unwrap();
                        }
                    }
                });

            if ec_group[0].docheckstate == 0 {
                return true;
            }

            if let Some(f) = &self.error_handle {
                for slave in ec_slave.iter().take(ec_slavecount as usize + 1).skip(1) {
                    if slave.islost != 0 {
                        f(&msg);
                        return false;
                    }
                }
            }
            true
        }
    }
}
