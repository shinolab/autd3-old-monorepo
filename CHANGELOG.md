# 2.3.0

- Add `SyncMode` setting to `link::SOEM` to address #20
  - Remove `cycle_ticks` parameter, add `send_cycle` and `sync0_cycle` instead
- `MOD_SAMPLING_FREQ_DIV_MIN`, `STM_SAMPLING_FREQ_DIV_MIN`, and `SILENCER_CYCLE_MIN` is halved
- Update firmware to v2.3

# 2.2.2

- Change the whole API; this library is no more using a template to change Transducer mode
- Add `gain::holo::LSSGreedy` and `gain::holo::APO`

# 2.2.1

- Remove `check_ack` flag, add `check_trials` parameter instead
  - `check_ack = true` equals to `check_trials = 50` and `check_ack = false`
    equals to `check_trials = 0`
- Add `send_interval` parameter
  - default is 1
- Remove `sound_speed` parameter from `AUTDGetWavelength` in C-API
- Remove `GaussNewton` and `GradientDescent`

# 2.2.0

- Remove `AUTDSendHeader`, `AUTDSendBody`, and `AUTDSend` in C-API, which are
  now merged into `AUTDSend`
- Remove `cycle_ticks` parameters in `link::TwinCAT` and `link::RemoteTwinCAT`,
  which are no more required
- (internal) Remove `cycle_ticks` method from `link::Link`
- Update firmware to v2.2

# 2.1.0

- Rename `Controller` to `ControllerX`, and `Controller` is now alias of
  `ControllerX<LegacyTransducer>`
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
- Change `AmplitudeConstraint` api in Holo Gain in C-API
- Fix `wavelength` and `wavenumber` of `NormalTransducer`

# 2.0.1

- Fix C-API
- Add `objective` parameter to `gain::holo::Greedy`
- Fix bug in sending modulation and gain separately
- Change Silencer API
