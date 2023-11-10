/*
 * File: Transform.cs
 * Project: Gain
 * Created Date: 13/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 07/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

using System;
using System.Collections.Generic;
using System.Linq;

namespace AUTD3Sharp.Gain
{

    public sealed class Transform : Internal.Gain
    {
        private readonly Internal.Gain _g;
        private readonly Func<Device, Transducer, Drive, Drive> _f;

        public Transform(Internal.Gain g, Func<Device, Transducer, Drive, Drive> f)
        {
            _g = g;
            _f = f;
        }

        internal override GainPtr GainPtr(Geometry geometry)
        {
            var res = NativeMethodsBase.AUTDGainCalc(_g.GainPtr(geometry), geometry.Ptr);
            if (res.result == IntPtr.Zero)
            {
                var err = new byte[res.errLen];
                unsafe
                {
                    fixed (byte* p = err) NativeMethodsDef.AUTDGetErr(res.err, p);
                }
                throw new AUTDException(err);
            }

            var drives = new Dictionary<int, Drive[]>();
            foreach (var dev in geometry.Devices())
            {
                var d = new Drive[dev.NumTransducers];
                unsafe
                {
                    fixed (Drive* p = d) NativeMethodsBase.AUTDGainCalcGetResult(res, p, (uint)dev.Idx);
                }

                foreach (var tr in dev)
                    d[tr.LocalIdx] = _f(dev, tr, d[tr.LocalIdx]);
                drives[dev.Idx] = d;
            }

            NativeMethodsBase.AUTDGainCalcFreeResult(res);
            return geometry.Devices().Aggregate(NativeMethodsBase.AUTDGainCustom(),
                (acc, dev) =>
                {
                    unsafe
                    {
                        fixed (Drive* p = drives[dev.Idx]) return NativeMethodsBase.AUTDGainCustomSet(acc, (uint)dev.Idx, p, (uint)drives[dev.Idx].Length);
                    }
                });
        }
    }

    public static class TransformGainExtensions
    {
        public static Transform WithTransform(this Internal.Gain s, Func<Device, Transducer, Drive, Drive> f)
        {
            return new Transform(s, f);
        }
    }
}
