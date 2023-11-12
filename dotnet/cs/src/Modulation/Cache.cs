/*
 * File: Cache.cs
 * Project: Modulation
 * Created Date: 13/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 13/11/2023
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
    /// <summary>
    /// Modulation to cache the result of calculation
    /// </summary>
    public class Cache : Internal.Modulation, IEnumerable<float_t>, IDisposable
    {
        private bool _isDisposed;

        private CachePtr _cache;
        private readonly float_t[] _buffer;

        public Cache(Internal.Modulation m)
        {
            unsafe
            {
                _cache = NativeMethodsBase.AUTDModulationWithCache(m.ModulationPtr()).Validate();
                _buffer = new float_t[NativeMethodsBase.AUTDModulationCacheGetBufferLen(_cache)];
                fixed (float_t* p = _buffer)
                    NativeMethodsBase.AUTDModulationCacheGetBuffer(_cache, p);
            }
        }

        ~Cache()
        {
            Dispose();
        }

        public void Dispose()
        {
            if (_isDisposed) return;

            if (_cache.Item1 != IntPtr.Zero) NativeMethodsBase.AUTDModulationCacheDelete(_cache);
            _cache.Item1 = IntPtr.Zero;

            _isDisposed = true;
            GC.SuppressFinalize(this);
        }

        internal override ModulationPtr ModulationPtr()
        {
            return NativeMethodsBase.AUTDModulationCacheIntoModulation(_cache);
        }

        public float_t this[int index] => _buffer[index];

        public ReadOnlyCollection<float_t> Buffer => Array.AsReadOnly(_buffer);

        public IEnumerator<float_t> GetEnumerator()
        {
            return (IEnumerator<float_t>)_buffer.GetEnumerator();
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
