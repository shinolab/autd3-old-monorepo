/*
 * File: DefExt.cs
 * Project: NativeMethods
 * Created Date: 07/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */


#if UNITY_2018_3_OR_NEWER
#define USE_SINGLE
#endif

#if USE_SINGLE
using float_t = System.Single;
#else
using float_t = System.Double;
#endif

using System;
using System.Runtime.InteropServices;

namespace AUTD3Sharp
{
    public enum GainSTMMode : byte
    {
        PhaseIntensityFull = 0,
        PhaseFull = 1,
        PhaseHalf = 2,
    }

    public enum TimerStrategy : byte
    {
        Sleep = 0,
        BusyWait = 1,
        NativeTimer = 2,
    }

    public enum SyncMode : byte
    {
        FreeRun = 0,
        DC = 1,
    }

    public static class SyncModeExt
    {
        public static NativeMethods.SyncMode Into(this SyncMode mode)
        {
            return mode switch
            {
                SyncMode.FreeRun => NativeMethods.SyncMode.FreeRun,
                SyncMode.DC => NativeMethods.SyncMode.DC,
                _ => throw new ArgumentOutOfRangeException(nameof(mode), mode, null)
            };
        }
    }

    namespace NativeMethods
    {

        public static partial class NativeMethodsDef
        {
            public const int AUTD3_ERR = -1;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct DriveRaw
        {
            public float_t Phase;
            public byte intensity;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct ControllerPtr
        {
            public IntPtr Item1;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct GeometryPtr
        {
            public IntPtr Item1;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct DevicePtr
        {
            public IntPtr Item1;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct TransducerPtr
        {
            public IntPtr Item1;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct LinkBuilderPtr
        {
            public IntPtr Item1;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct LinkPtr
        {
            public IntPtr Item1;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct DatagramPtr
        {
            public IntPtr Item1;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct DatagramSpecialPtr
        {
            public IntPtr Item1;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct GainPtr
        {
            public IntPtr Item1;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct ModulationPtr
        {
            public IntPtr Item1;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct STMPropsPtr
        {
            public IntPtr Item1;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct BackendPtr
        {
            public IntPtr Item1;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct CachePtr
        {
            public IntPtr Item1;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct FirmwareInfoListPtr
        {
            public IntPtr Item1;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct EmissionConstraintPtr
        {
            public IntPtr Item1;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct GainCalcDrivesMapPtr
        {
            public IntPtr Item1;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct GroupGainMapPtr
        {
            public IntPtr Item1;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct GroupKVMapPtr
        {
            public IntPtr Item1;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct ResultI32
        {
            public int result;
            public uint err_len;
            public IntPtr err;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct ResultGainCalcDrivesMap
        {
            public GainCalcDrivesMapPtr result;
            public uint err_len;
            public IntPtr err;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct ResultModulation
        {
            public ModulationPtr result;
            public uint err_len;
            public IntPtr err;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct ResultController
        {
            public ControllerPtr result;
            public uint err_len;
            public IntPtr err;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct ResultBackend
        {
            public BackendPtr result;
            public uint err_len;
            public IntPtr err;
        }


        [StructLayout(LayoutKind.Sequential)]
        public struct ResultDatagram
        {
            public DatagramPtr result;
            public uint err_len;
            public IntPtr err;
        }

        public static class ResultExtensions
        {
            public static int Validate(this ResultI32 res)
            {
                if (res.result != NativeMethodsDef.AUTD3_ERR) return res.result;
                var err = new byte[res.err_len];
                unsafe
                {
                    fixed (byte* p = err) NativeMethodsDef.AUTDGetErr(res.err, p);
                }
                throw new AUTDException(err);
            }

            public static GainCalcDrivesMapPtr Validate(this ResultGainCalcDrivesMap res)
            {
                if (res.result.Item1 != IntPtr.Zero) return res.result;
                var err = new byte[res.err_len];
                unsafe
                {
                    fixed (byte* p = err) NativeMethodsDef.AUTDGetErr(res.err, p);
                }
                throw new AUTDException(err);
            }

            public static FirmwareInfoListPtr Validate(this ResultFirmwareInfoList res)
            {
                if (res.result.Item1 != IntPtr.Zero) return res.result;
                var err = new byte[res.err_len];
                unsafe
                {
                    fixed (byte* p = err) NativeMethodsDef.AUTDGetErr(res.err, p);
                }
                throw new AUTDException(err);
            }

            public static GroupKVMapPtr Validate(this ResultGroupKVMap res)
            {
                if (res.result.Item1 != IntPtr.Zero) return res.result;
                var err = new byte[res.err_len];
                unsafe
                {
                    fixed (byte* p = err) NativeMethodsDef.AUTDGetErr(res.err, p);
                }
                throw new AUTDException(err);
            }

            public static CachePtr Validate(this ResultCache res)
            {
                if (res.result.Item1 != IntPtr.Zero) return res.result;
                var err = new byte[res.err_len];
                unsafe
                {
                    fixed (byte* p = err) NativeMethodsDef.AUTDGetErr(res.err, p);
                }
                throw new AUTDException(err);
            }

            public static DatagramPtr Validate(this ResultDatagram res)
            {
                if (res.result.Item1 != IntPtr.Zero) return res.result;
                var err = new byte[res.err_len];
                unsafe
                {
                    fixed (byte* p = err) NativeMethodsDef.AUTDGetErr(res.err, p);
                }
                throw new AUTDException(err);
            }

            public static SamplingConfigurationRaw Validate(this ResultSamplingConfig res)
            {
                if (res.result.div != 0) return res.result;
                var err = new byte[res.err_len];
                unsafe
                {
                    fixed (byte* p = err) NativeMethodsDef.AUTDGetErr(res.err, p);
                }
                throw new AUTDException(err);
            }
        }
    }
}