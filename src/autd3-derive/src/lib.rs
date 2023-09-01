/*
 * File: lib.rs
 * Project: src
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Modulation)]
pub fn modulation_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_modulation_macro(&ast)
}

fn impl_modulation_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let linetimes_prop = generics.lifetimes();
    let linetimes_impl = generics.lifetimes();
    let linetimes_datagram = generics.lifetimes();
    let type_params_prop = generics.type_params();
    let type_params_impl = generics.type_params();
    let type_params_datagram = generics.type_params();
    let (_, ty_generics, where_clause) = generics.split_for_impl();
    let gen = quote! {
        impl <#(#linetimes_prop,)* #(#type_params_prop,)*> autd3_driver::modulation::ModulationProperty for #name #ty_generics #where_clause {
            fn sampling_frequency_division(&self) -> u32 {
                self.freq_div
            }
        }

        impl <#(#linetimes_impl,)* #(#type_params_impl,)*> #name #ty_generics #where_clause {
            /// Set sampling frequency division
            ///
            /// # Arguments
            ///
            /// * `freq_div` - Sampling frequency division. The sampling frequency will be [autd3_driver::FPGA_SUB_CLK_FREQ] / `freq_div`. The value must be and must be at least [autd3_driver::SAMPLING_FREQ_DIV_MIN]
            ///
            #[allow(clippy::needless_update)]
            pub fn with_sampling_frequency_division(self, freq_div: u32) -> Self {
                Self {freq_div, ..self}
            }

            /// Set sampling frequency
            ///
            /// # Arguments
            ///
            /// * `freq` - Sampling frequency. The sampling frequency closest to `freq` from the possible sampling frequencies is set.
            ///
            #[allow(clippy::needless_update)]
            pub fn with_sampling_frequency(self, freq: autd3_driver::defined::float) -> Self {
                let freq_div = autd3_driver::fpga::FPGA_SUB_CLK_FREQ as autd3_driver::defined::float / freq;
                self.with_sampling_frequency_division(freq_div as _)
            }

            /// Set sampling period
            ///
            /// # Arguments
            ///
            /// * `period` - Sampling period. The sampling period closest to `period` from the possible sampling periods is set.
            ///
            #[allow(clippy::needless_update)]
            pub fn with_sampling_period(self, period: std::time::Duration) -> Self {
                let freq_div = autd3_driver::fpga::FPGA_SUB_CLK_FREQ as autd3_driver::defined::float / 1000000000. * period.as_nanos() as autd3_driver::defined::float;
                self.with_sampling_frequency_division(freq_div as _)
            }
        }

        impl <#(#linetimes_datagram,)* #(#type_params_datagram,)* T: autd3_driver::geometry::Transducer> autd3_driver::datagram::Datagram<T> for #name #ty_generics #where_clause {
            type O1 = autd3_driver::operation::ModulationOp;
            type O2 = autd3_driver::operation::NullOp;

            fn operation(self) -> Result<(Self::O1, Self::O2), autd3_driver::error::AUTDInternalError> {
                let freq_div = self.freq_div;
                Ok((Self::O1::new(self.calc()?, freq_div), Self::O2::default()))
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Gain)]
pub fn gain_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_gain_macro(ast)
}

fn impl_gain_macro(ast: syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let linetimes = generics.lifetimes();
    let type_params_for_any = generics.type_params();
    let linetimes_for_any = generics.lifetimes();
    let (_, ty_generics, where_clause) = generics.split_for_impl();
    let type_params = generics
        .type_params()
        .filter(|ty| ty.ident != "T")
        .collect::<Vec<_>>();
    let gen = quote! {
        impl <#(#linetimes_for_any,)* #(#type_params_for_any,)*> autd3_driver::gain::GainAsAny for #name #ty_generics #where_clause {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }

        impl <#(#linetimes,)* #(#type_params,)* T: autd3_driver::geometry::Transducer> autd3_driver::datagram::Datagram<T> for #name #ty_generics #where_clause
            where autd3_driver::operation::GainOp<T,Self>: autd3_driver::operation::Operation<T>
        {
            type O1 = autd3_driver::operation::GainOp<T,Self>;
            type O2 = autd3_driver::operation::NullOp;

            fn operation(self) -> Result<(Self::O1, Self::O2), autd3_driver::error::AUTDInternalError> {
                Ok((Self::O1::new(self), Self::O2::default()))
            }
        }
    };
    gen.into()
}
