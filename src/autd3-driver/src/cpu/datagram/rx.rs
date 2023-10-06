/*
 * File: rx_message.rs
 * Project: cpu
 * Created Date: 29/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#[derive(Clone, Copy)]
#[repr(C)]
pub struct RxMessage {
    pub data: u8,
    pub ack: u8,
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use super::*;

    #[test]
    fn rx_message() {
        assert_eq!(size_of::<RxMessage>(), 2);
    }

    #[test]
    fn rx_message_clone() {
        let msg = RxMessage {
            ack: 0x01,
            data: 0x02,
        };
        let msg2 = msg.clone();

        assert_eq!(msg.ack, msg2.ack);
        assert_eq!(msg.data, msg2.data);
    }

    #[test]
    fn rx_message_copy() {
        let msg = RxMessage {
            ack: 0x01,
            data: 0x02,
        };
        let msg2 = msg;

        assert_eq!(msg.ack, msg2.ack);
        assert_eq!(msg.data, msg2.data);
    }
}
