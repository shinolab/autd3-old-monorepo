/*
 * File: Cache.cs
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

#if UNITY_2018_3_OR_NEWER
#define USE_SINGLE
#endif

using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;

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

        public ReadOnlyDictionary<int, Drive[]> Drives()
        {
            return new ReadOnlyDictionary<int, Drive[]>(_cache);
        }

        private void Init(Geometry geometry)
        {
            var deviceIndices = geometry.Devices().Select(d => d.Idx).ToArray();
            if (_cache.Count == deviceIndices.Length && deviceIndices.All(i => _cache.ContainsKey(i))) return;
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

            foreach (var dev in geometry.Devices())
            {
                var drives = new Drive[dev.NumTransducers];
                unsafe
                {
                    fixed (Drive* p = drives)
                        NativeMethodsBase.AUTDGainCalcGetResult(res, p, (uint)dev.Idx);
                }
                _cache[dev.Idx] = drives;
            }
            NativeMethodsBase.AUTDGainCalcFreeResult(res);
        }

        internal override GainPtr GainPtr(Geometry geometry)
        {
            Init(geometry);
            return geometry.Devices().Aggregate(NativeMethodsBase.AUTDGainCustom(), (acc, dev) =>
            {
                unsafe
                {
                    fixed (Drive* p = _cache[dev.Idx])
                        return NativeMethodsBase.AUTDGainCustomSet(acc, (uint)dev.Idx, p, (uint)_cache[dev.Idx].Length);
                }
            });
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
