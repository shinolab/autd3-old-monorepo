/*
 * File: lib.rs
 * Project: src
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
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
        impl #impl_generics #name #ty_generics #where_clause {
            fn sampling_frequency_division(&mut self) -> &mut u32 {
                &mut self.freq_div
            }

            fn sampling_freq(&self) -> f64 {
                autd3_core::FPGA_CLK_FREQ as f64 / self.freq_div as f64
            }
        }

        impl <#(#type_params),* T: autd3_core::geometry::Transducer> autd3_core::sendable::Sendable<T> for #name #ty_generics #where_clause {
            type H = autd3_core::Modulation;
            type B = autd3_core::NullBody;

            fn header_operation(&mut self) -> Result<Self::H, autd3_core::error::AUTDInternalError> {
                Ok(Self::H::new(self.calc()?, self.freq_div))
            }

            fn body_operation(
                &mut self,
                _geometry: &autd3_core::geometry::Geometry<T>,
            ) -> Result<Self::B, autd3_core::error::AUTDInternalError> {
                Ok(Self::B::default())
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
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics autd3_core::sendable::Sendable<autd3_core::geometry::LegacyTransducer> for #name #ty_generics #where_clause {
            type H = autd3_core::NullHeader;
            type B = autd3_core::GainLegacy;

            fn header_operation(&mut self) -> Result<Self::H, AUTDInternalError> {
                Ok(Self::H::default())
            }

            fn body_operation(
                &mut self,
                geometry: &Geometry<autd3_core::geometry::LegacyTransducer>,
            ) -> Result<Self::B, autd3_core::error::AUTDInternalError> {
                Ok(Self::B::new(self.calc(geometry)?))
            }
        }

        impl #impl_generics autd3_core::sendable::Sendable<autd3_core::geometry::AdvancedTransducer> for #name #ty_generics #where_clause {
            type H = autd3_core::NullHeader;
            type B = autd3_core::GainAdvanced;

            fn header_operation(&mut self) -> Result<Self::H, AUTDInternalError> {
                Ok(Self::H::default())
            }

            fn body_operation(
                &mut self,
                geometry: &Geometry<autd3_core::geometry::AdvancedTransducer>,
            ) -> Result<Self::B, autd3_core::error::AUTDInternalError> {
                Ok(Self::B::new(self.calc(geometry)?, geometry.transducers().map(|tr| tr.cycle()).collect()))
            }
        }

        impl #impl_generics autd3_core::sendable::Sendable<autd3_core::geometry::AdvancedPhaseTransducer> for #name #ty_generics #where_clause {
            type H = autd3_core::NullHeader;
            type B = autd3_core::GainAdvancedPhase;

            fn header_operation(&mut self) -> Result<Self::H, AUTDInternalError> {
                Ok(Self::H::default())
            }

            fn body_operation(
                &mut self,
                geometry: &Geometry<autd3_core::geometry::AdvancedPhaseTransducer>,
            ) -> Result<Self::B, autd3_core::error::AUTDInternalError> {
                Ok(
                    Self::B::new(
                        self.calc(geometry)?,
                        geometry.transducers().map(|tr| tr.cycle()).collect(),
                    ),
                )
            }
        }
    };
    gen.into()
}
