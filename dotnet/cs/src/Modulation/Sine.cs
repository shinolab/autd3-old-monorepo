/*
 * File: Sine.cs
 * Project: Modulation
 * Created Date: 13/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/12/2023
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

namespace AUTD3Sharp.Modulation
{
    /// <summary>
    /// Sine wave modulation
    /// </summary>
    public sealed class Sine : Internal.ModulationWithSamplingConfig<Sine>
    {
        private readonly float_t _freq;
        private EmitIntensity? _intensity;
        private EmitIntensity? _offset;
        private float_t? _phase;
        private SamplingMode? _mode;

        /// <summary>
        /// Constructor
        /// </summary>
        /// <param name="freq">Frequency of sine wave</param>
        /// <remarks>The sine wave is defined as `amp / 2 * sin(2π * freq * t) + offset`, where `t` is time, and `amp = EmitIntensity.Max`, `offset = EmitIntensity.Max/2` by default.</remarks>
        public Sine(float_t freq)
        {
            _freq = freq;
            _intensity = null;
            _phase = null;
            _offset = null;
            _mode = null;
        }

        /// <summary>
        /// Set intensity
        /// </summary>
        /// <param name="intensity">Intensity</param>
        /// <returns></returns>
        public Sine WithIntensity(EmitIntensity intensity)
        {
            _intensity = intensity;
            return this;
        }

        /// <summary>
        /// Set intensity
        /// </summary>
        /// <param name="intensity">Intensity</param>
        /// <returns></returns>
        public Sine WithIntensity(byte intensity)
        {
            _intensity = new EmitIntensity(intensity);
            return this;
        }

        /// <summary>
        /// Set offset
        /// </summary>
        /// <param name="offset">Offset of the sine wave</param>
        /// <returns></returns>
        public Sine WithOffset(byte offset)
        {
            _offset = new EmitIntensity(offset);
            return this;
        }

        /// <summary>
        /// Set offset
        /// </summary>
        /// <param name="offset">Offset of the sine wave</param>
        /// <returns></returns>
        public Sine WithOffset(EmitIntensity offset)
        {
            _offset = offset;
            return this;
        }

        /// <summary>
        /// Set phase
        /// </summary>
        /// <param name="phase"> phase (0.0 - 2pi)</param>
        /// <returns></returns>
        public Sine WithPhase(float_t phase)
        {
            _phase = phase;
            return this;
        }

        /// <summary>
        /// Set sampling mode
        /// </summary>
        /// <param name="mode">sampling mode</param>
        /// <returns></returns>
        public Sine WithMode(SamplingMode mode)
        {
            _mode = mode;
            return this;
        }

        public static Fourier operator +(Sine a, Sine b)
            => new Fourier(a).AddComponent(b);

        internal override ModulationPtr ModulationPtr()
        {
            var ptr = NativeMethodsBase.AUTDModulationSine(_freq);
            if (_intensity != null)
                ptr = NativeMethodsBase.AUTDModulationSineWithIntensity(ptr, _intensity.Value.Value);
            if (_offset != null)
                ptr = NativeMethodsBase.AUTDModulationSineWithOffset(ptr, _offset.Value.Value);
            if (_phase != null)
                ptr = NativeMethodsBase.AUTDModulationSineWithPhase(ptr, _phase.Value);
            if (Config != null)
                ptr = NativeMethodsBase.AUTDModulationSineWithSamplingConfig(ptr, Config.Value.Internal);
            if (_mode != null)
                ptr = NativeMethodsBase.AUTDModulationSineWithMode(ptr, _mode.Value);
            return ptr;
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
