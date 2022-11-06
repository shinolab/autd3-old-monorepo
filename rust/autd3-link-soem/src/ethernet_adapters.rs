/*
 * File: ethernet_adapters.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/11/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::fmt;
use std::vec::Vec;

use crate::native_methods;

use std::ffi::CStr;
use std::ops::Index;
use std::slice;

#[derive(Copy, Clone)]
pub struct EthernetAdapter<'a> {
    pub desc: &'a str,
    pub name: &'a str,
}

#[derive(Clone)]
pub struct EthernetAdapters<'a> {
    adapters: Vec<EthernetAdapter<'a>>,
}

impl<'a> EthernetAdapters<'a> {
    pub fn len(&self) -> usize {
        self.adapters.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a> Default for EthernetAdapters<'a> {
    fn default() -> Self {
        let mut adapters = Vec::new();
        unsafe {
            let mut adapter = native_methods::ec_find_adapters();
            while !adapter.is_null() {
                let desc = CStr::from_ptr(((*adapter).desc).as_ptr()).to_str().unwrap();
                let name = CStr::from_ptr(((*adapter).name).as_ptr()).to_str().unwrap();
                adapters.push(EthernetAdapter { desc, name });
                adapter = (*adapter).next;
            }
            native_methods::ec_free_adapters(adapter);
            EthernetAdapters { adapters }
        }
    }
}

impl<'a> Index<usize> for EthernetAdapters<'a> {
    type Output = EthernetAdapter<'a>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.adapters[index]
    }
}

impl<'a> IntoIterator for &'a EthernetAdapters<'a> {
    type Item = &'a EthernetAdapter<'a>;
    type IntoIter = slice::Iter<'a, EthernetAdapter<'a>>;

    fn into_iter(self) -> slice::Iter<'a, EthernetAdapter<'a>> {
        self.adapters.iter()
    }
}

impl<'a> fmt::Display for EthernetAdapter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.desc, self.name)
    }
}
