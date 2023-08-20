/*
 * File: Primitive.cs
 * Project: Gain
 * Created Date: 20/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */


#if UNITY_2018_3_OR_NEWER
#define USE_SINGLE
#endif

using AUTD3Sharp.Internal;

using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Linq;

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

namespace AUTD3Sharp
{
    using Base = NativeMethods.Base;
    using Def = NativeMethods.Def;

    namespace Gain
    {
        [ComVisible(false)]
        public abstract class IGain : IBody
        {
            public DatagramBodyPtr Ptr(Geometry geometry) => Base.AUTDGainIntoDatagram(GainPtr(geometry));

            public abstract GainPtr GainPtr(Geometry geometry);
        }

        /// <summary>
        /// Gain to produce single focal point
        /// </summary>
        public sealed class Focus : IGain
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

        public sealed class GroupByDevice<K> : IGain
            where K : class
        {
            private readonly Func<int, K?> _f;
            private readonly Dictionary<K, IGain> _map;

            public GroupByDevice(Func<int, K?> f)
            {
                _f = f;
                _map = new Dictionary<K, IGain>();
            }

            public GroupByDevice<K> Set(K key, IGain gain)
            {
                _map[key] = gain;
                return this;
            }

            public override GainPtr GainPtr(Geometry geometry)
            {
                var keymap = new Dictionary<K, int>();
                var map = new int[geometry.NumDevices];

                int k = 0;
                for (var dev = 0; dev < geometry.NumDevices; dev++)
                {
                    var key = _f(dev);
                    if (key != null)
                    {
                        if (!keymap.ContainsKey(key)) keymap[key] = k++;
                        map[dev] = keymap[key];
                    }
                    else
                        map[dev] = -1;
                }
                var keys = new int[_map.Count];
                var values = new GainPtr[_map.Count];
                foreach (var (kv, i) in _map.Select((v, i) => (v, i)))
                {
                    keys[i] = keymap[kv.Key];
                    values[i] = kv.Value.GainPtr(geometry);
                }
                return Base.AUTDGainGroupByDevice(
                        map,
                        (ulong)map.Length,
                        keys,
                        values,
                        (ulong)values.Length);
            }
        }

        public sealed class GroupByTransducer<K> : IGain
            where K : class
        {
            private readonly Func<Transducer, K?> _f;
            private readonly Dictionary<K, IGain> _map;

            public GroupByTransducer(Func<Transducer, K?> f)
            {
                _f = f;
                _map = new Dictionary<K, IGain>();
            }

            public GroupByTransducer<K> Set(K key, IGain gain)
            {
                _map[key] = gain;
                return this;
            }

            public override GainPtr GainPtr(Geometry geometry)
            {
                var keymap = new Dictionary<K, int>();
                var map = new int[geometry.NumTransducers];

                int k = 0;
                foreach (var tr in geometry)
                {
                    var key = _f(tr);
                    if (key != null)
                    {
                        if (!keymap.ContainsKey(key)) keymap[key] = k++;
                        map[tr.Idx] = keymap[key];
                    }
                    else
                        map[tr.Idx] = -1;
                }
                var keys = new int[_map.Count];
                var values = new GainPtr[_map.Count];
                foreach (var (kv, i) in _map.Select((v, i) => (v, i)))
                {
                    keys[i] = keymap[kv.Key];
                    values[i] = kv.Value.GainPtr(geometry);
                }
                return Base.AUTDGainGroupByDevice(
                        map,
                        (ulong)map.Length,
                        keys,
                        values,
                        (ulong)values.Length);
            }
        }


        public sealed class Group
        {
            public static GroupByDevice<K> ByDevice<K>(Func<int, K?> f) where K : class
            {
                return new GroupByDevice<K>(f);
            }
            public static GroupByTransducer<K> ByTransducer<K>(Func<Transducer, K?> f) where K : class
            {
                return new GroupByTransducer<K>(f);
            }
        }

        /// <summary>
        /// Gain to produce a Bessel beam
        /// </summary>
        public sealed class Bessel : IGain
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
        public sealed class Plane : IGain
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

        public abstract class Gain : IGain
        {
            sealed public override GainPtr GainPtr(Geometry geometry)
            {
                var drives = Calc(geometry);
                return Base.AUTDGainCustom(drives, (ulong)drives.Length);
            }

            public abstract Drive[] Calc(Geometry geometry);

            public static Drive[] Transform(Geometry geometry, Func<Transducer, Drive> f)
            {
                return geometry.Select(f).ToArray();
            }
        }

        /// <summary>
        /// Gain to cache the result of calculation
        /// </summary>
        public sealed class Cache : IGain, IEnumerable<Drive>
        {
            public Cache(IGain g, Geometry geometry)
            {
                var err = new byte[256];
                Drives = new Drive[geometry.NumTransducers];
                if (Base.AUTDGainCalc(g.GainPtr(geometry), geometry.Ptr, Drives, err) == Def.Autd3Err)
                    throw new AUTDException(err);
            }

            public override GainPtr GainPtr(Geometry geometry)
            {
                return Base.AUTDGainCustom(Drives, (ulong)Drives.Length);
            }

            public Drive this[int index]
            {
                get => Drives[index];
                set => Drives[index] = value;
            }

            public Drive[] Drives { get; }

            public IEnumerator<Drive> GetEnumerator() => Drives.AsEnumerable().GetEnumerator();

            System.Collections.IEnumerator System.Collections.IEnumerable.GetEnumerator() => GetEnumerator();
        }

        public static class CacheGainExtensions
        {
            public static Cache WithCache(this IGain s, Geometry geometry)
            {
                return new Cache(s, geometry);
            }
        }

        /// <summary>
        /// Gain to output nothing
        /// </summary>
        public sealed class Null : IGain
        {
            public override GainPtr GainPtr(Geometry geometry) => Base.AUTDGainNull();
        }
    }
}
