/*
 * File: TransTest.cs
 * Project: Gain
 * Created Date: 13/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

#if UNITY_2018_3_OR_NEWER
#define USE_SINGLE
#endif

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

using System;
using System.Runtime.InteropServices;
using AUTD3Sharp.NativeMethods;


namespace AUTD3Sharp.Gain
{
    /// <summary>
    /// Gain to set amp and phase uniformly
    /// </summary>
    public sealed class TransducerTest : Internal.Gain
    {
        [UnmanagedFunctionPointer(CallingConvention.Cdecl)] public unsafe delegate void TransducerTestDelegate(ContextPtr context, GeometryPtr geometryPtr, uint devIdx, byte trIdx, DriveRaw* raw);

        private readonly TransducerTestDelegate _f;

        public TransducerTest(Func<Device, Transducer, Drive?> f)
        {
            unsafe
            {
                _f = (context, geometryPtr, devIdx, trIdx, raw) =>
                {
                    var dev = new Device((int)devIdx, NativeMethodsBase.AUTDDevice(geometryPtr, devIdx));
                    var tr = new Transducer(trIdx, dev.Ptr);
                    var d = f(dev, tr);
                    if (d == null) return;
                    raw->Phase = d?.Phase ?? 0;
                    raw->intensity = d?.Intensity.Value ?? 0;
                };
            }
        }
        internal override GainPtr GainPtr(Geometry geometry)
        {
            return NativeMethodsBase.AUTDGainTransducerTest(Marshal.GetFunctionPointerForDelegate(_f), new ContextPtr { Item1 = IntPtr.Zero }, geometry.Ptr);
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif