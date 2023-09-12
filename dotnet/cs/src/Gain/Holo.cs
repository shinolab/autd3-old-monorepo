/*
 * File: Holo.cs
 * Project: Gain
 * Created Date: 20/08/2023
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
using System.Collections.Generic;
using System.Runtime.InteropServices;

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

namespace AUTD3Sharp.Gain
{
    namespace Holo
    {
        [ComVisible(false)]
        public abstract class Backend
        {
            internal BackendPtr Ptr;

            public abstract GainPtr Sdp(float_t[]? foci, float_t[]? amps, ulong size);
            public abstract GainPtr SdpWithAlpha(GainPtr ptr, float_t v);
            public abstract GainPtr SdpWithRepeat(GainPtr ptr, uint v);
            public abstract GainPtr SdpWithLambda(GainPtr ptr, float_t v);
            public abstract GainPtr SdpWithConstraint(GainPtr ptr, ConstraintPtr v);

            public abstract GainPtr Evp(float_t[]? foci, float_t[]? amps, ulong size);
            public abstract GainPtr EvpWithGamma(GainPtr ptr, float_t v);
            public abstract GainPtr EvpWithConstraint(GainPtr ptr, ConstraintPtr v);

            public abstract GainPtr Gs(float_t[]? foci, float_t[]? amps, ulong size);
            public abstract GainPtr GsWithRepeat(GainPtr ptr, uint v);
            public abstract GainPtr GsWithConstraint(GainPtr ptr, ConstraintPtr v);

            public abstract GainPtr Gspat(float_t[]? foci, float_t[]? amps, ulong size);
            public abstract GainPtr GspatWithRepeat(GainPtr ptr, uint v);
            public abstract GainPtr GspatWithConstraint(GainPtr ptr,
                                                                                          ConstraintPtr v);

            public abstract GainPtr Naive(float_t[]? foci, float_t[]? amps, ulong size);
            public abstract GainPtr NaiveWithConstraint(GainPtr ptr,
                                                                                          ConstraintPtr v);

            public abstract GainPtr Lm(float_t[]? foci, float_t[]? amps, ulong size);
            public abstract GainPtr LmWithEps1(GainPtr ptr, float_t v);
            public abstract GainPtr LmWithEps2(GainPtr ptr, float_t v);
            public abstract GainPtr LmWithTau(GainPtr ptr, float_t v);
            public abstract GainPtr LmWithKMax(GainPtr ptr, uint v);
            public abstract GainPtr LmWithInitial(GainPtr ptr, float_t[]? v,
                                                                                    ulong size);
            public abstract GainPtr LmWithConstraint(GainPtr ptr, ConstraintPtr v);
        }

        /// <summary>
        /// Backend using <see cref="https://nalgebra.org/">Nalgebra</see>
        /// </summary>
        public sealed class NalgebraBackend : Backend
        {
            public NalgebraBackend()
            {
                Ptr = NativeMethods.GainHolo.AUTDNalgebraBackend();
            }

            ~NalgebraBackend()
            {
                if (Ptr._0 != IntPtr.Zero)
                {
                    NativeMethods.GainHolo.AUTDDeleteNalgebraBackend(Ptr);
                    Ptr._0 = IntPtr.Zero;
                }
            }

            public override GainPtr Sdp(float_t[]? foci, float_t[]? amps, ulong size)
            {
                return NativeMethods.GainHolo.AUTDGainHoloSDP(Ptr, foci, amps, size);
            }

            public override GainPtr SdpWithAlpha(GainPtr ptr, float_t v)
            {
                return NativeMethods.GainHolo.AUTDGainHoloSDPWithAlpha(ptr, v);
            }

            public override GainPtr SdpWithRepeat(GainPtr ptr, uint v)
            {
                return NativeMethods.GainHolo.AUTDGainHoloSDPWithRepeat(ptr, v);
            }

            public override GainPtr SdpWithLambda(GainPtr ptr, float_t v)
            {
                return NativeMethods.GainHolo.AUTDGainHoloSDPWithLambda(ptr, v);
            }

            public override GainPtr SdpWithConstraint(GainPtr ptr, ConstraintPtr v)
            {
                return NativeMethods.GainHolo.AUTDGainHoloSDPWithConstraint(ptr, v);
            }

            public override GainPtr Evp(float_t[]? foci, float_t[]? amps, ulong size)
            {
                return NativeMethods.GainHolo.AUTDGainHoloEVP(Ptr, foci, amps, size);
            }

            public override GainPtr EvpWithGamma(GainPtr ptr, float_t v)
            {
                return NativeMethods.GainHolo.AUTDGainHoloEVPWithGamma(ptr, v);
            }

            public override GainPtr EvpWithConstraint(GainPtr ptr, ConstraintPtr v)
            {
                return NativeMethods.GainHolo.AUTDGainHoloEVPWithConstraint(ptr, v);
            }

            public override GainPtr Gs(float_t[]? foci, float_t[]? amps, ulong size)
            {
                return NativeMethods.GainHolo.AUTDGainHoloGS(Ptr, foci, amps, size);
            }

            public override GainPtr GsWithRepeat(GainPtr ptr, uint v)
            {
                return NativeMethods.GainHolo.AUTDGainHoloGSWithRepeat(ptr, v);
            }

            public override GainPtr GsWithConstraint(GainPtr ptr, ConstraintPtr v)
            {
                return NativeMethods.GainHolo.AUTDGainHoloGSWithConstraint(ptr, v);
            }

            public override GainPtr Gspat(float_t[]? foci, float_t[]? amps, ulong size)
            {
                return NativeMethods.GainHolo.AUTDGainHoloGSPAT(Ptr, foci, amps, size);
            }

            public override GainPtr GspatWithRepeat(GainPtr ptr, uint v)
            {
                return NativeMethods.GainHolo.AUTDGainHoloGSPATWithRepeat(ptr, v);
            }

            public override GainPtr GspatWithConstraint(GainPtr ptr, ConstraintPtr v)
            {
                return NativeMethods.GainHolo.AUTDGainHoloGSPATWithConstraint(ptr, v);
            }

            public override GainPtr Naive(float_t[]? foci, float_t[]? amps, ulong size)
            {
                return NativeMethods.GainHolo.AUTDGainHoloNaive(Ptr, foci, amps, size);
            }

            public override GainPtr NaiveWithConstraint(GainPtr ptr, ConstraintPtr v)
            {
                return NativeMethods.GainHolo.AUTDGainHoloNaiveWithConstraint(ptr, v);
            }

            public override GainPtr Lm(float_t[]? foci, float_t[]? amps, ulong size)
            {
                return NativeMethods.GainHolo.AUTDGainHoloLM(Ptr, foci, amps, size);
            }

            public override GainPtr LmWithEps1(GainPtr ptr, float_t v)
            {
                return NativeMethods.GainHolo.AUTDGainHoloLMWithEps1(ptr, v);
            }

            public override GainPtr LmWithEps2(GainPtr ptr, float_t v)
            {
                return NativeMethods.GainHolo.AUTDGainHoloLMWithEps2(ptr, v);
            }

            public override GainPtr LmWithTau(GainPtr ptr, float_t v)
            {
                return NativeMethods.GainHolo.AUTDGainHoloLMWithTau(ptr, v);
            }

            public override GainPtr LmWithKMax(GainPtr ptr, uint v)
            {
                return NativeMethods.GainHolo.AUTDGainHoloLMWithKMax(ptr, v);
            }

            public override GainPtr LmWithInitial(GainPtr ptr, float_t[]? v,
                                                                                     ulong size)
            {
                return NativeMethods.GainHolo.AUTDGainHoloLMWithInitial(ptr, v, size);
            }

            public override GainPtr LmWithConstraint(GainPtr ptr, ConstraintPtr v)
            {
                return NativeMethods.GainHolo.AUTDGainHoloLMWithConstraint(ptr, v);
            }
        }

        /// <summary>
        /// Amplitude constraint
        /// </summary>
        public interface IAmplitudeConstraint
        {
            public ConstraintPtr Ptr();
        }

        /// <summary>
        /// Do nothing (this is equivalent to `Clamp(0, 1)`)
        /// </summary>
        public sealed class DontCare : IAmplitudeConstraint
        {
            public ConstraintPtr Ptr() => NativeMethods.GainHolo.AUTDGainHoloDotCareConstraint();
        }

        /// <summary>
        /// Normalize the value by dividing the maximum value
        /// </summary>
        public sealed class Normalize : IAmplitudeConstraint
        {
            public ConstraintPtr Ptr() => NativeMethods.GainHolo.AUTDGainHoloNormalizeConstraint();
        }

        /// <summary>
        /// Set all amplitudes to the specified value
        /// </summary>
        public sealed class Uniform : IAmplitudeConstraint
        {

            internal readonly float_t Value;

            public Uniform(float_t value = 1)
            {
                Value = value;
            }

            public ConstraintPtr Ptr() => NativeMethods.GainHolo.AUTDGainHoloUniformConstraint(Value);
        }

        /// <summary>
        /// Clamp all amplitudes to the specified range
        /// </summary>
        public sealed class Clamp : IAmplitudeConstraint
        {
            internal readonly float_t Min;
            internal readonly float_t Max;

            public Clamp(float_t min = 0, float_t max = 1)
            {
                Min = min;
                Max = max;
            }

            public ConstraintPtr Ptr() => NativeMethods.GainHolo.AUTDGainHoloClampConstraint(Min, Max);
        }

        /// <summary>
        /// Gain to produce multiple foci by solving Semi-Definite Programming
        /// </summary>
        /// <remarks>Inoue, Seki, Yasutoshi Makino, and Hiroyuki Shinoda. "Active touch perception produced by airborne ultrasonic haptic hologram." 2015 IEEE World Haptics Conference (WHC). IEEE, 2015.</remarks>
        /// <typeparam name="TB">Backend</typeparam>
        public sealed class SDP<TB> : Internal.Gain
            where TB : Backend
        {
            private readonly TB _backend;
            private readonly List<float_t> _foci;
            private readonly List<float_t> _amps;
            private float_t? _alpha;
            private float_t? _lambda;
            private uint? _repeat;
            private IAmplitudeConstraint? _constraint;

            public SDP(TB backend)
            {
                _backend = backend;
                _foci = new List<float_t>();
                _amps = new List<float_t>();
            }

            public SDP<TB> AddFocus(Vector3 focus, float_t amp)
            {
                _foci.Add(focus.x);
                _foci.Add(focus.y);
                _foci.Add(focus.z);
                _amps.Add(amp);
                return this;
            }

            /// <summary>
            /// Add foci
            /// </summary>
            /// <param name="iter">Enumerable of foci and amps</param>
            public SDP<TB> AddFociFromIter(IEnumerable<(Vector3, float_t)> iter)
            {
                return iter.Aggregate(this, (holo, point) => holo.AddFocus(point.Item1, point.Item2));
            }

            /// <summary>
            /// Parameter. See the paper for details.
            /// </summary>
            /// <param name="value"></param>
            /// <returns></returns>
            public SDP<TB> WithAlpha(float_t value)
            {
                _alpha = value;
                return this;
            }

            /// <summary>
            /// Parameter. See the paper for details.
            /// </summary>
            /// <param name="value"></param>
            /// <returns></returns>
            public SDP<TB> WithLambda(float_t value)
            {
                _lambda = value;
                return this;
            }

            /// <summary>
            /// Parameter. See the paper for details.
            /// </summary>
            /// <param name="value"></param>
            /// <returns></returns>
            public SDP<TB> WithRepeat(uint value)
            {
                _repeat = value;
                return this;
            }

            /// <summary>
            /// Set amplitude constraint
            /// </summary>
            /// <param name="constraint"></param>
            /// <returns></returns>
            public SDP<TB> WithConstraint(IAmplitudeConstraint constraint)
            {
                _constraint = constraint;
                return this;
            }

            public override GainPtr GainPtr(Geometry geometry)
            {
                var ptr = _backend.Sdp(_foci.ToArray(), _amps.ToArray(),
                    (ulong)_amps.Count);
                if (_alpha.HasValue) ptr = _backend.SdpWithAlpha(ptr, _alpha.Value);
                if (_lambda.HasValue) ptr = _backend.SdpWithLambda(ptr, _lambda.Value);
                if (_repeat.HasValue) ptr = _backend.SdpWithRepeat(ptr, _repeat.Value);
                if (_constraint != null) ptr = _backend.SdpWithConstraint(ptr, _constraint.Ptr());
                return ptr;
            }
        }

        /// <summary>
        /// Gain to produce multiple foci by solving Eigen Value Problem
        /// </summary>
        /// <remarks>Long, Benjamin, et al. "Rendering volumetric haptic shapes in mid-air using ultrasound." ACM Transactions on Graphics (TOG) 33.6 (2014): 1-10.</remarks>
        /// <typeparam name="TB">Backend</typeparam>
        public sealed class EVP<TB> : Internal.Gain
            where TB : Backend
        {
            private readonly TB _backend;
            private readonly List<float_t> _foci;
            private readonly List<float_t> _amps;
            private float_t? _gamma;
            private IAmplitudeConstraint? _constraint;

            public EVP(TB backend)
            {
                _backend = backend;
                _foci = new List<float_t>();
                _amps = new List<float_t>();
            }

            public EVP<TB> AddFocus(Vector3 focus, float_t amp)
            {
                _foci.Add(focus.x);
                _foci.Add(focus.y);
                _foci.Add(focus.z);
                _amps.Add(amp);
                return this;
            }

            /// <summary>
            /// Add foci
            /// </summary>
            /// <param name="iter">Enumerable of foci and amps</param>
            public EVP<TB> AddFociFromIter(IEnumerable<(Vector3, float_t)> iter)
            {
                return iter.Aggregate(this, (holo, point) => holo.AddFocus(point.Item1, point.Item2));
            }

            /// <summary>
            /// Parameter. See the paper for details.
            /// </summary>
            /// <param name="value"></param>
            /// <returns></returns>
            public EVP<TB> WithGamma(float_t value)
            {
                _gamma = value;
                return this;
            }

            /// <summary>
            /// Set amplitude constraint
            /// </summary>
            /// <param name="constraint"></param>
            /// <returns></returns>
            public EVP<TB> WithConstraint(IAmplitudeConstraint constraint)
            {
                _constraint = constraint;
                return this;
            }

            public override GainPtr GainPtr(Geometry geometry)
            {
                var ptr = _backend.Evp(_foci.ToArray(), _amps.ToArray(),
                    (ulong)_amps.Count);
                if (_gamma.HasValue) ptr = _backend.EvpWithGamma(ptr, _gamma.Value);
                if (_constraint != null) ptr = _backend.EvpWithConstraint(ptr, _constraint.Ptr());
                return ptr;
            }
        }

        /// <summary>
        /// Gain to produce multiple foci with GS algorithm
        /// </summary>
        /// <remarks>Asier Marzo and Bruce W Drinkwater. Holographic acoustic tweezers.Proceedings of theNational Academy of Sciences, 116(1):84–89, 2019.</remarks>
        /// <typeparam name="TB">Backend</typeparam>
        public sealed class GS<TB> : Internal.Gain
            where TB : Backend
        {
            private readonly TB _backend;
            private readonly List<float_t> _foci;
            private readonly List<float_t> _amps;
            private uint? _repeat;
            private IAmplitudeConstraint? _constraint;

            public GS(TB backend)
            {
                _backend = backend;
                _foci = new List<float_t>();
                _amps = new List<float_t>();
            }

            public GS<TB> AddFocus(Vector3 focus, float_t amp)
            {
                _foci.Add(focus.x);
                _foci.Add(focus.y);
                _foci.Add(focus.z);
                _amps.Add(amp);
                return this;
            }

            /// <summary>
            /// Add foci
            /// </summary>
            /// <param name="iter">Enumerable of foci and amps</param>
            public GS<TB> AddFociFromIter(IEnumerable<(Vector3, float_t)> iter)
            {
                return iter.Aggregate(this, (holo, point) => holo.AddFocus(point.Item1, point.Item2));
            }

            /// <summary>
            /// Parameter. See the paper for details.
            /// </summary>
            /// <param name="value"></param>
            /// <returns></returns>
            public GS<TB> WithRepeat(uint value)
            {
                _repeat = value;
                return this;
            }

            /// <summary>
            /// Set amplitude constraint
            /// </summary>
            /// <param name="constraint"></param>
            /// <returns></returns>
            public GS<TB> WithConstraint(IAmplitudeConstraint constraint)
            {
                _constraint = constraint;
                return this;
            }

            public override GainPtr GainPtr(Geometry geometry)
            {
                var ptr = _backend.Gs(_foci.ToArray(), _amps.ToArray(),
                    (ulong)_amps.Count);
                if (_repeat.HasValue) ptr = _backend.GsWithRepeat(ptr, _repeat.Value);
                if (_constraint != null) ptr = _backend.GsWithConstraint(ptr, _constraint.Ptr());
                return ptr;
            }
        }

        /// <summary>
        /// Gain to produce multiple foci with GS-PAT algorithm
        /// </summary>
        /// <remarks>Diego Martinez Plasencia et al. "GS-pat: high-speed multi-point sound-fields for phased arrays of transducers," ACMTrans-actions on Graphics (TOG), 39(4):138–1, 2020.</remarks>
        /// <typeparam name="TB">Backend</typeparam>
        public sealed class GSPAT<TB> : Internal.Gain
            where TB : Backend
        {
            private readonly TB _backend;
            private readonly List<float_t> _foci;
            private readonly List<float_t> _amps;
            private uint? _repeat;
            private IAmplitudeConstraint? _constraint;

            public GSPAT(TB backend)
            {
                _backend = backend;
                _foci = new List<float_t>();
                _amps = new List<float_t>();
            }

            public GSPAT<TB> AddFocus(Vector3 focus, float_t amp)
            {
                _foci.Add(focus.x);
                _foci.Add(focus.y);
                _foci.Add(focus.z);
                _amps.Add(amp);
                return this;
            }

            /// <summary>
            /// Add foci
            /// </summary>
            /// <param name="iter">Enumerable of foci and amps</param>
            public GSPAT<TB> AddFociFromIter(IEnumerable<(Vector3, float_t)> iter)
            {
                return iter.Aggregate(this, (holo, point) => holo.AddFocus(point.Item1, point.Item2));
            }

            /// <summary>
            /// Parameter. See the paper for details.
            /// </summary>
            /// <param name="value"></param>
            /// <returns></returns>
            public GSPAT<TB> WithRepeat(uint value)
            {
                _repeat = value;
                return this;
            }

            /// <summary>
            /// Set amplitude constraint
            /// </summary>
            /// <param name="constraint"></param>
            /// <returns></returns>
            public GSPAT<TB> WithConstraint(IAmplitudeConstraint constraint)
            {
                _constraint = constraint;
                return this;
            }

            public override GainPtr GainPtr(Geometry geometry)
            {
                var ptr = _backend.Gspat(_foci.ToArray(), _amps.ToArray(),
                    (ulong)_amps.Count);
                if (_repeat.HasValue) ptr = _backend.GspatWithRepeat(ptr, _repeat.Value);
                if (_constraint != null) ptr = _backend.GspatWithConstraint(ptr, _constraint.Ptr());
                return ptr;
            }
        }

        /// <summary>
        /// Gain to produce multiple foci with naive linear synthesis
        /// </summary>
        /// <typeparam name="TB">Backend</typeparam>
        public sealed class Naive<TB> : Internal.Gain
            where TB : Backend
        {
            private readonly TB _backend;
            private readonly List<float_t> _foci;
            private readonly List<float_t> _amps;
            private IAmplitudeConstraint? _constraint;


            public Naive(TB backend)
            {
                _backend = backend;
                _foci = new List<float_t>();
                _amps = new List<float_t>();

            }

            public Naive<TB> AddFocus(Vector3 focus, float_t amp)
            {
                _foci.Add(focus.x);
                _foci.Add(focus.y);
                _foci.Add(focus.z);
                _amps.Add(amp);
                return this;
            }

            /// <summary>
            /// Add foci
            /// </summary>
            /// <param name="iter">Enumerable of foci and amps</param>
            public Naive<TB> AddFociFromIter(IEnumerable<(Vector3, float_t)> iter)
            {
                return iter.Aggregate(this, (holo, point) => holo.AddFocus(point.Item1, point.Item2));
            }

            /// <summary>
            /// Set amplitude constraint
            /// </summary>
            /// <param name="constraint"></param>
            /// <returns></returns>
            public Naive<TB> WithConstraint(IAmplitudeConstraint constraint)
            {
                _constraint = constraint;
                return this;
            }

            public override GainPtr GainPtr(Geometry geometry)
            {
                var ptr = _backend.Naive(_foci.ToArray(), _amps.ToArray(),
                    (ulong)_amps.Count);
                if (_constraint != null) ptr = _backend.NaiveWithConstraint(ptr, _constraint.Ptr());
                return ptr;
            }
        }

        /// <summary>
        /// Gain to produce multiple foci with Levenberg-Marquardt algorithm
        /// </summary>
        /// <remarks>
        /// <para>K.Levenberg, “A method for the solution of certain non-linear problems in least squares,” Quarterly of applied mathematics, vol.2, no.2, pp.164–168, 1944.</para> 
        /// <para> D.W.Marquardt, “An algorithm for least-squares estimation of non-linear parameters,” Journal of the society for Industrial and AppliedMathematics, vol.11, no.2, pp.431–441, 1963.</para> 
        /// <para>K.Madsen, H.Nielsen, and O.Tingleff, “Methods for non-linear least squares problems (2nd ed.),” 2004.</para> 
        /// </remarks>
        /// <typeparam name="TB">Backend</typeparam>
        public sealed class LM<TB> : Internal.Gain
            where TB : Backend
        {
            private readonly TB _backend;
            private readonly List<float_t> _foci;
            private readonly List<float_t> _amps;
            private float_t? _eps1;
            private float_t? _eps2;
            private float_t? _tau;
            private uint? _kMax;
            private float_t[]? _initial;

            private IAmplitudeConstraint? _constraint;

            public LM(TB backend)
            {
                _backend = backend;
                _foci = new List<float_t>();
                _amps = new List<float_t>();
            }

            /// <summary>
            /// Parameter. See the paper for details.
            /// </summary>
            /// <param name="focus"></param>
            /// <param name="amp"></param>
            /// <returns></returns>
            public LM<TB> AddFocus(Vector3 focus, float_t amp)
            {
                _foci.Add(focus.x);
                _foci.Add(focus.y);
                _foci.Add(focus.z);
                _amps.Add(amp);
                return this;
            }

            /// <summary>
            /// Add foci
            /// </summary>
            /// <param name="iter">Enumerable of foci and amps</param>
            public LM<TB> AddFociFromIter(IEnumerable<(Vector3, float_t)> iter)
            {
                return iter.Aggregate(this, (holo, point) => holo.AddFocus(point.Item1, point.Item2));
            }

            /// <summary>
            /// Parameter. See the paper for details.
            /// </summary>
            /// <param name="value"></param>
            /// <returns></returns>
            public LM<TB> WithEps1(float_t value)
            {
                _eps1 = value;
                return this;
            }

            /// <summary>
            /// Parameter. See the paper for details.
            /// </summary>
            /// <param name="value"></param>
            /// <returns></returns>
            public LM<TB> WithEps2(float_t value)
            {
                _eps2 = value;
                return this;
            }

            /// <summary>
            /// Parameter. See the paper for details.
            /// </summary>
            /// <param name="value"></param>
            /// <returns></returns>
            public LM<TB> WithTau(float_t value)
            {
                _tau = value;
                return this;
            }

            /// <summary>
            /// Parameter. See the paper for details.
            /// </summary>
            /// <param name="value"></param>
            /// <returns></returns>
            public LM<TB> WithKMax(uint value)
            {
                _kMax = value;
                return this;
            }

            /// <summary>
            /// Set amplitude constraint
            /// </summary>
            /// <param name="constraint"></param>
            /// <returns></returns>
            public LM<TB> WithConstraint(IAmplitudeConstraint constraint)
            {
                _constraint = constraint;
                return this;
            }

            /// <summary>
            /// Parameter. See the paper for details.
            /// </summary>
            /// <param name="value"></param>
            /// <returns></returns>
            public LM<TB> WithInitial(float_t[] value)
            {
                _initial = value;
                return this;
            }

            public override GainPtr GainPtr(Geometry geometry)
            {
                var ptr = _backend.Lm(_foci.ToArray(), _amps.ToArray(),
                    (ulong)_amps.Count);
                if (_eps1.HasValue) ptr = _backend.LmWithEps1(ptr, _eps1.Value);
                if (_eps2.HasValue) ptr = _backend.LmWithEps2(ptr, _eps2.Value);
                if (_tau.HasValue) ptr = _backend.LmWithTau(ptr, _tau.Value);
                if (_kMax.HasValue) ptr = _backend.LmWithKMax(ptr, _kMax.Value);
                if (_initial != null)
                    ptr = _backend.LmWithInitial(ptr, _initial, (ulong)_initial.Length);
                if (_constraint != null) ptr = _backend.LmWithConstraint(ptr, _constraint.Ptr());
                return ptr;
            }
        }

        /// <summary>
        /// Gain to produce multiple foci with greedy algorithm
        /// </summary>
        /// <remarks>
        /// Shun Suzuki, Masahiro Fujiwara, Yasutoshi Makino, and Hiroyuki Shinoda, “Radiation Pressure Field Reconstruction for Ultrasound Midair Haptics by Greedy Algorithm with Brute-Force Search,” in IEEE Transactions on Haptics, doi: 10.1109/TOH.2021.3076489
        /// </remarks>
        public sealed class Greedy : Internal.Gain
        {
            private readonly List<float_t> _foci = new List<float_t>();
            private readonly List<float_t> _amps = new List<float_t>();
            private uint? _phaseDiv;
            private IAmplitudeConstraint? _constraint;

            public Greedy AddFocus(Vector3 focus, float_t amp)
            {
                _foci.Add(focus.x);
                _foci.Add(focus.y);
                _foci.Add(focus.z);
                _amps.Add(amp);
                return this;
            }

            /// <summary>
            /// Add foci
            /// </summary>
            /// <param name="iter">Enumerable of foci and amps</param>
            public Greedy AddFociFromIter(IEnumerable<(Vector3, float_t)> iter)
            {
                return iter.Aggregate(this, (holo, point) => holo.AddFocus(point.Item1, point.Item2));
            }

            /// <summary>
            /// Parameter. See the paper for details.
            /// </summary>
            /// <param name="value"></param>
            /// <returns></returns>
            public Greedy WithPhaseDiv(uint value)
            {
                _phaseDiv = value;
                return this;
            }

            /// <summary>
            /// Set amplitude constraint
            /// </summary>
            /// <param name="constraint"></param>
            /// <returns></returns>
            public Greedy WithConstraint(IAmplitudeConstraint constraint)
            {
                _constraint = constraint;
                return this;
            }

            public override GainPtr GainPtr(Geometry geometry)
            {
                var ptr = NativeMethods.GainHolo.AUTDGainHoloGreedy(_foci.ToArray(), _amps.ToArray(),
                    (ulong)_amps.Count);
                if (_phaseDiv.HasValue)
                    ptr = NativeMethods.GainHolo.AUTDGainHoloGreedyWithPhaseDiv(ptr, _phaseDiv.Value);
                if (_constraint != null) ptr = NativeMethods.GainHolo.AUTDGainHoloGreedyWithConstraint(ptr, _constraint.Ptr());
                return ptr;
            }
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
