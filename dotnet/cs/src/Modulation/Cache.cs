/*
 * File: Cache.cs
 * Project: Modulation
 * Created Date: 13/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 07/12/2023
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
using System.Diagnostics.CodeAnalysis;
using AUTD3Sharp.NativeMethods;

namespace AUTD3Sharp.Modulation
{
    /// <summary>
    /// Modulation to cache the result of calculation
    /// </summary>
    public class Cache : Internal.Modulation, IEnumerable<EmitIntensity>, IDisposable
    {
        private bool _isDisposed;

        private CachePtr _cache;
        private readonly EmitIntensity[] _buffer;

        public Cache(Internal.Modulation m)
        {
            unsafe
            {
                _cache = NativeMethodsBase.AUTDModulationWithCache(m.ModulationPtr()).Validate();
                _buffer = new EmitIntensity[NativeMethodsBase.AUTDModulationCacheGetBufferLen(_cache)];
                fixed (EmitIntensity* p = &_buffer[0])
                    NativeMethodsBase.AUTDModulationCacheGetBuffer(_cache, (byte*)p);
            }
        }


        [ExcludeFromCodeCoverage]
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

        public EmitIntensity this[int index] => _buffer[index];

        public ReadOnlyCollection<EmitIntensity> Buffer => Array.AsReadOnly(_buffer);

        public IEnumerator<EmitIntensity> GetEnumerator()
        {
            foreach (var e in _buffer)
                yield return e;
        }


        [ExcludeFromCodeCoverage] System.Collections.IEnumerator System.Collections.IEnumerable.GetEnumerator() => GetEnumerator();
    }

    public static class CacheModulationExtensions
    {
        public static Cache WithCache(this Internal.Modulation s)
        {
            return new Cache(s);
        }
    }
}
