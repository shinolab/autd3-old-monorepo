// File: AUTD3.h
// Project: src
// Created Date: 13/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 13/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#ifndef _AUTD3_H_
#define _AUTD3_H_

#include <inttypes.h>

class AUTD3 {
 public:
  AUTD3();

  int open();
  void run();

  void set_gain(char* gain);
  void set_modulation(char* mod, int size);
  void stop();

 private:
  char get_msg_id();

  char _msg_id;
  int64_t _toff;
  int64_t _ts;
  int _expected_wkc;
};

#endif
