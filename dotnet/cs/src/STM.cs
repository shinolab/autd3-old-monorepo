/*
 * File: STM.cs
 * Project: src
 * Created Date: 20/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2023
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
using AUTD3Sharp.NativeMethods;

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
            private readonly TimeSpan? _period;
            private readonly SamplingConfiguration? _samplingConfig;
            protected int StartIdxV;
            protected int FinishIdxV;

            protected STM(float_t? freq, TimeSpan? period, SamplingConfiguration? config)
            {
                _freq = freq;
                _period = period;
                _samplingConfig = config;
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
                    ptr = NativeMethodsBase.AUTDSTMPropsNew(_freq.Value);
                if (_period != null)
                    ptr = NativeMethodsBase.AUTDSTMPropsFromPeriod((ulong)(_period.Value.TotalSeconds * 1000 * 1000 *
                                                                              1000));
                if (_samplingConfig != null)
                    ptr = NativeMethodsBase.AUTDSTMPropsFromSamplingConfig(_samplingConfig.Value.Internal);
                ptr = NativeMethodsBase.AUTDSTMPropsWithStartIdx(ptr, StartIdxV);
                ptr = NativeMethodsBase.AUTDSTMPropsWithFinishIdx(ptr, FinishIdxV);
                return ptr;
            }

            protected float_t FreqFromSize(int size) => NativeMethodsBase.AUTDSTMPropsFrequency(Props(), (ulong)size);

            protected TimeSpan PeriodFromSize(int size) =>
                TimeSpan.FromSeconds(NativeMethodsBase.AUTDSTMPropsPeriod(Props(), (ulong)size) / 1000.0 / 1000.0 /
                                     1000.0);

            protected SamplingConfiguration SamplingConfigFromSize(int size) => new SamplingConfiguration(
                NativeMethodsBase.AUTDSTMPropsSamplingConfig(Props(), (ulong)size).Validate());
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
            private readonly List<EmitIntensity> _intensities;

            private FocusSTM(float_t? freq, TimeSpan? period, SamplingConfiguration? config) : base(
                freq, period, config)
            {
                _points = new List<float_t>();
                _intensities = new List<EmitIntensity>();
            }

            public FocusSTM(float_t freq) : this(freq, null, null)
            {
            }

            public static FocusSTM FromPeriod(TimeSpan period)
            {
                return new FocusSTM(null, period, null);
            }

            public static FocusSTM FromSamplingConfig(SamplingConfiguration config)
            {
                return new FocusSTM(null, null, config);
            }

            /// <summary>
            /// Add focus point
            /// </summary>
            /// <param name="point">Focus point</param>
            /// <param name="intensity">Emission intensity</param>
            /// <returns></returns>
            public FocusSTM AddFocus(Vector3 point, EmitIntensity? intensity = null)
            {
                _points.Add(point.x);
                _points.Add(point.y);
                _points.Add(point.z);
                _intensities.Add(intensity ?? EmitIntensity.Max);
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
            public FocusSTM AddFociFromIter(IEnumerable<(Vector3, EmitIntensity)> iter)
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

            public float_t Frequency => FreqFromSize(_intensities.Count);
            public TimeSpan Period => PeriodFromSize(_intensities.Count);
            public SamplingConfiguration SamplingConfiguration => SamplingConfigFromSize(_intensities.Count);

            internal override DatagramPtr STMPtr(Geometry geometry)
            {
                var points = _points.ToArray();
                var intensities = _intensities.ToArray();
                unsafe
                {
                    fixed (float_t* pp = &points[0])
                    fixed (EmitIntensity* ps = &intensities[0])
                        return NativeMethodsBase.AUTDSTMFocus(Props(), pp, (byte*)ps, (ulong)_intensities.Count)
                            .Validate();
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

            private GainSTM(float_t? freq, TimeSpan? period, SamplingConfiguration? config) : base(
                freq, period, config)
            {
                _gains = new List<Internal.Gain>();
                _mode = GainSTMMode.PhaseIntensityFull;
            }

            public GainSTM(float_t freq) : this(freq, null, null)
            {
            }

            public static GainSTM FromPeriod(TimeSpan period)
            {
                return new GainSTM(null, period, null);
            }

            public static GainSTM FromSamplingConfig(SamplingConfiguration config)
            {
                return new GainSTM(null, null, config);
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
            public TimeSpan Period => PeriodFromSize(_gains.Count);
            public SamplingConfiguration SamplingConfiguration => SamplingConfigFromSize(_gains.Count);


            internal override DatagramPtr STMPtr(Geometry geometry)
            {
                var gains = _gains.Select(g => g.GainPtr(geometry)).ToArray();
                unsafe
                {
                    fixed (GainPtr* gp = &gains[0])
                        return NativeMethodsBase.AUTDSTMGain(Props(), gp, (uint)gains.Length, _mode).Validate();
                }
            }
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
