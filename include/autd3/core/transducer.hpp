// File: transducer.hpp
// Project: geometry
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 07/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <utility>

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

#include "autd3/driver/fpga/defined.hpp"

namespace autd3::core {

using Vector3 = Eigen::Matrix<double, 3, 1>;
using Vector4 = Eigen::Matrix<double, 4, 1>;
using Matrix4X4 = Eigen::Matrix<double, 4, 4>;
using Quaternion = Eigen::Quaternion<double>;

/**
 * \brief Transduce contains a position and id, direction, frequency of a transducer
 */
struct Transducer {
  Transducer(const size_t id, Vector3 pos, Vector3 x_direction, Vector3 y_direction, Vector3 z_direction) noexcept
      : _id(id),
        _pos(std::move(pos)),
        _x_direction(std::move(x_direction)),
        _y_direction(std::move(y_direction)),
        _z_direction(std::move(z_direction)),
        _mod_delay(0),
        _cycle(4096) {}
  ~Transducer() = default;
  Transducer(const Transducer& v) noexcept = default;
  Transducer& operator=(const Transducer& obj) = default;
  Transducer(Transducer&& obj) = default;
  Transducer& operator=(Transducer&& obj) = default;

  [[nodiscard]] double align_phase_at(const double dist, const double sound_speed) const { return dist * wavenumber(sound_speed); }

  /**
   * \brief Position of the transducer
   */
  [[nodiscard]] const Vector3& position() const noexcept { return _pos; }
  /**
   * \brief ID of the transducer
   */
  [[nodiscard]] size_t id() const noexcept { return _id; }
  /**
   * \brief x direction of the transducer
   */
  [[nodiscard]] const Vector3& x_direction() const noexcept { return _x_direction; }
  /**
   * \brief y direction of the transducer
   */
  [[nodiscard]] const Vector3& y_direction() const noexcept { return _y_direction; }
  /**
   * \brief z direction of the transducer
   */
  [[nodiscard]] const Vector3& z_direction() const noexcept { return _z_direction; }
  /**
   * \brief modulation delay of the transducer
   */
  [[nodiscard]] uint16_t mod_delay() const noexcept { return _mod_delay; }
  /**
   * \brief modulation delay of the transducer
   */
  uint16_t& mod_delay() noexcept { return _mod_delay; }
  /**
   * \brief Frequency division ratio. The frequency will be autd3::driver::FPGA_CLK_FREQ/cycle.
   */
  [[nodiscard]] uint16_t cycle() const { return _cycle; }
  /**
   * \brief Frequency of the transducer
   */
  [[nodiscard]] double frequency() const { return driver::FPGA_CLK_FREQ / static_cast<double>(_cycle); }

  /**
   * \brief Set fFrequency division ratio. The frequency will be autd3::driver::FPGA_CLK_FREQ/cycle.
   * \details This has no effect in LegacyMode.
   */
  void set_cycle(const uint16_t cycle) noexcept { _cycle = cycle; }

  /**
   * \brief Set fFrequency of the transducer.
   * \details This has no effect in LegacyMode.
   */
  void set_frequency(const double freq) noexcept {
    const auto cycle = static_cast<uint16_t>(std::round(static_cast<double>(driver::FPGA_CLK_FREQ) / freq));
    set_cycle(cycle);
  }

  /**
   * \brief Wavelength of the ultrasound emitted from the transducer
   * @param sound_speed Speed of sound
   */
  [[nodiscard]] double wavelength(double sound_speed) const { return sound_speed / frequency(); }
  /**
   * \brief Wavenumber of the ultrasound emitted from the transducer
   * @param sound_speed Speed of sound
   */
  [[nodiscard]] double wavenumber(double sound_speed) const { return 2.0 * driver::pi * frequency() / sound_speed; }

 private:
  size_t _id;
  Vector3 _pos;
  Vector3 _x_direction;
  Vector3 _y_direction;
  Vector3 _z_direction;
  uint16_t _mod_delay;
  uint16_t _cycle;
};

}  // namespace autd3::core
