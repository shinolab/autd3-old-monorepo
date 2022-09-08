// File: backend_cuda.cpp
// Project: cuda
// Created Date: 13/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 08/09/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/gain/backend_arrayfire.hpp"

#include <arrayfire.h>

namespace autd3::gain::holo {

class BufferPool final {
 public:
  BufferPool() = default;
  ~BufferPool() { clear(); }
  BufferPool(const BufferPool& v) noexcept = default;
  BufferPool& operator=(const BufferPool& obj) = default;
  BufferPool(BufferPool&& obj) = default;
  BufferPool& operator=(BufferPool&& obj) = default;

  af::array get(const VectorXc& v) {
    const auto key = reinterpret_cast<std::uintptr_t>(v.data());
    if (_pool.find(key) != _pool.end()) {
      af_array ret;
      af_retain_array(&ret, _pool[key]);
      return af::array(ret);
    }

    af::array va(v.size(), reinterpret_cast<const af::cdouble*>(v.data()));
    af_array vap;
    af_retain_array(&vap, va.get());
    _pool.emplace(key, vap);
    return va;
  }

  af::array get(const MatrixXc& v) {
    const auto key = reinterpret_cast<std::uintptr_t>(v.data());
    if (_pool.find(key) != _pool.end()) {
      af_array ret;
      af_retain_array(&ret, _pool[key]);
      return af::array(ret);
    }

    af::array va(v.rows(), v.cols(), reinterpret_cast<const af::cdouble*>(v.data()));
    af_array vap;
    af_retain_array(&vap, va.get());
    _pool.emplace(key, vap);
    return va;
  }

  af::array get(const VectorXd& v) {
    const auto key = reinterpret_cast<std::uintptr_t>(v.data());
    if (_pool.find(key) != _pool.end()) {
      af_array ret;
      af_retain_array(&ret, _pool[key]);
      return af::array(ret);
    }

    af::array va(v.size(), v.data());
    af_array vap;
    af_retain_array(&vap, va.get());
    _pool.emplace(key, vap);
    return va;
  }

  af::array get(const MatrixXd& v) {
    const auto key = reinterpret_cast<std::uintptr_t>(v.data());
    if (_pool.find(key) != _pool.end()) {
      af_array ret;
      af_retain_array(&ret, _pool[key]);
      return af::array(ret);
    }

    af::array va(v.rows(), v.cols(), v.data());
    af_array vap;
    af_retain_array(&vap, va.get());
    _pool.emplace(key, vap);
    return va;
  }

  void set(const VectorXc& v, const af::array& in) {
    const auto key = reinterpret_cast<std::uintptr_t>(v.data());
    af_array p;
    af_retain_array(&p, in.get());
    _pool.insert_or_assign(key, p);
  }

  void set(const MatrixXc& v, const af::array& in) {
    const auto key = reinterpret_cast<std::uintptr_t>(v.data());
    af_array p;
    af_retain_array(&p, in.get());
    _pool.insert_or_assign(key, p);
  }

  void set(const VectorXd& v, const af::array& in) {
    const auto key = reinterpret_cast<std::uintptr_t>(v.data());
    af_array p;
    af_retain_array(&p, in.get());
    _pool.insert_or_assign(key, p);
  }

  void set(const MatrixXd& v, const af::array& in) {
    const auto key = reinterpret_cast<std::uintptr_t>(v.data());
    af_array p;
    af_retain_array(&p, in.get());
    _pool.insert_or_assign(key, p);
  }

  void clear() { _pool.clear(); }

 private:
  std::unordered_map<std::uintptr_t, af_array> _pool;
};

class ArrayFireBackendImpl final : public ArrayFireBackend {
 public:
  explicit ArrayFireBackendImpl(const af::Backend backend) : _backend(backend) {}
  ~ArrayFireBackendImpl() override = default;
  ArrayFireBackendImpl(const ArrayFireBackendImpl& v) = default;
  ArrayFireBackendImpl& operator=(const ArrayFireBackendImpl& obj) = default;
  ArrayFireBackendImpl(ArrayFireBackendImpl&& obj) = default;
  ArrayFireBackendImpl& operator=(ArrayFireBackendImpl&& obj) = default;

  void init() override {
    af::setBackend(_backend);
    _pool.clear();
  }

  void to_host(VectorXc& dst) override {
    const auto dst_arr = _pool.get(dst);
    dst_arr.host(dst.data());
  }
  void to_host(MatrixXc& dst) override {
    const auto dst_arr = _pool.get(dst);
    dst_arr.host(dst.data());
  }
  void to_host(VectorXd& dst) override {
    const auto dst_arr = _pool.get(dst);
    dst_arr.host(dst.data());
  }
  void to_host(MatrixXd& dst) override {
    const auto dst_arr = _pool.get(dst);
    dst_arr.host(dst.data());
  }

  void copy_to(const MatrixXc& src, MatrixXc& dst) override {
    const auto src_arr = _pool.get(src);
    auto dst_arr = _pool.get(dst);
    copy(dst_arr, src_arr, af::span);
  }
  void copy_to(const MatrixXd& src, MatrixXd& dst) override {
    const auto src_arr = _pool.get(src);
    auto dst_arr = _pool.get(dst);
    copy(dst_arr, src_arr, af::span);
  }
  void copy_to(const VectorXd& src, VectorXd& dst) override {
    const auto src_arr = _pool.get(src);
    auto dst_arr = _pool.get(dst);
    copy(dst_arr, src_arr, af::span);
  }
  void copy_to(const VectorXc& src, VectorXc& dst) override {
    const auto src_arr = _pool.get(src);
    auto dst_arr = _pool.get(dst);
    copy(dst_arr, src_arr, af::span);
  }

  void abs(const VectorXc& src, VectorXd& dst) override { _pool.set(dst, af::abs(_pool.get(src))); }
  void abs(const VectorXc& src, VectorXc& dst) override { _pool.set(dst, af::complex(af::abs(_pool.get(src)), af::constant(0.0, src.size(), f64))); }
  void sqrt(const VectorXd& src, VectorXd& dst) override { _pool.set(dst, af::sqrt(_pool.get(src))); }
  void conj(const VectorXc& src, VectorXc& dst) override { _pool.set(dst, conjg(_pool.get(src))); }
  void arg(const VectorXc& src, VectorXc& dst) override {
    _pool.set(dst, af::exp(af::complex(af::constant(0, src.size()), af::arg(_pool.get(src)))));
  }
  void reciprocal(const VectorXc& src, VectorXc& dst) override { _pool.set(dst, af::pow(_pool.get(src), -1)); }
  void exp(const VectorXc& src, VectorXc& dst) override { _pool.set(dst, af::exp(_pool.get(src))); }
  void pow(const VectorXd& src, const double p, VectorXd& dst) override { _pool.set(dst, af::pow(_pool.get(src), p)); }

  void real(const MatrixXc& src, MatrixXd& re) override { _pool.set(re, af::real(_pool.get(src))); }
  void imag(const MatrixXc& src, MatrixXd& im) override { _pool.set(im, af::imag(_pool.get(src))); }

  void make_complex(const VectorXd& re, const VectorXd& im, VectorXc& dst) override { _pool.set(dst, af::complex(_pool.get(re), _pool.get(im))); }

  void create_diagonal(const VectorXc& src, MatrixXc& dst) override {
    const auto d = diag(_pool.get(src), 0, false);
    _pool.set(dst, d);
  }

  void get_diagonal(const MatrixXc& src, VectorXc& dst) override { _pool.set(dst, diag(_pool.get(src))); }
  void get_diagonal(const MatrixXd& src, VectorXd& dst) override { _pool.set(dst, diag(_pool.get(src))); }

  void set(const size_t i, const complex value, VectorXc& dst) override {
    auto d = _pool.get(dst);
    d(static_cast<int>(i)) = af::cdouble{value.real(), value.imag()};
    _pool.set(dst, d);
  }

  void set_row(VectorXc& src, const size_t i, const size_t begin, const size_t end, MatrixXc& dst) override {
    auto d = _pool.get(dst);
    d(static_cast<int>(i), af::seq(static_cast<double>(begin), static_cast<double>(end) - 1)) =
        _pool.get(src)(af::seq(static_cast<double>(begin), static_cast<double>(end) - 1), 0);
    _pool.set(dst, d);
  }

  void set_col(VectorXc& src, const size_t i, const size_t begin, const size_t end, MatrixXc& dst) override {
    auto d = _pool.get(dst);
    d(af::seq(static_cast<double>(begin), static_cast<double>(end) - 1), static_cast<int>(i)) =
        _pool.get(src)(af::seq(static_cast<double>(begin), static_cast<double>(end) - 1), 0);
    _pool.set(dst, d);
  }

  void get_col(const MatrixXc& src, const size_t i, VectorXc& dst) override { _pool.set(dst, _pool.get(src).col(static_cast<int>(i))); }

  complex max_abs_element(const VectorXc& src) override {
    af::cdouble v{};
    (af::max)(_pool.get(src)).host(&v);
    return {v.real, v.imag};
  }

  double max_element(const VectorXd& src) override {
    double v{};
    (af::max)(_pool.get(src)).host(&v);
    return std::abs(v);
  }

  void scale(const complex value, VectorXc& dst) override { _pool.set(dst, _pool.get(dst) * af::cdouble(value.real(), value.imag())); }

  void scale(const double value, VectorXd& dst) override { _pool.set(dst, _pool.get(dst) * value); }

  complex dot(const VectorXc& a, const VectorXc& b) override {
    complex v{};
    const auto r = af::dot(_pool.get(a), _pool.get(b), AF_MAT_CONJ);
    r.host(&v);
    return v;
  }

  double dot(const VectorXd& a, const VectorXd& b) override {
    double v{};
    const auto r = af::dot(_pool.get(a), _pool.get(b));
    r.host(&v);
    return v;
  }

  void add(const double alpha, const MatrixXd& a, MatrixXd& b) override {
    const auto aa = _pool.get(a);
    const auto ba = _pool.get(b);
    _pool.set(b, alpha * aa + ba);
  }

  void add(const double alpha, const VectorXd& a, VectorXd& b) override {
    const auto aa = _pool.get(a);
    const auto ba = _pool.get(b);
    _pool.set(b, alpha * aa + ba);
  }

  void add(const complex alpha, const MatrixXc& a, MatrixXc& b) override {
    const auto aa = _pool.get(a);
    const auto ba = _pool.get(b);
    _pool.set(b, af::cdouble(alpha.real(), alpha.imag()) * aa + ba);
  }

  void add(const complex alpha, const VectorXc& a, VectorXc& b) override {
    const auto aa = _pool.get(a);
    const auto ba = _pool.get(b);
    _pool.set(b, af::cdouble(alpha.real(), alpha.imag()) * aa + ba);
  }

  void mul(const TRANSPOSE trans_a, const TRANSPOSE trans_b, const complex alpha, const MatrixXc& a, const MatrixXc& b, const complex beta,
           MatrixXc& c) override {
    const auto aa = _pool.get(a);
    const auto ba = _pool.get(b);
    auto ca = _pool.get(c);
    ca *= af::cdouble(beta.real(), beta.imag());
    switch (trans_a) {
      case TRANSPOSE::CONJ_TRANS:
        switch (trans_b) {
          case TRANSPOSE::CONJ_TRANS:
            ca += af::cdouble(alpha.real(), alpha.imag()) * af::matmul(aa, ba, AF_MAT_CTRANS, AF_MAT_CTRANS);
            break;
          case TRANSPOSE::TRANS:
            ca += af::cdouble(alpha.real(), alpha.imag()) * matmul(aa, ba, AF_MAT_CTRANS, AF_MAT_TRANS);
            break;
          case TRANSPOSE::NO_TRANS:
            ca += af::cdouble(alpha.real(), alpha.imag()) * matmul(aa, ba, AF_MAT_CTRANS, AF_MAT_NONE);
            break;
        }
        break;
      case TRANSPOSE::TRANS:
        switch (trans_b) {
          case TRANSPOSE::CONJ_TRANS:
            ca += af::cdouble(alpha.real(), alpha.imag()) * matmul(aa, ba, AF_MAT_TRANS, AF_MAT_CTRANS);
            break;
          case TRANSPOSE::TRANS:
            ca += af::cdouble(alpha.real(), alpha.imag()) * matmul(aa, ba, AF_MAT_TRANS, AF_MAT_TRANS);
            break;
          case TRANSPOSE::NO_TRANS:
            ca += af::cdouble(alpha.real(), alpha.imag()) * matmul(aa, ba, AF_MAT_TRANS, AF_MAT_NONE);
            break;
        }
        break;
      case TRANSPOSE::NO_TRANS:
        switch (trans_b) {
          case TRANSPOSE::CONJ_TRANS:
            ca += af::cdouble(alpha.real(), alpha.imag()) * matmul(aa, ba, AF_MAT_NONE, AF_MAT_CTRANS);
            break;
          case TRANSPOSE::TRANS:
            ca += af::cdouble(alpha.real(), alpha.imag()) * matmul(aa, ba, AF_MAT_NONE, AF_MAT_TRANS);
            break;
          case TRANSPOSE::NO_TRANS:
            ca += af::cdouble(alpha.real(), alpha.imag()) * matmul(aa, ba, AF_MAT_NONE, AF_MAT_NONE);
            break;
        }
        break;
    }
    _pool.set(c, ca);
  }

  void mul(const TRANSPOSE trans_a, const complex alpha, const MatrixXc& a, const VectorXc& b, const complex beta, VectorXc& c) override {
    const auto aa = _pool.get(a);
    const auto ba = _pool.get(b);
    auto ca = _pool.get(c);
    ca *= af::cdouble(beta.real(), beta.imag());
    switch (trans_a) {
      case TRANSPOSE::CONJ_TRANS:
        ca += af::cdouble(alpha.real(), alpha.imag()) * matmul(aa, ba, AF_MAT_CTRANS, AF_MAT_NONE);
        break;
      case TRANSPOSE::TRANS:
        ca += af::cdouble(alpha.real(), alpha.imag()) * matmul(aa, ba, AF_MAT_TRANS, AF_MAT_NONE);
        break;
      case TRANSPOSE::NO_TRANS:
        ca += af::cdouble(alpha.real(), alpha.imag()) * matmul(aa, ba, AF_MAT_NONE, AF_MAT_NONE);
        break;
    }
    _pool.set(c, ca);
  }

  void mul(const TRANSPOSE trans_a, const TRANSPOSE trans_b, const double alpha, const MatrixXd& a, const MatrixXd& b, const double beta,
           MatrixXd& c) override {
    const auto aa = _pool.get(a);
    const auto ba = _pool.get(b);
    auto ca = _pool.get(c);
    ca *= beta;
    switch (trans_a) {
      case TRANSPOSE::CONJ_TRANS:
        switch (trans_b) {
          case TRANSPOSE::CONJ_TRANS:
            ca += alpha * af::matmul(aa, ba, AF_MAT_CTRANS, AF_MAT_CTRANS);
            break;
          case TRANSPOSE::TRANS:
            ca += alpha * matmul(aa, ba, AF_MAT_CTRANS, AF_MAT_TRANS);
            break;
          case TRANSPOSE::NO_TRANS:
            ca += alpha * matmul(aa, ba, AF_MAT_CTRANS, AF_MAT_NONE);
            break;
        }
        break;
      case TRANSPOSE::TRANS:
        switch (trans_b) {
          case TRANSPOSE::CONJ_TRANS:
            ca += alpha * matmul(aa, ba, AF_MAT_TRANS, AF_MAT_CTRANS);
            break;
          case TRANSPOSE::TRANS:
            ca += alpha * matmul(aa, ba, AF_MAT_TRANS, AF_MAT_TRANS);
            break;
          case TRANSPOSE::NO_TRANS:
            ca += alpha * matmul(aa, ba, AF_MAT_TRANS, AF_MAT_NONE);
            break;
        }
        break;
      case TRANSPOSE::NO_TRANS:
        switch (trans_b) {
          case TRANSPOSE::CONJ_TRANS:
            ca += alpha * matmul(aa, ba, AF_MAT_NONE, AF_MAT_CTRANS);
            break;
          case TRANSPOSE::TRANS:
            ca += alpha * matmul(aa, ba, AF_MAT_NONE, AF_MAT_TRANS);
            break;
          case TRANSPOSE::NO_TRANS:
            ca += alpha * matmul(aa, ba, AF_MAT_NONE, AF_MAT_NONE);
            break;
        }
        break;
    }
    _pool.set(c, ca);
  }
  void mul(const TRANSPOSE trans_a, const double alpha, const MatrixXd& a, const VectorXd& b, const double beta, VectorXd& c) override {
    const auto aa = _pool.get(a);
    const auto ba = _pool.get(b);
    auto ca = _pool.get(c);
    ca *= beta;
    switch (trans_a) {
      case TRANSPOSE::CONJ_TRANS:

        ca += alpha * matmul(aa, ba, AF_MAT_CTRANS, AF_MAT_NONE);
        break;
      case TRANSPOSE::TRANS:
        ca += alpha * matmul(aa, ba, AF_MAT_TRANS, AF_MAT_NONE);
        break;
      case TRANSPOSE::NO_TRANS:
        ca += alpha * matmul(aa, ba, AF_MAT_NONE, AF_MAT_NONE);
        break;
    }
    _pool.set(c, ca);
  }

  void hadamard_product(const VectorXc& a, const VectorXc& b, VectorXc& c) override { _pool.set(c, _pool.get(a) * _pool.get(b)); }

  void hadamard_product(const MatrixXc& a, const MatrixXc& b, MatrixXc& c) override { _pool.set(c, _pool.get(a) * _pool.get(b)); }

  void concat_col(const MatrixXc& a, const MatrixXc& b, MatrixXc& dst) override { _pool.set(dst, af::join(1, _pool.get(a), _pool.get(b))); }
  void concat_row(const MatrixXc& a, const MatrixXc& b, MatrixXc& dst) override { _pool.set(dst, af::join(0, _pool.get(a), _pool.get(b))); }
  void concat_row(const VectorXc& a, const VectorXc& b, VectorXc& dst) override { _pool.set(dst, af::join(0, _pool.get(a), _pool.get(b))); }

  void max_eigen_vector(MatrixXc& src, VectorXc& dst) override {
    Eigen::Matrix<complex, -1, -1, Eigen::ColMajor> data(src.rows(), src.cols());
    _pool.get(src).host(data.data());
    const Eigen::ComplexEigenSolver<Eigen::Matrix<complex, -1, -1, Eigen::ColMajor>> ces(data);
    auto idx = 0;
    ces.eigenvalues().cwiseAbs2().maxCoeff(&idx);
    dst = ces.eigenvectors().col(idx);
  }

  void pseudo_inverse_svd(MatrixXc& src, const double alpha, MatrixXc& u, MatrixXc&, MatrixXc& vt, MatrixXc&, MatrixXc& dst) override {
    const auto srca = _pool.get(src);
    auto ua = _pool.get(u);
    auto vta = _pool.get(vt);
    const auto m = static_cast<dim_t>(src.rows());
    const auto n = static_cast<dim_t>(src.cols());
    af::array s_vec;
    svd(ua, s_vec, vta, srca);
    s_vec = s_vec / (s_vec * s_vec + af::constant(alpha, s_vec.dims(0), af::dtype::f64));
    const af::array s_mat = diag(s_vec, 0, false);
    const af::array zero = af::constant(0.0, n - m, m, af::dtype::f64);
    const auto sa = af::complex(join(0, s_mat, zero), 0);
    const auto bufa = matmul(sa, ua, AF_MAT_NONE, AF_MAT_CTRANS);
    _pool.set(dst, matmul(vta, bufa, AF_MAT_CTRANS, AF_MAT_NONE));
  }

  void pseudo_inverse_svd(MatrixXd& src, const double alpha, MatrixXd& u, MatrixXd&, MatrixXd& vt, MatrixXd&, MatrixXd& dst) override {
    const auto srca = _pool.get(src);
    auto ua = _pool.get(u);
    auto vta = _pool.get(vt);
    const auto m = src.rows();
    const auto n = src.cols();
    af::array s_vec;
    svd(ua, s_vec, vta, srca);
    s_vec = s_vec / (s_vec * s_vec + af::constant(alpha, s_vec.dims(0), af::dtype::f64));
    const af::array s_mat = diag(s_vec, 0, false);
    const af::array zero = af::constant(0.0, n - m, m, af::dtype::f64);
    const auto sa = join(0, s_mat, zero);
    const auto bufa = matmul(sa, ua, AF_MAT_NONE, AF_MAT_TRANS);
    _pool.set(dst, matmul(vta, bufa, AF_MAT_TRANS, AF_MAT_NONE));
  }

  void solvet(MatrixXd& a, VectorXd& b) override { _pool.set(b, af::solve(_pool.get(a), _pool.get(b))); }

  void solveh(MatrixXc& a, VectorXc& b) override { _pool.set(b, af::solve(_pool.get(a), _pool.get(b))); }

  void reduce_col(const MatrixXd& a, VectorXd& b) override { _pool.set(b, af::sum(_pool.get(a), 1)); }

 private:
  af::Backend _backend;
  BufferPool _pool;
};

BackendPtr ArrayFireBackend::create(const af::Backend backend) { return std::make_shared<ArrayFireBackendImpl>(backend); }

}  // namespace autd3::gain::holo
