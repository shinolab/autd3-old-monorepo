/*
 * File: clear.rs
 * Project: operation
 * Created Date: 08/01/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use super::Operation;
use crate::{TxDatagram, MSG_CLEAR};
use anyhow::Result;

#[derive(Default)]
pub struct Clear {
    sent: bool,
}

impl Operation for Clear {
    fn pack(&mut self, tx: &mut TxDatagram) -> Result<()> {
        tx.header_mut().msg_id = MSG_CLEAR;
        tx.num_bodies = 0;
        self.sent = true;
        Ok(())
    }

    fn init(&mut self) {
        self.sent = false;
    }

    fn is_finished(&self) -> bool {
        self.sent
    }
}
