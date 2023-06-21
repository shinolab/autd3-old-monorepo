#pragma once

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <cstdint>

namespace autd3::internal::native_methods {

struct ControllerBuilderPtr {
  void* _0;
};

struct FirmwareInfoListPtr {
  void* _0;
};

extern "C" {

[[nodiscard]] ControllerBuilderPtr AUTDCreateControllerBuilder();

ControllerBuilderPtr AUTDAddDevice(ControllerBuilderPtr builder,
                                   double x,
                                   double y,
                                   double z,
                                   double rz1,
                                   double ry,
                                   double rz2);

ControllerBuilderPtr AUTDAddDeviceQuaternion(ControllerBuilderPtr builder,
                                             double x,
                                             double y,
                                             double z,
                                             double qw,
                                             double qx,
                                             double qy,
                                             double qz);

[[nodiscard]]
ControllerPtr AUTDControllerOpenWith(ControllerBuilderPtr builder,
                                     LinkPtr link,
                                     char *err);

[[nodiscard]] bool AUTDClose(ControllerPtr cnt, char *err);

void AUTDFreeController(ControllerPtr cnt);

void AUTDSetReadsFPGAInfo(ControllerPtr cnt, bool value);

void AUTDSetForceFan(ControllerPtr cnt, bool value);

[[nodiscard]] GeometryPtr AUTDGetGeometry(ControllerPtr cnt);

[[nodiscard]] double AUTDGetSoundSpeed(GeometryPtr geo);

void AUTDSetSoundSpeed(GeometryPtr geo, double value);

void AUTDSetSoundSpeedFromTemp(GeometryPtr geo, double temp, double k, double r, double m);

[[nodiscard]] double AUTDGetTransFrequency(GeometryPtr geo, uint32_t idx);

[[nodiscard]] bool AUTDSetTransFrequency(GeometryPtr geo, uint32_t idx, double value, char *err);

[[nodiscard]] uint16_t AUTDGetTransCycle(GeometryPtr geo, uint32_t idx);

[[nodiscard]] bool AUTDSetTransCycle(GeometryPtr geo, uint32_t idx, uint16_t value, char *err);

[[nodiscard]] double AUTDGetWavelength(GeometryPtr geo, uint32_t idx, double sound_speed);

[[nodiscard]] double AUTDGetAttenuation(GeometryPtr geo);

void AUTDSetAttenuation(GeometryPtr geo, double value);

[[nodiscard]] uint32_t AUTDNumTransducers(GeometryPtr geo);

[[nodiscard]] uint32_t AUTDNumDevices(GeometryPtr geo);

void AUTDGeometryCenter(GeometryPtr geo, double *x, double *y, double *z);

void AUTDGeometryCenterOf(GeometryPtr geo, uint32_t dev_idx, double *x, double *y, double *z);

void AUTDTransPosition(GeometryPtr geo, uint32_t tr_idx, double *x, double *y, double *z);

void AUTDTransRotation(GeometryPtr geo,
                       uint32_t tr_idx,
                       double *w,
                       double *x,
                       double *y,
                       double *z);

void AUTDTransXDirection(GeometryPtr geo, uint32_t tr_idx, double *x, double *y, double *z);

void AUTDTransYDirection(GeometryPtr geo, uint32_t tr_idx, double *x, double *y, double *z);

void AUTDTransZDirection(GeometryPtr geo, uint32_t tr_idx, double *x, double *y, double *z);

[[nodiscard]] uint16_t AUTDGetTransModDelay(GeometryPtr geo, uint32_t tr_idx);

void AUTDSetTransModDelay(GeometryPtr geo, uint32_t tr_idx, uint16_t delay);

[[nodiscard]] bool AUTDGetFPGAInfo(ControllerPtr cnt, const uint8_t *out, char *err);

[[nodiscard]] FirmwareInfoListPtr AUTDGetFirmwareInfoListPointer(ControllerPtr cnt, char *err);

void AUTDGetFirmwareInfo(FirmwareInfoListPtr p_info_list,
                         uint32_t idx,
                         char *info,
                         bool *is_valid,
                         bool *is_supported);

void AUTDFreeFirmwareInfoListPointer(FirmwareInfoListPtr p_info_list);

void AUTDGetLatestFirmware(char *latest);

[[nodiscard]] GainPtr AUTDGainNull();

[[nodiscard]] GainPtr AUTDGainGrouped();

[[nodiscard]] GainPtr AUTDGainGroupedAdd(GainPtr grouped_gain, uint32_t device_id, GainPtr gain);

[[nodiscard]] GainPtr AUTDGainFocus(double x, double y, double z);

[[nodiscard]] GainPtr AUTDGainFocusWithAmp(GainPtr focus, double amp);

[[nodiscard]]
GainPtr AUTDGainBessel(double x,
                       double y,
                       double z,
                       double nx,
                       double ny,
                       double nz,
                       double theta_z);

[[nodiscard]] GainPtr AUTDGainBesselWithAmp(GainPtr bessel, double amp);

[[nodiscard]] GainPtr AUTDGainPlane(double nx, double ny, double nz);

[[nodiscard]] GainPtr AUTDGainPlaneWithAmp(GainPtr plane, double amp);

[[nodiscard]] GainPtr AUTDGainTransducerTest();

[[nodiscard]]
GainPtr AUTDGainTransducerTestSet(GainPtr trans_test,
                                  uint32_t id,
                                  double phase,
                                  double amp);

[[nodiscard]] GainPtr AUTDGainCustom(const double *amp, const double *phase, uint64_t size);

[[nodiscard]] DatagramBodyPtr AUTDGainIntoDatagram(GainPtr gain);

[[nodiscard]]
int32_t AUTDGainCalc(GainPtr gain,
                     GeometryPtr geometry,
                     double *amp,
                     double *phase,
                     char *err);

[[nodiscard]] ModulationPtr AUTDModulationStatic();

[[nodiscard]] ModulationPtr AUTDModulationStaticWithAmp(ModulationPtr m, double amp);

[[nodiscard]]
ModulationPtr AUTDModulationStaticWithSamplingFrequencyDivision(ModulationPtr m,
                                                                uint32_t div);

[[nodiscard]] ModulationPtr AUTDModulationSine(uint32_t freq);

[[nodiscard]] ModulationPtr AUTDModulationSineWithAmp(ModulationPtr m, double amp);

[[nodiscard]] ModulationPtr AUTDModulationSineWithOffset(ModulationPtr m, double offset);

[[nodiscard]]
ModulationPtr AUTDModulationSineWithSamplingFrequencyDivision(ModulationPtr m,
                                                              uint32_t div);

[[nodiscard]] ModulationPtr AUTDModulationSinePressure(uint32_t freq);

[[nodiscard]] ModulationPtr AUTDModulationSinePressureWithAmp(ModulationPtr m, double amp);

[[nodiscard]] ModulationPtr AUTDModulationSinePressureWithOffset(ModulationPtr m, double offset);

[[nodiscard]]
ModulationPtr AUTDModulationSinePressureWithSamplingFrequencyDivision(ModulationPtr m,
                                                                      uint32_t div);

[[nodiscard]] ModulationPtr AUTDModulationSineLegacy(double freq);

[[nodiscard]] ModulationPtr AUTDModulationSineLegacyWithAmp(ModulationPtr m, double amp);

[[nodiscard]] ModulationPtr AUTDModulationSineLegacyWithOffset(ModulationPtr m, double offset);

[[nodiscard]]
ModulationPtr AUTDModulationSineLegacyWithSamplingFrequencyDivision(ModulationPtr m,
                                                                    uint32_t div);

[[nodiscard]] ModulationPtr AUTDModulationSquare(uint32_t freq);

[[nodiscard]] ModulationPtr AUTDModulationSquareWithLow(ModulationPtr m, double low);

[[nodiscard]] ModulationPtr AUTDModulationSquareWithHigh(ModulationPtr m, double high);

[[nodiscard]] ModulationPtr AUTDModulationSquareWithDuty(ModulationPtr m, double duty);

[[nodiscard]]
ModulationPtr AUTDModulationSquareWithSamplingFrequencyDivision(ModulationPtr m,
                                                                uint32_t div);

[[nodiscard]]
ModulationPtr AUTDModulationCustom(uint32_t freq_div,
                                   const double *amp,
                                   uint64_t size);

[[nodiscard]] uint32_t AUTDModulationSamplingFrequencyDivision(ModulationPtr m);

[[nodiscard]] double AUTDModulationSamplingFrequency(ModulationPtr m);

[[nodiscard]] DatagramHeaderPtr AUTDModulationIntoDatagram(ModulationPtr m);

[[nodiscard]] int32_t AUTDModulationSize(ModulationPtr m, char *err);

[[nodiscard]] int32_t AUTDModulationCalc(ModulationPtr m, double *buffer, char *err);

[[nodiscard]] STMPropsPtr AUTDSTMProps(double freq);

[[nodiscard]] STMPropsPtr AUTDSTMPropsWithSamplingFreq(double freq);

[[nodiscard]] STMPropsPtr AUTDSTMPropsWithSamplingFreqDiv(uint32_t div);

[[nodiscard]] STMPropsPtr AUTDSTMPropsWithStartIdx(STMPropsPtr props, int32_t idx);

[[nodiscard]] STMPropsPtr AUTDSTMPropsWithFinishIdx(STMPropsPtr props, int32_t idx);

[[nodiscard]] double AUTDSTMPropsFrequency(STMPropsPtr props, uint64_t size);

[[nodiscard]] double AUTDSTMPropsSamplingFrequency(STMPropsPtr props, uint64_t size);

[[nodiscard]] uint32_t AUTDSTMPropsSamplingFrequencyDivision(STMPropsPtr props, uint64_t size);

[[nodiscard]] int32_t AUTDSTMPropsStartIdx(STMPropsPtr props);

[[nodiscard]] int32_t AUTDSTMPropsFinishIdx(STMPropsPtr props);

[[nodiscard]]
DatagramBodyPtr AUTDFocusSTM(STMPropsPtr props,
                             const double *points,
                             const uint8_t *shift,
                             uint64_t size);

[[nodiscard]] DatagramBodyPtr AUTDGainSTMWithMode(STMPropsPtr props, GainSTMMode mode);

[[nodiscard]] DatagramBodyPtr AUTDGainSTM(STMPropsPtr props);

[[nodiscard]] DatagramBodyPtr AUTDGainSTMAddGain(DatagramBodyPtr stm, GainPtr gain);

[[nodiscard]] DatagramSpecialPtr AUTDSynchronize();

[[nodiscard]] DatagramSpecialPtr AUTDClear();

[[nodiscard]] DatagramSpecialPtr AUTDUpdateFlags();

[[nodiscard]] DatagramSpecialPtr AUTDStop();

[[nodiscard]] DatagramSpecialPtr AUTDModDelayConfig();

[[nodiscard]] DatagramHeaderPtr AUTDCreateSilencer(uint16_t step);

[[nodiscard]] DatagramBodyPtr AUTDCreateAmplitudes(double amp);

[[nodiscard]]
int32_t AUTDSend(ControllerPtr cnt,
                 TransMode mode,
                 DatagramHeaderPtr header,
                 DatagramBodyPtr body,
                 int64_t timeout_ns,
                 char *err);

[[nodiscard]]
int32_t AUTDSendSpecial(ControllerPtr cnt,
                        TransMode mode,
                        DatagramSpecialPtr special,
                        int64_t timeout_ns,
                        char *err);

[[nodiscard]] LinkPtr AUTDLinkDebug();

[[nodiscard]] LinkPtr AUTDLinkDebugWithLogLevel(LinkPtr debug, Level level);

[[nodiscard]] LinkPtr AUTDLinkDebugWithLogFunc(LinkPtr debug, void* out_func, void* flush_func);

[[nodiscard]] LinkPtr AUTDLinkDebugWithTimeout(LinkPtr debug, uint64_t timeout_ns);

[[nodiscard]] LinkPtr AUTDLinkLog(LinkPtr link);

[[nodiscard]] LinkPtr AUTDLinkLogWithLogLevel(LinkPtr log, Level level);

[[nodiscard]] LinkPtr AUTDLinkLogWithLogFunc(LinkPtr log, void* out_func, void* flush_func);

} // extern "C"

} // namespace autd3::internal::native_methods
