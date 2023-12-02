#include "autd3.hpp"

class BurstModulation final : public autd3::Modulation {
 public:
  std::vector<autd3::EmitIntensity> calc() const override {
    std::vector buffer(_buf_size, autd3::EmitIntensity::minimum());
    buffer[_buf_size - 1] = autd3::EmitIntensity::maximum();
    return buffer;
  }

  explicit BurstModulation(const size_t buf_size = 4000) noexcept
      : autd3::Modulation(autd3::SamplingConfiguration::from_frequency(4e3)), _buf_size(buf_size) {}

 private:
  size_t _buf_size;
};