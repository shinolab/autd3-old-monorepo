/*
 * File: lib.rs
 * Project: src
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/01/2023
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
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            fn sampling_frequency_division(&mut self) -> &mut u32 {
                &mut self.freq_div
            }

            fn sampling_freq(&self) -> f64 {
                autd3_driver::FPGA_CLK_FREQ as f64 / self.freq_div as f64
            }
        }

        impl #impl_generics autd3_core::datagram::DatagramHeader for #name #ty_generics #where_clause {
            type O = autd3_core::Modulation;

            fn operation(&mut self) -> Result<Self::O> {
                let data = self.calc()?;
                Ok(Self::O::new(data, self.freq_div))
            }
        }

        impl #impl_generics autd3_core::datagram::Sendable<autd3_core::geometry::LegacyTransducer> for #name #ty_generics #where_clause {
            type H = autd3_core::datagram::Filled;
            type B = autd3_core::datagram::Empty;
            type O = <Self as autd3_core::datagram::DatagramHeader>::O;

            fn operation(&mut self, _: &autd3_core::geometry::Geometry<autd3_core::geometry::LegacyTransducer>) -> Result<Self::O> {
                <Self as autd3_core::datagram::DatagramHeader>::operation(self)
            }
        }

        impl #impl_generics autd3_core::datagram::Sendable<autd3_core::geometry::NormalTransducer> for #name #ty_generics #where_clause {
            type H = autd3_core::datagram::Filled;
            type B = autd3_core::datagram::Empty;
            type O = <Self as autd3_core::datagram::DatagramHeader>::O;

            fn operation(&mut self, _: &autd3_core::geometry::Geometry<autd3_core::geometry::NormalTransducer>) -> Result<Self::O> {
                <Self as autd3_core::datagram::DatagramHeader>::operation(self)
            }
        }

        impl #impl_generics autd3_core::datagram::Sendable<autd3_core::geometry::NormalPhaseTransducer> for #name #ty_generics #where_clause {
            type H = autd3_core::datagram::Filled;
            type B = autd3_core::datagram::Empty;
            type O = <Self as autd3_core::datagram::DatagramHeader>::O;

            fn operation(&mut self, _: &autd3_core::geometry::Geometry<autd3_core::geometry::NormalPhaseTransducer>) -> Result<Self::O> {
                <Self as autd3_core::datagram::DatagramHeader>::operation(self)
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
        impl #impl_generics autd3_core::datagram::DatagramBody<autd3_core::geometry::LegacyTransducer> for #name #ty_generics #where_clause {
            type O = autd3_driver::GainLegacy;

            fn operation(
                &mut self,
                geometry: &autd3_core::geometry::Geometry<autd3_core::geometry::LegacyTransducer>,
            ) -> anyhow::Result<Self::O> {
                let drives = self.calc(geometry)?;
                Ok(Self::O::new(drives))
            }
        }

        impl #impl_generics autd3_core::datagram::Sendable<autd3_core::geometry::LegacyTransducer> for #name #ty_generics #where_clause {
            type H = autd3_core::datagram::Empty;
            type B = autd3_core::datagram::Filled;
            type O =
                <Self as autd3_core::datagram::DatagramBody<autd3_core::geometry::LegacyTransducer>>::O;

            fn operation(
                &mut self,
                geometry: &Geometry<autd3_core::geometry::LegacyTransducer>,
            ) -> anyhow::Result<Self::O> {
                <Self as autd3_core::datagram::DatagramBody<autd3_core::geometry::LegacyTransducer>>::operation(self, geometry)
            }
        }

        impl #impl_generics autd3_core::datagram::DatagramBody<autd3_core::geometry::NormalTransducer> for #name #ty_generics #where_clause {
            type O = autd3_driver::GainNormal;

            fn operation(
                &mut self,
                geometry: &autd3_core::geometry::Geometry<autd3_core::geometry::NormalTransducer>,
            ) -> anyhow::Result<Self::O> {
                let drives = self.calc(geometry)?;
                let cycles = geometry.transducers().map(|tr| tr.cycle()).collect();
                Ok(Self::O::new(drives, cycles))
            }
        }

        impl #impl_generics autd3_core::datagram::Sendable<autd3_core::geometry::NormalTransducer> for #name #ty_generics #where_clause {
            type H = autd3_core::datagram::Empty;
            type B = autd3_core::datagram::Filled;
            type O =
                <Self as autd3_core::datagram::DatagramBody<autd3_core::geometry::NormalTransducer>>::O;

            fn operation(
                &mut self,
                geometry: &Geometry<autd3_core::geometry::NormalTransducer>,
            ) -> anyhow::Result<Self::O> {
                <Self as autd3_core::datagram::DatagramBody<autd3_core::geometry::NormalTransducer>>::operation(self, geometry)
            }
        }

        impl #impl_generics autd3_core::datagram::DatagramBody<autd3_core::geometry::NormalPhaseTransducer> for #name #ty_generics #where_clause {
            type O = autd3_driver::GainNormalPhase;

            fn operation(
                &mut self,
                geometry: &autd3_core::geometry::Geometry<autd3_core::geometry::NormalPhaseTransducer>,
            ) -> anyhow::Result<Self::O> {
                let drives = self.calc(geometry)?;
                let cycles = geometry.transducers().map(|tr| tr.cycle()).collect();
                Ok(
                    Self::O::new(
                        drives,
                        cycles,
                    ),
                )
            }
        }

        impl #impl_generics autd3_core::datagram::Sendable<autd3_core::geometry::NormalPhaseTransducer> for #name #ty_generics #where_clause {
            type H = autd3_core::datagram::Empty;
            type B = autd3_core::datagram::Filled;
            type O = <Self as autd3_core::datagram::DatagramBody<
                autd3_core::geometry::NormalPhaseTransducer,
            >>::O;

            fn operation(
                &mut self,
                geometry: &Geometry<autd3_core::geometry::NormalPhaseTransducer>,
            ) -> anyhow::Result<Self::O> {
                <Self as autd3_core::datagram::DatagramBody<autd3_core::geometry::NormalPhaseTransducer>>::operation(self, geometry)
            }
        }
    };
    gen.into()
}
