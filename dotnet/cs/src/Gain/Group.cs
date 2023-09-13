/*
 * File: Group.cs
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

using System;
using System.Collections.Generic;
using System.Linq;
using AUTD3Sharp.NativeMethods;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

#if UNITY_2018_3_OR_NEWER
using UnityEngine;
using Vector3 = UnityEngine.Vector3;
#else
using Vector3 = AUTD3Sharp.Utils.Vector3d;
#endif

#if USE_SINGLE
using float_t = System.Single;
#else
using float_t = System.Double;
#endif

namespace AUTD3Sharp.Gain
{
    public sealed class Group<TK> : Internal.Gain
        where TK : class
    {
        private readonly Func<Device, Transducer, TK?> _f;
        private readonly Dictionary<TK, Internal.Gain> _map;

        public Group(Func<Device, Transducer, TK?> f)
        {
            _f = f;
            _map = new Dictionary<TK, Internal.Gain>();
        }

        public Group<TK> Set(TK key, Internal.Gain gain)
        {
            _map[key] = gain;
            return this;
        }

        public override GainPtr GainPtr(Geometry geometry)
        {
            var keymap = new Dictionary<TK, int>();
            var deviceIndices = geometry.Select(dev => (uint)dev.Idx).ToArray();
            var map = Base.AUTDGainGroupCreateMap(deviceIndices, (uint)deviceIndices.Length);
            var k = 0;
            foreach (var dev in geometry)
            {
                var m = new int[dev.NumTransducers];
                foreach (var tr in dev)
                {
                    var key = _f(dev, tr);
                    if (key != null)
                    {
                        if (!keymap.ContainsKey(key)) keymap[key] = k++;
                        m[tr.LocalIdx] = keymap[key];
                    }
                    else
                        m[tr.LocalIdx] = -1;
                }
                map = Base.AUTDGainGroupMapSet(map, (uint)dev.Idx, m);
            }
            var keys = new int[_map.Count];
            var values = new GainPtr[_map.Count];
            foreach (var (kv, i) in _map.Select((v, i) => (v, i)))
            {
                keys[i] = keymap[kv.Key];
                values[i] = kv.Value.GainPtr(geometry);
            }
            return Base.AUTDGainGroup(
                    map,
                    keys,
                    values,
                    (uint)values.Length);
        }
    }

}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
