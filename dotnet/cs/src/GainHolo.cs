/*
* File: GainHolo.cs
* Project: src
* Created Date: 23/05/2021
* Author: Shun Suzuki
* -----
* Last Modified: 25/04/2023
* Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
* -----
* Copyright (c) 2021-2023 Shun Suzuki. All rights reserved.
* 
*/

#if UNITY_2018_3_OR_NEWER
#define USE_SINGLE
#endif

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
            public class Backend
            {
                ~Backend()
                {
                    NativeMethods.GainHolo.AUTDDeleteBackend(Ptr);
                }
                internal BackendPtr Ptr;
            }

            public sealed class BackendDefault : Backend
            {
                public BackendDefault()
                {
                    Ptr = NativeMethods.GainHolo.AUTDDefaultBackend();
                }
            }

            public interface IAmplitudeConstraint
            {
                public ConstraintPtr Ptr();
            }

            public sealed class DontCare : IAmplitudeConstraint
            {
                public ConstraintPtr Ptr() => NativeMethods.GainHolo.AUTDGainHoloDotCareConstraint();
            }

            public sealed class Normalize : IAmplitudeConstraint
            {
                public ConstraintPtr Ptr() => NativeMethods.GainHolo.AUTDGainHoloNormalizeConstraint();
            }

            public sealed class Uniform : IAmplitudeConstraint
            {

                internal readonly float_t Value;

                public Uniform(float_t value = 1)
                {
                    Value = value;
                }

                public ConstraintPtr Ptr() => NativeMethods.GainHolo.AUTDGainHoloUniformConstraint(Value);
            }

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

            public abstract class Holo : GainBase
            {
                protected List<float_t> Foci;
                protected List<float_t> Amps;
                protected Backend Backend;

                protected Holo() : this(new BackendDefault())
                {
                }

                protected Holo(Backend backend)
                {
                    Foci = new List<float_t>();
                    Amps = new List<float_t>();
                    Backend = backend;
                }

                public void AddFocus(Vector3 focus, float_t amp)
                {
                    Foci.Add(focus.x);
                    Foci.Add(focus.y);
                    Foci.Add(focus.z);
                    Amps.Add(amp);
                }
            }

            public sealed class SDP : Holo
            {
                private float_t? _alpha;
                private float_t? _lambda;
                private uint? _repeat;
                private IAmplitudeConstraint? _constraint;

                public SDP()
                { }
                public SDP(Backend backend) : base(backend)
                { }

                public SDP WithAlpha(float_t value)
                {
                    _alpha = value;
                    return this;
                }

                public SDP WithLambda(float_t value)
                {
                    _lambda = value;
                    return this;
                }

                public SDP WithRepeat(uint value)
                {
                    _repeat = value;
                    return this;
                }

                public SDP WithConstraint(IAmplitudeConstraint constraint)
                {
                    _constraint = constraint;
                    return this;
                }

                public override GainPtr GainPtr(Geometry geometry)
                {
                    var ptr = NativeMethods.GainHolo.AUTDGainHoloSDP(Backend.Ptr, Foci.ToArray(), Amps.ToArray(),
                        (ulong)Amps.Count);
                    if (_alpha.HasValue) ptr = NativeMethods.GainHolo.AUTDGainHoloSDPWithAlpha(ptr, _alpha.Value);
                    if (_lambda.HasValue) ptr = NativeMethods.GainHolo.AUTDGainHoloSDPWithLambda(ptr, _lambda.Value);
                    if (_repeat.HasValue) ptr = NativeMethods.GainHolo.AUTDGainHoloSDPWithRepeat(ptr, _repeat.Value);
                    if (_constraint != null) ptr = NativeMethods.GainHolo.AUTDGainHoloSDPWithConstraint(ptr, _constraint.Ptr());
                    return ptr;
                }
            }


            public sealed class EVP : Holo
            {
                private float_t? _gamma;
                private IAmplitudeConstraint? _constraint;

                public EVP()
                { }

                public EVP(Backend backend) : base(backend)
                { }

                public EVP WithGamma(float_t value)
                {
                    _gamma = value;
                    return this;
                }

                public EVP WithConstraint(IAmplitudeConstraint constraint)
                {
                    _constraint = constraint;
                    return this;
                }

                public override GainPtr GainPtr(Geometry geometry)
                {
                    var ptr = NativeMethods.GainHolo.AUTDGainHoloEVP(Backend.Ptr, Foci.ToArray(), Amps.ToArray(),
                        (ulong)Amps.Count);
                    if (_gamma.HasValue) ptr = NativeMethods.GainHolo.AUTDGainHoloEVPWithGamma(ptr, _gamma.Value);
                    if (_constraint != null) ptr = NativeMethods.GainHolo.AUTDGainHoloEVPWithConstraint(ptr, _constraint.Ptr());
                    return ptr;
                }
            }

            public sealed class GS : Holo
            {
                private uint? _repeat;
                private IAmplitudeConstraint? _constraint;

                public GS()
                { }

                public GS(Backend backend) : base(backend)
                { }


                public GS WithRepeat(uint value)
                {
                    _repeat = value;
                    return this;
                }

                public GS WithConstraint(IAmplitudeConstraint constraint)
                {
                    _constraint = constraint;
                    return this;
                }

                public override GainPtr GainPtr(Geometry geometry)
                {
                    var ptr = NativeMethods.GainHolo.AUTDGainHoloGS(Backend.Ptr, Foci.ToArray(), Amps.ToArray(),
                        (ulong)Amps.Count);
                    if (_repeat.HasValue) ptr = NativeMethods.GainHolo.AUTDGainHoloGSWithRepeat(ptr, _repeat.Value);
                    if (_constraint != null) ptr = NativeMethods.GainHolo.AUTDGainHoloGSWithConstraint(ptr, _constraint.Ptr());
                    return ptr;
                }
            }

            public sealed class GSPAT : Holo
            {
                private uint? _repeat;
                private IAmplitudeConstraint? _constraint;

                public GSPAT()
                { }
                public GSPAT(Backend backend) : base(backend)
                { }

                public GSPAT WithRepeat(uint value)
                {
                    _repeat = value;
                    return this;
                }

                public GSPAT WithConstraint(IAmplitudeConstraint constraint)
                {
                    _constraint = constraint;
                    return this;
                }

                public override GainPtr GainPtr(Geometry geometry)
                {
                    var ptr = NativeMethods.GainHolo.AUTDGainHoloGSPAT(Backend.Ptr, Foci.ToArray(), Amps.ToArray(),
                        (ulong)Amps.Count);
                    if (_repeat.HasValue) ptr = NativeMethods.GainHolo.AUTDGainHoloGSPATWithRepeat(ptr, _repeat.Value);
                    if (_constraint != null) ptr = NativeMethods.GainHolo.AUTDGainHoloGSPATWithConstraint(ptr, _constraint.Ptr());
                    return ptr;
                }
            }

            public sealed class Naive : Holo
            {
                private IAmplitudeConstraint? _constraint;

                public Naive()
                { }

                public Naive(Backend backend) : base(backend)
                {
                }

                public Naive WithConstraint(IAmplitudeConstraint constraint)
                {
                    _constraint = constraint;
                    return this;
                }

                public Naive WithBackend(Backend backend)
                {
                    Backend = backend;
                    return this;
                }

                public override GainPtr GainPtr(Geometry geometry)
                {
                    var ptr = NativeMethods.GainHolo.AUTDGainHoloNaive(Backend.Ptr, Foci.ToArray(), Amps.ToArray(),
                        (ulong)Amps.Count);
                    if (_constraint != null) ptr = NativeMethods.GainHolo.AUTDGainHoloNaiveWithConstraint(ptr, _constraint.Ptr());
                    return ptr;
                }
            }

            public sealed class LM : Holo
            {
                private float_t? _eps1;
                private float_t? _eps2;
                private float_t? _tau;
                private uint? _kMax;
                private float_t[]? _initial;

                private IAmplitudeConstraint? _constraint;

                public LM()
                { }
                public LM(Backend backend) : base(backend)
                {
                }

                public LM WithEps1(float_t value)
                {
                    _eps1 = value;
                    return this;
                }

                public LM WithEps2(float_t value)
                {
                    _eps2 = value;
                    return this;
                }

                public LM WithTau(float_t value)
                {
                    _tau = value;
                    return this;
                }

                public LM WithKMax(uint value)
                {
                    _kMax = value;
                    return this;
                }

                public LM WithConstraint(IAmplitudeConstraint constraint)
                {
                    _constraint = constraint;
                    return this;
                }

                public LM WithInitial(float_t[] initial)
                {
                    _initial = initial;
                    return this;
                }

                public override GainPtr GainPtr(Geometry geometry)
                {
                    var ptr = NativeMethods.GainHolo.AUTDGainHoloLM(Backend.Ptr, Foci.ToArray(), Amps.ToArray(),
                        (ulong)Amps.Count);
                    if (_eps1.HasValue) ptr = NativeMethods.GainHolo.AUTDGainHoloLMWithEps1(ptr, _eps1.Value);
                    if (_eps2.HasValue) ptr = NativeMethods.GainHolo.AUTDGainHoloLMWithEps2(ptr, _eps2.Value);
                    if (_tau.HasValue) ptr = NativeMethods.GainHolo.AUTDGainHoloLMWithTau(ptr, _tau.Value);
                    if (_kMax.HasValue) ptr = NativeMethods.GainHolo.AUTDGainHoloLMWithKMax(ptr, _kMax.Value);
                    if (_initial != null)
                        ptr = NativeMethods.GainHolo.AUTDGainHoloLMWithInitial(ptr, _initial, (ulong)_initial.Length);
                    if (_constraint != null) ptr = NativeMethods.GainHolo.AUTDGainHoloLMWithConstraint(ptr, _constraint.Ptr());
                    return ptr;
                }
            }

            public sealed class Greedy : Holo
            {
                private uint? _phaseDiv;
                private IAmplitudeConstraint? _constraint;

                public Greedy WithPhaseDiv(uint value)
                {
                    _phaseDiv = value;
                    return this;
                }

                public Greedy WithConstraint(IAmplitudeConstraint constraint)
                {
                    _constraint = constraint;
                    return this;
                }

                public override GainPtr GainPtr(Geometry geometry)
                {
                    var ptr = NativeMethods.GainHolo.AUTDGainHoloGreedy(Foci.ToArray(), Amps.ToArray(),
                        (ulong)Amps.Count);
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
#nullable disable
#endif
