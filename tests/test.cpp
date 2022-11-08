// File: test.cpp
// Project: tests
// Created Date: 14/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 08/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#if _MSC_VER
#pragma warning(push)
#pragma warning(disable : 26439 26495 26812)
#endif
#include <gtest/gtest.h>
#if _MSC_VER
#pragma warning(pop)
#endif

#include <autd3.hpp>

#include "null_link.hpp"

TEST(ControllerTest, stream) {
  using autd3::clear;
  using autd3::gain::Null;
  using autd3::modulation::Sine;

  autd3::Controller autd;

  autd.geometry().add_device(autd3::Vector3::Zero(), autd3::Vector3::Zero());
  autd.open(autd3::test::NullLink().build());

  autd << clear;
  autd << Sine(150);
  autd << Null();
  autd << Sine(150), Null();

  autd << clear << clear                             //
       << clear << Sine(150)                         //
       << clear << Null();                           //
  autd << clear << Sine(150), Null();                //
  autd << Sine(150) << clear                         //
       << Sine(150) << Sine(150)                     //
       << Sine(150) << Null();                       //
  autd << Sine(150) << Sine(150), Null();            //
  autd << Null() << clear                            //
       << Null() << Sine(150)                        //
       << Null() << Null();                          //
  autd << Null() << (Sine(150), Null());             //
  autd << (Sine(150), Null()) << clear               //
       << (Sine(150), Null()) << Sine(150)           //
       << (Sine(150), Null()) << Null();             //
  autd << (Sine(150), Null()) << Sine(150), Null();  //

  auto s = Sine(150);
  auto n = Null();

  autd << s;
  autd << n;
  autd << s, n;

  autd << clear << clear   //
       << clear << s       //
       << clear << n;      //
  autd << clear << s, n;   //
  autd << s << clear       //
       << s << s           //
       << s << n;          //
  autd << s << s, n;       //
  autd << n << clear       //
       << n << s           //
       << n << n;          //
  autd << n << (s, n);     //
  autd << (s, n) << clear  //
       << (s, n) << s      //
       << (s, n) << n;     //
  autd << (s, n) << s, n;  //
}

TEST(ControllerTest, stream_async) {
  using autd3::async;
  using autd3::clear;
  using autd3::gain::Null;
  using autd3::modulation::Sine;

  autd3::Controller autd;

  autd.geometry().add_device(autd3::Vector3::Zero(), autd3::Vector3::Zero());
  autd.open(autd3::test::NullLink().build());

  autd << async << clear;
  autd << async << Sine(150);
  autd << async << Null();
  autd << async << Sine(150), Null();

  autd << async                                               //
       << clear << clear                                      //
       << clear << Sine(150)                                  //
       << clear << Null();                                    //
  autd << async << clear << Sine(150), Null();                //
  autd << async                                               //
       << Sine(150) << clear                                  //
       << Sine(150) << Sine(150)                              //
       << Sine(150) << Null();                                //
  autd << async << Sine(150) << Sine(150), Null();            //
  autd << async                                               //
       << Null() << clear                                     //
       << Null() << Sine(150)                                 //
       << Null() << Null();                                   //
  autd << async << Null() << (Sine(150), Null());             //
  autd << async                                               //
       << (Sine(150), Null()) << clear                        //
       << (Sine(150), Null()) << Sine(150)                    //
       << (Sine(150), Null()) << Null();                      //
  autd << async << (Sine(150), Null()) << Sine(150), Null();  //

  auto s = Sine(150);
  auto n = Null();

  autd << async << std::move(s);
  autd << async << std::move(n);

  s = Sine(150);
  n = Null();
  autd << async << std::move(s), std::move(n);

  {
    auto s1 = Sine(150);
    auto s2 = Sine(150);
    auto n1 = Null();
    auto n2 = Null();
    autd << async                                            //
         << clear << clear                                   //
         << clear << std::move(s1)                           //
         << clear << std::move(n1);                          //
    autd << async << clear << std::move(s2), std::move(n2);  //
  }

  {
    auto s1 = Sine(150);
    auto s2 = Sine(150);
    auto s3 = Sine(150);
    auto s4 = Sine(150);
    auto s5 = Sine(150);
    auto s6 = Sine(150);
    auto n1 = Null();
    auto n2 = Null();
    autd << async                                                    //
         << std::move(s1) << clear                                   //
         << std::move(s2) << std::move(s3)                           //
         << std::move(s4) << std::move(n1);                          //
    autd << async << std::move(s5) << std::move(s6), std::move(n2);  //
  }

  {
    auto s1 = Sine(150);
    auto s2 = Sine(150);
    auto n1 = Null();
    auto n2 = Null();
    auto n3 = Null();
    auto n4 = Null();
    auto n5 = Null();
    auto n6 = Null();
    autd << async                                                      //
         << std::move(n1) << clear                                     //
         << std::move(n2) << std::move(s1)                             //
         << std::move(n3) << std::move(n4);                            //
    autd << async << std::move(n5) << (std::move(s2), std::move(n6));  //
  }

  {
    auto s1 = Sine(150);
    auto s2 = Sine(150);
    auto s3 = Sine(150);
    auto s4 = Sine(150);
    auto s5 = Sine(150);
    auto s6 = Sine(150);
    auto n1 = Null();
    auto n2 = Null();
    auto n3 = Null();
    auto n4 = Null();
    auto n5 = Null();
    auto n6 = Null();
    autd << async                                                                     //
         << (std::move(s1), std::move(n1)) << clear                                   //
         << (std::move(s2), std::move(n2)) << std::move(s3)                           //
         << (std::move(s4), std::move(n3)) << std::move(n4);                          //
    autd << async << (std::move(s5), std::move(n5)) << std::move(s6), std::move(n6);  //
  }
}

int main(int argc, char** argv) {
  testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
