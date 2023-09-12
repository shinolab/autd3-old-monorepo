/*
 * File: Primitive.cs
 * Project: Modulation
 * Created Date: 20/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

#if UNITY_2018_3_OR_NEWER
#define USE_SINGLE
#endif

using System;
using System.Collections.Generic;
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

    /// <summary>
    /// Without modulation
    /// </summary>
    public sealed class Static : Internal.Modulation
    {
        private float_t? _amp;

        /// <summary>
        /// Set amplitude
        /// </summary>
        /// <param name="amp">normalized amplitude (0.0 - 1.0)</param>
        /// <returns></returns>
        public Static WithAmp(float_t amp)
        {
            _amp = amp;
            return this;
        }

        public override ModulationPtr ModulationPtr()
        {
            var ptr = Base.AUTDModulationStatic();
            if (_amp != null)
                ptr = Base.AUTDModulationStaticWithAmp(ptr, _amp.Value);
            return ptr;
        }
    }

    /// <summary>
    /// Sine wave modulation
    /// </summary>
    public sealed class Sine : Internal.Modulation
    {
        private readonly int _freq;
        private float_t? _amp;
        private float_t? _phase;
        private float_t? _offset;
        private uint? _freqDiv;

        /// <summary>
        /// Constructor
        /// </summary>
        /// <param name="freq">Frequency of sine wave</param>
        /// <remarks>The sine wave is defined as `amp / 2 * sin(2π * freq * t) + offset`, where `t` is time, and `amp = 1`, `offset = 0.5` by default.</remarks>
        public Sine(int freq)
        {
            _freq = freq;
            _phase = null;
            _amp = null;
            _offset = null;
            _freqDiv = null;
        }

        /// <summary>
        /// Set amplitude
        /// </summary>
        /// <param name="amp">normalized amplitude (0.0 - 1.0)</param>
        /// <returns></returns>
        public Sine WithAmp(float_t amp)
        {
            _amp = amp;
            return this;
        }

        /// <summary>
        /// Set phase
        /// </summary>
        /// <param name="phase">Phase of the sine wave</param>
        /// <returns></returns>
        public Sine WithPhase(float_t phase)
        {
            _phase = phase;
            return this;
        }

        /// <summary>
        /// Set offset
        /// </summary>
        /// <param name="offset">Offset of the sine wave</param>
        /// <returns></returns>
        public Sine WithOffset(float_t offset)
        {
            _offset = offset;
            return this;
        }

        /// <summary>
        /// Set sampling frequency division
        /// </summary>
        /// <param name="div">The sampling frequency is <see cref="AUTD3.FPGASubClkFreq">AUTD3.FpgaSubClkFreq</see> / div.</param>
        /// <returns></returns>
        public Sine WithSamplingFrequencyDivision(uint div)
        {
            _freqDiv = div;
            return this;
        }

        /// <summary>
        /// Set sampling frequency
        /// </summary>
        /// <returns></returns>
        public Sine WithSamplingFrequency(float_t freq)
        {
            return WithSamplingFrequencyDivision((uint)((float_t)Def.FpgaSubClkFreq / freq));
        }

        public override ModulationPtr ModulationPtr()
        {
            var ptr = Base.AUTDModulationSine((uint)_freq);
            if (_amp != null)
                ptr = Base.AUTDModulationSineWithAmp(ptr, _amp.Value);
            if (_phase != null)
                ptr = Base.AUTDModulationSineWithPhase(ptr, _phase.Value);
            if (_offset != null)
                ptr = Base.AUTDModulationSineWithOffset(ptr, _offset.Value);
            if (_freqDiv != null)
                ptr = Base.AUTDModulationSineWithSamplingFrequencyDivision(ptr, _freqDiv.Value);
            return ptr;
        }
    }

    /// <summary>
    /// Multi-frequency sine wave modulation
    /// </summary>
    public sealed class Fourier : Internal.Modulation
    {
        private readonly List<Sine> _components;

        /// <summary>
        /// Constructor
        /// </summary>
        public Fourier()
        {
            _components = new List<Sine>();
        }

        public Fourier AddComponent(Sine sine)
        {
            _components.Add(sine);
            return this;
        }

        public override ModulationPtr ModulationPtr()
        {
            return _components.Aggregate(Base.AUTDModulationFourier(), (current, sine) => Base.AUTDModulationFourierAddComponent(current, sine.ModulationPtr()));
        }
    }

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
            return WithSamplingFrequencyDivision((uint)((float_t)Def.FpgaSubClkFreq / freq));
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
            return WithSamplingFrequencyDivision((uint)((float_t)Def.FpgaSubClkFreq / freq));
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

        protected Modulation(float_t samplingFreq)
        {
            _freqDiv = (uint)(Def.FpgaSubClkFreq / samplingFreq);
        }

        sealed public override ModulationPtr ModulationPtr()
        {
            var data = Calc();
            return Base.AUTDModulationCustom(_freqDiv, data, (ulong)data.Length);
        }

        public abstract float_t[] Calc();
    }

    /// <summary>
    /// Modulation to cache the result of calculation
    /// </summary>
    public class Cache : Internal.Modulation, IEnumerable<float_t>
    {
        private readonly uint _freqDiv;

        public Cache(Internal.Modulation m)
        {
            _freqDiv = m.SamplingFrequencyDivision;

            var err = new byte[256];
            var size = Base.AUTDModulationSize(m.ModulationPtr(), err);
            if (size == Def.Autd3Err) throw new AUTDException(err);
            Buffer = new float_t[size];
            if (Base.AUTDModulationCalc(m.ModulationPtr(), Buffer, err) == Def.Autd3Err)
                throw new AUTDException(err);
        }

        public override ModulationPtr ModulationPtr()
        {
            return Base.AUTDModulationCustom(_freqDiv, Buffer, (ulong)Buffer.Length);
        }

        public float_t this[int index]
        {
            get => Buffer[index];
            set => Buffer[index] = value;
        }

        public float_t[] Buffer { get; }

        public IEnumerator<float_t> GetEnumerator() => Buffer.AsEnumerable().GetEnumerator();

        System.Collections.IEnumerator System.Collections.IEnumerable.GetEnumerator() => GetEnumerator();
    }

    public static class CacheModulationExtensions
    {
        public static Cache WithCache(this Internal.Modulation s)
        {
            return new Cache(s);
        }
    }

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

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
