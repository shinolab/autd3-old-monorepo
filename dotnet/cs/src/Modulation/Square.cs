/*
 * File: Square.cs
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
    /// <summary>
    /// Square wave modulation
    /// </summary>
    public sealed class Square : Internal.ModulationWithFreqDiv<Square>
    {
        private readonly int _freq;
        private EmitIntensity? _low;
        private EmitIntensity? _high;
        private float_t? _duty;

        public Square(int freq)
        {
            _freq = freq;
            _low = null;
            _high = null;
            _duty = null;
        }

        /// <summary>
        /// Set low level amplitude
        /// </summary>
        /// <param name="low">low level intensity</param>
        /// <returns></returns>
        public Square WithLow(byte low)
        {
            _low = new EmitIntensity(low);
            return this;
        }

        /// <summary>
        /// Set low level amplitude
        /// </summary>
        /// <param name="low">low level intensity</param>
        /// <returns></returns>
        public Square WithLow(EmitIntensity low)
        {
            _low = low;
            return this;
        }

        /// <summary>
        /// Set high level amplitude
        /// </summary>
        /// <param name="high">high level intensity</param>
        /// <returns></returns>
        public Square WithHigh(byte high)
        {
            _high = new EmitIntensity(high);
            return this;
        }

        /// <summary>
        /// Set high level amplitude
        /// </summary>
        /// <param name="high">high level intensity</param>
        /// <returns></returns>
        public Square WithHigh(EmitIntensity high)
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

        internal override ModulationPtr ModulationPtr()
        {
            var ptr = NativeMethodsBase.AUTDModulationSquare((uint)_freq);
            if (_low != null)
                ptr = NativeMethodsBase.AUTDModulationSquareWithLow(ptr, _low.Value.Value);
            if (_high != null)
                ptr = NativeMethodsBase.AUTDModulationSquareWithHigh(ptr, _high.Value.Value);
            if (_duty != null)
                ptr = NativeMethodsBase.AUTDModulationSquareWithDuty(ptr, _duty.Value);
            if (Config != null)
                ptr = NativeMethodsBase.AUTDModulationSquareWithSamplingConfig(ptr, Config.Value.Internal);
            return ptr;
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
