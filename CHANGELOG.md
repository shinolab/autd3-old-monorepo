# 8.2.0

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
