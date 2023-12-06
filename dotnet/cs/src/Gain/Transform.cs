/*
 * File: Transform.cs
 * Project: Gain
 * Created Date: 13/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

using System;
using System.Collections.Generic;
using System.Linq;
using AUTD3Sharp.NativeMethods;


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
            var res = NativeMethodsBase.AUTDGainCalc(_g.GainPtr(geometry), geometry.Ptr).Validate();
            var drives = new Dictionary<int, Drive[]>();
            foreach (var dev in geometry.Devices())
            {
                var d = new Drive[dev.NumTransducers];
                unsafe
                {
                    fixed (Drive* p = &d[0]) NativeMethodsBase.AUTDGainCalcGetResult(res, (DriveRaw*)p, (uint)dev.Idx);
                }

                foreach (var tr in dev)
                    d[tr.Idx] = _f(dev, tr, d[tr.Idx]);
                drives[dev.Idx] = d;
            }

            NativeMethodsBase.AUTDGainCalcFreeResult(res);
            return geometry.Devices().Aggregate(NativeMethodsBase.AUTDGainCustom(),
                (acc, dev) =>
                {
                    unsafe
                    {
                        fixed (Drive* p = &drives[dev.Idx][0]) return NativeMethodsBase.AUTDGainCustomSet(acc, (uint)dev.Idx, (DriveRaw*)p, (uint)drives[dev.Idx].Length);
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
