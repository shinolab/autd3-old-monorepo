/*
 * File: lib.rs
 * Project: src
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/01/2023
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
        use autd3_core::Operation;

        impl #impl_generics autd3_core::modulation::Modulation for #name #ty_generics #where_clause {
            fn buffer(&self) -> &[u8] {
                &self.op.mod_data
            }

            fn sampling_frequency_division(&mut self) -> &mut u32 {
                &mut self.op.freq_div
            }

            fn sampling_freq(&self) -> f64 {
                autd3_driver::FPGA_CLK_FREQ as f64 / self.op.freq_div as f64
            }
        }

        impl #impl_generics autd3_core::datagram::DatagramHeader for #name #ty_generics #where_clause {
            fn init(&mut self) -> anyhow::Result<()> {
                self.op.init();
                self.calc()
            }

            fn pack(&mut self, tx: &mut autd3_core::TxDatagram) -> anyhow::Result<()> {
                self.op.pack(tx)
            }

            fn is_finished(&self) -> bool {
                self.op.is_finished()
            }
        }

        impl<T: autd3_core::geometry::Transducer> autd3_core::datagram::Sendable<T> for #name #ty_generics #where_clause {
            type H = autd3_core::datagram::Filled;
            type B = autd3_core::datagram::Empty;

            fn init(&mut self, _geometry: &autd3_core::geometry::Geometry<T>) -> anyhow::Result<()> {
                autd3_core::datagram::DatagramHeader::init(self)
            }

            fn pack(&mut self, tx: &mut autd3_core::TxDatagram) -> anyhow::Result<()> {
                autd3_core::datagram::DatagramHeader::pack(self, tx)
            }

            fn is_finished(&self) -> bool {
                autd3_core::datagram::DatagramHeader::is_finished(self)
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
    let gen = quote! {
        use autd3_core::Operation;
        use autd3_core::GainOp;

    impl autd3_core::gain::Gain<autd3_core::geometry::LegacyTransducer> for #name <autd3_core::geometry::LegacyTransducer>
    {
        fn drives(&self) -> &[autd3_core::Drive] {
            self.op.drives()
        }
    }

    impl autd3_core::gain::Gain<autd3_core::geometry::NormalTransducer> for #name <autd3_core::geometry::NormalTransducer>
    {
        fn drives(&self) -> &[autd3_core::Drive] {
            self.op.drives()
        }
    }

    impl autd3_core::gain::Gain<autd3_core::geometry::NormalPhaseTransducer> for #name <autd3_core::geometry::NormalPhaseTransducer>
    {
        fn drives(&self) -> &[autd3_core::Drive] {
            self.op.drives()
        }
    }

    impl autd3_core::datagram::DatagramBody<autd3_core::geometry::LegacyTransducer> for #name <autd3_core::geometry::LegacyTransducer>
    {
        fn init(
            &mut self,
            geometry: &autd3_core::geometry::Geometry<autd3_core::geometry::LegacyTransducer>,
        ) -> anyhow::Result<()> {
            self.op.init();
            self.op.drives.resize(geometry.num_transducers(), autd3_core::Drive{amp: 0.0, phase: 0.0});
            self.calc(geometry)
        }

        fn pack(&mut self, tx: &mut autd3_core::TxDatagram) -> anyhow::Result<()> {
            self.op.pack(tx)
        }

        fn is_finished(&self) -> bool {
            self.op.is_finished()
        }
    }

    impl autd3_core::datagram::DatagramBody<autd3_core::geometry::NormalTransducer> for #name <autd3_core::geometry::NormalTransducer>
    {
        fn init(
            &mut self,
            geometry: &autd3_core::geometry::Geometry<autd3_core::geometry::NormalTransducer>,
        ) -> anyhow::Result<()> {
            self.op.init();
            self.op.drives.resize(geometry.num_transducers(), autd3_core::Drive{amp: 0.0, phase: 0.0});
            self.op.cycles = geometry.transducers().map(|tr| tr.cycle()).collect();
            self.calc(geometry)
        }

        fn pack(&mut self, tx: &mut autd3_core::TxDatagram) -> anyhow::Result<()> {
            self.op.pack(tx)
        }

        fn is_finished(&self) -> bool {
            self.op.is_finished()
        }
    }

    impl autd3_core::datagram::DatagramBody<autd3_core::geometry::NormalPhaseTransducer> for #name <autd3_core::geometry::NormalPhaseTransducer>
    {
        fn init(
            &mut self,
            geometry: &autd3_core::geometry::Geometry<autd3_core::geometry::NormalPhaseTransducer>,
        ) -> anyhow::Result<()> {
            self.op.init();
            self.op.drives.resize(geometry.num_transducers(), autd3_core::Drive{amp: 0.0, phase: 0.0});
            self.op.cycles = geometry.transducers().map(|tr| tr.cycle()).collect();
            self.calc(geometry)
        }

        fn pack(&mut self, tx: &mut autd3_core::TxDatagram) -> anyhow::Result<()> {
            self.op.pack(tx)
        }

        fn is_finished(&self) -> bool {
            self.op.is_finished()
        }
    }

    impl autd3_core::datagram::Sendable<autd3_core::geometry::LegacyTransducer> for #name <autd3_core::geometry::LegacyTransducer>
    {
        type H = autd3_core::datagram::Empty;
        type B = autd3_core::datagram::Filled;

        fn init(
            &mut self,
            geometry: &autd3_core::geometry::Geometry<autd3_core::geometry::LegacyTransducer>,
        ) -> anyhow::Result<()> {
            autd3_core::datagram::DatagramBody::<autd3_core::geometry::LegacyTransducer>::init(
                self, geometry,
            )
        }

        fn pack(&mut self, tx: &mut autd3_core::TxDatagram) -> anyhow::Result<()> {
            autd3_core::datagram::DatagramBody::<autd3_core::geometry::LegacyTransducer>::pack(self, tx)
        }

        fn is_finished(&self) -> bool {
            autd3_core::datagram::DatagramBody::<autd3_core::geometry::LegacyTransducer>::is_finished(
                self,
            )
        }
    }

    impl autd3_core::datagram::Sendable<autd3_core::geometry::NormalTransducer> for #name <autd3_core::geometry::NormalTransducer>
    {
        type H = autd3_core::datagram::Empty;
        type B = autd3_core::datagram::Filled;

        fn init(
            &mut self,
            geometry: &autd3_core::geometry::Geometry<autd3_core::geometry::NormalTransducer>,
        ) -> anyhow::Result<()> {
            autd3_core::datagram::DatagramBody::<autd3_core::geometry::NormalTransducer>::init(
                self, geometry,
            )
        }

        fn pack(&mut self, tx: &mut autd3_core::TxDatagram) -> anyhow::Result<()> {
            autd3_core::datagram::DatagramBody::<autd3_core::geometry::NormalTransducer>::pack(self, tx)
        }

        fn is_finished(&self) -> bool {
            autd3_core::datagram::DatagramBody::<autd3_core::geometry::NormalTransducer>::is_finished(
                self,
            )
        }
    }

    impl autd3_core::datagram::Sendable<autd3_core::geometry::NormalPhaseTransducer> for #name <autd3_core::geometry::NormalPhaseTransducer>
    {
        type H = autd3_core::datagram::Empty;
        type B = autd3_core::datagram::Filled;

        fn init(
            &mut self,
            geometry: &autd3_core::geometry::Geometry<autd3_core::geometry::NormalPhaseTransducer>,
        ) -> anyhow::Result<()> {
            autd3_core::datagram::DatagramBody::<autd3_core::geometry::NormalPhaseTransducer>::init(
                self, geometry,
            )
        }

        fn pack(&mut self, tx: &mut autd3_core::TxDatagram) -> anyhow::Result<()> {
            autd3_core::datagram::DatagramBody::<autd3_core::geometry::NormalPhaseTransducer>::pack(
                self, tx,
            )
        }

        fn is_finished(&self) -> bool {
            autd3_core::datagram::DatagramBody::<autd3_core::geometry::NormalPhaseTransducer>::is_finished(
                self,
            )
        }
    }
        };
    gen.into()
}
