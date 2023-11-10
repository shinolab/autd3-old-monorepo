/*
 * File: Cache.cs
 * Project: Modulation
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

        private ResultCache _cache;
        private readonly float_t[] _buffer;

        public Cache(Internal.Modulation m)
        {
            unsafe
            {
                _cache = NativeMethodsBase.AUTDModulationWithCache(m.ModulationPtr());
                if (_cache.result == IntPtr.Zero)
                {
                    var err = new byte[_cache.err_len];
                    fixed (byte* p = err)
                        NativeMethodsDef.AUTDGetErr(_cache.err, p);
                    throw new AUTDException(err);
                }
                _buffer = new float_t[_cache.buffer_len];
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

            if (_cache.result != IntPtr.Zero) NativeMethodsBase.AUTDModulationCacheDelete(_cache);
            _cache.result = IntPtr.Zero;

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
