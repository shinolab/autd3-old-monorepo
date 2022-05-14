// File: holo.cpp
// Project: holo
// Created Date: 09/12/2021
// Author: Shun Suzuki
// -----
// Last Modified: 14/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2021 Hapis Lab. All rights reserved.
//

#include "autd3/gain/holo.hpp"

#include <random>

#include "autd3/core/geometry/legacy_transducer.hpp"
#include "autd3/core/geometry/normal_transducer.hpp"

namespace autd3::gain::holo {

namespace {

template <typename T>
void generate_transfer_matrix(const std::vector<core::Vector3>& foci, const core::Geometry<T>& geometry, MatrixXc& dst) {
  for (size_t i = 0; i < foci.size(); i++)
    for (const auto& dev : geometry)
      for (const auto& tr : dev)
        dst(i, tr.id()) = core::propagate(tr.position(), tr.z_direction(), geometry.attenuation, tr.wavenumber(geometry.sound_speed), foci[i]);
}

void back_prop(const BackendPtr& backend, const MatrixXc& transfer, const VectorXc& amps, MatrixXc& b) {
  const auto m = transfer.rows();

  VectorXc denominator(m);
  backend->reduce_col(transfer, denominator);
  backend->abs(denominator, denominator);
  backend->reciprocal(denominator, denominator);
  backend->hadamard_product(amps, denominator, denominator);

  MatrixXc b_tmp(m, m);
  backend->create_diagonal(denominator, b_tmp);

  backend->mul(TRANSPOSE::CONJ_TRANS, TRANSPOSE::NO_TRANS, ONE, transfer, b_tmp, ZERO, b);
}

template <typename T>
void sdp_calc_impl(const BackendPtr& backend, const std::vector<core::Vector3>& foci, std::vector<complex>& amps, const core::Geometry<T>& geometry,
                   const double alpha, const double lambda, const size_t repeat, AmplitudeConstraint constraint, typename T::D& drives) {
  backend->init();

  const auto m = foci.size();
  const auto n = geometry.num_transducers();

  const VectorXc amps_ = Eigen::Map<VectorXc, Eigen::Unaligned>(amps.data(), static_cast<Eigen::Index>(amps.size()));

  MatrixXc p(m, m);
  backend->create_diagonal(amps_, p);

  MatrixXc b(m, n);
  generate_transfer_matrix(foci, geometry, b);

  MatrixXc pseudo_inv_b = MatrixXc::Zero(n, m);
  MatrixXc u_(m, m);
  MatrixXc s(n, m);
  MatrixXc vt(n, n);
  MatrixXc buf = MatrixXc::Zero(n, m);
  MatrixXc b_tmp(m, n);
  backend->copy_to(b, b_tmp);
  backend->pseudo_inverse_svd(b_tmp, alpha, u_, s, vt, buf, pseudo_inv_b);

  MatrixXc mm(m, m);
  VectorXc one = VectorXc::Ones(static_cast<Eigen::Index>(m));
  backend->create_diagonal(one, mm);

  backend->mul(TRANSPOSE::NO_TRANS, TRANSPOSE::NO_TRANS, -ONE, b, pseudo_inv_b, ONE, mm);

  MatrixXc tmp = VectorXc::Zero(static_cast<Eigen::Index>(m), static_cast<Eigen::Index>(m));
  backend->mul(TRANSPOSE::NO_TRANS, TRANSPOSE::NO_TRANS, ONE, p, mm, ZERO, tmp);
  backend->mul(TRANSPOSE::NO_TRANS, TRANSPOSE::NO_TRANS, ONE, tmp, p, ZERO, mm);

  MatrixXc x_mat(m, m);
  backend->create_diagonal(one, x_mat);

  std::random_device rnd;
  std::mt19937 mt(rnd());
  std::uniform_real_distribution<double> range(0, 1);
  VectorXc zero = VectorXc::Zero(static_cast<Eigen::Index>(m));
  VectorXc x = VectorXc::Zero(static_cast<Eigen::Index>(m));
  VectorXc x_conj(m);
  VectorXc mmc(m);
  for (size_t i = 0; i < repeat; i++) {
    const auto ii = static_cast<size_t>(std::floor(static_cast<double>(m) * range(mt)));

    backend->get_col(mm, ii, mmc);
    backend->set(ii, ZERO, mmc);

    backend->mul(TRANSPOSE::NO_TRANS, ONE, x_mat, mmc, ZERO, x);
    if (complex gamma = backend->dot(x, mmc); gamma.real() > 0) {
      backend->scale(complex(-std::sqrt(lambda / gamma.real()), 0.0), x);
      backend->conj(x, x_conj);

      backend->set_row(x_conj, ii, 0, ii, x_mat);
      backend->set_row(x_conj, ii, ii + 1, m, x_mat);
      backend->set_col(x, ii, 0, ii, x_mat);
      backend->set_col(x, ii, ii + 1, m, x_mat);
    } else {
      backend->set_row(zero, ii, 0, ii, x_mat);
      backend->set_row(zero, ii, ii + 1, m, x_mat);
      backend->set_col(zero, ii, 0, ii, x_mat);
      backend->set_col(zero, ii, ii + 1, m, x_mat);
    }
  }

  VectorXc u(m);
  backend->max_eigen_vector(x_mat, u);

  VectorXc ut = VectorXc::Zero(static_cast<Eigen::Index>(m));
  backend->mul(TRANSPOSE::NO_TRANS, ONE, p, u, ZERO, ut);

  VectorXc q = VectorXc::Zero(n);
  backend->mul(TRANSPOSE::NO_TRANS, ONE, pseudo_inv_b, ut, ZERO, q);
  backend->to_host(q);

  const auto max_coefficient = std::abs(backend->max_abs_element(q));
  for (auto& dev : geometry)
    for (auto& tr : dev) {
      const auto phase = std::arg(q(tr.id())) / (2.0 * driver::pi) + 0.5;
      const auto raw = std::abs(q(tr.id()));
      const auto power = std::visit([&](auto& c) { return c.convert(raw, max_coefficient); }, constraint);
      drives.set_drive(tr, phase, power);
    }
}

template <typename T>
void naive_calc_impl(const BackendPtr& backend, const std::vector<core::Vector3>& foci, std::vector<complex>& amps, const core::Geometry<T>& geometry,
                     AmplitudeConstraint constraint, typename T::D& drives) {
  backend->init();

  const auto m = foci.size();
  const auto n = geometry.num_transducers();

  const VectorXc p = Eigen::Map<VectorXc, Eigen::Unaligned>(amps.data(), static_cast<Eigen::Index>(amps.size()));

  MatrixXc g(m, n);
  generate_transfer_matrix(foci, geometry, g);

  VectorXc q = VectorXc::Zero(n);
  backend->mul(TRANSPOSE::CONJ_TRANS, ONE, g, p, ZERO, q);
  backend->to_host(q);

  const auto max_coefficient = std::abs(backend->max_abs_element(q));
  for (auto& dev : geometry)
    for (auto& tr : dev) {
      const auto phase = std::arg(q(tr.id())) / (2.0 * driver::pi) + 0.5;
      const auto raw = std::abs(q(tr.id()));
      const auto power = std::visit([&](auto& c) { return c.convert(raw, max_coefficient); }, constraint);
      drives.set_drive(tr, phase, power);
    }
}

template <typename T>
void gs_calc_impl(const BackendPtr& backend, const std::vector<core::Vector3>& foci, std::vector<complex>& amps, const core::Geometry<T>& geometry,
                  const size_t repeat, AmplitudeConstraint constraint, typename T::D& drives) {
  backend->init();

  const auto m = static_cast<Eigen::Index>(foci.size());
  const auto n = geometry.num_transducers();

  const VectorXc amps_ = Eigen::Map<VectorXc, Eigen::Unaligned>(amps.data(), static_cast<Eigen::Index>(amps.size()));

  MatrixXc g(m, n);
  generate_transfer_matrix(foci, geometry, g);

  const VectorXc q0 = VectorXc::Ones(n);

  VectorXc q = q0;

  VectorXc gamma = VectorXc::Zero(m);
  VectorXc p(m);
  VectorXc xi = VectorXc::Zero(n);
  for (size_t k = 0; k < repeat; k++) {
    backend->mul(TRANSPOSE::NO_TRANS, ONE, g, q, ZERO, gamma);
    backend->arg(gamma, gamma);
    backend->hadamard_product(gamma, amps_, p);
    backend->mul(TRANSPOSE::CONJ_TRANS, ONE, g, p, ZERO, xi);
    backend->arg(xi, xi);
    backend->hadamard_product(xi, q0, q);
  }

  const auto max_coefficient = std::abs(backend->max_abs_element(q));
  backend->to_host(q);
  for (auto& dev : geometry)
    for (auto& tr : dev) {
      const auto phase = std::arg(q(tr.id())) / (2.0 * driver::pi) + 0.5;
      const auto raw = std::abs(q(tr.id()));
      const auto power = std::visit([&](auto& c) { return c.convert(raw, max_coefficient); }, constraint);
      drives.set_drive(tr, phase, power);
    }
}

template <typename T>
void gspat_calc_impl(const BackendPtr& backend, const std::vector<core::Vector3>& foci, std::vector<complex>& amps, const core::Geometry<T>& geometry,
                     const size_t repeat, AmplitudeConstraint constraint, typename T::D& drives) {
  backend->init();

  const auto m = static_cast<Eigen::Index>(foci.size());
  const auto n = geometry.num_transducers();

  const VectorXc amps_ = Eigen::Map<VectorXc, Eigen::Unaligned>(amps.data(), static_cast<Eigen::Index>(amps.size()));

  MatrixXc g(m, n);
  generate_transfer_matrix(foci, geometry, g);

  MatrixXc b = MatrixXc::Zero(n, m);
  back_prop(backend, g, amps_, b);

  MatrixXc r = MatrixXc::Zero(m, m);
  backend->mul(TRANSPOSE::NO_TRANS, TRANSPOSE::NO_TRANS, ONE, g, b, ZERO, r);

  VectorXc p = amps_;

  VectorXc gamma = VectorXc::Zero(m);
  backend->mul(TRANSPOSE::NO_TRANS, ONE, r, p, ZERO, gamma);
  for (size_t k = 0; k < repeat; k++) {
    backend->arg(gamma, gamma);
    backend->hadamard_product(gamma, amps_, p);
    backend->mul(TRANSPOSE::NO_TRANS, ONE, r, p, ZERO, gamma);
  }

  VectorXc tmp(m);
  backend->abs(gamma, tmp);
  backend->reciprocal(tmp, tmp);
  backend->hadamard_product(tmp, amps_, tmp);
  backend->hadamard_product(tmp, amps_, tmp);
  backend->arg(gamma, gamma);
  backend->hadamard_product(gamma, tmp, p);

  VectorXc q = VectorXc::Zero(n);
  backend->mul(TRANSPOSE::NO_TRANS, ONE, b, p, ZERO, q);

  const auto max_coefficient = std::abs(backend->max_abs_element(q));
  backend->to_host(q);
  for (auto& dev : geometry)
    for (auto& tr : dev) {
      const auto phase = std::arg(q(tr.id())) / (2.0 * driver::pi) + 0.5;
      const auto raw = std::abs(q(tr.id()));
      const auto power = std::visit([&](auto& c) { return c.convert(raw, max_coefficient); }, constraint);
      drives.set_drive(tr, phase, power);
    }
}

}  // namespace

void SDP<core::LegacyTransducer>::calc(const core::Geometry<core::LegacyTransducer>& geometry) {
  sdp_calc_impl(_backend, _foci, _amps, geometry, alpha, lambda, repeat, constraint, this->_props.drives);
}
void SDP<core::NormalTransducer>::calc(const core::Geometry<core::NormalTransducer>& geometry) {
  sdp_calc_impl(_backend, _foci, _amps, geometry, alpha, lambda, repeat, constraint, this->_props.drives);
}

void Naive<core::LegacyTransducer>::calc(const core::Geometry<core::LegacyTransducer>& geometry) {
  naive_calc_impl(_backend, _foci, _amps, geometry, constraint, this->_props.drives);
}
void Naive<core::NormalTransducer>::calc(const core::Geometry<core::NormalTransducer>& geometry) {
  naive_calc_impl(_backend, _foci, _amps, geometry, constraint, this->_props.drives);
}

void GS<core::LegacyTransducer>::calc(const core::Geometry<core::LegacyTransducer>& geometry) {
  gs_calc_impl(_backend, _foci, _amps, geometry, repeat, constraint, this->_props.drives);
}
void GS<core::NormalTransducer>::calc(const core::Geometry<core::NormalTransducer>& geometry) {
  gs_calc_impl(_backend, _foci, _amps, geometry, repeat, constraint, this->_props.drives);
}

void GSPAT<core::LegacyTransducer>::calc(const core::Geometry<core::LegacyTransducer>& geometry) {
  gspat_calc_impl(_backend, _foci, _amps, geometry, repeat, constraint, this->_props.drives);
}
void GSPAT<core::NormalTransducer>::calc(const core::Geometry<core::NormalTransducer>& geometry) {
  gspat_calc_impl(_backend, _foci, _amps, geometry, repeat, constraint, this->_props.drives);
}

}  // namespace autd3::gain::holo
