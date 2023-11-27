/*
 * File: Cache.cs
 * Project: Modulation
 * Created Date: 13/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/11/2023
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

namespace AUTD3Sharp.Modulation
{
    /// <summary>
    /// Modulation to cache the result of calculation
    /// </summary>
    public class Cache : Internal.Modulation, IEnumerable<byte>, IDisposable
    {
        private bool _isDisposed;

        private CachePtr _cache;
        private readonly byte[] _buffer;

        public Cache(Internal.Modulation m)
        {
            unsafe
            {
                _cache = NativeMethodsBase.AUTDModulationWithCache(m.ModulationPtr()).Validate();
                _buffer = new byte[NativeMethodsBase.AUTDModulationCacheGetBufferLen(_cache)];
                fixed (byte* p = _buffer)
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

        public byte this[int index] => _buffer[index];

        public ReadOnlyCollection<byte> Buffer => Array.AsReadOnly(_buffer);

        public IEnumerator<byte> GetEnumerator()
        {
            return (IEnumerator<byte>)_buffer.GetEnumerator();
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
