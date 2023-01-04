// File: gain.cpp
// Project: stm
// Created Date: 04/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 04/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include "autd3/core/stm/gain.hpp"

#include "../../spdlog.hpp"
#include "autd3/driver/driver.hpp"

namespace autd3::core {

bool GainSTM::pack(const Mode mode, const Geometry& geometry, driver::TxDatagram& tx) {
  if (mode == Mode::Legacy)
    driver::GainSTMHeader<driver::Legacy>().pack(tx);
  else
    driver::GainSTMHeader<driver::Normal>().pack(tx);

  if (is_finished()) return true;

  switch (mode) {
    case Mode::Legacy:
      return driver::GainSTMBody<driver::Legacy>()
          .drives(_gains)
          .sent(&_sent)
          .freq_div(_freq_div)
          .mode(_mode)
          .start_idx(start_idx)
          .finish_idx(finish_idx)
          .pack(tx);
    case Mode::Normal:
      if (_sent == 0)
        return driver::GainSTMBody<driver::NormalPhase>()
            .drives(_gains)
            .cycles(geometry.cycles())
            .sent(&_sent)
            .freq_div(_freq_div)
            .mode(_mode)
            .start_idx(start_idx)
            .finish_idx(finish_idx)
            .pack(tx);
      switch (_mode) {
        case driver::GainSTMMode::PhaseDutyFull:
          _next_duty = !_next_duty;
          return _next_duty ? driver::GainSTMBody<driver::NormalPhase>()
                                  .drives(_gains)
                                  .cycles(geometry.cycles())
                                  .sent(&_sent)
                                  .freq_div(_freq_div)
                                  .mode(_mode)
                                  .start_idx(start_idx)
                                  .finish_idx(finish_idx)
                                  .pack(tx)
                            : driver::GainSTMBody<driver::NormalDuty>()
                                  .drives(_gains)
                                  .cycles(geometry.cycles())
                                  .sent(&_sent)
                                  .freq_div(_freq_div)
                                  .mode(_mode)
                                  .start_idx(start_idx)
                                  .finish_idx(finish_idx)
                                  .pack(tx);
        case driver::GainSTMMode::PhaseFull:
          return driver::GainSTMBody<driver::NormalPhase>()
              .drives(_gains)
              .cycles(geometry.cycles())
              .sent(&_sent)
              .freq_div(_freq_div)
              .mode(_mode)
              .start_idx(start_idx)
              .finish_idx(finish_idx)
              .pack(tx);
        case driver::GainSTMMode::PhaseHalf:
          spdlog::error("This mode is not supported");
          return false;
      }
      throw std::runtime_error("Unreachable!");
    case Mode::NormalPhase:
      return driver::GainSTMBody<driver::NormalPhase>()
          .drives(_gains)
          .cycles(geometry.cycles())
          .sent(&_sent)
          .freq_div(_freq_div)
          .mode(_mode)
          .start_idx(start_idx)
          .finish_idx(finish_idx)
          .pack(tx);
  }

  throw std::runtime_error("Unreachable!");
}
}  // namespace autd3::core
