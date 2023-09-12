/*
 * File: Primitive.cs
 * Project: Gain
 * Created Date: 20/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/09/2023
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
    /// <summary>
    /// Gain to produce single focal point
    /// </summary>
    public sealed class Focus : Internal.Gain
    {
        private readonly Vector3 _point;
        private float_t? _amp;

        public Focus(Vector3 point)
        {
            _point = point;
            _amp = null;
        }

        /// <summary>
        /// Set amplitude
        /// </summary>
        /// <param name="amp">normalized amplitude (from 0 to 1)</param>
        /// <returns></returns>
        public Focus WithAmp(float_t amp)
        {
            _amp = amp;
            return this;
        }

        public override GainPtr GainPtr(Geometry geometry)
        {
            var ptr = Base.AUTDGainFocus(_point.x, _point.y, _point.z);
            if (_amp != null)
                ptr = Base.AUTDGainFocusWithAmp(ptr, _amp.Value);
            return ptr;
        }
    }

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

    /// <summary>
    /// Gain to produce a Bessel beam
    /// </summary>
    public sealed class Bessel : Internal.Gain
    {
        private readonly Vector3 _point;
        private readonly Vector3 _dir;
        private readonly float_t _thetaZ;
        private float_t? _amp;

        public Bessel(Vector3 point, Vector3 dir, float_t thetaZ)
        {
            _point = point;
            _dir = dir;
            _thetaZ = thetaZ;
            _amp = null;
        }

        /// <summary>
        /// Set amplitude
        /// </summary>
        /// <param name="amp">normalized amplitude (from 0 to 1)</param>
        /// <returns></returns>
        public Bessel WithAmp(float_t amp)
        {
            _amp = amp;
            return this;
        }

        public override GainPtr GainPtr(Geometry geometry)
        {
            var ptr = Base.AUTDGainBessel(_point.x, _point.y, _point.z, _dir.x, _dir.y, _dir.z, _thetaZ);
            if (_amp != null)
                ptr = Base.AUTDGainBesselWithAmp(ptr, _amp.Value);
            return ptr;
        }
    }

    /// <summary>
    /// Gain to produce a plane wave
    /// </summary>
    public sealed class Plane : Internal.Gain
    {
        private readonly Vector3 _dir;
        private float_t? _amp;

        public Plane(Vector3 dir)
        {
            _dir = dir;
            _amp = null;
        }

        /// <summary>
        /// Set amplitude
        /// </summary>
        /// <param name="amp">normalized amplitude (from 0 to 1)</param>
        /// <returns></returns>
        public Plane WithAmp(float_t amp)
        {
            _amp = amp;
            return this;
        }

        public override GainPtr GainPtr(Geometry geometry)
        {
            var ptr = Base.AUTDGainPlane(_dir.x, _dir.y, _dir.z);
            if (_amp != null)
                ptr = Base.AUTDGainPlaneWithAmp(ptr, _amp.Value);
            return ptr;
        }
    }

    public abstract class Gain : Internal.Gain
    {
        public override GainPtr GainPtr(Geometry geometry)
        {
            return Calc(geometry).Aggregate(Base.AUTDGainCustom(), (acc, d) => Base.AUTDGainCustomSet(acc, (uint)d.Key, d.Value, (uint)d.Value.Length));
        }

        public abstract Dictionary<int, Drive[]> Calc(Geometry geometry);

        public static Dictionary<int, Drive[]> Transform(Geometry geometry, Func<Device, Transducer, Drive> f)
        {
            return geometry.Select((dev) => (dev.Idx, dev.Select((tr) => f(dev, tr)).ToArray())).ToDictionary(x => x.Idx, x => x.Item2);
        }
    }

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

        public override GainPtr GainPtr(Geometry geometry)
        {
            var deviceIndices = geometry.Select(d => d.Idx).ToArray();
            if (_cache.Count != geometry.NumDevices|| deviceIndices.Any(i => !_cache.ContainsKey(i)))
            {
                var drives = geometry.Select(d => new Drive[d.NumTransducers]).ToArray();
                var err = new byte[256];
                if (Base.AUTDGainCalc(_g.GainPtr(geometry), geometry.Ptr, drives, err) ==
                    Def.Autd3Err) throw new AUTDException(err);
                for (var i = 0; i < geometry.NumDevices; i++) _cache[deviceIndices[i]] = drives[i];
            }
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

    /// <summary>
    /// Gain to output nothing
    /// </summary>
    public sealed class Null : Internal.Gain
    {
        public override GainPtr GainPtr(Geometry geometry) => Base.AUTDGainNull();
    }
}
