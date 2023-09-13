/*
 * File: SineLegacy.cs
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

#if UNITY_2020_2_OR_NEWER
#nullable enable
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
    /// Sine wave modulation
    /// </summary>
    public sealed class SineLegacy : Internal.Modulation
    {
        private readonly float_t _freq;
        private float_t? _amp;
        private float_t? _offset;
        private uint? _freqDiv;

        /// <summary>
        /// Constructor
        /// </summary>
        /// <param name="freq">Frequency of sine wave</param>
        /// <remarks>The sine wave is defined as `amp / 2 * sin(2π * freq * t) + offset`, where `t` is time, and `amp = 1`, `offset = 0.5` by default.</remarks>
        public SineLegacy(float_t freq)
        {
            _freq = freq;
            _amp = null;
            _offset = null;
            _freqDiv = null;
        }

        /// <summary>
        /// Set amplitude
        /// </summary>
        /// <param name="amp">normalized amplitude (0.0 - 1.0)</param>
        /// <returns></returns>
        public SineLegacy WithAmp(float_t amp)
        {
            _amp = amp;
            return this;
        }

        /// <summary>
        /// Set offset
        /// </summary>
        /// <param name="offset">Offset of the sine wave</param>
        /// <returns></returns>
        public SineLegacy WithOffset(float_t offset)
        {
            _offset = offset;
            return this;
        }

        /// <summary>
        /// Set sampling frequency division
        /// </summary>
        /// <param name="div">The sampling frequency is <see cref="AUTD3.FPGASubClkFreq">AUTD3.FpgaSubClkFreq</see> / div.</param>
        /// <returns></returns>
        public SineLegacy WithSamplingFrequencyDivision(uint div)
        {
            _freqDiv = div;
            return this;
        }

        /// <summary>
        /// Set sampling frequency
        /// </summary>
        /// <returns></returns>
        public SineLegacy WithSamplingFrequency(float_t freq)
        {
            return WithSamplingFrequencyDivision((uint)(Def.FpgaSubClkFreq / freq));
        }

        /// <summary>
        /// Set sampling period
        /// </summary>
        /// <returns></returns>
        public SineLegacy WithSamplingPeriod(TimeSpan period)
        {
            return WithSamplingFrequencyDivision((uint)(Def.FpgaSubClkFreq / 1000000000.0 * (period.TotalMilliseconds * 1000.0 * 1000.0)));
        }

        public override ModulationPtr ModulationPtr()
        {
            var ptr = Base.AUTDModulationSineLegacy(_freq);
            if (_amp != null)
                ptr = Base.AUTDModulationSineLegacyWithAmp(ptr, _amp.Value);
            if (_offset != null)
                ptr = Base.AUTDModulationSineLegacyWithOffset(ptr, _offset.Value);
            if (_freqDiv != null)
                ptr = Base.AUTDModulationSineLegacyWithSamplingFrequencyDivision(ptr, _freqDiv.Value);
            return ptr;
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
