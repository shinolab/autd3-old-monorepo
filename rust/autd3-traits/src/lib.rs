/*
 * File: lib.rs
 * Project: src
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/12/2022
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
        impl #impl_generics Modulation for #name #ty_generics #where_clause {
            fn build(&mut self) -> anyhow::Result<()> {
                if self.props.built {
                    return Ok(());
                }

                self.calc()?;

                self.props.built = true;

                Ok(())
            }

            fn rebuild(&mut self) -> anyhow::Result<()>{
                self.props.built = false;
                self.build()
            }

            fn buffer(&self) -> &[u8] {
                &self.props.buffer
            }

            fn sampling_frequency_division(&mut self) -> &mut u32 {
                &mut self.props.freq_div
            }

            fn sampling_freq(&self) -> f64 {
                autd3_driver::FPGA_CLK_FREQ as f64 / self.props.freq_div as f64
            }
        }

        impl #impl_generics autd3_core::datagram::DatagramHeader for #name #ty_generics #where_clause {
            fn init(&mut self) -> anyhow::Result<()> {
                self.props.sent = 0;
                self.build()
            }

            fn pack(
                &mut self,
                msg_id: u8,
                tx: &mut autd3_core::TxDatagram,
            ) -> anyhow::Result<()> {
                autd3_driver::modulation(msg_id, &self.props.buffer, &mut self.props.sent, self.props.freq_div, tx)
            }

            fn is_finished(&self) -> bool {
                self.props.sent == self.buffer().len()
            }
        }

        impl <T: autd3_core::geometry::Transducer> autd3_core::datagram::Sendable<T> for #name #ty_generics #where_clause {
            type H = autd3_core::datagram::Filled;
            type B = autd3_core::datagram::Empty;

            fn init(&mut self) -> anyhow::Result<()> {
                autd3_core::datagram::DatagramHeader::init(self)
            }

            fn pack(
                &mut self,
                msg_id: u8,
                _geometry: &autd3_core::geometry::Geometry<T>,
                tx: &mut autd3_core::TxDatagram,
            ) -> anyhow::Result<()> {
                autd3_core::datagram::DatagramHeader::pack(self, msg_id, tx)
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
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics autd3_core::gain::Gain<autd3_core::geometry::LegacyTransducer> for #name #ty_generics #where_clause {
            fn build(&mut self, geometry: &Geometry<autd3_core::geometry::LegacyTransducer>) -> anyhow::Result<()> {
                if self.props.built {
                    return Ok(());
                }

                self.props.init(geometry);

                self.calc::<autd3_core::geometry::LegacyTransducer>(geometry)?;

                self.props.built = true;

                Ok(())
            }

            fn rebuild(&mut self, geometry: &Geometry<autd3_core::geometry::LegacyTransducer>) -> anyhow::Result<()> {
                self.props.built = false;
                autd3_core::gain::Gain::<autd3_core::geometry::LegacyTransducer>::build(self, geometry)
            }

            fn drives(&self) -> &[autd3_core::Drive] {
                &self.props.drives
            }

            fn take_drives(self) -> Vec<autd3_core::Drive> {
                self.props.drives
            }

            fn built(&self) -> bool {
                self.props.built
            }
        }

        impl #impl_generics autd3_core::gain::Gain<autd3_core::geometry::NormalTransducer> for #name #ty_generics #where_clause {
            fn build(&mut self, geometry: &Geometry<autd3_core::geometry::NormalTransducer>) -> anyhow::Result<()> {
                if self.props.built {
                    return Ok(());
                }

                self.props.init(geometry);

                self.calc::<autd3_core::geometry::NormalTransducer>(geometry)?;

                self.props.built = true;

                Ok(())
            }

            fn rebuild(&mut self, geometry: &Geometry<autd3_core::geometry::NormalTransducer>) -> anyhow::Result<()> {
                self.props.built = false;
                autd3_core::gain::Gain::<autd3_core::geometry::NormalTransducer>::build(self, geometry)
            }

            fn drives(&self) -> &[autd3_core::Drive] {
                &self.props.drives
            }

            fn take_drives(self) -> Vec<autd3_core::Drive> {
                self.props.drives
            }

            fn built(&self) -> bool {
                self.props.built
            }
        }

        impl #impl_generics autd3_core::gain::Gain<autd3_core::geometry::NormalPhaseTransducer> for #name #ty_generics #where_clause {
            fn build(&mut self, geometry: &Geometry<autd3_core::geometry::NormalPhaseTransducer>) -> anyhow::Result<()> {
                if self.props.built {
                    return Ok(());
                }

                self.props.init(geometry);

                self.calc::<autd3_core::geometry::NormalPhaseTransducer>(geometry)?;

                self.props.built = true;

                Ok(())
            }

            fn rebuild(&mut self, geometry: &Geometry<autd3_core::geometry::NormalPhaseTransducer>) -> anyhow::Result<()> {
                self.props.built = false;
                autd3_core::gain::Gain::<autd3_core::geometry::NormalPhaseTransducer>::build(self, geometry)
            }

            fn drives(&self) -> &[autd3_core::Drive] {
                &self.props.drives
            }

            fn take_drives(self) -> Vec<autd3_core::Drive> {
                self.props.drives
            }

            fn built(&self) -> bool {
                self.props.built
            }
        }

        impl #impl_generics autd3_core::datagram::DatagramBody<autd3_core::geometry::LegacyTransducer> for #name #ty_generics #where_clause {
            fn init(&mut self) -> anyhow::Result<()> {
                self.props.phase_sent = false;
                self.props.duty_sent = false;
                Ok(())
            }

            fn pack(
                &mut self,
                geometry: &autd3_core::geometry::Geometry<autd3_core::geometry::LegacyTransducer>,
                tx: &mut autd3_core::TxDatagram,
            ) -> anyhow::Result<()> {
                autd3_driver::normal_legacy_header(tx);
                if autd3_core::datagram::DatagramBody::<autd3_core::geometry::LegacyTransducer>::is_finished(self) {
                    return Ok(());
                }
                autd3_core::gain::Gain::<autd3_core::geometry::LegacyTransducer>::build(self, geometry)?;
                autd3_driver::normal_legacy_body(&self.props.drives, tx)?;
                self.props.phase_sent = true;
                self.props.duty_sent = true;
                Ok(())
            }

            fn is_finished(&self) -> bool {
                self.props.phase_sent && self.props.duty_sent
            }
        }

        impl #impl_generics autd3_core::datagram::DatagramBody<autd3_core::geometry::NormalTransducer> for #name #ty_generics #where_clause {
            fn init(&mut self) -> anyhow::Result<()> {
                self.props.phase_sent = false;
                self.props.duty_sent = false;
                Ok(())
            }

            fn pack(
                &mut self,
                geometry: &autd3_core::geometry::Geometry<autd3_core::geometry::NormalTransducer>,
                tx: &mut autd3_core::TxDatagram,
            ) -> anyhow::Result<()> {
                autd3_driver::normal_header(tx);
                if autd3_core::datagram::DatagramBody::<autd3_core::geometry::NormalTransducer>::is_finished(self) {
                    return Ok(());
                }
                autd3_core::gain::Gain::<autd3_core::geometry::NormalTransducer>::build(self, geometry)?;
                if !self.props.phase_sent {
                    autd3_driver::normal_phase_body(&self.props.drives, tx)?;
                    self.props.phase_sent = true;
                } else {
                    autd3_driver::normal_duty_body(&self.props.drives, tx)?;
                    self.props.duty_sent = true;
                }
                Ok(())
            }

            fn is_finished(&self) -> bool {
                self.props.phase_sent && self.props.duty_sent
            }
        }

        impl #impl_generics autd3_core::datagram::DatagramBody<autd3_core::geometry::NormalPhaseTransducer> for #name #ty_generics #where_clause {
            fn init(&mut self) -> anyhow::Result<()> {
                self.props.phase_sent = false;
                self.props.duty_sent = false;
                Ok(())
            }

            fn pack(
                &mut self,
                geometry: &autd3_core::geometry::Geometry<autd3_core::geometry::NormalPhaseTransducer>,
                tx: &mut autd3_core::TxDatagram,
            ) -> anyhow::Result<()> {
                autd3_driver::normal_header(tx);
                if autd3_core::datagram::DatagramBody::<autd3_core::geometry::NormalPhaseTransducer>::is_finished(self) {
                    return Ok(());
                }
                autd3_core::gain::Gain::<autd3_core::geometry::NormalPhaseTransducer>::build(self, geometry)?;
                autd3_driver::normal_phase_body(&self.props.drives, tx)?;
                self.props.phase_sent = true;
                self.props.duty_sent = true;
                Ok(())
            }

            fn is_finished(&self) -> bool {
                self.props.phase_sent && self.props.duty_sent
            }
        }

        impl #impl_generics autd3_core::datagram::Sendable<autd3_core::geometry::LegacyTransducer> for #name #ty_generics #where_clause {
            type H = autd3_core::datagram::Empty;
            type B = autd3_core::datagram::Filled;

            fn init(&mut self) -> anyhow::Result<()> {
                autd3_core::datagram::DatagramBody::<autd3_core::geometry::LegacyTransducer>::init(self)
            }

            fn pack(
                &mut self,
                msg_id: u8,
                geometry: &autd3_core::geometry::Geometry<autd3_core::geometry::LegacyTransducer>,
                tx: &mut autd3_core::TxDatagram,
            ) -> anyhow::Result<()> {
                autd3_core::datagram::DatagramBody::<autd3_core::geometry::LegacyTransducer>::pack(self, geometry, tx)
            }

            fn is_finished(&self) -> bool {
                autd3_core::datagram::DatagramBody::<autd3_core::geometry::LegacyTransducer>::is_finished(self)
            }
        }


        impl #impl_generics autd3_core::datagram::Sendable<autd3_core::geometry::NormalTransducer> for #name #ty_generics #where_clause {
            type H = autd3_core::datagram::Empty;
            type B = autd3_core::datagram::Filled;

            fn init(&mut self) -> anyhow::Result<()> {
                autd3_core::datagram::DatagramBody::<autd3_core::geometry::NormalTransducer>::init(self)
            }

            fn pack(
                &mut self,
                msg_id: u8,
                geometry: &autd3_core::geometry::Geometry<autd3_core::geometry::NormalTransducer>,
                tx: &mut autd3_core::TxDatagram,
            ) -> anyhow::Result<()> {
                autd3_core::datagram::DatagramBody::<autd3_core::geometry::NormalTransducer>::pack(self, geometry, tx)
            }

            fn is_finished(&self) -> bool {
                autd3_core::datagram::DatagramBody::<autd3_core::geometry::NormalTransducer>::is_finished(self)
            }
        }


        impl #impl_generics autd3_core::datagram::Sendable<autd3_core::geometry::NormalPhaseTransducer> for #name #ty_generics #where_clause {
            type H = autd3_core::datagram::Empty;
            type B = autd3_core::datagram::Filled;

            fn init(&mut self) -> anyhow::Result<()> {
                autd3_core::datagram::DatagramBody::<autd3_core::geometry::NormalPhaseTransducer>::init(self)
            }

            fn pack(
                &mut self,
                msg_id: u8,
                geometry: &autd3_core::geometry::Geometry<autd3_core::geometry::NormalPhaseTransducer>,
                tx: &mut autd3_core::TxDatagram,
            ) -> anyhow::Result<()> {
                autd3_core::datagram::DatagramBody::<autd3_core::geometry::NormalPhaseTransducer>::pack(self, geometry, tx)
            }

            fn is_finished(&self) -> bool {
                autd3_core::datagram::DatagramBody::<autd3_core::geometry::NormalPhaseTransducer>::is_finished(self)
            }
        }
    };
    gen.into()
}
