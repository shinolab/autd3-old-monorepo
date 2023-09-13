/*
 * File: Square.cs
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
    /// Square wave modulation
    /// </summary>
    public sealed class Square : Internal.Modulation
    {
        private readonly int _freq;
        private float_t? _low;
        private float_t? _high;
        private float_t? _duty;
        private uint? _freqDiv;

        public Square(int freq)
        {
            _freq = freq;
            _low = null;
            _high = null;
            _duty = null;
            _freqDiv = null;
        }

        /// <summary>
        /// Set low level amplitude
        /// </summary>
        /// <param name="low">low level amplitude (0.0 - 1.0)</param>
        /// <returns></returns>
        public Square WithLow(float_t low)
        {
            _low = low;
            return this;
        }

        /// <summary>
        /// Set high level amplitude
        /// </summary>
        /// <param name="high">high level amplitude (0.0 - 1.0)</param>
        /// <returns></returns>
        public Square WithHigh(float_t high)
        {
            _high = high;
            return this;
        }

        /// <summary>
        /// Set duty ratio
        /// </summary>
        /// <remarks>Duty ratio is defined as `Th / (Th + Tl)`, where `Th` is high level duration, and `Tl` is low level duration.</remarks>
        /// <param name="duty">normalized amplitude (0.0 - 1.0)</param>
        /// <returns></returns>
        public Square WithDuty(float_t duty)
        {
            _duty = duty;
            return this;
        }

        /// <summary>
        /// Set sampling frequency division
        /// </summary>
        /// <param name="div">The sampling frequency is <see cref="AUTD3.FPGASubClkFreq">AUTD3.FpgaSubClkFreq</see> / div.</param>
        /// <returns></returns>
        public Square WithSamplingFrequencyDivision(uint div)
        {
            _freqDiv = div;
            return this;
        }

        /// <summary>
        /// Set sampling frequency
        /// </summary>
        /// <returns></returns>
        public Square WithSamplingFrequency(float_t freq)
        {
            return WithSamplingFrequencyDivision((uint)(Def.FpgaSubClkFreq / freq));
        }

        /// <summary>
        /// Set sampling period
        /// </summary>
        /// <returns></returns>
        public Square WithSamplingPeriod(TimeSpan period)
        {
            return WithSamplingFrequencyDivision((uint)(Def.FpgaSubClkFreq / 1000000000.0 * (period.TotalMilliseconds * 1000.0 * 1000.0)));
        }

        public override ModulationPtr ModulationPtr()
        {
            var ptr = Base.AUTDModulationSquare((uint)_freq);
            if (_low != null)
                ptr = Base.AUTDModulationSquareWithLow(ptr, _low.Value);
            if (_high != null)
                ptr = Base.AUTDModulationSquareWithHigh(ptr, _high.Value);
            if (_duty != null)
                ptr = Base.AUTDModulationSquareWithDuty(ptr, _duty.Value);
            if (_freqDiv != null)
                ptr = Base.AUTDModulationSquareWithSamplingFrequencyDivision(ptr, _freqDiv.Value);
            return ptr;
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
