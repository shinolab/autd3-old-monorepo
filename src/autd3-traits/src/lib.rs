/*
 * File: lib.rs
 * Project: src
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/06/2023
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
    let type_params = generics.type_params();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics autd3_core::modulation::ModulationProperty for #name #ty_generics #where_clause {
            fn sampling_frequency_division(&self) -> u32 {
                self.freq_div
            }

            #[allow(clippy::needless_update)]
            fn with_sampling_frequency_division(self, freq_div: u32) -> Self {
                Self {freq_div, ..self}
            }

            fn sampling_freq(&self) -> autd3_core::float {
                autd3_core::FPGA_SUB_CLK_FREQ as autd3_core::float / self.freq_div as autd3_core::float
            }
        }

        impl <#(#type_params,)* T: autd3_core::geometry::Transducer> autd3_core::datagram::Datagram<T> for #name #ty_generics #where_clause {
            type H = autd3_core::Modulation;
            type B = autd3_core::NullBody;

            fn operation(
                &mut self,
                _geometry: &autd3_core::geometry::Geometry<T>,
            ) -> Result<(Self::H, Self::B), autd3_core::error::AUTDInternalError> {
                let freq_div = self.freq_div;
                Ok((Self::H::new(self.calc()?, freq_div), Self::B::default()))
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
    let type_params = generics.type_params();
    let (_, ty_generics, where_clause) = generics.split_for_impl();
    if generics.type_params().any(|ty| ty.ident == "T") {
        let gen = quote! {
            impl <#(#linetimes,)* #(#type_params,)*> autd3_core::datagram::Datagram<T> for #name #ty_generics #where_clause {
                type H = autd3_core::NullHeader;
                type B = T::Gain;

                fn operation(
                    &mut self,
                    geometry: &Geometry<T>,
                ) -> Result<(Self::H, Self::B), autd3_core::error::AUTDInternalError> {
                    Ok((Self::H::default(), <Self::B as autd3_core::GainOp>::new(self.calc(geometry)?, || {
                        geometry.transducers().map(|tr| tr.cycle()).collect()
                    })))
                }
            }
        };
        gen.into()
    } else {
        let gen = quote! {
            impl <#(#linetimes,)* #(#type_params,)* T: autd3_core::geometry::Transducer> autd3_core::datagram::Datagram<T> for #name #ty_generics #where_clause {
                type H = autd3_core::NullHeader;
                type B = T::Gain;

                fn operation(
                    &mut self,
                    geometry: &Geometry<T>,
                ) -> Result<(Self::H, Self::B), autd3_core::error::AUTDInternalError> {
                    Ok((Self::H::default(), <Self::B as autd3_core::GainOp>::new(self.calc(geometry)?, || {
                        geometry.transducers().map(|tr| tr.cycle()).collect()
                    })))
                }
            }
        };
        gen.into()
    }
}
