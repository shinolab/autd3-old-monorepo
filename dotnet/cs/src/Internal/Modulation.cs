/*
 * File: Modulation.cs
 * Project: Internal
 * Created Date: 08/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#if UNITY_2018_3_OR_NEWER
#define USE_SINGLE
#endif

using System;
using System.Runtime.InteropServices;
using AUTD3Sharp.NativeMethods;

#if USE_SINGLE
using float_t = System.Single;
#else
using float_t = System.Double;
#endif

namespace AUTD3Sharp.Internal
{
    [ComVisible(false)]
    public abstract class Modulation : IDatagram
    {
        public float_t SamplingFrequency => (float_t)Def.FpgaSubClkFreq / SamplingFrequencyDivision;
        public uint SamplingFrequencyDivision => Base.AUTDModulationSamplingFrequencyDivision(ModulationPtr());

        public DatagramPtr Ptr(Geometry geometry) => Base.AUTDModulationIntoDatagram(ModulationPtr());

        public abstract ModulationPtr ModulationPtr();

        public int Length
        {
            get
            {
                var err = new byte[256];
                var n = Base.AUTDModulationSize(ModulationPtr(), err);
                if (n < 0) throw new AUTDException(err);
                return n;
            }
        }
    }

    public abstract class ModulationWithFreqDiv<T> : Modulation
        where T : ModulationWithFreqDiv<T>
    {
        protected uint? FreqDiv;

        /// <summary>
        /// Set sampling frequency division
        /// </summary>
        /// <param name="div">The sampling frequency is <see cref="AUTD3.FPGASubClkFreq">AUTD3.FPGASubClkFreq</see> / div.</param>
        /// <returns></returns>
        public T WithSamplingFrequencyDivision(uint div)
        {
            FreqDiv = div;
            return (T)this;
        }

        /// <summary>
        /// Set sampling frequency
        /// </summary>
        /// <returns></returns>
        public T WithSamplingFrequency(float_t freq)
        {
            return WithSamplingFrequencyDivision((uint)(Def.FpgaSubClkFreq / freq));
        }

        /// <summary>
        /// Set sampling period
        /// </summary>
        /// <returns></returns>
        public T WithSamplingPeriod(TimeSpan period)
        {
            return WithSamplingFrequencyDivision((uint)(Def.FpgaSubClkFreq / 1000000000.0 * (period.TotalMilliseconds * 1000.0 * 1000.0)));
        }
    }
}
