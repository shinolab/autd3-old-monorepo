// File: error_handler.hpp
// Project: ecat_thread
// Created Date: 12/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 27/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <string>

namespace autd3::link {

void check_state(const uint16_t slave, std::stringstream& ss) {
  if (ec_slave[slave].state == EC_STATE_OPERATIONAL) return;

  ec_group[0].docheckstate = 1;
  if (ec_slave[slave].state == EC_STATE_SAFE_OP + EC_STATE_ERROR) {
    ss << "ERROR : slave " << slave << " is in SAFE_OP + ERROR, attempting ack\n";
    ec_slave[slave].state = EC_STATE_SAFE_OP + EC_STATE_ACK;
    ec_writestate(slave);
  } else if (ec_slave[slave].state == EC_STATE_SAFE_OP) {
    ss << "WARNING : slave " << slave << " is in SAFE_OP, change to OPERATIONAL\n";
    ec_slave[slave].state = EC_STATE_OPERATIONAL;
    ec_writestate(slave);
  } else if (ec_slave[slave].state > EC_STATE_NONE) {
    if (ec_reconfig_slave(slave, 500)) {
      ec_slave[slave].islost = 0;
      ss << "MESSAGE : slave " << slave << " reconfigured\n";
    }
  } else if (!ec_slave[slave].islost) {
    ec_statecheck(slave, EC_STATE_OPERATIONAL, EC_TIMEOUTRET);
    if (ec_slave[slave].state == EC_STATE_NONE) {
      ec_slave[slave].islost = 1;
      ss << "ERROR : slave " << slave << " lost\n";
    }
  }
}

void check_lost(uint16_t slave, std::stringstream& ss) {
  if (ec_slave[slave].islost == 0) return;
  if (ec_slave[slave].state == EC_STATE_NONE) {
    if (ec_recover_slave(slave, 500)) {
      ec_slave[slave].islost = 0;
      ss << "MESSAGE : slave " << slave << " recovered\n";
    }
  } else {
    ec_slave[slave].islost = 0;
    ss << "MESSAGE : slave " << slave << " found\n";
  }
}

bool error_handle(std::atomic<bool>* is_open, std::function<void(std::string)>& on_lost) {
  ec_group[0].docheckstate = 0;
  ec_readstate();
  std::stringstream ss;
  for (uint16_t slave = 1; slave <= static_cast<uint16_t>(ec_slavecount); slave++) {
    check_state(slave, ss);
    check_lost(slave, ss);
  }
  if (ec_group[0].docheckstate == 0) return true;

  for (uint16_t slave = 1; slave <= static_cast<uint16_t>(ec_slavecount); slave++) {
    if (ec_slave[slave].islost == 0) continue;
    is_open->store(false);
    if (on_lost != nullptr) on_lost(ss.str());
    return false;
  }
  return true;
}

}  // namespace autd3::link
