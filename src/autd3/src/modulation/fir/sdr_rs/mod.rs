#![allow(dead_code)]

use num::Complex;

pub type IQ<T> = Complex<T>;
pub type Real<T> = T;

#[macro_export]
macro_rules! chain_blocks {
    ($x:ident, $($f:ident),*) => {{
        let x = $x;
        $(
            let x = $f.process(&x);
        )*
        x
    }}
}

pub mod fir;
pub use fir::FIR;
