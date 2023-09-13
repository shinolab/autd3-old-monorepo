/*
 * File: Transform.cs
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

using System;
using System.Linq;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

#if UNITY_2018_3_OR_NEWER
using Math = UnityEngine.Mathf;
#endif

#if USE_SINGLE
using float_t = System.Single;
#else
using float_t = System.Double;
#endif

namespace AUTD3Sharp.Modulation
{
    using Base = NativeMethods.Base;
    using Def = NativeMethods.Def;

    public class Transform : Internal.Modulation
    {
        private readonly float_t[] _buffer;
        private readonly uint _freqDiv;

        public Transform(Internal.Modulation m, Func<int, float_t, float_t> f)
        {
            _freqDiv = m.SamplingFrequencyDivision;

            var err = new byte[256];
            var size = Base.AUTDModulationSize(m.ModulationPtr(), err);
            if (size == Def.Autd3Err) throw new AUTDException(err);
            _buffer = new float_t[size];
            if (Base.AUTDModulationCalc(m.ModulationPtr(), _buffer, err) == Def.Autd3Err)
                throw new AUTDException(err);
            _buffer = _buffer.Select((v, i) => f(i, v)).ToArray();
        }

        public override ModulationPtr ModulationPtr()
        {
            return Base.AUTDModulationCustom(_freqDiv, _buffer, (ulong)_buffer.Length);
        }
    }

    public static class TransformModulationExtensions
    {
        public static Transform WithTransform(this Internal.Modulation s, Func<int, float_t, float_t> f)
        {
            return new Transform(s, f);
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
