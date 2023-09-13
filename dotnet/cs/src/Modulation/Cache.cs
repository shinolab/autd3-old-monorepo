/*
 * File: Cache.cs
 * Project: Modulation
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
    public class Cache : Internal.Modulation, IEnumerable<float_t>
    {
        private readonly uint _freqDiv;

        public Cache(Internal.Modulation m)
        {
            _freqDiv = m.SamplingFrequencyDivision;

            var err = new byte[256];
            var size = Base.AUTDModulationSize(m.ModulationPtr(), err);
            if (size == Def.Autd3Err) throw new AUTDException(err);
            Buffer = new float_t[size];
            if (Base.AUTDModulationCalc(m.ModulationPtr(), Buffer, err) == Def.Autd3Err)
                throw new AUTDException(err);
        }

        public override ModulationPtr ModulationPtr()
        {
            return Base.AUTDModulationCustom(_freqDiv, Buffer, (ulong)Buffer.Length);
        }

        public float_t this[int index]
        {
            get => Buffer[index];
            set => Buffer[index] = value;
        }

        public float_t[] Buffer { get; }

        public IEnumerator<float_t> GetEnumerator() => Buffer.AsEnumerable().GetEnumerator();

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
