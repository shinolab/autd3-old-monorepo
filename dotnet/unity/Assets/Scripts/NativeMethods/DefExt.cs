/*
 * File: DefExt.cs
 * Project: NativeMethods
 * Created Date: 07/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

using System;
using System.Runtime.InteropServices;

namespace AUTD3Sharp
{

    internal static unsafe partial class NativeMethodsDef
    {
        internal const int AUTD3_ERR = -1;
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
    internal unsafe partial struct ControllerPtr
    {
        internal IntPtr Item1;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct GeometryPtr
    {
        internal IntPtr Item1;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct DevicePtr
    {
        internal IntPtr Item1;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct TransducerPtr
    {
        internal IntPtr Item1;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct LinkBuilderPtr
    {
        internal IntPtr Item1;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct LinkPtr
    {
        internal IntPtr Item1;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct DatagramPtr
    {
        internal IntPtr Item1;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct DatagramSpecialPtr
    {
        internal IntPtr Item1;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct GainPtr
    {
        internal IntPtr Item1;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct ModulationPtr
    {
        internal IntPtr Item1;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct STMPropsPtr
    {
        internal IntPtr Item1;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct BackendPtr
    {
        internal IntPtr Item1;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct ConstraintPtr
    {
        internal IntPtr Item1;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct GainCalcDrivesMapPtr
    {
        internal IntPtr Item1;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct GroupGainMapPtr
    {
        internal IntPtr Item1;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct GroupKVMapPtr
    {
        internal IntPtr Item1;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe struct ResultI32
    {
        internal int result;
        internal uint errLen;
        internal IntPtr err;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe struct ResultGainCalcDrivesMap
    {
        internal IntPtr result;
        internal uint errLen;
        internal IntPtr err;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe struct ResultModulation
    {
        internal ModulationPtr result;
        internal uint errLen;
        internal IntPtr err;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe struct ResultController
    {
        internal ControllerPtr result;
        internal uint errLen;
        internal IntPtr err;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe struct ResultBackend
    {
        internal BackendPtr result;
        internal uint errLen;
        internal IntPtr err;
    }
}