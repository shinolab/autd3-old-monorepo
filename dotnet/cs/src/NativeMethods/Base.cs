// <auto-generated>
// This code is generated by csbindgen.
// DON'T CHANGE THIS DIRECTLY.
// </auto-generated>
#pragma warning disable CS8500
#pragma warning disable CS8981
using System;
using System.Runtime.InteropServices;


namespace AUTD3Sharp.NativeMethods
{
    public static unsafe partial class NativeMethodsBase
    {
        const string __DllName = "autd3capi";



        [DllImport(__DllName, EntryPoint = "AUTDGainBessel", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern GainPtr AUTDGainBessel(double x, double y, double z, double nx, double ny, double nz, double theta_z);

        [DllImport(__DllName, EntryPoint = "AUTDGainBesselWithIntensity", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern GainPtr AUTDGainBesselWithIntensity(GainPtr bessel, byte intensity);

        [DllImport(__DllName, EntryPoint = "AUTDGainCustom", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern GainPtr AUTDGainCustom();

        [DllImport(__DllName, EntryPoint = "AUTDGainCustomSet", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern GainPtr AUTDGainCustomSet(GainPtr custom, uint dev_idx, DriveRaw* ptr, uint len);

        [DllImport(__DllName, EntryPoint = "AUTDGainFocus", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern GainPtr AUTDGainFocus(double x, double y, double z);

        [DllImport(__DllName, EntryPoint = "AUTDGainFocusWithIntensity", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern GainPtr AUTDGainFocusWithIntensity(GainPtr focus, byte intensity);

        [DllImport(__DllName, EntryPoint = "AUTDGainGroupCreateMap", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern GroupGainMapPtr AUTDGainGroupCreateMap(uint* device_indices_ptr, uint num_devices);

        [DllImport(__DllName, EntryPoint = "AUTDGainGroupMapSet", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern GroupGainMapPtr AUTDGainGroupMapSet(GroupGainMapPtr map, uint dev_idx, int* map_data);

        [DllImport(__DllName, EntryPoint = "AUTDGainGroup", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern GainPtr AUTDGainGroup(GroupGainMapPtr map, int* keys_ptr, GainPtr* values_ptr, uint kv_len);

        [DllImport(__DllName, EntryPoint = "AUTDGainIntoDatagram", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern DatagramPtr AUTDGainIntoDatagram(GainPtr gain);

        [DllImport(__DllName, EntryPoint = "AUTDGainCalc", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ResultGainCalcDrivesMap AUTDGainCalc(GainPtr gain, GeometryPtr geometry);

        [DllImport(__DllName, EntryPoint = "AUTDGainCalcGetResult", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDGainCalcGetResult(GainCalcDrivesMapPtr src, DriveRaw* dst, uint idx);

        [DllImport(__DllName, EntryPoint = "AUTDGainCalcFreeResult", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDGainCalcFreeResult(GainCalcDrivesMapPtr src);

        [DllImport(__DllName, EntryPoint = "AUTDGainNull", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern GainPtr AUTDGainNull();

        [DllImport(__DllName, EntryPoint = "AUTDGainPlane", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern GainPtr AUTDGainPlane(double nx, double ny, double nz);

        [DllImport(__DllName, EntryPoint = "AUTDGainPlaneWithIntensity", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern GainPtr AUTDGainPlaneWithIntensity(GainPtr plane, byte intensity);

        [DllImport(__DllName, EntryPoint = "AUTDGainTransducerTest", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern GainPtr AUTDGainTransducerTest();

        [DllImport(__DllName, EntryPoint = "AUTDGainTransducerTestSet", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern GainPtr AUTDGainTransducerTestSet(GainPtr trans_test, TransducerPtr tr, double phase, byte intensity);

        [DllImport(__DllName, EntryPoint = "AUTDGainUniform", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern GainPtr AUTDGainUniform(byte intensity);

        [DllImport(__DllName, EntryPoint = "AUTDGainUniformWithPhase", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern GainPtr AUTDGainUniformWithPhase(GainPtr uniform, double phase);

        [DllImport(__DllName, EntryPoint = "AUTDDevice", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern DevicePtr AUTDDevice(GeometryPtr geo, uint dev_idx);

        [DllImport(__DllName, EntryPoint = "AUTDDeviceNumTransducers", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern uint AUTDDeviceNumTransducers(DevicePtr dev);

        [DllImport(__DllName, EntryPoint = "AUTDDeviceGetSoundSpeed", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern double AUTDDeviceGetSoundSpeed(DevicePtr dev);

        [DllImport(__DllName, EntryPoint = "AUTDDeviceSetSoundSpeed", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDDeviceSetSoundSpeed(DevicePtr dev, double value);

        [DllImport(__DllName, EntryPoint = "AUTDDeviceSetSoundSpeedFromTemp", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDDeviceSetSoundSpeedFromTemp(DevicePtr dev, double temp, double k, double r, double m);

        [DllImport(__DllName, EntryPoint = "AUTDDeviceGetAttenuation", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern double AUTDDeviceGetAttenuation(DevicePtr dev);

        [DllImport(__DllName, EntryPoint = "AUTDDeviceSetAttenuation", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDDeviceSetAttenuation(DevicePtr dev, double value);

        [DllImport(__DllName, EntryPoint = "AUTDDeviceCenter", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDDeviceCenter(DevicePtr dev, double* center);

        [DllImport(__DllName, EntryPoint = "AUTDDeviceTranslate", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDDeviceTranslate(DevicePtr dev, double x, double y, double z);

        [DllImport(__DllName, EntryPoint = "AUTDDeviceRotate", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDDeviceRotate(DevicePtr dev, double w, double i, double j, double k);

        [DllImport(__DllName, EntryPoint = "AUTDDeviceAffine", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDDeviceAffine(DevicePtr dev, double x, double y, double z, double w, double i, double j, double k);

        [DllImport(__DllName, EntryPoint = "AUTDDeviceSetReadsFPGAInfo", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDDeviceSetReadsFPGAInfo(DevicePtr dev, [MarshalAs(UnmanagedType.U1)] bool value);

        [DllImport(__DllName, EntryPoint = "AUTDDeviceSetForceFan", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDDeviceSetForceFan(DevicePtr dev, [MarshalAs(UnmanagedType.U1)] bool value);

        [DllImport(__DllName, EntryPoint = "AUTDDeviceEnableSet", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDDeviceEnableSet(DevicePtr dev, [MarshalAs(UnmanagedType.U1)] bool value);

        [DllImport(__DllName, EntryPoint = "AUTDDeviceEnableGet", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.U1)]
        public static extern bool AUTDDeviceEnableGet(DevicePtr dev);

        [DllImport(__DllName, EntryPoint = "AUTDGeometry", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern GeometryPtr AUTDGeometry(ControllerPtr cnt);

        [DllImport(__DllName, EntryPoint = "AUTDGeometryNumDevices", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern uint AUTDGeometryNumDevices(GeometryPtr geo);

        [DllImport(__DllName, EntryPoint = "AUTDRotationFromEulerZYZ", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDRotationFromEulerZYZ(double x, double y, double z, double* rot);

        [DllImport(__DllName, EntryPoint = "AUTDTransducer", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern TransducerPtr AUTDTransducer(DevicePtr dev, uint tr_idx);

        [DllImport(__DllName, EntryPoint = "AUTDTransducerPosition", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDTransducerPosition(TransducerPtr tr, double* pos);

        [DllImport(__DllName, EntryPoint = "AUTDTransducerRotation", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDTransducerRotation(TransducerPtr tr, double* rot);

        [DllImport(__DllName, EntryPoint = "AUTDTransducerDirectionX", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDTransducerDirectionX(TransducerPtr tr, double* dir);

        [DllImport(__DllName, EntryPoint = "AUTDTransducerDirectionY", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDTransducerDirectionY(TransducerPtr tr, double* dir);

        [DllImport(__DllName, EntryPoint = "AUTDTransducerDirectionZ", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDTransducerDirectionZ(TransducerPtr tr, double* dir);

        [DllImport(__DllName, EntryPoint = "AUTDTransducerWavelength", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern double AUTDTransducerWavelength(TransducerPtr tr, double sound_speed);

        [DllImport(__DllName, EntryPoint = "AUTDTransducerModDelayGet", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ushort AUTDTransducerModDelayGet(TransducerPtr tr);

        [DllImport(__DllName, EntryPoint = "AUTDTransducerModDelaySet", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDTransducerModDelaySet(TransducerPtr tr, ushort delay);

        [DllImport(__DllName, EntryPoint = "AUTDControllerBuilder", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ControllerBuilderPtr AUTDControllerBuilder();

        [DllImport(__DllName, EntryPoint = "AUTDControllerBuilderAddDevice", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ControllerBuilderPtr AUTDControllerBuilderAddDevice(ControllerBuilderPtr builder, double x, double y, double z, double qw, double qx, double qy, double qz);

        [DllImport(__DllName, EntryPoint = "AUTDControllerOpenWith", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ResultController AUTDControllerOpenWith(ControllerBuilderPtr builder, LinkBuilderPtr link_builder);

        [DllImport(__DllName, EntryPoint = "AUTDControllerClose", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ResultI32 AUTDControllerClose(ControllerPtr cnt);

        [DllImport(__DllName, EntryPoint = "AUTDControllerDelete", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDControllerDelete(ControllerPtr cnt);

        [DllImport(__DllName, EntryPoint = "AUTDControllerFPGAInfo", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ResultI32 AUTDControllerFPGAInfo(ControllerPtr cnt, byte* @out);

        [DllImport(__DllName, EntryPoint = "AUTDControllerFirmwareInfoListPointer", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ResultFirmwareInfoList AUTDControllerFirmwareInfoListPointer(ControllerPtr cnt);

        [DllImport(__DllName, EntryPoint = "AUTDControllerFirmwareInfoGet", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDControllerFirmwareInfoGet(FirmwareInfoListPtr p_info_list, uint idx, byte* info);

        [DllImport(__DllName, EntryPoint = "AUTDControllerFirmwareInfoListPointerDelete", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDControllerFirmwareInfoListPointerDelete(FirmwareInfoListPtr p_info_list);

        [DllImport(__DllName, EntryPoint = "AUTDFirmwareLatest", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDFirmwareLatest(byte* latest);

        [DllImport(__DllName, EntryPoint = "AUTDDatagramSynchronize", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern DatagramPtr AUTDDatagramSynchronize();

        [DllImport(__DllName, EntryPoint = "AUTDDatagramClear", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern DatagramPtr AUTDDatagramClear();

        [DllImport(__DllName, EntryPoint = "AUTDDatagramUpdateFlags", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern DatagramPtr AUTDDatagramUpdateFlags();

        [DllImport(__DllName, EntryPoint = "AUTDDatagramStop", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern DatagramSpecialPtr AUTDDatagramStop();

        [DllImport(__DllName, EntryPoint = "AUTDDatagramConfigureModDelay", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern DatagramPtr AUTDDatagramConfigureModDelay();

        [DllImport(__DllName, EntryPoint = "AUTDDatagramConfigureDebugOutoutIdx", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern DatagramPtr AUTDDatagramConfigureDebugOutoutIdx();

        [DllImport(__DllName, EntryPoint = "AUTDDatagramConfigureDebugOutoutIdxSet", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern DatagramPtr AUTDDatagramConfigureDebugOutoutIdxSet(DatagramPtr dbg, TransducerPtr tr);

        [DllImport(__DllName, EntryPoint = "AUTDDatagramSilencer", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ResultDatagram AUTDDatagramSilencer(ushort step_intensity, ushort step_phase);

        [DllImport(__DllName, EntryPoint = "AUTDControllerSend", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ResultI32 AUTDControllerSend(ControllerPtr cnt, DatagramPtr d1, DatagramPtr d2, long timeout_ns);

        [DllImport(__DllName, EntryPoint = "AUTDControllerSendSpecial", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ResultI32 AUTDControllerSendSpecial(ControllerPtr cnt, DatagramSpecialPtr special, long timeout_ns);

        [DllImport(__DllName, EntryPoint = "AUTDControllerGroupCreateKVMap", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern GroupKVMapPtr AUTDControllerGroupCreateKVMap();

        [DllImport(__DllName, EntryPoint = "AUTDControllerGroupKVMapSet", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ResultGroupKVMap AUTDControllerGroupKVMapSet(GroupKVMapPtr map, int key, DatagramPtr d1, DatagramPtr d2, long timeout_ns);

        [DllImport(__DllName, EntryPoint = "AUTDControllerGroupKVMapSetSpecial", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ResultGroupKVMap AUTDControllerGroupKVMapSetSpecial(GroupKVMapPtr map, int key, DatagramSpecialPtr special, long timeout_ns);

        [DllImport(__DllName, EntryPoint = "AUTDControllerGroup", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ResultI32 AUTDControllerGroup(ControllerPtr cnt, int* map, GroupKVMapPtr kv_map);

        [DllImport(__DllName, EntryPoint = "AUTDLinkAudit", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern LinkAuditBuilderPtr AUTDLinkAudit();

        [DllImport(__DllName, EntryPoint = "AUTDLinkAuditWithTimeout", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern LinkAuditBuilderPtr AUTDLinkAuditWithTimeout(LinkAuditBuilderPtr audit, ulong timeout_ns);

        [DllImport(__DllName, EntryPoint = "AUTDLinkAuditIntoBuilder", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern LinkBuilderPtr AUTDLinkAuditIntoBuilder(LinkAuditBuilderPtr audit);

        [DllImport(__DllName, EntryPoint = "AUTDLinkAuditIsOpen", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.U1)]
        public static extern bool AUTDLinkAuditIsOpen(LinkPtr audit);

        [DllImport(__DllName, EntryPoint = "AUTDLinkAuditTimeoutNs", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ulong AUTDLinkAuditTimeoutNs(LinkPtr audit);

        [DllImport(__DllName, EntryPoint = "AUTDLinkAuditLastTimeoutNs", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ulong AUTDLinkAuditLastTimeoutNs(LinkPtr audit);

        [DllImport(__DllName, EntryPoint = "AUTDLinkAuditDown", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDLinkAuditDown(LinkPtr audit);

        [DllImport(__DllName, EntryPoint = "AUTDLinkAuditUp", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDLinkAuditUp(LinkPtr audit);

        [DllImport(__DllName, EntryPoint = "AUTDLinkAuditBreakDown", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDLinkAuditBreakDown(LinkPtr audit);

        [DllImport(__DllName, EntryPoint = "AUTDLinkAuditRepair", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDLinkAuditRepair(LinkPtr audit);

        [DllImport(__DllName, EntryPoint = "AUTDLinkAuditCpuUpdate", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDLinkAuditCpuUpdate(LinkPtr audit, uint idx);

        [DllImport(__DllName, EntryPoint = "AUTDLinkAuditCpuIdx", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern uint AUTDLinkAuditCpuIdx(LinkPtr audit, uint idx);

        [DllImport(__DllName, EntryPoint = "AUTDLinkAuditCpuNumTransducers", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern uint AUTDLinkAuditCpuNumTransducers(LinkPtr audit, uint idx);

        [DllImport(__DllName, EntryPoint = "AUTDLinkAuditCpuAck", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern byte AUTDLinkAuditCpuAck(LinkPtr audit, uint idx);

        [DllImport(__DllName, EntryPoint = "AUTDLinkAuditCpuRxData", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern byte AUTDLinkAuditCpuRxData(LinkPtr audit, uint idx);

        [DllImport(__DllName, EntryPoint = "AUTDLinkAuditCpuFpgaFlags", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern byte AUTDLinkAuditCpuFpgaFlags(LinkPtr audit, uint idx);

        [DllImport(__DllName, EntryPoint = "AUTDLinkAuditFpgaAssertThermalSensor", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDLinkAuditFpgaAssertThermalSensor(LinkPtr audit, uint idx);

        [DllImport(__DllName, EntryPoint = "AUTDLinkAuditFpgaDeassertThermalSensor", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDLinkAuditFpgaDeassertThermalSensor(LinkPtr audit, uint idx);

        [DllImport(__DllName, EntryPoint = "AUTDLinkAuditFpgaIsForceFan", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.U1)]
        public static extern bool AUTDLinkAuditFpgaIsForceFan(LinkPtr audit, uint idx);

        [DllImport(__DllName, EntryPoint = "AUTDLinkAuditFpgaIsStmMode", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.U1)]
        public static extern bool AUTDLinkAuditFpgaIsStmMode(LinkPtr audit, uint idx);

        [DllImport(__DllName, EntryPoint = "AUTDLinkAuditFpgaIsStmGainMode", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.U1)]
        public static extern bool AUTDLinkAuditFpgaIsStmGainMode(LinkPtr audit, uint idx);

        [DllImport(__DllName, EntryPoint = "AUTDLinkAuditFpgaSilencerStepIntensity", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ushort AUTDLinkAuditFpgaSilencerStepIntensity(LinkPtr audit, uint idx);

        [DllImport(__DllName, EntryPoint = "AUTDLinkAuditFpgaSilencerStepPhase", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ushort AUTDLinkAuditFpgaSilencerStepPhase(LinkPtr audit, uint idx);

        [DllImport(__DllName, EntryPoint = "AUTDLinkAuditFpgaModDelays", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDLinkAuditFpgaModDelays(LinkPtr audit, uint idx, ushort* delay);

        [DllImport(__DllName, EntryPoint = "AUTDLinkAuditFpgaStmFrequencyDivision", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern uint AUTDLinkAuditFpgaStmFrequencyDivision(LinkPtr audit, uint idx);

        [DllImport(__DllName, EntryPoint = "AUTDLinkAuditFpgaStmCycle", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern uint AUTDLinkAuditFpgaStmCycle(LinkPtr audit, uint idx);

        [DllImport(__DllName, EntryPoint = "AUTDLinkAuditFpgaSoundSpeed", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern uint AUTDLinkAuditFpgaSoundSpeed(LinkPtr audit, uint idx);

        [DllImport(__DllName, EntryPoint = "AUTDLinkAuditFpgaStmStartIdx", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern int AUTDLinkAuditFpgaStmStartIdx(LinkPtr audit, uint idx);

        [DllImport(__DllName, EntryPoint = "AUTDLinkAuditFpgaStmFinishIdx", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern int AUTDLinkAuditFpgaStmFinishIdx(LinkPtr audit, uint idx);

        [DllImport(__DllName, EntryPoint = "AUTDLinkAuditFpgaModulationFrequencyDivision", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern uint AUTDLinkAuditFpgaModulationFrequencyDivision(LinkPtr audit, uint idx);

        [DllImport(__DllName, EntryPoint = "AUTDLinkAuditFpgaModulationCycle", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern uint AUTDLinkAuditFpgaModulationCycle(LinkPtr audit, uint idx);

        [DllImport(__DllName, EntryPoint = "AUTDLinkAuditFpgaModulation", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDLinkAuditFpgaModulation(LinkPtr audit, uint idx, byte* data);

        [DllImport(__DllName, EntryPoint = "AUTDLinkAuditFpgaIntensitiesAndPhases", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDLinkAuditFpgaIntensitiesAndPhases(LinkPtr audit, uint idx, uint stm_idx, byte* intensities, byte* phases);

        [DllImport(__DllName, EntryPoint = "AUTDLinkGet", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern LinkPtr AUTDLinkGet(ControllerPtr cnt);

        [DllImport(__DllName, EntryPoint = "AUTDLinkNop", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern LinkBuilderPtr AUTDLinkNop();

        [DllImport(__DllName, EntryPoint = "AUTDModulationWithCache", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ResultCache AUTDModulationWithCache(ModulationPtr m);

        [DllImport(__DllName, EntryPoint = "AUTDModulationCacheGetBufferLen", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern uint AUTDModulationCacheGetBufferLen(CachePtr m);

        [DllImport(__DllName, EntryPoint = "AUTDModulationCacheGetBuffer", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDModulationCacheGetBuffer(CachePtr m, byte* buf);

        [DllImport(__DllName, EntryPoint = "AUTDModulationCacheIntoModulation", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ModulationPtr AUTDModulationCacheIntoModulation(CachePtr m);

        [DllImport(__DllName, EntryPoint = "AUTDModulationCacheDelete", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void AUTDModulationCacheDelete(CachePtr m);

        [DllImport(__DllName, EntryPoint = "AUTDModulationCustom", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ModulationPtr AUTDModulationCustom(SamplingConfigurationRaw config, byte* ptr, ulong len);

        [DllImport(__DllName, EntryPoint = "AUTDModulationFourier", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ModulationPtr AUTDModulationFourier(ModulationPtr m);

        [DllImport(__DllName, EntryPoint = "AUTDModulationFourierAddComponent", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ModulationPtr AUTDModulationFourierAddComponent(ModulationPtr fourier, ModulationPtr m);

        [DllImport(__DllName, EntryPoint = "AUTDModulationSamplingConfig", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern SamplingConfigurationRaw AUTDModulationSamplingConfig(ModulationPtr m);

        [DllImport(__DllName, EntryPoint = "AUTDModulationIntoDatagram", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern DatagramPtr AUTDModulationIntoDatagram(ModulationPtr m);

        [DllImport(__DllName, EntryPoint = "AUTDModulationSize", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ResultI32 AUTDModulationSize(ModulationPtr m);

        [DllImport(__DllName, EntryPoint = "AUTDModulationWithRadiationPressure", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ModulationPtr AUTDModulationWithRadiationPressure(ModulationPtr m);

        [DllImport(__DllName, EntryPoint = "AUTDModulationSine", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ModulationPtr AUTDModulationSine(uint freq);

        [DllImport(__DllName, EntryPoint = "AUTDModulationSineWithSamplingConfig", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ModulationPtr AUTDModulationSineWithSamplingConfig(ModulationPtr m, SamplingConfigurationRaw config);

        [DllImport(__DllName, EntryPoint = "AUTDModulationSineWithIntensity", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ModulationPtr AUTDModulationSineWithIntensity(ModulationPtr m, byte intensity);

        [DllImport(__DllName, EntryPoint = "AUTDModulationSineWithPhase", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ModulationPtr AUTDModulationSineWithPhase(ModulationPtr m, double phase);

        [DllImport(__DllName, EntryPoint = "AUTDModulationSineWithOffset", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ModulationPtr AUTDModulationSineWithOffset(ModulationPtr m, byte offset);

        [DllImport(__DllName, EntryPoint = "AUTDModulationSquare", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ModulationPtr AUTDModulationSquare(uint freq);

        [DllImport(__DllName, EntryPoint = "AUTDModulationSquareWithLow", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ModulationPtr AUTDModulationSquareWithLow(ModulationPtr m, byte low);

        [DllImport(__DllName, EntryPoint = "AUTDModulationSquareWithHigh", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ModulationPtr AUTDModulationSquareWithHigh(ModulationPtr m, byte high);

        [DllImport(__DllName, EntryPoint = "AUTDModulationSquareWithDuty", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ModulationPtr AUTDModulationSquareWithDuty(ModulationPtr m, double duty);

        [DllImport(__DllName, EntryPoint = "AUTDModulationSquareWithSamplingConfig", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ModulationPtr AUTDModulationSquareWithSamplingConfig(ModulationPtr m, SamplingConfigurationRaw config);

        [DllImport(__DllName, EntryPoint = "AUTDModulationStatic", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ModulationPtr AUTDModulationStatic();

        [DllImport(__DllName, EntryPoint = "AUTDModulationStaticWithIntensity", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ModulationPtr AUTDModulationStaticWithIntensity(ModulationPtr m, byte intensity);

        [DllImport(__DllName, EntryPoint = "AUTDModulationWithTransform", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ModulationPtr AUTDModulationWithTransform(ModulationPtr m, IntPtr f, IntPtr context);

        [DllImport(__DllName, EntryPoint = "AUTDSTMFocus", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ResultDatagram AUTDSTMFocus(STMPropsPtr props, double* points, byte* intensities, ulong size);

        [DllImport(__DllName, EntryPoint = "AUTDSTMGain", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ResultDatagram AUTDSTMGain(STMPropsPtr props, GainPtr* gains, uint size, GainSTMMode mode);

        [DllImport(__DllName, EntryPoint = "AUTDSTMGainAddGain", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ResultDatagram AUTDSTMGainAddGain(DatagramPtr stm, GainPtr gain);

        [DllImport(__DllName, EntryPoint = "AUTDSTMPropsNew", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern STMPropsPtr AUTDSTMPropsNew(double freq);

        [DllImport(__DllName, EntryPoint = "AUTDSTMPropsNewWithPeriod", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern STMPropsPtr AUTDSTMPropsNewWithPeriod(ulong p);

        [DllImport(__DllName, EntryPoint = "AUTDSTMPropsNewWithSamplingConfig", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern STMPropsPtr AUTDSTMPropsNewWithSamplingConfig(SamplingConfigurationRaw config);

        [DllImport(__DllName, EntryPoint = "AUTDSTMPropsWithStartIdx", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern STMPropsPtr AUTDSTMPropsWithStartIdx(STMPropsPtr props, int idx);

        [DllImport(__DllName, EntryPoint = "AUTDSTMPropsWithFinishIdx", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern STMPropsPtr AUTDSTMPropsWithFinishIdx(STMPropsPtr props, int idx);

        [DllImport(__DllName, EntryPoint = "AUTDSTMPropsFrequency", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern double AUTDSTMPropsFrequency(STMPropsPtr props, ulong size);

        [DllImport(__DllName, EntryPoint = "AUTDSTMPropsPeriod", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ulong AUTDSTMPropsPeriod(STMPropsPtr props, ulong size);

        [DllImport(__DllName, EntryPoint = "AUTDSTMPropsSamplingConfig", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ResultSamplingConfig AUTDSTMPropsSamplingConfig(STMPropsPtr props, ulong size);

        [DllImport(__DllName, EntryPoint = "AUTDSTMPropsStartIdx", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern int AUTDSTMPropsStartIdx(STMPropsPtr props);

        [DllImport(__DllName, EntryPoint = "AUTDSTMPropsFinishIdx", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern int AUTDSTMPropsFinishIdx(STMPropsPtr props);


    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe partial struct ControllerBuilderPtr
    {
        public IntPtr Item1;
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe partial struct ResultFirmwareInfoList
    {
        public FirmwareInfoListPtr result;
        public uint err_len;
        public IntPtr err;
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe partial struct ResultGroupKVMap
    {
        public GroupKVMapPtr result;
        public uint err_len;
        public IntPtr err;
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe partial struct LinkAuditBuilderPtr
    {
        public IntPtr Item1;
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe partial struct ResultCache
    {
        public CachePtr result;
        public uint err_len;
        public IntPtr err;
    }



}
    