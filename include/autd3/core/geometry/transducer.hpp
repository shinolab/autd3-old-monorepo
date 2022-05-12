// File: transducer.hpp
// Project: geometry
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 12/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
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
#endif
#include <Eigen/Dense>
#if _MSC_VER
#pragma warning(pop)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic pop
#endif

namespace autd3::core {

using Vector3 = Eigen::Matrix<double, 3, 1>;
using Vector4 = Eigen::Matrix<double, 4, 1>;
using Matrix4X4 = Eigen::Matrix<double, 4, 4>;
using Quaternion = Eigen::Quaternion<double>;

template <typename T>
struct DriveData {
  DriveData() = default;
  virtual ~DriveData() = default;
  DriveData(const DriveData& v) noexcept = default;
  DriveData& operator=(const DriveData& obj) = default;
  DriveData(DriveData&& obj) = default;
  DriveData& operator=(DriveData&& obj) = default;

  virtual void init(size_t size) = 0;
  virtual void set_drive(const T& tr, double phase, double amp) = 0;
  virtual void copy_from(size_t idx, const typename T::D& src) = 0;
};

template <typename T>
struct Transducer {
  using D = T;

  Transducer(const size_t id, Vector3 pos, Vector3 x_direction, Vector3 y_direction, Vector3 z_direction) noexcept
      : _id(id),
        _pos(std::move(pos)),
        _x_direction(std::move(x_direction)),
        _y_direction(std::move(y_direction)),
        _z_direction(std::move(z_direction)) {}
  virtual ~Transducer() = default;
  Transducer(const Transducer& v) noexcept = default;
  Transducer& operator=(const Transducer& obj) = default;
  Transducer(Transducer&& obj) = default;
  Transducer& operator=(Transducer&& obj) = default;

  [[nodiscard]] double align_phase_at(const double dist, const double sound_speed) const { return dist / wavelength(sound_speed); }
  [[nodiscard]] const Vector3& position() const noexcept { return _pos; }
  [[nodiscard]] size_t id() const noexcept { return _id; }
  [[nodiscard]] const Vector3& x_direction() const noexcept { return _x_direction; }
  [[nodiscard]] const Vector3& y_direction() const noexcept { return _y_direction; }
  [[nodiscard]] const Vector3& z_direction() const noexcept { return _z_direction; }

  [[nodiscard]] virtual uint16_t cycle() const = 0;
  [[nodiscard]] virtual double frequency() const = 0;
  [[nodiscard]] virtual double wavelength(double sound_speed) const = 0;
  [[nodiscard]] virtual double wavenumber(double sound_speed) const = 0;

 private:
  size_t _id;
  Vector3 _pos;
  Vector3 _x_direction;
  Vector3 _y_direction;
  Vector3 _z_direction;
};

}  // namespace autd3::core
