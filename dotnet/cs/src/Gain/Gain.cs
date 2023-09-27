/*
 * File: Gain.cs
 * Project: Gain
 * Created Date: 13/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 27/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

#if UNITY_2018_3_OR_NEWER
#define USE_SINGLE
#endif

using System;
using System.Collections.Generic;
using System.Linq;
using AUTD3Sharp.NativeMethods;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

namespace AUTD3Sharp.Gain
{
    public abstract class Gain : Internal.Gain
    {
        public override GainPtr GainPtr(Geometry geometry)
        {
            return Calc(geometry).Aggregate(Base.AUTDGainCustom(), (acc, d) => Base.AUTDGainCustomSet(acc, (uint)d.Key, d.Value, (uint)d.Value.Length));
        }

        public abstract Dictionary<int, Drive[]> Calc(Geometry geometry);

        public static Dictionary<int, Drive[]> Transform(Geometry geometry, Func<Device, Transducer, Drive> f)
        {
            return geometry.Devices().Select(dev => (dev.Idx, dev.Select(tr => f(dev, tr)).ToArray())).ToDictionary(x => x.Idx, x => x.Item2);
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
