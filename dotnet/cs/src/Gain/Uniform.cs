/*
 * File: Uniform.cs
 * Project: Gain
 * Created Date: 13/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 07/11/2023
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

namespace AUTD3Sharp.Gain
{
    /// <summary>
    /// Gain to set amp and phase uniformly
    /// </summary>
    public sealed class Uniform : Internal.Gain
    {
        private readonly float_t _amp;
        private float_t? _phase;

        public Uniform(float_t amp)
        {
            _amp = amp;
            _phase = null;
        }

        /// <summary>
        /// Set phase
        /// </summary>
        /// <param name="phase">phase (from 0 to 2pi)</param>
        /// <returns></returns>
        public Uniform WithPhase(float_t phase)
        {
            _phase = phase;
            return this;
        }

        internal override GainPtr GainPtr(Geometry geometry)
        {
            var ptr = NativeMethodsBase.AUTDGainUniform(_amp);
            if (_phase != null)
                ptr = NativeMethodsBase.AUTDGainUniformWithPhase(ptr, _phase.Value);
            return ptr;
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
