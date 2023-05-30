/*
 * File: ethernet_adapters.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::fmt;
use std::vec::Vec;

use crate::local::native_methods;

use std::ffi::CStr;
use std::ops::Index;
use std::slice;

#[derive(Clone)]
pub struct EthernetAdapter {
    desc: String,
    name: String,
}

impl EthernetAdapter {
    pub fn desc(&self) -> &str {
        &self.desc
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone)]
pub struct EthernetAdapters {
    adapters: Vec<EthernetAdapter>,
}

impl EthernetAdapters {
    pub fn new() -> Self {
        let mut adapters = Vec::new();
        unsafe {
            let mut adapter = native_methods::ec_find_adapters();
            while !adapter.is_null() {
                let desc = CStr::from_ptr(((*adapter).desc).as_ptr())
                    .to_str()
                    .unwrap()
                    .to_string();
                let name = CStr::from_ptr(((*adapter).name).as_ptr())
                    .to_str()
                    .unwrap()
                    .to_string();
                adapters.push(EthernetAdapter { desc, name });
                adapter = (*adapter).next;
            }
            native_methods::ec_free_adapters(adapter);
            EthernetAdapters { adapters }
        }
    }

    pub fn len(&self) -> usize {
        self.adapters.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for EthernetAdapters {
    fn default() -> Self {
        Self::new()
    }
}

impl Index<usize> for EthernetAdapters {
    type Output = EthernetAdapter;
    fn index(&self, index: usize) -> &Self::Output {
        &self.adapters[index]
    }
}

impl<'a> IntoIterator for &'a EthernetAdapters {
    type Item = &'a EthernetAdapter;
    type IntoIter = slice::Iter<'a, EthernetAdapter>;

    fn into_iter(self) -> slice::Iter<'a, EthernetAdapter> {
        self.adapters.iter()
    }
}

impl fmt::Display for EthernetAdapter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.name, self.desc)
    }
}
