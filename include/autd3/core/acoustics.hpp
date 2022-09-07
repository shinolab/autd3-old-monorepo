// File: acoustics.hpp
// Project: core
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 07/09/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cmath>
#include <complex>
#include <type_traits>

#include "autd3/core/transducer.hpp"
#include "autd3/driver/hardware.hpp"

namespace autd3::core {

constexpr double DIR_COEFFICIENT_A[] = {1.0, 1.0, 1.0, 0.891250938, 0.707945784, 0.501187234, 0.354813389, 0.251188643, 0.199526231};
constexpr double DIR_COEFFICIENT_B[] = {
    0., 0., -0.00459648054721, -0.0155520765675, -0.0208114779827, -0.0182211227016, -0.0122437497109, -0.00780345575475, -0.00312857467007};
constexpr double DIR_COEFFICIENT_C[] = {
    0., 0., -0.000787968093807, -0.000307591508224, -0.000218348633296, 0.00047738416141, 0.000120353137658, 0.000323676257958, 0.000143850511};
constexpr double DIR_COEFFICIENT_D[] = {
    0., 0., 1.60125528528e-05, 2.9747624976e-06, 2.31910931569e-05, -1.1901034125e-05, 6.77743734332e-06, -5.99548024824e-06, -4.79372835035e-06};

/**
 * \brief Utility class to calculate directivity of ultrasound transducer.
 */
class Directivity {
 public:
  /**
   * \brief Directivity of T4010A1
   * \param theta_deg zenith angle in degree
   * \return directivity
   */
  static double t4010a1(double theta_deg) {
    theta_deg = std::abs(theta_deg);
    while (theta_deg > 90) theta_deg = std::abs(180 - theta_deg);
    const auto i = static_cast<size_t>(std::ceil(theta_deg / 10));
    if (i == 0) return 1;
    const auto a = DIR_COEFFICIENT_A[i - 1];
    const auto b = DIR_COEFFICIENT_B[i - 1];
    const auto c = DIR_COEFFICIENT_C[i - 1];
    const auto d = DIR_COEFFICIENT_D[i - 1];
    const auto x = theta_deg - static_cast<double>(i - 1) * 10;
    return a + b * x + c * x * x + d * x * x * x;
  }
};

inline std::complex<double> propagate(const core::Vector3& source_pos, const core::Vector3& source_dir, const double attenuation,
                                      const double wavenumber, const core::Vector3& target) {
  const auto diff = target - source_pos;
  const auto dist = diff.norm();

  const auto theta = std::atan2(source_dir.cross(diff).norm(), source_dir.dot(diff)) * 180.0 / driver::pi;

  const auto d = Directivity::t4010a1(theta);
  const auto r = d * std::exp(-dist * attenuation) / dist;
  const auto phi = -wavenumber * dist;
  return r * std::exp(std::complex<double>(0., phi));
}

}  // namespace autd3::core
