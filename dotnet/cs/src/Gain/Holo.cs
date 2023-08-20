/*
 * File: Holo.cs
 * Project: Gain
 * Created Date: 20/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/08/2023
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

namespace AUTD3Sharp
{
    namespace Gain
    {
        namespace Holo
        {
            [ComVisible(false)]
            public abstract class Backend
            {
                internal BackendPtr Ptr;

                public abstract GainPtr SDP(float_t[]? foci, float_t[]? amps, ulong size);
                public abstract GainPtr SDPWithAlpha(GainPtr ptr, float_t v);
                public abstract GainPtr SDPWithRepeat(GainPtr ptr, uint v);
                public abstract GainPtr SDPWithLambda(GainPtr ptr, float_t v);
                public abstract GainPtr SDPWithConstraint(GainPtr ptr, ConstraintPtr v);

                public abstract GainPtr EVP(float_t[]? foci, float_t[]? amps, ulong size);
                public abstract GainPtr EVPWithGamma(GainPtr ptr, float_t v);
                public abstract GainPtr EVPWithConstraint(GainPtr ptr, ConstraintPtr v);

                public abstract GainPtr GS(float_t[]? foci, float_t[]? amps, ulong size);
                public abstract GainPtr GSWithRepeat(GainPtr ptr, uint v);
                public abstract GainPtr GSWithConstraint(GainPtr ptr, ConstraintPtr v);

                public abstract GainPtr GSPAT(float_t[]? foci, float_t[]? amps, ulong size);
                public abstract GainPtr GSPATWithRepeat(GainPtr ptr, uint v);
                public abstract GainPtr GSPATWithConstraint(GainPtr ptr,
                                                                                              ConstraintPtr v);

                public abstract GainPtr Naive(float_t[]? foci, float_t[]? amps, ulong size);
                public abstract GainPtr NaiveWithConstraint(GainPtr ptr,
                                                                                              ConstraintPtr v);

                public abstract GainPtr LM(float_t[]? foci, float_t[]? amps, ulong size);
                public abstract GainPtr LMWithEps1(GainPtr ptr, float_t v);
                public abstract GainPtr LMWithEps2(GainPtr ptr, float_t v);
                public abstract GainPtr LMWithTau(GainPtr ptr, float_t v);
                public abstract GainPtr LMWithKMax(GainPtr ptr, uint v);
                public abstract GainPtr LMWithInitial(GainPtr ptr, float_t[]? v,
                                                                                        ulong size);
                public abstract GainPtr LMWithConstraint(GainPtr ptr, ConstraintPtr v);
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

                public override GainPtr SDP(float_t[]? foci, float_t[]? amps, ulong size)
                {
                    return NativeMethods.GainHolo.AUTDGainHoloSDP(Ptr, foci, amps, size);
                }

                public override GainPtr SDPWithAlpha(GainPtr ptr, float_t v)
                {
                    return NativeMethods.GainHolo.AUTDGainHoloSDPWithAlpha(ptr, v);
                }

                public override GainPtr SDPWithRepeat(GainPtr ptr, uint v)
                {
                    return NativeMethods.GainHolo.AUTDGainHoloSDPWithRepeat(ptr, v);
                }

                public override GainPtr SDPWithLambda(GainPtr ptr, float_t v)
                {
                    return NativeMethods.GainHolo.AUTDGainHoloSDPWithLambda(ptr, v);
                }

                public override GainPtr SDPWithConstraint(GainPtr ptr, ConstraintPtr v)
                {
                    return NativeMethods.GainHolo.AUTDGainHoloSDPWithConstraint(ptr, v);
                }

                public override GainPtr EVP(float_t[]? foci, float_t[]? amps, ulong size)
                {
                    return NativeMethods.GainHolo.AUTDGainHoloEVP(Ptr, foci, amps, size);
                }

                public override GainPtr EVPWithGamma(GainPtr ptr, float_t v)
                {
                    return NativeMethods.GainHolo.AUTDGainHoloEVPWithGamma(ptr, v);
                }

                public override GainPtr EVPWithConstraint(GainPtr ptr, ConstraintPtr v)
                {
                    return NativeMethods.GainHolo.AUTDGainHoloEVPWithConstraint(ptr, v);
                }

                public override GainPtr GS(float_t[]? foci, float_t[]? amps, ulong size)
                {
                    return NativeMethods.GainHolo.AUTDGainHoloGS(Ptr, foci, amps, size);
                }

                public override GainPtr GSWithRepeat(GainPtr ptr, uint v)
                {
                    return NativeMethods.GainHolo.AUTDGainHoloGSWithRepeat(ptr, v);
                }

                public override GainPtr GSWithConstraint(GainPtr ptr, ConstraintPtr v)
                {
                    return NativeMethods.GainHolo.AUTDGainHoloGSWithConstraint(ptr, v);
                }

                public override GainPtr GSPAT(float_t[]? foci, float_t[]? amps, ulong size)
                {
                    return NativeMethods.GainHolo.AUTDGainHoloGSPAT(Ptr, foci, amps, size);
                }

                public override GainPtr GSPATWithRepeat(GainPtr ptr, uint v)
                {
                    return NativeMethods.GainHolo.AUTDGainHoloGSPATWithRepeat(ptr, v);
                }

                public override GainPtr GSPATWithConstraint(GainPtr ptr, ConstraintPtr v)
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

                public override GainPtr LM(float_t[]? foci, float_t[]? amps, ulong size)
                {
                    return NativeMethods.GainHolo.AUTDGainHoloLM(Ptr, foci, amps, size);
                }

                public override GainPtr LMWithEps1(GainPtr ptr, float_t v)
                {
                    return NativeMethods.GainHolo.AUTDGainHoloLMWithEps1(ptr, v);
                }

                public override GainPtr LMWithEps2(GainPtr ptr, float_t v)
                {
                    return NativeMethods.GainHolo.AUTDGainHoloLMWithEps2(ptr, v);
                }

                public override GainPtr LMWithTau(GainPtr ptr, float_t v)
                {
                    return NativeMethods.GainHolo.AUTDGainHoloLMWithTau(ptr, v);
                }

                public override GainPtr LMWithKMax(GainPtr ptr, uint v)
                {
                    return NativeMethods.GainHolo.AUTDGainHoloLMWithKMax(ptr, v);
                }

                public override GainPtr LMWithInitial(GainPtr ptr, float_t[]? v,
                                                                                         ulong size)
                {
                    return NativeMethods.GainHolo.AUTDGainHoloLMWithInitial(ptr, v, size);
                }

                public override GainPtr LMWithConstraint(GainPtr ptr, ConstraintPtr v)
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
            /// Gain to produce multiple foci by solving Semi-Denfinite Programming
            /// </summary>
            /// <remarks>Inoue, Seki, Yasutoshi Makino, and Hiroyuki Shinoda. "Active touch perception produced by airborne ultrasonic haptic hologram." 2015 IEEE World Haptics Conference (WHC). IEEE, 2015.</remarks>
            /// <typeparam name="B">Backend</typeparam>
            public sealed class SDP<B> : IGain
                where B : Backend
            {
                private readonly B _backend;
                private readonly List<float_t> _foci;
                private readonly List<float_t> _amps;
                private float_t? _alpha;
                private float_t? _lambda;
                private uint? _repeat;
                private IAmplitudeConstraint? _constraint;

                public SDP(B backend)
                {
                    _backend = backend;
                    _foci = new List<float_t>();
                    _amps = new List<float_t>();
                }

                public SDP<B> AddFocus(Vector3 focus, float_t amp)
                {
                    _foci.Add(focus.x);
                    _foci.Add(focus.y);
                    _foci.Add(focus.z);
                    _amps.Add(amp);
                    return this;
                }

                /// <summary>
                /// Parameter. See the paper for details.
                /// </summary>
                /// <param name="value"></param>
                /// <returns></returns>
                public SDP<B> WithAlpha(float_t value)
                {
                    _alpha = value;
                    return this;
                }

                /// <summary>
                /// Parameter. See the paper for details.
                /// </summary>
                /// <param name="value"></param>
                /// <returns></returns>
                public SDP<B> WithLambda(float_t value)
                {
                    _lambda = value;
                    return this;
                }

                /// <summary>
                /// Parameter. See the paper for details.
                /// </summary>
                /// <param name="value"></param>
                /// <returns></returns>
                public SDP<B> WithRepeat(uint value)
                {
                    _repeat = value;
                    return this;
                }

                /// <summary>
                /// Set amplitude constraint
                /// </summary>
                /// <param name="constraint"></param>
                /// <returns></returns>
                public SDP<B> WithConstraint(IAmplitudeConstraint constraint)
                {
                    _constraint = constraint;
                    return this;
                }

                public override GainPtr GainPtr(Geometry geometry)
                {
                    var ptr = _backend.SDP(_foci.ToArray(), _amps.ToArray(),
                        (ulong)_amps.Count);
                    if (_alpha.HasValue) ptr = _backend.SDPWithAlpha(ptr, _alpha.Value);
                    if (_lambda.HasValue) ptr = _backend.SDPWithLambda(ptr, _lambda.Value);
                    if (_repeat.HasValue) ptr = _backend.SDPWithRepeat(ptr, _repeat.Value);
                    if (_constraint != null) ptr = _backend.SDPWithConstraint(ptr, _constraint.Ptr());
                    return ptr;
                }
            }

            /// <summary>
            /// Gain to produce multiple foci by solving Eigen Value Problem
            /// </summary>
            /// <remarks>Long, Benjamin, et al. "Rendering volumetric haptic shapes in mid-air using ultrasound." ACM Transactions on Graphics (TOG) 33.6 (2014): 1-10.</remarks>
            /// <typeparam name="B">Backend</typeparam>
            public sealed class EVP<B> : IGain
                where B : Backend
            {
                private readonly B _backend;
                private readonly List<float_t> _foci;
                private readonly List<float_t> _amps;
                private float_t? _gamma;
                private IAmplitudeConstraint? _constraint;

                public EVP(B backend)
                {
                    _backend = backend;
                    _foci = new List<float_t>();
                    _amps = new List<float_t>();
                }

                public EVP<B> AddFocus(Vector3 focus, float_t amp)
                {
                    _foci.Add(focus.x);
                    _foci.Add(focus.y);
                    _foci.Add(focus.z);
                    _amps.Add(amp);
                    return this;
                }

                /// <summary>
                /// Parameter. See the paper for details.
                /// </summary>
                /// <param name="value"></param>
                /// <returns></returns>
                public EVP<B> WithGamma(float_t value)
                {
                    _gamma = value;
                    return this;
                }

                /// <summary>
                /// Set amplitude constraint
                /// </summary>
                /// <param name="constraint"></param>
                /// <returns></returns>
                public EVP<B> WithConstraint(IAmplitudeConstraint constraint)
                {
                    _constraint = constraint;
                    return this;
                }

                public override GainPtr GainPtr(Geometry geometry)
                {
                    var ptr = _backend.EVP(_foci.ToArray(), _amps.ToArray(),
                        (ulong)_amps.Count);
                    if (_gamma.HasValue) ptr = _backend.EVPWithGamma(ptr, _gamma.Value);
                    if (_constraint != null) ptr = _backend.EVPWithConstraint(ptr, _constraint.Ptr());
                    return ptr;
                }
            }

            /// <summary>
            /// Gain to produce multiple foci with GS algorithm
            /// </summary>
            /// <remarks>Asier Marzo and Bruce W Drinkwater. Holographic acoustic tweezers.Proceedings of theNational Academy of Sciences, 116(1):84–89, 2019.</remarks>
            /// <typeparam name="B">Backend</typeparam>
            public sealed class GS<B> : IGain
                where B : Backend
            {
                private readonly B _backend;
                private readonly List<float_t> _foci;
                private readonly List<float_t> _amps;
                private uint? _repeat;
                private IAmplitudeConstraint? _constraint;

                public GS(B backend)
                {
                    _backend = backend;
                    _foci = new List<float_t>();
                    _amps = new List<float_t>();
                }

                public GS<B> AddFocus(Vector3 focus, float_t amp)
                {
                    _foci.Add(focus.x);
                    _foci.Add(focus.y);
                    _foci.Add(focus.z);
                    _amps.Add(amp);
                    return this;
                }

                /// <summary>
                /// Parameter. See the paper for details.
                /// </summary>
                /// <param name="value"></param>
                /// <returns></returns>
                public GS<B> WithRepeat(uint value)
                {
                    _repeat = value;
                    return this;
                }

                /// <summary>
                /// Set amplitude constraint
                /// </summary>
                /// <param name="constraint"></param>
                /// <returns></returns>
                public GS<B> WithConstraint(IAmplitudeConstraint constraint)
                {
                    _constraint = constraint;
                    return this;
                }

                public override GainPtr GainPtr(Geometry geometry)
                {
                    var ptr = _backend.GS(_foci.ToArray(), _amps.ToArray(),
                        (ulong)_amps.Count);
                    if (_repeat.HasValue) ptr = _backend.GSWithRepeat(ptr, _repeat.Value);
                    if (_constraint != null) ptr = _backend.GSWithConstraint(ptr, _constraint.Ptr());
                    return ptr;
                }
            }

            /// <summary>
            /// Gain to produce multiple foci with GS-PAT algorithm
            /// </summary>
            /// <remarks>Diego Martinez Plasencia et al. "Gs-pat: high-speed multi-point sound-fields for phased arrays of transducers," ACMTrans-actions on Graphics (TOG), 39(4):138–1, 2020.</remarks>
            /// <typeparam name="B">Backend</typeparam>
            public sealed class GSPAT<B> : IGain
                where B : Backend
            {
                private readonly B _backend;
                private readonly List<float_t> _foci;
                private readonly List<float_t> _amps;
                private uint? _repeat;
                private IAmplitudeConstraint? _constraint;

                public GSPAT(B backend)
                {
                    _backend = backend;
                    _foci = new List<float_t>();
                    _amps = new List<float_t>();
                }

                public GSPAT<B> AddFocus(Vector3 focus, float_t amp)
                {
                    _foci.Add(focus.x);
                    _foci.Add(focus.y);
                    _foci.Add(focus.z);
                    _amps.Add(amp);
                    return this;
                }

                /// <summary>
                /// Parameter. See the paper for details.
                /// </summary>
                /// <param name="value"></param>
                /// <returns></returns>
                public GSPAT<B> WithRepeat(uint value)
                {
                    _repeat = value;
                    return this;
                }

                /// <summary>
                /// Set amplitude constraint
                /// </summary>
                /// <param name="constraint"></param>
                /// <returns></returns>
                public GSPAT<B> WithConstraint(IAmplitudeConstraint constraint)
                {
                    _constraint = constraint;
                    return this;
                }

                public override GainPtr GainPtr(Geometry geometry)
                {
                    var ptr = _backend.GSPAT(_foci.ToArray(), _amps.ToArray(),
                        (ulong)_amps.Count);
                    if (_repeat.HasValue) ptr = _backend.GSPATWithRepeat(ptr, _repeat.Value);
                    if (_constraint != null) ptr = _backend.GSPATWithConstraint(ptr, _constraint.Ptr());
                    return ptr;
                }
            }

            /// <summary>
            /// Gain to produce multiple foci with naive linear synthesis
            /// </summary>
            /// <typeparam name="B">Backend</typeparam>
            public sealed class Naive<B> : IGain
                where B : Backend
            {
                private readonly B _backend;
                private readonly List<float_t> _foci;
                private readonly List<float_t> _amps;
                private IAmplitudeConstraint? _constraint;


                public Naive(B backend)
                {
                    _backend = backend;
                    _foci = new List<float_t>();
                    _amps = new List<float_t>();

                }

                public Naive<B> AddFocus(Vector3 focus, float_t amp)
                {
                    _foci.Add(focus.x);
                    _foci.Add(focus.y);
                    _foci.Add(focus.z);
                    _amps.Add(amp);
                    return this;
                }

                /// <summary>
                /// Set amplitude constraint
                /// </summary>
                /// <param name="constraint"></param>
                /// <returns></returns>
                public Naive<B> WithConstraint(IAmplitudeConstraint constraint)
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
            /// <typeparam name="B">Backend</typeparam>
            public sealed class LM<B> : IGain
                where B : Backend
            {
                private readonly B _backend;
                private readonly List<float_t> _foci;
                private readonly List<float_t> _amps;
                private float_t? _eps1;
                private float_t? _eps2;
                private float_t? _tau;
                private uint? _kMax;
                private float_t[]? _initial;

                private IAmplitudeConstraint? _constraint;

                public LM(B backend)
                {
                    _backend = backend;
                    _foci = new List<float_t>();
                    _amps = new List<float_t>();
                }

                /// <summary>
                /// Parameter. See the paper for details.
                /// </summary>
                /// <param name="value"></param>
                /// <returns></returns>
                public LM<B> AddFocus(Vector3 focus, float_t amp)
                {
                    _foci.Add(focus.x);
                    _foci.Add(focus.y);
                    _foci.Add(focus.z);
                    _amps.Add(amp);
                    return this;
                }

                /// <summary>
                /// Parameter. See the paper for details.
                /// </summary>
                /// <param name="value"></param>
                /// <returns></returns>
                public LM<B> WithEps1(float_t value)
                {
                    _eps1 = value;
                    return this;
                }

                /// <summary>
                /// Parameter. See the paper for details.
                /// </summary>
                /// <param name="value"></param>
                /// <returns></returns>
                public LM<B> WithEps2(float_t value)
                {
                    _eps2 = value;
                    return this;
                }

                /// <summary>
                /// Parameter. See the paper for details.
                /// </summary>
                /// <param name="value"></param>
                /// <returns></returns>
                public LM<B> WithTau(float_t value)
                {
                    _tau = value;
                    return this;
                }

                /// <summary>
                /// Parameter. See the paper for details.
                /// </summary>
                /// <param name="value"></param>
                /// <returns></returns>
                public LM<B> WithKMax(uint value)
                {
                    _kMax = value;
                    return this;
                }

                /// <summary>
                /// Set amplitude constraint
                /// </summary>
                /// <param name="constraint"></param>
                /// <returns></returns>
                public LM<B> WithConstraint(IAmplitudeConstraint constraint)
                {
                    _constraint = constraint;
                    return this;
                }

                /// <summary>
                /// Parameter. See the paper for details.
                /// </summary>
                /// <param name="value"></param>
                /// <returns></returns>
                public LM<B> WithInitial(float_t[] initial)
                {
                    _initial = initial;
                    return this;
                }

                public override GainPtr GainPtr(Geometry geometry)
                {
                    var ptr = _backend.LM(_foci.ToArray(), _amps.ToArray(),
                        (ulong)_amps.Count);
                    if (_eps1.HasValue) ptr = _backend.LMWithEps1(ptr, _eps1.Value);
                    if (_eps2.HasValue) ptr = _backend.LMWithEps2(ptr, _eps2.Value);
                    if (_tau.HasValue) ptr = _backend.LMWithTau(ptr, _tau.Value);
                    if (_kMax.HasValue) ptr = _backend.LMWithKMax(ptr, _kMax.Value);
                    if (_initial != null)
                        ptr = _backend.LMWithInitial(ptr, _initial, (ulong)_initial.Length);
                    if (_constraint != null) ptr = _backend.LMWithConstraint(ptr, _constraint.Ptr());
                    return ptr;
                }
            }

            /// <summary>
            /// Gain to produce multiple foci with greedy algorithm
            /// </summary>
            /// <remarks>
            /// Shun Suzuki, Masahiro Fujiwara, Yasutoshi Makino, and Hiroyuki Shinoda, “Radiation Pressure Field Reconstruction for Ultrasound Midair Haptics by Greedy Algorithm with Brute-Force Search,” in IEEE Transactions on Haptics, doi: 10.1109/TOH.2021.3076489
            /// </remarks>
            /// <typeparam name="B">Backend</typeparam>
            public sealed class Greedy : IGain
            {
                private readonly List<float_t> _foci;
                private readonly List<float_t> _amps;
                private uint? _phaseDiv;
                private IAmplitudeConstraint? _constraint;

                public Greedy()
                {
                    _foci = new List<float_t>();
                    _amps = new List<float_t>();
                }

                public Greedy AddFocus(Vector3 focus, float_t amp)
                {
                    _foci.Add(focus.x);
                    _foci.Add(focus.y);
                    _foci.Add(focus.z);
                    _amps.Add(amp);
                    return this;
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
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
