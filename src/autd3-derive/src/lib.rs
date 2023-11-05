/*
 * File: lib.rs
 * Project: src
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Meta};

#[proc_macro_derive(Modulation, attributes(no_change))]
pub fn modulation_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let freq_div_no_change = if let syn::Data::Struct(syn::DataStruct { fields, .. }) = input.data {
        fields.iter().any(|field| {
            let is_freq_div = field
                .ident
                .as_ref()
                .map(|ident| ident == "freq_div")
                .unwrap_or(false);
            let no_change = field
                .attrs
                .iter()
                .any(|attr| matches!(&attr.meta, Meta::Path(path) if path.is_ident("no_change")));
            is_freq_div && no_change
        })
    } else {
        false
    };

    let name = &input.ident;
    let generics = &input.generics;
    let linetimes_prop = generics.lifetimes();
    let linetimes_impl = generics.lifetimes();
    let linetimes_datagram = generics.lifetimes();
    let type_params_prop = generics.type_params();
    let type_params_impl = generics.type_params();
    let type_params_datagram = generics.type_params();
    let (_, ty_generics, where_clause) = generics.split_for_impl();

    let freq_config = if freq_div_no_change {
        quote! {}
    } else {
        quote! {
            impl <#(#linetimes_impl,)* #(#type_params_impl,)*> #name #ty_generics #where_clause {
                /// Set sampling frequency division
                ///
                /// # Arguments
                ///
                /// * `freq_div` - Sampling frequency division. The sampling frequency will be [FPGA_CLK_FREQ] / `freq_div`. The value must be at least [SAMPLING_FREQ_DIV_MIN]
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
                pub fn with_sampling_frequency(self, freq: float) -> Self {
                    let freq_div = FPGA_CLK_FREQ as float / freq;
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
                    let freq_div = FPGA_CLK_FREQ as float / 1000000000. * period.as_nanos() as float;
                    self.with_sampling_frequency_division(freq_div as _)
                }
            }
        }
    };

    let gen = quote! {
        impl <#(#linetimes_prop,)* #(#type_params_prop,)*> ModulationProperty for #name #ty_generics #where_clause {
            fn sampling_frequency_division(&self) -> u32 {
                self.freq_div
            }
        }

        #freq_config

        impl <#(#linetimes_datagram,)* #(#type_params_datagram,)* T: Transducer> Datagram<T> for #name #ty_generics #where_clause {
            type O1 = ModulationOp;
            type O2 = NullOp;

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
        impl <#(#linetimes_for_any,)* #(#type_params_for_any,)*> GainAsAny for #name #ty_generics #where_clause {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }

        impl <#(#linetimes,)* #(#type_params,)* T: Transducer> Datagram<T> for #name #ty_generics #where_clause
            where GainOp<T,Self>: Operation<T>
        {
            type O1 = GainOp<T,Self>;
            type O2 = NullOp;

            fn operation(self) -> Result<(Self::O1, Self::O2), autd3_driver::error::AUTDInternalError> {
                Ok((Self::O1::new(self), Self::O2::default()))
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(LinkSync)]
pub fn link_sync_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;
    let name_builder = quote::format_ident!("{}Builder", name);
    let name_sync = quote::format_ident!("{}Sync", name);
    let name_sync_builder = quote::format_ident!("{}SyncBuilder", name);

    let generics = &ast.generics;
    let (_, ty_generics_def, where_clause_def) = generics.split_for_impl();
    let (_, ty_generics_builder_def, where_clause_builder_def) = generics.split_for_impl();
    let (_, ty_generics_open, where_clause_open) = generics.split_for_impl();
    let type_params_open = generics.type_params();
    let (_, ty_generics_impl, where_clause_impl) = generics.split_for_impl();
    let type_params_impl = generics.type_params();
    let (_, ty_generics_blocking, where_clause_blocking) = generics.split_for_impl();
    let type_params_blocking = generics.type_params();

    let gen = quote! {
        #[cfg(feature = "sync")]
        pub struct #name_sync #ty_generics_def #where_clause_def {
            pub inner: #name #ty_generics_def,
            pub runtime: tokio::runtime::Runtime,
        }

        #[cfg(feature = "sync")]
        pub struct #name_sync_builder #ty_generics_builder_def #where_clause_builder_def {
            inner: #name_builder #ty_generics_builder_def,
            runtime: tokio::runtime::Runtime,
        }

        #[cfg(feature = "sync")]
        impl <#(#type_params_impl,)*>  autd3_driver::link::LinkSync for #name_sync #ty_generics_impl #where_clause_impl{
            fn close(&mut self) -> Result<(), autd3_driver::error::AUTDInternalError> {
                self.runtime.block_on(self.inner.close())
            }
            fn send(&mut self, tx: &autd3_driver::cpu::TxDatagram) -> Result<bool, autd3_driver::error::AUTDInternalError> {
                self.runtime.block_on(self.inner.send(tx))
            }
            fn receive(&mut self, rx: &mut [autd3_driver::cpu::RxMessage]) -> Result<bool, autd3_driver::error::AUTDInternalError> {
                self.runtime.block_on(self.inner.receive(rx))
            }
            fn is_open(&self) -> bool {
                self.inner.is_open()
            }
            fn timeout(&self) -> std::time::Duration {
                self.inner.timeout()
            }
            fn send_receive(
                &mut self,
                tx: &autd3_driver::cpu::TxDatagram,
                rx: &mut [autd3_driver::cpu::RxMessage],
                timeout: Option<std::time::Duration>,
            ) -> Result<bool, autd3_driver::error::AUTDInternalError> {
                self.runtime
                    .block_on(self.inner.send_receive(tx, rx, timeout))
            }
            fn wait_msg_processed(
                &mut self,
                tx: &autd3_driver::cpu::TxDatagram,
                rx: &mut [autd3_driver::cpu::RxMessage],
                timeout: std::time::Duration,
            ) -> Result<bool, autd3_driver::error::AUTDInternalError> {
                self.runtime
                    .block_on(self.inner.wait_msg_processed(tx, rx, timeout))
            }
        }

        #[cfg(feature = "sync")]
        impl<#(#type_params_open,)* T: autd3_driver::geometry::Transducer> autd3_driver::link::LinkSyncBuilder<T> for #name_sync_builder #ty_generics_open #where_clause_open {
            type L = #name_sync #ty_generics_open;

            fn open(self, geometry: &autd3_driver::geometry::Geometry<T>) -> Result<Self::L, autd3_driver::error::AUTDInternalError> {
                let Self { inner, runtime } = self;
                let inner = runtime.block_on(inner.open(geometry))?;
                Ok(Self::L { inner, runtime })
            }
        }

        #[cfg(feature = "sync")]
        impl <#(#type_params_blocking,)*> #name_builder #ty_generics_blocking #where_clause_blocking{
            pub fn blocking(self) -> #name_sync_builder #ty_generics_blocking {
                #name_sync_builder {
                    inner: self,
                    runtime: tokio::runtime::Builder::new_multi_thread()
                        .enable_all()
                        .build()
                        .unwrap(),
                }
            }
        }
    };
    gen.into()
}
