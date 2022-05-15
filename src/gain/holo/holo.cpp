// File: holo.cpp
// Project: holo
// Created Date: 09/12/2021
// Author: Shun Suzuki
// -----
// Last Modified: 15/05/2022
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

  MatrixXc tmp = MatrixXc::Zero(m, m);
  backend->mul(TRANSPOSE::NO_TRANS, TRANSPOSE::CONJ_TRANS, ONE, transfer, transfer, ZERO, tmp);

  VectorXc denominator(m);
  backend->get_diagonal(tmp, denominator);
  backend->reciprocal(denominator, denominator);
  backend->hadamard_product(amps, denominator, denominator);

  backend->create_diagonal(denominator, tmp);
  backend->mul(TRANSPOSE::CONJ_TRANS, TRANSPOSE::NO_TRANS, ONE, transfer, tmp, ZERO, b);
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
  const auto n = static_cast<Eigen::Index>(geometry.num_transducers());

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

template <typename T>
void make_bhb(const BackendPtr& backend, const std::vector<core::Vector3>& foci, std::vector<complex>& amps, const core::Geometry<T>& geometry,
              MatrixXc& bhb) {
  const auto m = foci.size();
  const auto n = geometry.num_transducers();
  const auto n_param = n + m;

  VectorXc amps_ = Eigen::Map<VectorXc, Eigen::Unaligned>(amps.data(), static_cast<Eigen::Index>(amps.size()));

  MatrixXc p(m, m);
  backend->scale(complex(-1.0, 0.0), amps_);
  backend->create_diagonal(amps_, p);

  MatrixXc g(m, n);
  generate_transfer_matrix(foci, geometry, g);

  MatrixXc b(m, m + n);
  backend->concat_col(g, p, b);

  backend->mul(TRANSPOSE::CONJ_TRANS, TRANSPOSE::NO_TRANS, ONE, b, b, ZERO, bhb);
}

void make_t(const BackendPtr& backend, const VectorXd& zero, const VectorXd& x, VectorXc& t) {
  backend->make_complex(zero, x, t);
  backend->scale(complex(-1, 0), t);
  backend->exp(t, t);
}

void calc_jtj_jtf(const BackendPtr& backend, const VectorXc& t, const MatrixXc& bhb, MatrixXc& tth, MatrixXc& bhb_tth, MatrixXd& bhb_tth_i,
                  MatrixXd& jtj, VectorXd& jtf) {
  backend->mul(TRANSPOSE::NO_TRANS, TRANSPOSE::CONJ_TRANS, ONE, t, t, ZERO, tth);
  backend->hadamard_product(bhb, tth, bhb_tth);
  backend->real(bhb_tth, jtj);
  backend->imag(bhb_tth, bhb_tth_i);
  backend->reduce_col(bhb_tth_i, jtf);
}

double calc_fx(const BackendPtr& backend, const VectorXd& zero, const VectorXd& x, const MatrixXc& bhb, VectorXc& tmp, VectorXc& t) {
  backend->make_complex(zero, x, t);
  backend->exp(t, t);
  backend->mul(TRANSPOSE::NO_TRANS, ONE, bhb, t, ZERO, tmp);
  return backend->dot(t, tmp).real();
}

template <typename T>
void lm_calc_impl(const BackendPtr& backend, const std::vector<core::Vector3>& foci, std::vector<complex>& amps, const core::Geometry<T>& geometry,
                  const double eps_1, const double eps_2, const double tau, const size_t k_max, const std::vector<double>& initial,
                  AmplitudeConstraint constraint, typename T::D& drives) {
  backend->init();

  const auto m = foci.size();
  const auto n = geometry.num_transducers();

  const auto n_param = n + m;

  MatrixXc bhb = MatrixXc::Zero(n_param, n_param);
  make_bhb(backend, foci, amps, geometry, bhb);

  VectorXd x = VectorXd::Zero(n_param);
  for (size_t i = 0; i < initial.size(); i++) x(i) = initial[i];

  auto nu = 2.0;

  VectorXd zero = VectorXd::Zero(n_param);

  VectorXc t(n_param);
  make_t(backend, zero, x, t);

  MatrixXc tth = VectorXc::Zero(n_param, n_param);
  MatrixXc bhb_tth(n_param, n_param);
  MatrixXd bhb_tth_i(n_param, n_param);
  MatrixXd a(n_param, n_param);
  VectorXd g(n_param);
  calc_jtj_jtf(backend, t, bhb, tth, bhb_tth, bhb_tth_i, a, g);

  VectorXd a_diag(n_param);
  backend->get_diagonal(a, a_diag);
  const auto a_max = backend->max_element(a_diag);

  auto mu = tau * a_max;

  VectorXc tmp = VectorXc::Zero(n_param);
  VectorXc t_(n_param);
  double fx = calc_fx(backend, zero, x, bhb, tmp, t);

  MatrixXd identity = MatrixXd::Identity(n_param, n_param);

  VectorXd tmp_vec(n_param);
  VectorXd h_lm(n_param);
  VectorXd x_new(n_param);
  MatrixXd tmp_mat(n_param, n_param);
  for (size_t k = 0; k < k_max; k++) {
    if (backend->max_element(g) <= eps_1) break;

    backend->copy_to(a, tmp_mat);
    backend->add(mu, identity, tmp_mat);

    backend->copy_to(g, h_lm);

    backend->solvet(tmp_mat, h_lm);
    if (std::sqrt(backend->dot(h_lm, h_lm)) <= eps_2 * (std::sqrt(backend->dot(x, x)) + eps_2)) break;

    backend->copy_to(x, x_new);
    backend->add(-1.0, h_lm, x_new);

    const double fx_new = calc_fx(backend, zero, x_new, bhb, tmp, t);

    backend->copy_to(g, tmp_vec);
    backend->add(mu, h_lm, tmp_vec);

    const double l0_lhlm = backend->dot(h_lm, tmp_vec) / 2;

    const auto rho = (fx - fx_new) / l0_lhlm;
    fx = fx_new;
    if (rho > 0) {
      backend->copy_to(x_new, x);

      make_t(backend, zero, x, t);
      calc_jtj_jtf(backend, t, bhb, tth, bhb_tth, bhb_tth_i, a, g);

      mu *= (std::max)(1. / 3., std::pow(1 - (2 * rho - 1), 3.0));
      nu = 2;
    } else {
      mu *= nu;
      nu *= 2;
    }
  }

  backend->to_host(x);
  for (auto& dev : geometry)
    for (auto& tr : dev) {
      const auto phase = x(tr.id());
      const auto power = std::visit([&](auto& c) { return c.convert(1.0, 1.0); }, constraint);
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

void LM<core::LegacyTransducer>::calc(const core::Geometry<core::LegacyTransducer>& geometry) {
  lm_calc_impl(_backend, _foci, _amps, geometry, eps_1, eps_2, tau, k_max, initial, constraint, this->_props.drives);
}
void LM<core::NormalTransducer>::calc(const core::Geometry<core::NormalTransducer>& geometry) {
  lm_calc_impl(_backend, _foci, _amps, geometry, eps_1, eps_2, tau, k_max, initial, constraint, this->_props.drives);
}

}  // namespace autd3::gain::holo
