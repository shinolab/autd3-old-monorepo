/*
 * File: dynamic_backend.rs
 * Project: src
 * Created Date: 08/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */


 use std::rc::Rc;

 use autd3_gain_holo::Backend;
 use autd3_core::float;

 pub struct DynamicBackend {
     backend_ptr: Rc<dyn Backend>,
 }
 
 impl DynamicBackend {
     pub fn new(backend_ptr: Rc<dyn Backend>) -> Rc<Self> {
         Rc::new(Self { backend_ptr })
     }
 }
 
 impl Backend for DynamicBackend {
     fn gs(
         &self,
         repeat: usize,
         amps: &[float],
         g: autd3_gain_holo::MatrixXc,
     ) -> Result<autd3_gain_holo::VectorXc, autd3_gain_holo::HoloError> {
         self.backend_ptr.gs(repeat, amps, g)
     }
 
     fn gspat(
         &self,
         repeat: usize,
         amps: &[float],
         g: autd3_gain_holo::MatrixXc,
     ) -> Result<autd3_gain_holo::VectorXc, autd3_gain_holo::HoloError> {
         self.backend_ptr.gspat(repeat, amps, g)
     }
 
     fn naive(
         &self,
         amps: &[float],
         g: autd3_gain_holo::MatrixXc,
     ) -> Result<autd3_gain_holo::VectorXc, autd3_gain_holo::HoloError> {
         self.backend_ptr.naive(amps, g)
     }
 
     fn evp(
         &self,
         gamma: float,
         amps: &[float],
         g: autd3_gain_holo::MatrixXc,
     ) -> Result<autd3_gain_holo::VectorXc, autd3_gain_holo::HoloError> {
         self.backend_ptr.evp(gamma, amps, g)
     }
 
     fn sdp(
         &self,
         alpha: float,
         repeat: usize,
         lambda: float,
         amps: &[float],
         g: autd3_gain_holo::MatrixXc,
     ) -> Result<autd3_gain_holo::VectorXc, autd3_gain_holo::HoloError> {
         self.backend_ptr.sdp(alpha, repeat, lambda, amps, g)
     }
 
     fn lm(
         &self,
         eps1: float,
         eps2: float,
         tau: float,
         kmax: usize,
         initial: &[float],
         amps: &[float],
         g: autd3_gain_holo::MatrixXc,
     ) -> Result<autd3_gain_holo::VectorX, autd3_gain_holo::HoloError> {
         self.backend_ptr.lm(eps1, eps2, tau, kmax, initial, amps, g)
     }
 }
 