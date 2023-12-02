/*
 * File: Uniform.cs
 * Project: Gain
 * Created Date: 13/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/12/2023
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

using AUTD3Sharp.NativeMethods;

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
        private readonly EmitIntensity _intensity;
        private Phase? _phase;

        public Uniform(byte intensity)
        {
            _intensity = new EmitIntensity(intensity);
            _phase = null;
        }

        public Uniform(EmitIntensity intensity)
        {
            _intensity = intensity;
            _phase = null;
        }

        /// <summary>
        /// Set phase
        /// </summary>
        /// <param name="phase">phase</param>
        /// <returns></returns>
        public Uniform WithPhase(Phase phase)
        {
            _phase = phase;
            return this;
        }

        internal override GainPtr GainPtr(Geometry geometry)
        {
            var ptr = NativeMethodsBase.AUTDGainUniform(_intensity.Value);
            if (_phase != null)
                ptr = NativeMethodsBase.AUTDGainUniformWithPhase(ptr, _phase.Value.Value);
            return ptr;
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
