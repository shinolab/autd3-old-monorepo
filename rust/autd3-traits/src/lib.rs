/*
 * File: lib.rs
 * Project: src
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/07/2022
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
                if self.buffer().len() > autd3_core::MOD_BUF_SIZE_MAX {
                    return Err(autd3_core::FPGAError::ModulationOutOfBuffer(self.buffer().len()).into());
                }

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
                autd3_core::FPGA_CLK_FREQ as f64 / self.props.freq_div as f64
            }
        }

        impl #impl_generics autd3_core::interface::DatagramHeader for #name #ty_generics #where_clause {
            fn init(&mut self) -> anyhow::Result<()> {
                self.build()?;
                self.props.sent = 0;
                Ok(())
            }

            fn pack(
                &mut self,
                msg_id: u8,
                tx: &mut autd3_core::TxDatagram,
            ) -> anyhow::Result<()> {
                let is_first_frame = self.props.sent == 0;
                let max_size = if is_first_frame {autd3_core::MOD_HEAD_DATA_SIZE} else {autd3_core::MOD_BODY_DATA_SIZE};
                let mod_size = (self.buffer().len() - self.props.sent).min(max_size);
                let is_last_frame = self.props.sent + mod_size == self.buffer().len();
                autd3_core::modulation(msg_id, &self.buffer()[self.props.sent..(self.props.sent + mod_size)], is_first_frame, self.props.freq_div, is_last_frame, tx)?;

                self.props.sent += mod_size;

                Ok(())
            }

            fn is_finished(&self) -> bool {
                self.props.sent == self.buffer().len()
            }
        }

        impl <T: autd3_core::geometry::Transducer> autd3_core::interface::Sendable<T> for #name #ty_generics #where_clause {
            type H = autd3_core::interface::Filled;
            type B = autd3_core::interface::Empty;

            fn init(&mut self) -> anyhow::Result<()> {
                autd3_core::interface::DatagramHeader::init(self)
            }

            fn pack(
                &mut self,
                msg_id: u8,
                _geometry: &autd3_core::geometry::Geometry<T>,
                tx: &mut autd3_core::TxDatagram,
            ) -> anyhow::Result<()> {
                autd3_core::interface::DatagramHeader::pack(self, msg_id, tx)
            }

            fn is_finished(&self) -> bool {
                autd3_core::interface::DatagramHeader::is_finished(self)
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
        impl #impl_generics Gain<T> for #name #ty_generics #where_clause {
            fn build(&mut self, geometry: &Geometry<T>) -> anyhow::Result<()> {
                if self.props.built {
                    return Ok(());
                }

                self.props.init(geometry);

                autd3_core::gain::IGain::calc(self, geometry)?;

                self.props.built = true;

                Ok(())
            }

            fn rebuild(&mut self, geometry: &Geometry<T>) -> anyhow::Result<()> {
                self.props.built = false;
                self.build(geometry)
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

        impl #impl_generics autd3_core::interface::DatagramBody<T> for #name #ty_generics #where_clause {
            fn init(&mut self) -> anyhow::Result<()> {
                self.props.phase_sent = false;
                self.props.duty_sent = false;
                Ok(())
            }

            fn pack(
                &mut self,
                geometry: &autd3_core::geometry::Geometry<T>,
                tx: &mut autd3_core::TxDatagram,
            ) -> anyhow::Result<()> {
                self.props.pack_head(tx);
                if self.is_finished() {
                    return Ok(());
                }
                self.build(geometry)?;
                self.props.pack_body(tx)?;
                Ok(())
            }

            fn is_finished(&self) -> bool {
                self.props.phase_sent && self.props.duty_sent
            }
        }


        impl #impl_generics autd3_core::interface::Sendable<T> for #name #ty_generics #where_clause {
            type H = autd3_core::interface::Empty;
            type B = autd3_core::interface::Filled;

            fn init(&mut self) -> anyhow::Result<()> {
                autd3_core::interface::DatagramBody::<T>::init(self)
            }

            fn pack(
                &mut self,
                msg_id: u8,
                geometry: &autd3_core::geometry::Geometry<T>,
                tx: &mut autd3_core::TxDatagram,
            ) -> anyhow::Result<()> {
                autd3_core::interface::DatagramBody::<T>::pack(self, geometry, tx)
            }

            fn is_finished(&self) -> bool {
                autd3_core::interface::DatagramBody::<T>::is_finished(self)
            }
        }
    };
    gen.into()
}
