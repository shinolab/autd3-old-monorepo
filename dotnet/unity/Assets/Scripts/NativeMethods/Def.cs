// This file is autogenerated
using System;
using System.Runtime.InteropServices;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

namespace AUTD3Sharp
{
    namespace NativeMethods
    {
        internal static class Def
        {
            private const string DLL = "autd3capi_def";

            public const uint NumTransInUnit = 249;

            public const uint NumTransInX = 18;

            public const uint NumTransInY = 14;

            public const float TransSpacingMm = 10.16f;

            public const float DeviceHeightMm = 151.4f;

            public const float DeviceWidthMm = 192.0f;

            public const uint FpgaClkFreq = 20480000;

            public const float UltrasoundFrequency = 40000.0f;

            public const int Autd3Err = -1;

            public const int Autd3True = 1;

            public const int Autd3False = 0;
        }
    }

    public enum GainSTMMode : byte
    {
        PhaseDutyFull = 0,
        PhaseFull = 1,
        PhaseHalf = 2,
    }

    public enum TimerStrategy : byte
    {
        Sleep = 0,
        BusyWait = 1,
        NativeTimer = 2,
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct ControllerPtr
    {
        public IntPtr _0;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct GeometryPtr
    {
        public IntPtr _0;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct DevicePtr
    {
        public IntPtr _0;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct TransducerPtr
    {
        public IntPtr _0;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct LinkBuilderPtr
    {
        public IntPtr _0;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct LinkPtr
    {
        public IntPtr _0;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct DatagramPtr
    {
        public IntPtr _0;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct DatagramSpecialPtr
    {
        public IntPtr _0;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct GainPtr
    {
        public IntPtr _0;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct ModulationPtr
    {
        public IntPtr _0;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct STMPropsPtr
    {
        public IntPtr _0;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct BackendPtr
    {
        public IntPtr _0;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct ConstraintPtr
    {
        public IntPtr _0;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct GainCalcDrivesMapPtr
    {
        public IntPtr _0;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct GroupGainMapPtr
    {
        public IntPtr _0;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct GroupKVMapPtr
    {
        public IntPtr _0;
    }

}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif


