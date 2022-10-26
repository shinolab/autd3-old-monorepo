# 2.4.6

- Rename `AUTDServer` to `TwinCATAUTDServer`
- Fix [#36](https://github.com/shinolab/autd3/issues/36)

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
