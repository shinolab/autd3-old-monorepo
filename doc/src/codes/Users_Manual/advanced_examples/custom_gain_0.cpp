#include "autd3.hpp"

class FocalPoint final : public autd3::Gain {
 public:
  explicit FocalPoint(autd3::Vector3 point) : _point(std::move(point)) {}

  std::unordered_map<size_t, std::vector<autd3::Drive>> calc(
      const autd3::Geometry& geometry) const override {
    return autd3::Gain::transform(
        geometry, [&](const auto& dev, const auto& tr) {
          const auto phase = (tr.position() - _point).norm() *
                             tr.wavelength(dev.sound_speed());
          return autd3::Drive{autd3::Phase::from_rad(phase),
                              autd3::EmitIntensity::maximum()};
        });
  }

 private:
  autd3::Vector3 _point;
};