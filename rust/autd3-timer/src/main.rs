/*
 * File: main.rs
 * Project: src
 * Created Date: 24/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/05/2021
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Hapis Lab. All rights reserved.
 *
 */

use autd3_timer::{Timer, TimerCallback};
use std::time::Instant;

pub struct Callback {
    start: Instant,
    i: i32,
}

impl Callback {
    pub fn new(i: i32) -> Self {
        Self {
            start: std::time::Instant::now(),
            i,
        }
    }
}

impl TimerCallback for Callback {
    fn rt_thread(&mut self) {
        println!(
            "{}: {}",
            self.i,
            self.start.elapsed().as_micros() as f64 / 1000.0
        );
    }
}
fn main() {
    use std::{thread, time};

    let timer = Timer::start(Callback::new(1), 1000 * 1000).unwrap();
    let timer2 = Timer::start(Callback::new(2), 1000 * 1000).unwrap();

    let ten_millis = time::Duration::from_millis(10);

    thread::sleep(ten_millis);

    timer.close().unwrap();
    timer2.close().unwrap();

    println!("fin");
}
