/*
 * File: sync_mode.rs
 * Project: src
 * Created Date: 12/07/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

/// Synchronization modes of an EtherCAT slave
/// See [Beckhoff's document](https://infosys.beckhoff.com/english.php?content=../content/1033/ethercatsystem/2469122443.html&id=) for more details.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum SyncMode {
    /// DC sync mode
    DC = 0,
    /// Free run mode
    FreeRun = 1,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sync_mode() {
        assert_eq!(std::mem::size_of::<SyncMode>(), 1);

        let s = SyncMode::DC;

        let sc = Clone::clone(&s);
        assert_eq!(s as u8, sc as u8);
    }

    #[test]
    fn debug() {
        let s = SyncMode::DC;
        assert_eq!(format!("{:?}", s), "DC");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde() {
        let s = SyncMode::DC;

        let serialized = serde_json::to_string(&s).unwrap();
        assert_eq!(serialized, "\"DC\"");

        let deserialized: SyncMode = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, SyncMode::DC);
    }
}
