// File: backend.hpp
// Project: gain
// Created Date: 10/09/2021
// Author: Shun Suzuki
// -----
// Last Modified: 13/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2021 Hapis Lab. All rights reserved.
//

#pragma once

#include <memory>

#if _MSC_VER
#pragma warning(push)
#pragma warning( \
    disable : 4068 6031 6255 6294 26408 26450 26426 26429 26432 26434 26440 26446 26447 26451 26454 26455 26461 26462 26471 26472 26474 26475 26495 26481 26482 26485 26490 26491 26493 26494 26496 26497 26812 26813 26814)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmaybe-uninitialized"
#pragma GCC diagnostic ignored "-Wclass-memaccess"
#endif
#include <Eigen/Dense>
#if _MSC_VER
#pragma warning(pop)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic pop
#endif

namespace autd3::gain::holo {

enum class TRANSPOSE { NO_TRANS = 111, TRANS = 112, CONJ_TRANS = 113 };

using complex = std::complex<double>;

constexpr complex ONE = complex(1.0, 0.0);
constexpr complex ZERO = complex(0.0, 0.0);

using VectorXd = Eigen::Matrix<double, -1, 1>;
using VectorXc = Eigen::Matrix<complex, -1, 1>;
using MatrixXd = Eigen::Matrix<double, -1, -1>;
using MatrixXc = Eigen::Matrix<complex, -1, -1>;

class Backend;
using BackendPtr = std::shared_ptr<Backend>;

/**
 * \brief Backend for HoloGain
 */
class Backend {
 public:
  Backend() = default;
  virtual ~Backend() = default;
  Backend(const Backend& v) noexcept = default;
  Backend& operator=(const Backend& obj) = default;
  Backend(Backend&& obj) = default;
  Backend& operator=(Backend&& obj) = default;

  virtual void make_complex(const VectorXd& r, const VectorXd& i, VectorXc& c) = 0;
  virtual void make_complex(const MatrixXd& r, const MatrixXd& i, MatrixXc& c) = 0;
};

/**
 * \brief Backend for HoloGain
 */
class EigenBackend final : public Backend {
 public:
  EigenBackend() = default;
  ~EigenBackend() override = default;
  EigenBackend(const EigenBackend& v) = default;
  EigenBackend& operator=(const EigenBackend& obj) = default;
  EigenBackend(EigenBackend&& obj) = default;
  EigenBackend& operator=(EigenBackend&& obj) = default;

  void make_complex(const VectorXd& r, const VectorXd& i, VectorXc& c) override;
  void make_complex(const MatrixXd& r, const MatrixXd& i, MatrixXc& c) override;

  static BackendPtr create() { return std::make_shared<EigenBackend>(); }
};

}  // namespace autd3::gain::holo
