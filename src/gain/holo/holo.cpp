// File: holo.cpp
// Project: holo
// Created Date: 09/12/2021
// Author: Shun Suzuki
// -----
// Last Modified: 13/05/2022
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
void sdp_calc_impl(const BackendPtr& backend, const std::vector<core::Vector3>& foci, std::vector<complex>& amps, const core::Geometry<T>& geometry,
                   const double alpha, const double lambda, const size_t repeat) {
  const auto m = foci.size();
  const auto n = geometry.num_transducers();

  VectorXc amps_ = Eigen::Map<VectorXc, Eigen::Unaligned>(amps.data(), static_cast<Eigen::Index>(amps.size()));

  MatrixXc p(m, m);
  backend->create_diagonal(amps_, p);

  MatrixXc b(m, n);
  std::vector<core::Vector3> transducers;
  std::for_each(geometry.begin(), geometry.end(), [&](const auto& dev) {
    std::transform(dev.begin(), dev.end(), std::back_inserter(transducers), [](const auto& tr) { return tr.position(); });
  });
  backend->generate_transfer_matrix(foci, transducers, b);

  MatrixXc pseudo_inv_b(n, m);
  MatrixXc u_(m, m);
  MatrixXc s(n, m);
  MatrixXc vt(n, n);
  MatrixXc buf(n, m);
  MatrixXc b_tmp(m, n);
  backend->copy(b, b_tmp);
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

  const auto max_coefficient = backend->max_abs_element(q);
  // backend->set_from_complex_drive(q, normalize, max_coefficient, dst);
}
}  // namespace

void SDP<core::LegacyTransducer>::calc(const core::Geometry<core::LegacyTransducer>& geometry) {
  sdp_calc_impl(_backend, _foci, _amps, geometry, _alpha, _lambda, _repeat);
}
void SDP<core::NormalTransducer>::calc(const core::Geometry<core::NormalTransducer>& geometry) {
  sdp_calc_impl(_backend, _foci, _amps, geometry, _alpha, _lambda, _repeat);
}

}  // namespace autd3::gain::holo
