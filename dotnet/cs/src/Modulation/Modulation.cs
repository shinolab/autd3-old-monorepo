/*
 * File: Modulation.cs
 * Project: Modulation
 * Created Date: 13/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

#if UNITY_2018_3_OR_NEWER
#define USE_SINGLE
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

    /// <summary>
    /// Base class for custom modulation
    /// </summary>
    public abstract class Modulation : Internal.Modulation
    {
        private readonly uint _freqDiv;

        protected Modulation(uint freqDiv)
        {
            _freqDiv = freqDiv;
        }

        public sealed override ModulationPtr ModulationPtr()
        {
            var data = Calc();
            return Base.AUTDModulationCustom(_freqDiv, data, (ulong)data.Length);
        }

        public abstract float_t[] Calc();
    }
}
