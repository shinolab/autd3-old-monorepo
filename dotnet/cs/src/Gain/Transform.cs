/*
 * File: Transform.cs
 * Project: Gain
 * Created Date: 13/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/09/2023
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

        public override GainPtr GainPtr(Geometry geometry)
        {
            var err = new byte[256];
            var res = Base.AUTDGainCalc(_g.GainPtr(geometry), geometry.Ptr, err);
            if (res._0 == IntPtr.Zero) throw new AUTDException(err);

            var drives = new Dictionary<int, Drive[]>();
            foreach (var dev in geometry)
            {
                var d = new Drive[dev.NumTransducers];
                Base.AUTDGainCalcGetResult(res, d, (uint)dev.Idx);
                foreach (var tr in dev)
                    d[tr.LocalIdx] = _f(dev, tr, d[tr.LocalIdx]);
                drives[dev.Idx] = d;
            }
            Base.AUTDGainCalcFreeResult(res);
            return geometry.Aggregate(Base.AUTDGainCustom(), (acc, dev) => Base.AUTDGainCustomSet(acc, (uint)dev.Idx, drives[dev.Idx], (uint)drives[dev.Idx].Length));
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
