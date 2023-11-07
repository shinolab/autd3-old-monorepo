/*
 * File: STM.cs
 * Project: src
 * Created Date: 20/08/2023
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

using System;
using System.Collections.Generic;
using System.Linq;

using AUTD3Sharp.Internal;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

#if UNITY_2018_3_OR_NEWER
using UnityEngine;
using Vector3 = UnityEngine.Vector3;
#else
using Vector3 = AUTD3Sharp.Utils.Vector3d;
#endif

#if USE_SINGLE
using float_t = System.Single;
#else
using float_t = System.Double;
#endif

namespace AUTD3Sharp
{
    namespace STM
    {
        public abstract class STM : IDatagram
        {
            private readonly float_t? _freq;
            private readonly float_t? _samplingFreq;
            private readonly uint? _samplingFreqDiv;
            private readonly TimeSpan? _samplingPeriod;
            protected int StartIdxV;
            protected int FinishIdxV;

            protected STM(float_t? freq, float_t? samplingFreq, uint? sampleFreqDiv, TimeSpan? samplingPeriod)
            {
                _freq = freq;
                _samplingFreq = samplingFreq;
                _samplingFreqDiv = sampleFreqDiv;
                _samplingPeriod = samplingPeriod;
                StartIdxV = -1;
                FinishIdxV = -1;
            }

            DatagramPtr IDatagram.Ptr(Geometry geometry) => STMPtr(geometry);

            internal abstract DatagramPtr STMPtr(Geometry geometry);

            public ushort? StartIdx => StartIdxV == -1 ? null : (ushort?)StartIdxV;

            public ushort? FinishIdx => FinishIdxV == -1 ? null : (ushort?)FinishIdxV;

            internal STMPropsPtr Props()
            {
                var ptr = new STMPropsPtr();
                if (_freq != null)
                    ptr = NativeMethodsBase.AUTDSTMProps(_freq.Value);
                if (_samplingFreq != null)
                    ptr = NativeMethodsBase.AUTDSTMPropsWithSamplingFreq(_samplingFreq.Value);
                if (_samplingFreqDiv != null)
                    ptr = NativeMethodsBase.AUTDSTMPropsWithSamplingFreqDiv(_samplingFreqDiv.Value);
                if (_samplingPeriod != null)
                    ptr = NativeMethodsBase.AUTDSTMPropsWithSamplingPeriod((ulong)(_samplingPeriod.Value.TotalMilliseconds * 1000 *
                                                                      1000));
                ptr = NativeMethodsBase.AUTDSTMPropsWithStartIdx(ptr, StartIdxV);
                ptr = NativeMethodsBase.AUTDSTMPropsWithFinishIdx(ptr, FinishIdxV);
                return ptr;
            }

            protected float_t FreqFromSize(int size) => NativeMethodsBase.AUTDSTMPropsFrequency(Props(), (ulong)size);

            protected float_t SamplingFreqFromSize(int size) =>
                NativeMethodsBase.AUTDSTMPropsSamplingFrequency(Props(), (ulong)size);

            protected uint SamplingFreqDivFromSize(int size) =>
                NativeMethodsBase.AUTDSTMPropsSamplingFrequencyDivision(Props(), (ulong)size);

            protected TimeSpan SamplingPeriodFromSize(int size) =>
                TimeSpan.FromMilliseconds(NativeMethodsBase.AUTDSTMPropsSamplingPeriod(Props(), (ulong)size) / 1000.0 / 1000.0);
        }

        /// <summary>
        /// FocusSTM is an STM for moving a focal point
        /// </summary>
        /// <remarks>
        /// <para>The sampling timing is determined by hardware, thus the sampling time is precise.</para>
        /// <para>FocusSTM has following restrictions:</para>
        /// <list>
        /// <item>The maximum number of sampling points is 65536.</item>
        /// <item>The sampling frequency is <see cref="AUTD3.FPGAClkFreq">AUTD3.FPGAClkFreq</see>/N, where `N` is a 32-bit unsigned integer and must be at 4096.</item>
        /// </list></remarks>
        public sealed class FocusSTM : STM
        {
            private readonly List<float_t> _points;
            private readonly List<byte> _shifts;

            private FocusSTM(float_t? freq, float_t? samplingFreq, uint? sampleFreqDiv, TimeSpan? samplePeriod) : base(
                freq, samplingFreq, sampleFreqDiv, samplePeriod)
            {
                _points = new List<float_t>();
                _shifts = new List<byte>();
            }

            public FocusSTM(float_t freq) : this(freq, null, null, null)
            {
            }

            public static FocusSTM WithSamplingFrequency(float_t freq)
            {
                return new FocusSTM(null, freq, null, null);
            }

            public static FocusSTM WithSamplingFrequencyDivision(uint freqDiv)
            {
                return new FocusSTM(null, null, freqDiv, null);
            }

            public static FocusSTM WithSamplingPeriod(TimeSpan period)
            {
                return new FocusSTM(null, null, null, period);
            }

            /// <summary>
            /// Add focus point
            /// </summary>
            /// <param name="point">Focus point</param>
            /// <param name="shift">Duty shift. Duty ratio of ultrasound will be `50% >> duty_shift`. If `duty_shift` is 0, duty ratio is 50%, which means the amplitude is the maximum.</param>
            /// <returns></returns>
            public FocusSTM AddFocus(Vector3 point, byte shift = 0)
            {
                _points.Add(point.x);
                _points.Add(point.y);
                _points.Add(point.z);
                _shifts.Add(shift);
                return this;
            }

            /// <summary>
            /// Add foci
            /// </summary>
            /// <param name="iter">Enumerable of foci</param>
            public FocusSTM AddFociFromIter(IEnumerable<Vector3> iter)
            {
                return iter.Aggregate(this, (stm, point) => stm.AddFocus(point));
            }

            /// <summary>
            /// Add foci
            /// </summary>
            /// <param name="iter">Enumerable of foci and duty shifts</param>
            public FocusSTM AddFociFromIter(IEnumerable<(Vector3, byte)> iter)
            {
                return iter.Aggregate(this, (stm, point) => stm.AddFocus(point.Item1, point.Item2));
            }

            public FocusSTM WithStartIdx(ushort? startIdx)
            {
                StartIdxV = startIdx ?? -1;
                return this;
            }

            public FocusSTM WithFinishIdx(ushort? finishIdx)
            {
                FinishIdxV = finishIdx ?? -1;
                return this;
            }

            public float_t Frequency => FreqFromSize(_shifts.Count);
            public float_t SamplingFrequency => SamplingFreqFromSize(_shifts.Count);
            public uint SamplingFrequencyDivision => SamplingFreqDivFromSize(_shifts.Count);
            public TimeSpan SamplingPeriod => SamplingPeriodFromSize(_shifts.Count);

            internal override DatagramPtr STMPtr(Geometry geometry)
            {
                unsafe
                {
                    fixed (float_t* pp = _points.ToArray())
                    fixed (byte* ps = _shifts.ToArray())
                        return NativeMethodsBase.AUTDSTMFocus(Props(), pp, ps, (ulong)_shifts.Count);
                }
            }
        }

        /// <summary>
        /// FocusSTM is an STM for moving Gains
        /// </summary>
        /// <remarks>
        /// <para>The sampling timing is determined by hardware, thus the sampling time is precise.</para>
        /// <para>FocusSTM has following restrictions:</para>
        /// <list>
        /// <item>The maximum number of sampling Gain is 2048.</item>
        /// <item>The sampling frequency is <see cref="AUTD3.FPGAClkFreq">AUTD3.FPGAClkFreq</see>/N, where `N` is a 32-bit unsigned integer and must be at 4096.</item>
        /// </list></remarks>
        public sealed class GainSTM : STM
        {
            private readonly List<Internal.Gain> _gains;
            private GainSTMMode _mode;

            private GainSTM(float_t? freq, float_t? samplingFreq, uint? sampleFreqDiv, TimeSpan? samplePeriod) : base(
                freq, samplingFreq, sampleFreqDiv, samplePeriod)
            {
                _gains = new List<Internal.Gain>();
                _mode = GainSTMMode.PhaseDutyFull;
            }

            public GainSTM(float_t freq) : this(freq, null, null, null)
            {
            }

            public static GainSTM WithSamplingFrequency(float_t freq)
            {
                return new GainSTM(null, freq, null, null);
            }

            public static GainSTM WithSamplingFrequencyDivision(uint freqDiv)
            {
                return new GainSTM(null, null, freqDiv, null);
            }

            public static GainSTM WithSamplingPeriod(TimeSpan period)
            {
                return new GainSTM(null, null, null, period);
            }

            /// <summary>
            /// Add Gain
            /// </summary>
            /// <param name="gain">Gain</param>
            /// <returns></returns>
            public GainSTM AddGain(Internal.Gain gain)
            {
                _gains.Add(gain);
                return this;
            }

            /// <summary>
            /// Add Gains
            /// </summary>
            /// <param name="iter">Enumerable of Gains</param>
            public GainSTM AddGainsFromIter(IEnumerable<Internal.Gain> iter)
            {
                return iter.Aggregate(this, (stm, gain) => stm.AddGain(gain));
            }

            public GainSTM WithStartIdx(ushort? startIdx)
            {
                StartIdxV = startIdx ?? -1;
                return this;
            }

            public GainSTM WithFinishIdx(ushort? finishIdx)
            {
                FinishIdxV = finishIdx ?? -1;
                return this;
            }

            public GainSTM WithMode(GainSTMMode mode)
            {
                _mode = mode;
                return this;
            }

            public float_t Frequency => FreqFromSize(_gains.Count);
            public float_t SamplingFrequency => SamplingFreqFromSize(_gains.Count);
            public uint SamplingFrequencyDivision => SamplingFreqDivFromSize(_gains.Count);
            public TimeSpan SamplingPeriod => SamplingPeriodFromSize(_gains.Count);

            internal override DatagramPtr STMPtr(Geometry geometry)
            {
                var gains = _gains.Select(g => g.GainPtr(geometry)).ToArray();
                unsafe
                {
                    fixed (GainPtr* gp = gains)
                        return NativeMethodsBase.AUTDSTMGain(Props(), gp, (uint)gains.Length, _mode);
                }
            }
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
