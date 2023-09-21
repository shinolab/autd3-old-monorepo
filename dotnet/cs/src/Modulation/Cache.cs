/*
 * File: Cache.cs
 * Project: Modulation
 * Created Date: 13/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/09/2023
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

#if USE_SINGLE
using float_t = System.Single;
#else
using float_t = System.Double;
#endif

namespace AUTD3Sharp.Modulation
{
    using Base = NativeMethods.Base;
    using Def = NativeMethods.Def;

    /// <summary>
    /// Modulation to cache the result of calculation
    /// </summary>
    public class Cache : Internal.Modulation, IEnumerable<float_t>, IDisposable
    {
        private bool _isDisposed;

        private ModulationCachePtr _cache;
        private readonly float_t[] _buffer;

        public Cache(Internal.Modulation m)
        {
            var err = new byte[256];
            _cache = Base.AUTDModulationWithCache(m.ModulationPtr(), err);
            if (_cache._0 == System.IntPtr.Zero) throw new AUTDException(err);

            var n = Base.AUTDModulationCacheGetBufferSize(_cache);
            _buffer = new float_t[n];
            Base.AUTDModulationCacheGetBuffer(_cache, _buffer);
        }

        ~Cache()
        {
            Dispose();
        }

        public void Dispose()
        {
            if (_isDisposed) return;

            if (_cache._0 != IntPtr.Zero) Base.AUTDModulationCacheDelete(_cache);
            _cache._0 = IntPtr.Zero;

            _isDisposed = true;
            GC.SuppressFinalize(this);
        }

        public override ModulationPtr ModulationPtr()
        {
            return Base.AUTDModulationCacheIntoModulation(_cache);
        }

        public float_t this[int index] => _buffer[index];

        public ReadOnlyCollection<float_t> Buffer => Array.AsReadOnly(_buffer);

        public IEnumerator<float_t> GetEnumerator()
        {
            foreach (var e in _buffer) yield return e;
        }

        System.Collections.IEnumerator System.Collections.IEnumerable.GetEnumerator() => GetEnumerator();
    }

    public static class CacheModulationExtensions
    {
        public static Cache WithCache(this Internal.Modulation s)
        {
            return new Cache(s);
        }
    }
}
