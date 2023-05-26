// This file was automatically generated from header file
using System;
using System.Runtime.InteropServices;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

namespace AUTD3Sharp.NativeMethods
{
    internal static class Base
    {
        private const string DLL = "autd3capi";

        public const uint NumTransInUnit = 249;

        public const uint NumTransInX = 18;

        public const uint NumTransInY = 14;

        public const double TransSpacingMm = 10.16;

        public const double DeviceHeight = 151.4;

        public const double DeviceWidth = 192.0;

        public const uint FpgaClkFreq = 163840000;

        public const uint FpgaSubClkFreq = 20480000;

        public const int Err = - 1;

        public const int True = 1;

        public const int False = 0;

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDCreateGeometryBuilder();

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDAddDevice(IntPtr builder, double x, double y, double z, double rz1, double ry, double rz2);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDAddDeviceQuaternion(IntPtr builder, double x, double y, double z, double qw, double qx, double qy, double qz);

        [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDBuildGeometry(IntPtr builder, System.Text.StringBuilder err);

        [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDOpenController(IntPtr geometry, IntPtr link, System.Text.StringBuilder err);

        [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)][return: MarshalAs(UnmanagedType.U1)] public static extern bool AUTDClose(IntPtr cnt, System.Text.StringBuilder err);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDFreeController(IntPtr cnt);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDSetReadsFPGAInfo(IntPtr cnt, [MarshalAs(UnmanagedType.U1)] bool value);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDSetForceFan(IntPtr cnt, [MarshalAs(UnmanagedType.U1)] bool value);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern double AUTDGetSoundSpeed(IntPtr cnt);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDSetSoundSpeed(IntPtr cnt, double value);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDSetSoundSpeedFromTemp(IntPtr cnt, double temp, double k, double r, double m);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern double AUTDGetTransFrequency(IntPtr cnt, uint idx);

        [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)][return: MarshalAs(UnmanagedType.U1)] public static extern bool AUTDSetTransFrequency(IntPtr cnt, uint idx, double value, System.Text.StringBuilder err);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern ushort AUTDGetTransCycle(IntPtr cnt, uint idx);

        [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)][return: MarshalAs(UnmanagedType.U1)] public static extern bool AUTDSetTransCycle(IntPtr cnt, uint idx, ushort value, System.Text.StringBuilder err);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern double AUTDGetWavelength(IntPtr cnt, uint idx);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern double AUTDGetAttenuation(IntPtr cnt);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDSetAttenuation(IntPtr cnt, double value);

        [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)][return: MarshalAs(UnmanagedType.U1)] public static extern bool AUTDGetFPGAInfo(IntPtr cnt, byte[]? @out, System.Text.StringBuilder err);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern uint AUTDNumTransducers(IntPtr cnt);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern uint AUTDNumDevices(IntPtr cnt);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGeometryCenter(IntPtr cnt, out double x, out double y, out double z);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGeometryCenterOf(IntPtr cnt, uint devIdx, out double x, out double y, out double z);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDTransPosition(IntPtr cnt, uint trIdx, out double x, out double y, out double z);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDTransXDirection(IntPtr cnt, uint trIdx, out double x, out double y, out double z);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDTransYDirection(IntPtr cnt, uint trIdx, out double x, out double y, out double z);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDTransZDirection(IntPtr cnt, uint trIdx, out double x, out double y, out double z);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern ushort AUTDGetTransModDelay(IntPtr cnt, uint trIdx);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDSetTransModDelay(IntPtr cnt, uint trIdx, ushort delay);

        [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDGetFirmwareInfoListPointer(IntPtr cnt, System.Text.StringBuilder err);

        [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGetFirmwareInfo(IntPtr pInfoList, uint idx, System.Text.StringBuilder info, out bool isValid, out bool isSupported);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDFreeFirmwareInfoListPointer(IntPtr pInfoList);

        [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGetLatestFirmware(System.Text.StringBuilder latest);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDGainNull();

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDGainGrouped();

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainGroupedAdd(IntPtr groupedGain, uint deviceId, IntPtr gain);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDGainFocus(double x, double y, double z, double amp);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDGainBesselBeam(double x, double y, double z, double nx, double ny, double nz, double thetaZ, double amp);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDGainPlaneWave(double nx, double ny, double nz, double amp);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDGainTransducerTest();

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainTransducerTestSet(IntPtr transTest, uint id, double phase, double amp);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDGainCustom(double[]? amp, double[]? phase, ulong size);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDDeleteGain(IntPtr gain);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDModulationStatic(double amp);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDModulationSine(uint freq, double amp, double offset);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDModulationSineSquared(uint freq, double amp, double offset);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDModulationSineLegacy(double freq, double amp, double offset);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDModulationSquare(uint freq, double low, double high, double duty);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDModulationCustom(double[]? amp, ulong size, uint freqDiv);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern uint AUTDModulationSamplingFrequencyDivision(IntPtr m);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDModulationSetSamplingFrequencyDivision(IntPtr m, uint freqDiv);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern double AUTDModulationSamplingFrequency(IntPtr m);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDDeleteModulation(IntPtr m);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDFocusSTM();

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDFocusSTMAdd(IntPtr stm, double x, double y, double z, byte shift);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern double AUTDFocusSTMSetFrequency(IntPtr stm, double freq);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern int AUTDFocusSTMGetStartIdx(IntPtr stm);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern int AUTDFocusSTMGetFinishIdx(IntPtr stm);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDFocusSTMSetStartIdx(IntPtr stm, int idx);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDFocusSTMSetFinishIdx(IntPtr stm, int idx);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern double AUTDFocusSTMFrequency(IntPtr stm);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern double AUTDFocusSTMSamplingFrequency(IntPtr stm);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern uint AUTDFocusSTMSamplingFrequencyDivision(IntPtr stm);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDFocusSTMSetSamplingFrequencyDivision(IntPtr stm, uint freqDiv);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDDeleteFocusSTM(IntPtr stm);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDGainSTM();

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainSTMAdd(IntPtr stm, IntPtr gain);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainSTMSetMode(IntPtr stm, GainSTMMode mode);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern double AUTDGainSTMSetFrequency(IntPtr stm, double freq);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern int AUTDGainSTMGetStartIdx(IntPtr stm);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern int AUTDGainSTMGetFinishIdx(IntPtr stm);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainSTMSetStartIdx(IntPtr stm, int idx);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainSTMSetFinishIdx(IntPtr stm, int idx);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern double AUTDGainSTMFrequency(IntPtr stm);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern double AUTDGainSTMSamplingFrequency(IntPtr stm);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern uint AUTDGainSTMSamplingFrequencyDivision(IntPtr stm);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainSTMSetSamplingFrequencyDivision(IntPtr stm, uint freqDiv);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDDeleteGainSTM(IntPtr stm);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDSynchronize();

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDClear();

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDUpdateFlags();

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDStop();

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDModDelayConfig();

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDDeleteSpecialData(IntPtr special);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDCreateSilencer(ushort step);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDDeleteSilencer(IntPtr silencer);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDCreateAmplitudes(double amp);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDDeleteAmplitudes(IntPtr amplitudes);

        [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)] public static extern int AUTDSend(IntPtr cnt, TransMode mode, IntPtr header, IntPtr body, long timeoutNs, System.Text.StringBuilder err);

        [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)] public static extern int AUTDSendSpecial(IntPtr cnt, TransMode mode, IntPtr special, long timeoutNs, System.Text.StringBuilder err);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDLinkDebug();

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDLinkDebugLogLevel(IntPtr builder, Level level);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDLinkDebugLogFunc(IntPtr builder, Level level, IntPtr outFunc, IntPtr flushFunc);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDLinkDebugTimeout(IntPtr builder, ulong timeoutNs);

        [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDLinkDebugBuild(IntPtr builder);
    }

    public enum GainSTMMode: byte
    {
        PhaseDutyFull = 0,
        PhaseFull = 1,
        PhaseHalf = 2,
    }

    public enum TransMode: byte
    {
        Legacy = 0,
        Advanced = 1,
        AdvancedPhase = 2,
    }

    public enum Level: byte
    {
        Critical = 0,
        Error = 1,
        Warn = 2,
        Info = 3,
        Debug = 4,
        Trace = 5,
        Off = 6,
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif
