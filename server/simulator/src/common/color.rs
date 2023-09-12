/*
 * File: color.rs
 * Project: common
 * Created Date: 22/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

pub trait Color {
    fn rgba(&self) -> [f32; 4];
    fn hsva(&self) -> [f32; 4];
}

pub struct Hsv {
    pub h: f32,
    pub s: f32,
    pub v: f32,
    pub a: f32,
}

impl Color for Hsv {
    #[allow(clippy::many_single_char_names)]
    fn rgba(&self) -> [f32; 4] {
        let h = self.h % 1.0;
        let s = self.s;
        let v = self.v;
        let alpha = self.a;

        if s == 0.0 {
            return [v, v, v, alpha];
        }
        let i = (h * 6.0).floor();
        let f = h * 6.0 - i;
        let p = v * (1. - s);
        let q = v * (1. - (s * f));
        let t = v * (1. - (s * (1. - f)));
        match i as i32 {
            0 => [v, t, p, alpha],
            1 => [q, v, p, alpha],
            2 => [p, v, t, alpha],
            3 => [p, q, v, alpha],
            4 => [t, p, v, alpha],
            5 => [v, p, q, alpha],
            _ => unreachable!(),
        }
    }

    fn hsva(&self) -> [f32; 4] {
        [self.h, self.s, self.v, self.a]
    }
}
