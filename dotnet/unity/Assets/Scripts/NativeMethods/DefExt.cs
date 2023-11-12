/*
 * File: DefExt.cs
 * Project: NativeMethods
 * Created Date: 07/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 13/11/2023
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
    internal unsafe partial struct CachePtr
    {
        internal IntPtr Item1;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct FirmwareInfoListPtr
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
    internal unsafe struct ResultGainCalcDrivesMap
    {
        internal GainCalcDrivesMapPtr result;
        internal uint err_len;
        internal IntPtr err;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe struct ResultModulation
    {
        internal ModulationPtr result;
        internal uint err_len;
        internal IntPtr err;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe struct ResultController
    {
        internal ControllerPtr result;
        internal uint err_len;
        internal IntPtr err;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe struct ResultBackend
    {
        internal BackendPtr result;
        internal uint err_len;
        internal IntPtr err;
    }


    internal static class ResultExtensions
    {
        internal static int Validate(this ResultI32 res)
        {
            if (res.result == NativeMethodsDef.AUTD3_ERR)
            {
                var err = new byte[res.err_len];
                unsafe
                {
                    fixed (byte* p = err) NativeMethodsDef.AUTDGetErr(res.err, p);
                }
                throw new AUTDException(err);
            }
            return res.result;
        }

        internal static GainCalcDrivesMapPtr Validate(this ResultGainCalcDrivesMap res)
        {
            if (res.result.Item1 == IntPtr.Zero)
            {
                var err = new byte[res.err_len];
                unsafe
                {
                    fixed (byte* p = err) NativeMethodsDef.AUTDGetErr(res.err, p);
                }
                throw new AUTDException(err);
            }
            return res.result;
        }

        internal static FirmwareInfoListPtr Validate(this ResultFirmwareInfoList res)
        {
            if (res.result.Item1 == IntPtr.Zero)
            {
                var err = new byte[res.err_len];
                unsafe
                {
                    fixed (byte* p = err) NativeMethodsDef.AUTDGetErr(res.err, p);
                }
                throw new AUTDException(err);
            }
            return res.result;
        }

        internal static GroupKVMapPtr Validate(this ResultGroupKVMap res)
        {
            if (res.result.Item1 == IntPtr.Zero)
            {
                var err = new byte[res.err_len];
                unsafe
                {
                    fixed (byte* p = err) NativeMethodsDef.AUTDGetErr(res.err, p);
                }
                throw new AUTDException(err);
            }
            return res.result;
        }

        internal static CachePtr Validate(this ResultCache res)
        {
            if (res.result.Item1 == IntPtr.Zero)
            {
                var err = new byte[res.err_len];
                unsafe
                {
                    fixed (byte* p = err) NativeMethodsDef.AUTDGetErr(res.err, p);
                }
                throw new AUTDException(err);
            }
            return res.result;
        }
    }
}
