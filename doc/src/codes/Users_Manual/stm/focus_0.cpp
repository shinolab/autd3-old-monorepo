#include <ranges>

#include "autd3.hpp"

using namespace std::ranges::views;

const autd3::Vector3 center =
    autd.geometry().center() + autd3::Vector3(0, 0, 150);
const auto points_num = 200;
const auto radius = 30.0;
auto stm = autd3::FocusSTM(1).add_foci_from_iter(
    iota(0) | take(points_num) | transform([&](auto i) {
      const auto theta = 2.0 * autd3::pi * static_cast<double>(i) /
                         static_cast<double>(points_num);
      autd3::Vector3 p = center + autd3::Vector3(radius * std::cos(theta),
                                                 radius * std::sin(theta), 0);
      return p;
    }));
