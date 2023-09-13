/*
 * File: RadiationPressure.cs
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
    /// Modulation for modulating radiation pressure
    /// </summary>
    public sealed class RadiationPressure : Internal.Modulation
    {
        private readonly uint _freqDiv;
        private readonly float_t[] _buffer;

        public RadiationPressure(Internal.Modulation m)
        {
            _freqDiv = m.SamplingFrequencyDivision;

            var err = new byte[256];
            var size = Base.AUTDModulationSize(m.ModulationPtr(), err);
            if (size == Def.Autd3Err) throw new AUTDException(err);
            var buf = new float_t[size];
            if (Base.AUTDModulationCalc(m.ModulationPtr(), buf, err) == Def.Autd3Err)
                throw new AUTDException(err);
            _buffer = buf.Select(Math.Sqrt).ToArray();
        }

        public override ModulationPtr ModulationPtr()
        {
            return Base.AUTDModulationCustom(_freqDiv, _buffer, (ulong)_buffer.Length);
        }
    }

    public static class RadiationPressureModulationExtensions
    {
        public static RadiationPressure WithRadiationPressure(this Internal.Modulation s)
        {
            return new RadiationPressure(s);
        }
    }
}
