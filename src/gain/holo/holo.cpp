// File: holo.cpp
// Project: holo
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 02/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/gain/holo.hpp"

#include <random>

#ifdef HOLO_PARALLEL_FOR
#include <execution>
#endif

#include "autd3/core/acoustics.hpp"

namespace autd3::gain::holo {

template <class InIt, class OutIt, class Fn>
OutIt transform(const InIt first, const InIt last, OutIt dest, Fn func) {
#ifdef HOLO_PARALLEL_FOR
  return std::transform(std::execution::par_unseq, first, last, dest, func);
#else
  return std::transform(first, last, dest, func);
#endif
}

template <class InIt, class Fn>
void for_each(const InIt first, const InIt last, Fn func) {
#ifdef HOLO_PARALLEL_FOR
  std::for_each(std::execution::par_unseq, first, last, func);
#else
  std::for_each(first, last, func);
#endif
}

template <class Fn>
std::vector<driver::Drive> transform(const core::Geometry& geometry, Fn func) {
  std::vector<driver::Drive> drives;
  drives.resize(geometry.num_transducers());
  autd3::gain::holo::transform(geometry.begin(), geometry.end(), drives.begin(), func);
  return drives;
}

namespace {

void generate_transfer_matrix(const std::vector<core::Vector3>& foci, const core::Geometry& geometry, MatrixXc& dst) {
  const auto sound_speed = geometry.sound_speed;
  const auto attenuation = geometry.attenuation;
  holo::for_each(geometry.begin(), geometry.end(), [&](const auto& transducer) {
    for (size_t i = 0; i < foci.size(); i++)
      dst(i, transducer.idx()) =
          core::propagate(transducer.position(), transducer.z_direction(), attenuation, transducer.wavenumber(sound_speed), foci[i]);
  });
}

void back_prop(const BackendPtr& backend, const MatrixXc& transfer, const VectorXc& amps, MatrixXc& b) {
  const auto m = transfer.rows();

  MatrixXc tmp = MatrixXc::Zero(m, m);
  backend->mul(Transpose::NoTrans, Transpose::ConjTrans, ONE, transfer, transfer, ZERO, tmp);

  VectorXc denominator(m);
  backend->get_diagonal(tmp, denominator);
  backend->reciprocal(denominator, denominator);
  backend->hadamard_product(amps, denominator, denominator);

  backend->create_diagonal(denominator, tmp);
  backend->mul(Transpose::ConjTrans, Transpose::NoTrans, ONE, transfer, tmp, ZERO, b);
}
}  // namespace

std::vector<driver::Drive> SDP::calc(const core::Geometry& geometry) {
  _backend->init();

  const auto m = static_cast<Eigen::Index>(_foci.size());
  const auto n = static_cast<Eigen::Index>(geometry.num_transducers());

  const VectorXc amps_ = Eigen::Map<VectorXc, Eigen::Unaligned>(_amps.data(), static_cast<Eigen::Index>(_amps.size()));

  MatrixXc p(m, m);
  _backend->create_diagonal(amps_, p);

  MatrixXc b(m, n);
  generate_transfer_matrix(_foci, geometry, b);

  MatrixXc pseudo_inv_b = MatrixXc::Zero(n, m);
  MatrixXc u_(m, m);
  MatrixXc s(n, m);
  MatrixXc vt(n, n);
  MatrixXc buf = MatrixXc::Zero(n, m);
  MatrixXc b_tmp(m, n);
  _backend->copy_to(b, b_tmp);
  _backend->pseudo_inverse_svd(b_tmp, alpha, u_, s, vt, buf, pseudo_inv_b);

  MatrixXc mm(m, m);
  VectorXc one = VectorXc::Ones(m);
  _backend->create_diagonal(one, mm);

  _backend->mul(Transpose::NoTrans, Transpose::NoTrans, -ONE, b, pseudo_inv_b, ONE, mm);

  MatrixXc tmp = MatrixXc::Zero(m, m);
  _backend->mul(Transpose::NoTrans, Transpose::NoTrans, ONE, p, mm, ZERO, tmp);
  _backend->mul(Transpose::NoTrans, Transpose::NoTrans, ONE, tmp, p, ZERO, mm);

  MatrixXc x_mat(m, m);
  _backend->create_diagonal(one, x_mat);

  std::random_device rnd;
  std::mt19937 mt(rnd());
  std::uniform_real_distribution<driver::autd3_float_t> range(0, 1);
  VectorXc zero = VectorXc::Zero(m);
  VectorXc x = VectorXc::Zero(m);
  VectorXc x_conj(m);
  VectorXc mmc(m);
  for (size_t i = 0; i < repeat; i++) {
    const auto ii = static_cast<size_t>(std::floor(static_cast<driver::autd3_float_t>(m) * range(mt)));

    _backend->get_col(mm, ii, mmc);
    _backend->set(ii, ZERO, mmc);

    _backend->mul(Transpose::NoTrans, ONE, x_mat, mmc, ZERO, x);
    if (complex gamma = _backend->dot(x, mmc); gamma.real() > 0) {
      _backend->scale(complex(-std::sqrt(lambda / gamma.real()), 0.0), x);
      _backend->conj(x, x_conj);

      _backend->set_row(x_conj, ii, 0, ii, x_mat);
      _backend->set_row(x_conj, ii, ii + 1, m, x_mat);
      _backend->set_col(x, ii, 0, ii, x_mat);
      _backend->set_col(x, ii, ii + 1, m, x_mat);
    } else {
      _backend->set_row(zero, ii, 0, ii, x_mat);
      _backend->set_row(zero, ii, ii + 1, m, x_mat);
      _backend->set_col(zero, ii, 0, ii, x_mat);
      _backend->set_col(zero, ii, ii + 1, m, x_mat);
    }
  }

  VectorXc u(m);
  _backend->max_eigen_vector(x_mat, u);

  VectorXc ut = VectorXc::Zero(m);
  _backend->mul(Transpose::NoTrans, ONE, p, u, ZERO, ut);

  VectorXc q = VectorXc::Zero(n);
  _backend->mul(Transpose::NoTrans, ONE, pseudo_inv_b, ut, ZERO, q);
  _backend->to_host(q);

  const auto max_coefficient = std::abs(_backend->max_abs_element(q));
  return holo::transform(geometry, [&](const auto& tr) {
    const auto phase = std::arg(q(tr.idx())) + driver::pi;
    const auto raw = std::abs(q(tr.idx()));
    const auto power = constraint->convert(raw, max_coefficient);
    return driver::Drive{phase, power};
  });
}

std::vector<driver::Drive> EVP::calc(const core::Geometry& geometry) {
  _backend->init();

  const auto m = static_cast<Eigen::Index>(_foci.size());
  const auto n = static_cast<Eigen::Index>(geometry.num_transducers());

  const VectorXc amps_ = Eigen::Map<VectorXc, Eigen::Unaligned>(_amps.data(), static_cast<Eigen::Index>(_amps.size()));

  MatrixXc g(m, n);
  generate_transfer_matrix(_foci, geometry, g);

  MatrixXc x = MatrixXc::Zero(n, m);
  back_prop(_backend, g, amps_, x);

  MatrixXc r = MatrixXc::Zero(m, m);
  _backend->mul(Transpose::NoTrans, Transpose::NoTrans, ONE, g, x, ZERO, r);
  VectorXc max_ev(m);
  _backend->max_eigen_vector(r, max_ev);

  MatrixXc sigma(n, n);
  {
    VectorXc sigma_tmp = VectorXc::Zero(n);
    _backend->mul(Transpose::Trans, ONE, g, amps_, ZERO, sigma_tmp);
    VectorXd sigma_tmp_real(n);
    _backend->abs(sigma_tmp, sigma_tmp_real);
    _backend->scale(1 / static_cast<driver::autd3_float_t>(m), sigma_tmp_real);
    _backend->sqrt(sigma_tmp_real, sigma_tmp_real);
    _backend->pow(sigma_tmp_real, gamma, sigma_tmp_real);
    const VectorXd zero = VectorXd::Zero(n);
    _backend->make_complex(sigma_tmp_real, zero, sigma_tmp);
    _backend->create_diagonal(sigma_tmp, sigma);
  }

  MatrixXc gr = MatrixXc::Zero(m + n, n);
  _backend->concat_row(g, sigma, gr);

  VectorXc fm(m);
  _backend->arg(max_ev, fm);
  _backend->hadamard_product(amps_, fm, fm);

  const VectorXc fn = VectorXc::Zero(n);
  VectorXc f(m + n);
  _backend->concat_row(fm, fn, f);

  MatrixXc gtg = MatrixXc::Zero(n, n);
  _backend->mul(Transpose::ConjTrans, Transpose::NoTrans, ONE, gr, gr, ZERO, gtg);

  VectorXc gtf = VectorXc::Zero(n);
  _backend->mul(Transpose::ConjTrans, ONE, gr, f, ZERO, gtf);

  _backend->solveh(gtg, gtf);

  _backend->to_host(gtf);
  const auto max_coefficient = std::abs(_backend->max_abs_element(gtf));
  return holo::transform(geometry, [&](const auto& tr) {
    const auto phase = std::arg(gtf(tr.idx())) + driver::pi;
    const auto raw = std::abs(gtf(tr.idx()));
    const auto power = constraint->convert(raw, max_coefficient);
    return driver::Drive{phase, power};
  });
}

std::vector<driver::Drive> LSS::calc(const core::Geometry& geometry) {
  _backend->init();

  const auto m = static_cast<Eigen::Index>(_foci.size());
  const auto n = static_cast<Eigen::Index>(geometry.num_transducers());

  const VectorXc p = Eigen::Map<VectorXc, Eigen::Unaligned>(_amps.data(), static_cast<Eigen::Index>(_amps.size()));

  MatrixXc g(m, n);
  generate_transfer_matrix(_foci, geometry, g);

  VectorXc q = VectorXc::Zero(n);
  _backend->mul(Transpose::ConjTrans, ONE, g, p, ZERO, q);
  _backend->to_host(q);

  const auto max_coefficient = std::abs(_backend->max_abs_element(q));
  return holo::transform(geometry, [&](const auto& tr) {
    const auto phase = std::arg(q(tr.idx())) + driver::pi;
    const auto raw = std::abs(q(tr.idx()));
    const auto power = constraint->convert(raw, max_coefficient);
    return driver::Drive{phase, power};
  });
}

std::vector<driver::Drive> GS::calc(const core::Geometry& geometry) {
  _backend->init();

  const auto m = static_cast<Eigen::Index>(_foci.size());
  const auto n = static_cast<Eigen::Index>(geometry.num_transducers());

  const VectorXc amps_ = Eigen::Map<VectorXc, Eigen::Unaligned>(_amps.data(), static_cast<Eigen::Index>(_amps.size()));

  MatrixXc g(m, n);
  generate_transfer_matrix(_foci, geometry, g);

  const VectorXc q0 = VectorXc::Ones(n);

  VectorXc q = q0;

  VectorXc gamma = VectorXc::Zero(m);
  VectorXc p(m);
  VectorXc xi = VectorXc::Zero(n);
  for (size_t k = 0; k < repeat; k++) {
    _backend->mul(Transpose::NoTrans, ONE, g, q, ZERO, gamma);
    _backend->arg(gamma, gamma);
    _backend->hadamard_product(gamma, amps_, p);
    _backend->mul(Transpose::ConjTrans, ONE, g, p, ZERO, xi);
    _backend->arg(xi, xi);
    _backend->hadamard_product(xi, q0, q);
  }

  const auto max_coefficient = std::abs(_backend->max_abs_element(q));
  _backend->to_host(q);
  return holo::transform(geometry, [&](const auto& tr) {
    const auto phase = std::arg(q(tr.idx())) + driver::pi;
    const auto raw = std::abs(q(tr.idx()));
    const auto power = constraint->convert(raw, max_coefficient);
    return driver::Drive{phase, power};
  });
}

std::vector<driver::Drive> GSPAT::calc(const core::Geometry& geometry) {
  _backend->init();

  const auto m = static_cast<Eigen::Index>(_foci.size());
  const auto n = static_cast<Eigen::Index>(geometry.num_transducers());

  const VectorXc amps_ = Eigen::Map<VectorXc, Eigen::Unaligned>(_amps.data(), static_cast<Eigen::Index>(_amps.size()));

  MatrixXc g(m, n);
  generate_transfer_matrix(_foci, geometry, g);

  MatrixXc b = MatrixXc::Zero(n, m);
  back_prop(_backend, g, amps_, b);

  MatrixXc r = MatrixXc::Zero(m, m);
  _backend->mul(Transpose::NoTrans, Transpose::NoTrans, ONE, g, b, ZERO, r);

  VectorXc p = amps_;

  VectorXc gamma = VectorXc::Zero(m);
  _backend->mul(Transpose::NoTrans, ONE, r, p, ZERO, gamma);
  for (size_t k = 0; k < repeat; k++) {
    _backend->arg(gamma, gamma);
    _backend->hadamard_product(gamma, amps_, p);
    _backend->mul(Transpose::NoTrans, ONE, r, p, ZERO, gamma);
  }

  VectorXc tmp(m);
  _backend->abs(gamma, tmp);
  _backend->reciprocal(tmp, tmp);
  _backend->hadamard_product(tmp, amps_, tmp);
  _backend->hadamard_product(tmp, amps_, tmp);
  _backend->arg(gamma, gamma);
  _backend->hadamard_product(gamma, tmp, p);

  VectorXc q = VectorXc::Zero(n);
  _backend->mul(Transpose::NoTrans, ONE, b, p, ZERO, q);

  const auto max_coefficient = std::abs(_backend->max_abs_element(q));
  _backend->to_host(q);
  return holo::transform(geometry, [&](const auto& tr) {
    const auto phase = std::arg(q(tr.idx())) + driver::pi;
    const auto raw = std::abs(q(tr.idx()));
    const auto power = constraint->convert(raw, max_coefficient);
    return driver::Drive{phase, power};
  });
}

void make_bhb(const BackendPtr& backend, const std::vector<core::Vector3>& foci, std::vector<complex>& amps, const core::Geometry& geometry,
              MatrixXc& bhb) {
  const auto m = foci.size();
  const auto n = geometry.num_transducers();

  VectorXc amps_ = Eigen::Map<VectorXc, Eigen::Unaligned>(amps.data(), static_cast<Eigen::Index>(amps.size()));

  MatrixXc p(m, m);
  backend->scale(complex(-1.0, 0.0), amps_);
  backend->create_diagonal(amps_, p);

  MatrixXc g(m, n);
  generate_transfer_matrix(foci, geometry, g);

  MatrixXc b(m, m + n);
  backend->concat_col(g, p, b);

  backend->mul(Transpose::ConjTrans, Transpose::NoTrans, ONE, b, b, ZERO, bhb);
}

void make_t(const BackendPtr& backend, const VectorXd& zero, const VectorXd& x, VectorXc& t) {
  backend->make_complex(zero, x, t);
  backend->scale(complex(-1, 0), t);
  backend->exp(t, t);
}

void calc_jtj_jtf(const BackendPtr& backend, const VectorXc& t, const MatrixXc& bhb, MatrixXc& tth, MatrixXc& bhb_tth, MatrixXd& bhb_tth_i,
                  MatrixXd& jtj, VectorXd& jtf) {
  backend->mul(Transpose::NoTrans, Transpose::ConjTrans, ONE, t, t, ZERO, tth);
  backend->hadamard_product(bhb, tth, bhb_tth);
  backend->real(bhb_tth, jtj);
  backend->imag(bhb_tth, bhb_tth_i);
  backend->reduce_col(bhb_tth_i, jtf);
}

driver::autd3_float_t calc_fx(const BackendPtr& backend, const VectorXd& zero, const VectorXd& x, const MatrixXc& bhb, VectorXc& tmp, VectorXc& t) {
  backend->make_complex(zero, x, t);
  backend->exp(t, t);
  backend->mul(Transpose::NoTrans, ONE, bhb, t, ZERO, tmp);
  return backend->dot(t, tmp).real();
}

std::vector<driver::Drive> LM::calc(const core::Geometry& geometry) {
  _backend->init();

  const auto m = static_cast<Eigen::Index>(_foci.size());
  const auto n = static_cast<Eigen::Index>(geometry.num_transducers());

  const auto n_param = n + m;

  MatrixXc bhb = MatrixXc::Zero(n_param, n_param);
  make_bhb(_backend, _foci, _amps, geometry, bhb);

  VectorXd x = VectorXd::Zero(n_param);
  std::copy(initial.begin(), initial.end(), x.begin());

  driver::autd3_float_t nu = 2;

  const VectorXd zero = VectorXd::Zero(n_param);

  VectorXc t(n_param);
  make_t(_backend, zero, x, t);

  MatrixXc tth = MatrixXc::Zero(n_param, n_param);
  MatrixXc bhb_tth(n_param, n_param);
  MatrixXd bhb_tth_i(n_param, n_param);
  MatrixXd a(n_param, n_param);
  VectorXd g(n_param);
  calc_jtj_jtf(_backend, t, bhb, tth, bhb_tth, bhb_tth_i, a, g);

  VectorXd a_diag(n_param);
  _backend->get_diagonal(a, a_diag);
  const auto a_max = _backend->max_element(a_diag);

  auto mu = tau * a_max;

  VectorXc tmp = VectorXc::Zero(n_param);
  VectorXc t_(n_param);
  driver::autd3_float_t fx = calc_fx(_backend, zero, x, bhb, tmp, t);

  const MatrixXd identity = MatrixXd::Identity(n_param, n_param);

  VectorXd tmp_vec(n_param);
  VectorXd h_lm(n_param);
  VectorXd x_new(n_param);
  MatrixXd tmp_mat(n_param, n_param);
  for (size_t k = 0; k < k_max; k++) {
    if (_backend->max_element(g) <= eps_1) break;

    _backend->copy_to(a, tmp_mat);

    _backend->add(mu, identity, tmp_mat);

    _backend->copy_to(g, h_lm);

    _backend->solvet(tmp_mat, h_lm);

    if (std::sqrt(_backend->dot(h_lm, h_lm)) <= eps_2 * (std::sqrt(_backend->dot(x, x)) + eps_2)) break;

    _backend->copy_to(x, x_new);
    _backend->add(-1.0, h_lm, x_new);

    const driver::autd3_float_t fx_new = calc_fx(_backend, zero, x_new, bhb, tmp, t);

    _backend->copy_to(g, tmp_vec);
    _backend->add(mu, h_lm, tmp_vec);

    const driver::autd3_float_t l0_lhlm = _backend->dot(h_lm, tmp_vec) / 2;

    const auto rho = (fx - fx_new) / l0_lhlm;
    fx = fx_new;

    if (rho > 0) {
      _backend->copy_to(x_new, x);

      make_t(_backend, zero, x, t);
      calc_jtj_jtf(_backend, t, bhb, tth, bhb_tth, bhb_tth_i, a, g);

      mu *= (std::max)(driver::autd3_float_t{1} / driver::autd3_float_t{3}, std::pow(1 - (2 * rho - 1), driver::autd3_float_t{3}));
      nu = 2;
    } else {
      mu *= nu;
      nu *= 2;
    }
  }

  _backend->to_host(x);
  return holo::transform(geometry, [&](const auto& tr) {
    const auto phase = driver::rem_euclid(x(tr.idx()), 2 * driver::pi);
    const auto power = constraint->convert(1.0, 1.0);
    return driver::Drive{phase, power};
  });
}

std::vector<driver::Drive> Greedy::calc(const core::Geometry& geometry) {
  const auto m = static_cast<Eigen::Index>(_foci.size());
  const auto n = geometry.num_transducers();

  const VectorXd amps_ = Eigen::Map<VectorXc, Eigen::Unaligned>(_amps.data(), static_cast<Eigen::Index>(_amps.size())).real();

  std::vector<complex> phases(phase_div);
  driver::autd3_float_t phase = 0;
  std::generate(phases.begin(), phases.end(), [&] {
    phase += 2 * driver::pi / static_cast<driver::autd3_float_t>(phase_div);
    return complex(std::cos(phase), std::sin(phase));
  });

  std::vector<VectorXc> tmp(phases.size());
  std::generate(tmp.begin(), tmp.end(), [m] { return VectorXc(m); });

  VectorXc cache = VectorXc::Zero(m);

  const auto sound_speed = geometry.sound_speed;
  const auto attenuation = geometry.attenuation;
  auto transfer_foci = [attenuation, sound_speed](const core::Transducer& trans, const complex p, const std::vector<core::Vector3>& foci_,
                                                  VectorXc& res) {
    std::transform(foci_.begin(), foci_.end(), res.begin(), [&trans, p, attenuation, sound_speed](const core::Vector3& focus) {
      return core::propagate(trans.position(), trans.z_direction(), attenuation, trans.wavenumber(sound_speed), focus) * p;
    });
  };

  std::vector<size_t> select(n);
  std::iota(select.begin(), select.end(), 0);
  std::random_device seed_gen;
  std::mt19937 engine(seed_gen());
  std::shuffle(select.begin(), select.end(), engine);
  std::vector<driver::Drive> drives;
  drives.resize(geometry.num_transducers());
  for (const auto i : select) {
    const auto& transducer = geometry[i];
    size_t min_idx = 0;
    auto min_v = std::numeric_limits<driver::autd3_float_t>::infinity();
    for (size_t p = 0; p < phases.size(); p++) {
      transfer_foci(transducer, phases[p], _foci, tmp[p]);
      if (const auto v = objective(amps_, tmp[p] + cache); v < min_v) {
        min_v = v;
        min_idx = p;
      }
    }
    cache += tmp[min_idx];

    const auto power = constraint->convert(1.0, 1.0);

    drives[transducer.idx()].amp = power;
    drives[transducer.idx()].phase = std::arg(phases[min_idx] + driver::pi);
  }
  return drives;
}

std::vector<driver::Drive> LSSGreedy::calc(const core::Geometry& geometry) {
  const auto m = static_cast<Eigen::Index>(_foci.size());
  const auto n = geometry.num_transducers();

  const VectorXd amps_ = Eigen::Map<VectorXc, Eigen::Unaligned>(_amps.data(), static_cast<Eigen::Index>(_amps.size())).real();

  std::vector<complex> phases(phase_div);
  driver::autd3_float_t phase = 0;
  std::generate(phases.begin(), phases.end(), [&] {
    phase += 2 * driver::pi / static_cast<driver::autd3_float_t>(phase_div);
    return complex(std::cos(phase), std::sin(phase));
  });

  std::vector<VectorXc> focus_phase_list;
  focus_phase_list.reserve(_foci.size());
  const auto sound_speed = geometry.sound_speed;
  std::transform(_foci.begin(), _foci.end(), std::back_inserter(focus_phase_list), [&](const auto& focus) {
    VectorXc q(n);
    holo::transform(geometry.begin(), geometry.end(), q.begin(), [&](const auto& tr) {
      const auto p = tr.align_phase_at(focus, sound_speed);
      return complex(std::cos(p), std::sin(p));
    });
    return q;
  });

  MatrixXc g(m, n);
  generate_transfer_matrix(_foci, geometry, g);

  std::vector<VectorXc> tmp(phases.size());
  std::generate(tmp.begin(), tmp.end(), [m] { return VectorXc::Zero(m); });

  VectorXc q = focus_phase_list[0];
  std::vector<size_t> select(m - 1);
  std::iota(select.begin(), select.end(), 1);
  std::random_device seed_gen;
  std::mt19937 engine(seed_gen());
  std::shuffle(select.begin(), select.end(), engine);
  for (const auto i : select) {
    size_t min_idx = 0;
    auto min_v = std::numeric_limits<driver::autd3_float_t>::infinity();
    for (size_t j = 0; j < phases.size(); j++) {
      const auto q_tmp = q + focus_phase_list[i] * phases[j];
      _backend->mul(Transpose::NoTrans, ONE, g, q_tmp, ZERO, tmp[j]);
      if (const auto v = objective(amps_, tmp[j]); v < min_v) {
        min_v = v;
        min_idx = j;
      }
    }
    q += focus_phase_list[i] * phases[min_idx];
  }

  _backend->to_host(q);
  const auto max_coefficient = std::abs(_backend->max_abs_element(q));
  return holo::transform(geometry, [&](const auto& tr) {
    const auto raw = std::abs(q(tr.idx()));
    const auto power = constraint->convert(raw, max_coefficient);
    return driver::Drive{std::arg(q(tr.idx()) + driver::pi), power};
  });
}

std::vector<driver::Drive> APO::calc(const core::Geometry& geometry) {
  auto make_ri = [&](const MatrixXc& g, const Eigen::Index m, const Eigen::Index n, const Eigen::Index i) {
    MatrixXc di = MatrixXc::Zero(m, m);
    di(i, i) = ONE;

    MatrixXc ri = MatrixXc::Zero(n, n);

    MatrixXc tmp = MatrixXc::Zero(n, m);

    _backend->mul(Transpose::ConjTrans, Transpose::NoTrans, ONE, g, di, ZERO, tmp);
    _backend->mul(Transpose::NoTrans, Transpose::NoTrans, ONE, tmp, g, ZERO, ri);

    return ri;
  };

  auto calc_nabla_j = [&](const VectorXc& q, const VectorXc& p2, const std::vector<MatrixXc>& ris, const size_t m, const size_t n,
                          VectorXc& nabla_j) {
    VectorXc tmp = VectorXc::Zero(static_cast<Eigen::Index>(n));
    for (size_t i = 0; i < m; i++) {
      _backend->mul(Transpose::NoTrans, ONE, ris[i], q, ZERO, tmp);
      const auto s = p2(static_cast<Eigen::Index>(i)) - _backend->dot(q, tmp);
      _backend->scale(s, tmp);
      _backend->add(ONE, tmp, nabla_j);
    }
    _backend->add(complex(lambda, 0), q, nabla_j);
  };

  auto calc_j = [&](const VectorXc& q, const VectorXc& p2, const std::vector<MatrixXc>& ris, const size_t n) {
    MatrixXc tmp = MatrixXc::Zero(static_cast<Eigen::Index>(n), 1);
    size_t i = 0;
    driver::autd3_float_t j =
        std::accumulate(ris.begin(), ris.end(), driver::autd3_float_t{0}, [&](const driver::autd3_float_t acc, const MatrixXc& r) {
          _backend->mul(Transpose::NoTrans, Transpose::NoTrans, ONE, r, q, ZERO, tmp);
          const auto s = p2(static_cast<Eigen::Index>(i++), 0) - _backend->dot(q, tmp);
          return acc + std::norm(s);
        });
    j += std::abs(_backend->dot(q, q)) * lambda;
    return j;
  };

  auto line_search = [&](const VectorXc& q, const VectorXc& p2, const std::vector<MatrixXc>& ris, const size_t n) {
    driver::autd3_float_t alpha = 0;
    auto min = (std::numeric_limits<driver::autd3_float_t>::max)();
    for (size_t i = 0; i < line_search_max; i++) {
      const auto a = static_cast<driver::autd3_float_t>(i) / static_cast<driver::autd3_float_t>(line_search_max);  // FIXME: only for 0-1
      if (const auto v = calc_j(q, p2, ris, n); v < min) {
        alpha = a;
        min = v;
      }
    }
    return alpha;
  };

  const auto m = _foci.size();
  const auto n = geometry.num_transducers();

  MatrixXc g(m, n);
  generate_transfer_matrix(_foci, geometry, g);

  const VectorXc p = Eigen::Map<VectorXc, Eigen::Unaligned>(_amps.data(), static_cast<Eigen::Index>(_amps.size()));

  VectorXc p2(m);
  _backend->hadamard_product(p, p, p2);

  MatrixXc h(n, n);
  const VectorXc one = VectorXc::Ones(static_cast<Eigen::Index>(n));
  _backend->create_diagonal(one, h);

  MatrixXc tmp = MatrixXc::Zero(static_cast<Eigen::Index>(n), static_cast<Eigen::Index>(n));
  _backend->mul(Transpose::ConjTrans, Transpose::NoTrans, ONE, g, g, ZERO, tmp);
  _backend->add(complex(lambda, 0.0), h, tmp);

  VectorXc q = VectorXc::Zero(static_cast<Eigen::Index>(n));
  _backend->mul(Transpose::ConjTrans, ONE, g, p, ZERO, q);
  _backend->solveh(tmp, q);

  std::vector<MatrixXc> ris(m);
  Eigen::Index i = 0;
  std::generate(ris.begin(), ris.end(), [&] { return make_ri(g, static_cast<Eigen::Index>(m), static_cast<Eigen::Index>(n), i++); });

  VectorXc nabla_j = VectorXc::Zero(static_cast<Eigen::Index>(n));
  calc_nabla_j(q, p2, ris, m, n, nabla_j);

  VectorXc d = VectorXc::Zero(static_cast<Eigen::Index>(n));
  VectorXc nabla_j_new = VectorXc::Zero(static_cast<Eigen::Index>(n));
  VectorXc s(n);
  VectorXc hs = VectorXc::Zero(static_cast<Eigen::Index>(n));
  for (size_t k = 0; k < k_max; k++) {
    _backend->mul(Transpose::NoTrans, -ONE, h, nabla_j, ZERO, d);

    _backend->scale(complex(line_search(q, p2, ris, n), 0), d);

    if (std::sqrt(_backend->dot(d, d).real()) < eps) break;

    _backend->add(ONE, d, q);
    calc_nabla_j(q, p2, ris, m, n, nabla_j_new);

    _backend->copy_to(nabla_j_new, s);
    _backend->add(-ONE, nabla_j, s);

    const auto ys = ONE / _backend->dot(d, s);
    _backend->mul(Transpose::NoTrans, ONE, h, s, ZERO, hs);
    const auto shs = -ONE / _backend->dot(s, hs);

    _backend->mul(Transpose::NoTrans, Transpose::ConjTrans, ys, d, d, ONE, h);
    _backend->mul(Transpose::NoTrans, Transpose::ConjTrans, shs, hs, hs, ONE, h);

    _backend->copy_to(nabla_j_new, nabla_j);
  }

  _backend->to_host(q);
  const auto max_coefficient = std::abs(_backend->max_abs_element(q));
  return holo::transform(geometry, [&](const auto& tr) {
    const auto phase = std::arg(q(tr.idx())) + driver::pi;
    const auto raw = std::abs(q(tr.idx()));
    const auto power = constraint->convert(raw, max_coefficient);
    return driver::Drive{phase, power};
  });
}

}  // namespace autd3::gain::holo
