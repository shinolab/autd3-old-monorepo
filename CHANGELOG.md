# 14.2.2

- Add C++ documentation
  - Fix [#89](https://github.com/shinolab/autd3/issues/89): [C++] Poor documentation
- Fix [#121](https://github.com/shinolab/autd3/issues/121): Phase parameters of autd3::modulation::Fourier
- Fix [#122](https://github.com/shinolab/autd3/issues/122): Calling GeometryViewer::run multiple times causes an error
- Fix [#123](https://github.com/shinolab/autd3/issues/123): impl Default for autd3-link-monitor::PyPlotConfig
- Fix [#124](https://github.com/shinolab/autd3/issues/124): Python backend of autd3-link-monitor causes indentation errors
- Fix [#125](https://github.com/shinolab/autd3/issues/125): Grouped without specified Gain for all devices causes an error

# 14.2.1

- Improve `autd3-gain-holo` performance
  - Close [#98](https://github.com/shinolab/autd3/issues/98): Add benchmarking
- Fix [#118](https://github.com/shinolab/autd3/issues/118): Cannot compile and test with `single_float` features
- Fix [#119](https://github.com/shinolab/autd3/issues/119): link-simulator sometimes panic
- Fix [#120](https://github.com/shinolab/autd3/issues/120): With 9 devices, CUDABackend causes an error with LM algorithm

# 14.2.0

- Add `modulation::Fourier`
  - Fix [#110](https://github.com/shinolab/autd3/issues/110): Multi-frequency sine modulation? 
- Fix [#111](https://github.com/shinolab/autd3/issues/111): Add macOS and Linux support for autd3-unity
- Fix [#115](https://github.com/shinolab/autd3/issues/115): `autd3-geometry-viewer` from git does not work
- Fix [#116](https://github.com/shinolab/autd3/issues/116): [autd3-unity] cannot launch simulator
- Fix [#117](https://github.com/shinolab/autd3/issues/117): [autd3-unity] There is no LICENSE.md but LICENSE.txt

# 14.1.0

- Fix [#93](https://github.com/shinolab/autd3/issues/93): pyautd3 package contains unnecessary dynamic libraries
- Fix [#94](https://github.com/shinolab/autd3/issues/94): pyautd3 library should clarify its dependence on numpy
- Fix [#108](https://github.com/shinolab/autd3/issues/108): OsalTimer on macOS causes segmentation fault
- Fix [#109](https://github.com/shinolab/autd3/issues/109): Add support for linux arm architecture for Raspberry Pi
- Fix [#112](https://github.com/shinolab/autd3/issues/112): Add gain to Grouped by device group
- Fix [#113](https://github.com/shinolab/autd3/issues/113): simulator_client example is broken in C++/C#/F#/Python
- Fix [#114](https://github.com/shinolab/autd3/issues/114): AUTD Server on Windows without npcap installed causes an error
- Add `with_sampling_period` to `Modulation`
- Add `with_period` and `with_sampling_period` to `STM`

# 14.0.1

- Fix [#107](https://github.com/shinolab/autd3/issues/107): There is no with_sampling_frequency method in FocusSTM and GainSTM in pyautd3
- Add sampling frequency option to `Wav` modulation in capi

# 14.0.0

- Fix [#84](https://github.com/shinolab/autd3/issues/84): AUTD Server should not require wpcap.dll
- Fix [#85](https://github.com/shinolab/autd3/issues/85): Dockerfile in doc is broken
- Fix [#86](https://github.com/shinolab/autd3/issues/86): Remove bindgen dependency from autd3-link-soem
- Fix [#87](https://github.com/shinolab/autd3/issues/87): Firmware version from Simulator is invalid in macOS
- Fix [#88](https://github.com/shinolab/autd3/issues/88): [Rust] Poor documentation
- Fix [#92](https://github.com/shinolab/autd3/issues/92): Support modulation::Radiation in C++/C#/Python
- Fix [#95](https://github.com/shinolab/autd3/issues/95): Poor typing in pyautd3
- Fix [#96](https://github.com/shinolab/autd3/issues/96): sudo pip is not recommended
- Fix [#97](https://github.com/shinolab/autd3/issues/97): Can AMS Net Id be displayed on terminal?
- Fix [#99](https://github.com/shinolab/autd3/issues/99): Add gain::Grouped support in lightweight mode
- Fix [#100](https://github.com/shinolab/autd3/issues/100): AUTD Server application should show License
- Fix [#102](https://github.com/shinolab/autd3/issues/102): error message when given an Interface name and no AUTD3 device is not found is a bit strange
- Fix [#103](https://github.com/shinolab/autd3/issues/103): warning: variable does not need to be mutable in tests
- Fix [#104](https://github.com/shinolab/autd3/issues/104): dependency on autd3-protobuf should be optional because it requires additional libraries to build and is not necessary for basic usage
- Fix [#105](https://github.com/shinolab/autd3/issues/105): pyautd3 cannot be used on macOS
- Fix [#106](https://github.com/shinolab/autd3/issues/106): tuple of Clear and Synchronize can be sent, but it must be not allowed
- Add plotters backend for `autd3-link-monitor` and make it default

# 13.0.0

- Remove `SinePressure`, add `RadiationPressure` instead
- Adopt gRPC for more stable remote communication
- Integrated SOEMAUTDServer/TwinCATAUTDServer/simulator into AUTD server app
- Send `Clear` and `Synchronize` in `open` automatically

# 12.3.1

- Fix [#82](https://github.com/shinolab/autd3/issues/82)
- Fix [#83](https://github.com/shinolab/autd3/issues/83)

# 12.3.0

- Fix [#81](https://github.com/shinolab/autd3/issues/81)
  - Raise minimum supported Python version to 3.9

# 12.2.0

- Add `send_async`
- Add `software_stm`
- Fix [#80](https://github.com/shinolab/autd3/issues/80)

# 12.1.1

- Fix [#78](https://github.com/shinolab/autd3/issues/78)
- Fix [#79](https://github.com/shinolab/autd3/issues/79)

# 12.1.0

- Fix [#76](https://github.com/shinolab/autd3/issues/76)
- Fix [#77](https://github.com/shinolab/autd3/issues/77)

# 12.0.0

- Fix [#75](https://github.com/shinolab/autd3/issues/75)

# 11.2.0

- Fix [#69](https://github.com/shinolab/autd3/issues/69)
- Fix [#70](https://github.com/shinolab/autd3/issues/70)
- Add `Bundle` link
- Add `Monitor` link
- Add `FIR` modulation
- Add `Transform` modulation
- Add `RawPCM` modulation
- Fix fluctuation when moving slice in simulator

# 11.1.0

- Fix [#68](https://github.com/shinolab/autd3/issues/68)
- Improve Simulator stability

# 11.0.2

- Fix [#74](https://github.com/shinolab/autd3/issues/74)

# 11.0.1

- minor fix

# 11.0.0

- Fix [#63](https://github.com/shinolab/autd3/issues/63)
- Fix [#64](https://github.com/shinolab/autd3/issues/64)
- Fix [#65](https://github.com/shinolab/autd3/issues/65)
- Fix [#67](https://github.com/shinolab/autd3/issues/67)
- Fix [#71](https://github.com/shinolab/autd3/issues/71)
- Fix [#72](https://github.com/shinolab/autd3/issues/72)

# 10.0.0

- Fix [#62](https://github.com/shinolab/autd3/issues/62)
- All codes are rewritten in Rust

# 9.0.1

- Minimize dependence on boost
- Add `link::RemoteSimulator` to capi

# 9.0.0

- AUTD Simulator can now be accessed over the network
  - Add `link::RemoteSimulator`
- Add logging and timeout options for all links 

# 8.5.0

- (internal) Refactor some modules and adopt Boost library

# 8.4.1

- Fix [#60](https://github.com/shinolab/autd3/issues/60)
- Fix [#61](https://github.com/shinolab/autd3/issues/61)

# 8.4.0

- Add default timeout option to `Link`

# 8.3.0

- Fix some minor bugs
- Add `timer_strategy` option for `link::SOEM`
  - `high_precision` is now deprecated
- (internal) Refactor some modules

# 8.2.0

- Move `Controller::set_sound_speed_from_temp` into `Geometry`
- Make `modulation::LPF` generic
- Add `modulation::Transform`
- Rename Normal mode to Advanced mode
- Rename `gain::holo::EVD` to `gain::holo::EVP`
- Remove `<<` operators
- Remove `ack_check_timeout` option, add `timeout` parameter to `send` function instead

# 8.1.2

- Fix [#59](https://github.com/shinolab/autd3/issues/59)

# 8.1.1

- Fix [#58](https://github.com/shinolab/autd3/issues/58)

# 8.1.0

- Introduce Semantic versioning
- Add thermal sensor option
- Add vivado lab edition supoprt for firmware update
- Add `link::Log`
- Add geometry transformation methods
- Remove async send methods
- Change `Controller::open` API
  - Add `Geometry::Builder` to create geometry

# 2.8.0

- Suppress midrange frequency noise

# 2.7.6

- Fix [#57](https://github.com/shinolab/autd3/issues/57)

# 2.7.5

- Fix [#37](https://github.com/shinolab/autd3/issues/37)

# 2.7.4

- Fix [#55](https://github.com/shinolab/autd3/issues/55)
- Fix [#56](https://github.com/shinolab/autd3/issues/56)

# 2.7.3

- Remove parameter from `FocusSTM`, `GainSTM`, and `gain::Grouped` constructor
- Remove `Driver` to drive old firmware
- Fix [#54](https://github.com/shinolab/autd3/issues/54)

# 2.7.2

- Fix [#52](https://github.com/shinolab/autd3/issues/52)
- Fix [#53](https://github.com/shinolab/autd3/issues/53)

# 2.7.1

- Fix [#51](https://github.com/shinolab/autd3/issues/51)
- Add `USE_SINGLE_FLOAT` option
- [Unity] Unity API now uses `float` instead of `double`

# 2.7.0 

- Fix [#50](https://github.com/shinolab/autd3/issues/50)
- Add `start_idx` and `finish_idx` to `STM`

# 2.6.8

- Fix [#49](https://github.com/shinolab/autd3/issues/49)
- Improve `Holo` gain performance

# 2.6.7

- Change `Controller::_send_interval` to 1ms by default
- Improve Simulator stability
- Add "Auto play" option for Simulator

# 2.6.6

- Rename `PointSTM` to `FocusSTM`
- Add `AUTDSetSoundSpeedFromTemp` in capi
- (internal) refactor to improve readability

# 2.6.5

- Flatten `Geometry`
- Fix [#48](https://github.com/shinolab/autd3/issues/48)

# 2.6.4

- Fix [#46](https://github.com/shinolab/autd3/issues/46)
- Fix [#47](https://github.com/shinolab/autd3/issues/47)

# 2.6.3

- Change sound speed configuration API
- Functions that can fail now return false instead of an exception
  - C API has also been changed
- Fix [#45](https://github.com/shinolab/autd3/issues/45)

# 2.6.2

- Remove `Controller::check_trials`, add `Controllse::set_ack_check_timeout` instead
- Add `driver::Driver` class to drive old firmware
- Change `gain::TransducerTest` API

# 2.6.1

- Fix [#44](https://github.com/shinolab/autd3/issues/44)

# 2.6.0

- Fix [#43](https://github.com/shinolab/autd3/issues/43)
- `MOD_SAMPLING_FREQ_DIV_MIN`, `POINT_STM_SAMPLING_FREQ_DIV_MIN`, `GAIN_STM_SAMPLING_FREQ_DIV_MIN`, `GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN`, and `SILENCER_CYCLE_MIN` are halved

# 2.5.2

- Fix [#37](https://github.com/shinolab/autd3/issues/37)
- Fix [#42](https://github.com/shinolab/autd3/issues/42)
- Change phase unit to radian
- Add stream operator to `Controller`
- Add `Controller::send_async` and `autd3::async` to send data asynchronously
- `Controller::synchronize()`, `Controller::update_flag()`, `Controller::clear()`, and `Controller::stop()` functions are now deprecated
  - Send `autd3::synchronize`, `autd3::update_flag`, `autd3::clear`, and `autd3::stop` instead

# 2.5.1

- Fix [#38](https://github.com/shinolab/autd3/issues/38)
- Fix [#39](https://github.com/shinolab/autd3/issues/39)
- Fix [#40](https://github.com/shinolab/autd3/issues/40)
- Fix [#41](https://github.com/shinolab/autd3/issues/41)
- Add simulator for Unity

# 2.5.0

- Rename `AUTDServer` to `TwinCATAUTDServer`
- Add `SOEMAUTDServer` and `link::RemoteSOEM`
- Add Windows arm support
- Fix [#36](https://github.com/shinolab/autd3/issues/36)
- Add log settings in SOEM Link CAPI
- Remove `port` and `ip` settings from Simulator

# 2.4.5

- Change unit of sound speed from m/s to mm/s
- Add `use_meter` and `use_left_handed` options to `Simulator`
- Change `Holo` constraint API

# 2.4.4

- Change default EtherCAT interval 500us to 1ms
- Improve `link::SOEM` performance
- [AUTD3Sharp] Change API to improve consistency with the C++ version
- [pyautd3] Change API to improve consistency with the C++ version

# 2.4.3

- Embed font into GeometryViewer and Simulator
- Embed model into GeometryViewer

# 2.4.2

- win-x86 is no more supported
- Fix [#30](https://github.com/shinolab/autd3/issues/30)
- Fix minor bugs

# 2.4.1

- Add `extra::simulator::Simulator`
- Rename `link::Emulator` to `link::Simulator`

# 2.4.0

- Add `GeometryViewer`
- Improve performance of `link::SOEM` on Windows
- Update maximum pattern size of `GaimSTM` in legacy mode to 2048
- Add `link::Bundle` and `link::Debug`
- Add `extra::firmware-emulator`
- Add `SoftwareSTM`
- Add `gain::TransducerTest`
- Add `modulation::LPF`
- Add `ArrayFireBackend` (experimental)
- Fix [#25](https://github.com/shinolab/autd3/issues/25), [#26](https://github.com/shinolab/autd3/issues/26)
- Update firmware to v2.4

# 2.3.1

- Remove the first argument (`Geometry&`) of a `link::Emulator` constructor
- Remove the first argument (interface name) and the second argument (number of devices) of a `link::SOEM` constructor
- (internal) `link::open` now requires `Geometry&`

# 2.3.0

- Add `SyncMode` setting to `link::SOEM` to address #20
  - Remove `cycle_ticks` parameter, add `send_cycle` and `sync0_cycle` instead
- `MOD_SAMPLING_FREQ_DIV_MIN`, `STM_SAMPLING_FREQ_DIV_MIN`, and `SILENCER_CYCLE_MIN` are halved
- Update firmware to v2.3

# 2.2.2

- Change the whole API; this library is no more using a template to change the Transducer mode
- Add `gain::holo::LSSGreedy` and `gain::holo::APO`

# 2.2.1

- Remove a `check_ack` flag, and add a `check_trials` parameter instead
  - `check_ack = true` equals to `check_trials = 50` and `check_ack = false`
    equals to `check_trials = 0`
- Add `send_interval` parameter
  - default is 1
- Remove a `sound_speed` parameter from `AUTDGetWavelength` in C-API
- Remove `GaussNewton` and `GradientDescent`

# 2.2.0

- Remove `AUTDSendHeader`, `AUTDSendBody`, and `AUTDSend` in C-API, which are now merged into `AUTDSend`
- Remove `cycle_ticks` parameters in `link::TwinCAT` and `link::RemoteTwinCAT`,
  which are no more required
- (internal) Remove `cycle_ticks` method from `link::Link`
- Update firmware to v2.2

# 2.1.0

- Rename `Controller` to `ControllerX`, and `Controller` is now an alias of `ControllerX<LegacyTransducer>`
- Add `NormalPhaseTransducer`
- Fix `SineLegacy`
- Fix firmware version when using v1.x firmware
- Add `Mode` configuration in `GainSTM`
- Add `mod_delay` configuration in `Transducer`
- Update firmware to v2.1

# 2.0.3

- Fix `AUTDSetSoundSpeed` in C-API

# 2.0.2

- Add `DynamicTransducer` for C-API
- Remove legacy C-API library
- Change `AmplitudeConstraint` API in Holo Gain in C-API
- Fix `wavelength` and `wavenumber` of `NormalTransducer`

# 2.0.1

- Fix C-API
- Add `objective` parameter to `gain::holo::Greedy`
- Fix a bug in sending modulation and gain separately
- Change Silencer API
