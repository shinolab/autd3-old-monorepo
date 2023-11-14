/*
 * File: lib.rs
 * Project: src
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 14/11/2023
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
                /// Set sampling configuration
                ///
                /// # Arguments
                ///
                /// * `config` - Sampling configuration
                ///
                #[allow(clippy::needless_update)]
                pub fn with_sampling_config(self, config: autd3_driver::common::SamplingConfiguration) -> Self {
                    Self {config, ..self}
                }
            }
        }
    };

    let gen = quote! {
        impl <#(#linetimes_prop,)* #(#type_params_prop,)*> ModulationProperty for #name #ty_generics #where_clause {
            fn sampling_config(&self) -> autd3_driver::common::SamplingConfiguration {
                self.config
            }
        }

        #freq_config

        impl <#(#linetimes_datagram,)* #(#type_params_datagram,)* > Datagram for #name #ty_generics #where_clause {
            type O1 = ModulationOp;
            type O2 = NullOp;

            fn operation(self) -> Result<(Self::O1, Self::O2), autd3_driver::error::AUTDInternalError> {
                let freq_div = self.config.frequency_division();
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
    let type_params = generics.type_params();
    let gen = quote! {
        impl <#(#linetimes_for_any,)* #(#type_params_for_any,)*> GainAsAny for #name #ty_generics #where_clause {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }

        impl <#(#linetimes,)* #(#type_params,)*> Datagram for #name #ty_generics #where_clause
            where GainOp<Self>: Operation
        {
            type O1 = GainOp<Self>;
            type O2 = NullOp;

            fn operation(self) -> Result<(Self::O1, Self::O2), autd3_driver::error::AUTDInternalError> {
                Ok((Self::O1::new(self), Self::O2::default()))
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Link)]
pub fn link_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;
    let name_builder = quote::format_ident!("{}Builder", name);

    let generics = &ast.generics;
    let (_, ty_generics_open, where_clause_open) = generics.split_for_impl();
    let type_params_open = generics.type_params();
    let (_, ty_generics_impl, where_clause_impl) = generics.split_for_impl();
    let type_params_impl = generics.type_params();

    let gen = quote! {
        #[async_trait::async_trait]
        impl <#(#type_params_impl,)*>  autd3_driver::link::Link for #name #ty_generics_impl #where_clause_impl{
            async fn close(&mut self) -> Result<(), autd3_driver::error::AUTDInternalError> {
                <Self as autd3_driver::link::LinkSync>::close(self)
            }
            async fn send(&mut self, tx: &autd3_driver::cpu::TxDatagram) -> Result<bool, autd3_driver::error::AUTDInternalError> {
                <Self as autd3_driver::link::LinkSync>::send(self, tx)
            }
            async fn receive(&mut self, rx: &mut [autd3_driver::cpu::RxMessage]) -> Result<bool, autd3_driver::error::AUTDInternalError> {
                <Self as autd3_driver::link::LinkSync>::receive(self, rx)
            }
            fn is_open(&self) -> bool {
                <Self as autd3_driver::link::LinkSync>::is_open(self)
            }
            fn timeout(&self) -> std::time::Duration {
                <Self as autd3_driver::link::LinkSync>::timeout(self)
            }
            async fn send_receive(
                &mut self,
                tx: &autd3_driver::cpu::TxDatagram,
                rx: &mut [autd3_driver::cpu::RxMessage],
                timeout: Option<std::time::Duration>,
            ) -> Result<bool, autd3_driver::error::AUTDInternalError> {
                <Self as autd3_driver::link::LinkSync>::send_receive(self, tx, rx, timeout)
            }
            async fn wait_msg_processed(
                &mut self,
                tx: &autd3_driver::cpu::TxDatagram,
                rx: &mut [autd3_driver::cpu::RxMessage],
                timeout: std::time::Duration,
            ) -> Result<bool, autd3_driver::error::AUTDInternalError> {
                <Self as autd3_driver::link::LinkSync>::wait_msg_processed(self, tx, rx, timeout)
            }
        }

        #[async_trait::async_trait]
        impl<#(#type_params_open,)*> autd3_driver::link::LinkBuilder for #name_builder #ty_generics_open #where_clause_open {
            type L = #name #ty_generics_open;

            async fn open(self, geometry: &autd3_driver::geometry::Geometry) -> Result<Self::L, autd3_driver::error::AUTDInternalError> {
                <Self as autd3_driver::link::LinkSyncBuilder>::open(self, geometry)
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
        impl<#(#type_params_open,)*> autd3_driver::link::LinkSyncBuilder for #name_sync_builder #ty_generics_open #where_clause_open {
            type L = #name_sync #ty_generics_open;

            fn open(self, geometry: &autd3_driver::geometry::Geometry) -> Result<Self::L, autd3_driver::error::AUTDInternalError> {
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
