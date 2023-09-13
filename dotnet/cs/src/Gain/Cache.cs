/*
 * File: Cache.cs
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

#if UNITY_2018_3_OR_NEWER
#define USE_SINGLE
#endif

using System.Collections.Generic;
using System.Linq;
using AUTD3Sharp.NativeMethods;

namespace AUTD3Sharp.Gain
{
    /// <summary>
    /// Gain to cache the result of calculation
    /// </summary>
    public sealed class Cache : Internal.Gain
    {
        private readonly Internal.Gain _g;
        private readonly Dictionary<int, Drive[]> _cache;

        public Cache(Internal.Gain g)
        {
            _g = g;
            _cache = new Dictionary<int, Drive[]>();
        }

        private void Init(Geometry geometry)
        {
            var deviceIndices = geometry.Select(d => d.Idx).ToArray();
            if (_cache.Count == geometry.NumDevices && deviceIndices.All(i => _cache.ContainsKey(i))) return;
            var drives = geometry.Select(d => new Drive[d.NumTransducers]).ToArray();
            var err = new byte[256];
            if (Base.AUTDGainCalc(_g.GainPtr(geometry), geometry.Ptr, drives, err) ==
                Def.Autd3Err) throw new AUTDException(err);
            for (var i = 0; i < geometry.NumDevices; i++) _cache[deviceIndices[i]] = drives[i];
        }

        public override GainPtr GainPtr(Geometry geometry)
        {
            Init(geometry);
            return geometry.Aggregate(Base.AUTDGainCustom(), (acc, dev) => Base.AUTDGainCustomSet(acc, (uint)dev.Idx, _cache[dev.Idx], (uint)_cache[dev.Idx].Length));
        }
    }

    public static class CacheGainExtensions
    {
        public static Cache WithCache(this Internal.Gain s)
        {
            return new Cache(s);
        }
    }
}
