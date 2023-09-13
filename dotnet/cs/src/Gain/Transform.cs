/*
 * File: Transform.cs
 * Project: Gain
 * Created Date: 13/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 13/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

using System;
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
            var drives = geometry.Select(d => new Drive[d.NumTransducers]).ToArray();
            var err = new byte[256];
            if (Base.AUTDGainCalc(_g.GainPtr(geometry), geometry.Ptr, drives, err) ==
                Def.Autd3Err) throw new AUTDException(err);

            foreach (var dev in geometry)
                foreach (var tr in dev)
                    drives[dev.Idx][tr.LocalIdx] = _f(dev, tr, drives[dev.Idx][tr.LocalIdx]);

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
