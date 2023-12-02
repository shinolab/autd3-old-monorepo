#include <ranges>

#include "autd3.hpp"

using namespace std::ranges::views;

const autd3::Vector3 center =
    autd.geometry().center() + autd3::Vector3(0, 0, 150);
const auto points_num = 200;
const auto radius = 30.0;
auto stm = autd3::GainSTM(1).add_gains_from_iter(
    iota(0) | take(points_num) | transform([&](auto i) {
      const auto theta = 2.0 * autd3::pi * static_cast<double>(i) /
                         static_cast<double>(points_num);
      return autd3::gain::Focus(center +
                                autd3::Vector3(radius * std::cos(theta),
                                               radius * std::sin(theta), 0));
    }));