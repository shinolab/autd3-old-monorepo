/*
 * File: lib.rs
 * Project: src
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/05/2023
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
        impl #impl_generics autd3_core::modulation::ModulationProperty for #name #ty_generics #where_clause {
            fn sampling_frequency_division(&self) -> u32 {
                self.freq_div
            }

            fn set_sampling_frequency_division(&mut self, freq_div: u32) {
                self.freq_div = freq_div;
            }

            fn sampling_freq(&self) -> autd3_core::float {
                autd3_core::FPGA_SUB_CLK_FREQ as autd3_core::float / self.freq_div as autd3_core::float
            }
        }

        #[cfg(not(feature = "dynamic"))]
        impl <#(#type_params,)* T: autd3_core::geometry::Transducer> autd3_core::sendable::Sendable<T> for #name #ty_generics #where_clause {
            type H = autd3_core::Modulation;
            type B = autd3_core::NullBody;

            fn operation(
                mut self,
                _geometry: &autd3_core::geometry::Geometry<T>,
            ) -> Result<(Self::H, Self::B), autd3_core::error::AUTDInternalError> {
                let freq_div = self.freq_div;
                Ok((Self::H::new(self.calc()?, freq_div), Self::B::default()))
            }
        }

        #[cfg(feature = "dynamic")]
        impl #impl_generics autd3_core::sendable::Sendable for #name #ty_generics #where_clause {
            fn operation(
                &mut self,
                _: &autd3_core::geometry::Geometry<autd3_core::geometry::DynamicTransducer>,
            ) -> Result<
                (
                    Box<dyn autd3_core::Operation>,
                    Box<dyn autd3_core::Operation>,
                ),
                autd3_core::error::AUTDInternalError,
            > {
                let freq_div = self.freq_div;
                Ok((
                    Box::new(autd3_core::Modulation::new(self.calc()?, freq_div)),
                    Box::new(autd3_core::NullBody::default()),
                ))
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
        #[cfg(not(feature = "dynamic"))]
        impl #impl_generics autd3_core::sendable::Sendable<autd3_core::geometry::LegacyTransducer> for #name #ty_generics #where_clause {
            type H = autd3_core::NullHeader;
            type B = autd3_core::GainLegacy;

            fn operation(
                mut self,
                geometry: &Geometry<autd3_core::geometry::LegacyTransducer>,
            ) -> Result<(Self::H, Self::B), autd3_core::error::AUTDInternalError> {
                Ok((Self::H::default(), Self::B::new(self.calc(geometry)?)))
            }
        }

        #[cfg(not(feature = "dynamic"))]
        impl #impl_generics autd3_core::sendable::Sendable<autd3_core::geometry::AdvancedTransducer> for #name #ty_generics #where_clause {
            type H = autd3_core::NullHeader;
            type B = autd3_core::GainAdvanced;

            fn operation(
                mut self,
                geometry: &Geometry<autd3_core::geometry::AdvancedTransducer>,
            ) -> Result<(Self::H, Self::B), autd3_core::error::AUTDInternalError> {
                Ok((Self::H::default(), Self::B::new(self.calc(geometry)?, geometry.transducers().map(|tr| tr.cycle()).collect())))
            }
        }

        #[cfg(not(feature = "dynamic"))]
        impl #impl_generics autd3_core::sendable::Sendable<autd3_core::geometry::AdvancedPhaseTransducer> for #name #ty_generics #where_clause {
            type H = autd3_core::NullHeader;
            type B = autd3_core::GainAdvancedPhase;

            fn operation(
                mut self,
                geometry: &Geometry<autd3_core::geometry::AdvancedPhaseTransducer>,
            ) -> Result<(Self::H, Self::B), autd3_core::error::AUTDInternalError> {
                Ok((Self::H::default(), Self::B::new(self.calc(geometry)?, geometry.transducers().map(|tr| tr.cycle()).collect())))
            }
        }


        #[cfg(feature = "dynamic")]
        impl #impl_generics autd3_core::sendable::Sendable for #name #ty_generics #where_clause  {
            fn operation(
                &mut self,
                geometry: &autd3_core::geometry::Geometry<autd3_core::geometry::DynamicTransducer>,
            ) -> Result<
                (
                    Box<dyn autd3_core::Operation>,
                    Box<dyn autd3_core::Operation>,
                ),
                autd3_core::error::AUTDInternalError,
            > {
                match geometry.mode() {
                    autd3_core::geometry::TransMode::Legacy => Ok((
                        Box::new(autd3_core::NullHeader::default()),
                        Box::new(autd3_core::GainLegacy::new(self.calc(geometry)?)),
                    )),
                    autd3_core::geometry::TransMode::Advanced => Ok((
                        Box::new(autd3_core::NullHeader::default()),
                        Box::new(autd3_core::GainAdvanced::new(self.calc(geometry)?, geometry.transducers().map(|tr| tr.cycle().unwrap()).collect())),
                    )),
                    autd3_core::geometry::TransMode::AdvancedPhase => Ok((
                        Box::new(autd3_core::NullHeader::default()),
                        Box::new(autd3_core::GainAdvancedPhase::new(self.calc(geometry)?, geometry.transducers().map(|tr| tr.cycle().unwrap()).collect())),
                    )),
                }
            }
        }

    };
    gen.into()
}
