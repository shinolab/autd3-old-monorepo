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

using System;
using System.Runtime.InteropServices;

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
                internal IntPtr Ptr { get; set; }

                internal Backend(IntPtr ptr)
                {
                    Ptr = ptr;
                }
            }

            public sealed class BackendDefault : Backend
            {
                public BackendDefault() : base(NativeMethods.GainHolo.AUTDDefaultBackend())
                {
                }
            }

            public abstract class AmplitudeConstraint { }

            public sealed class DontCare : AmplitudeConstraint
            {
                public DontCare()
                {
                }
            }

            public sealed class Normalize : AmplitudeConstraint
            {
                public Normalize()
                {
                }
            }

            public sealed class Uniform : AmplitudeConstraint
            {

                internal readonly float_t Value;

                public Uniform(float_t value = (float_t)1.0)
                {
                    Value = value;
                }
            }

            public sealed class Clamp : AmplitudeConstraint
            {
                internal readonly float_t Min;
                internal readonly float_t Max;

                public Clamp(float_t min = (float_t)0.0, float_t max = (float_t)1.0)
                {
                    Min = min;
                    Max = max;
                }
            }

            public sealed class SDP : Gain
            {
                public SDP(Backend backend) : base(NativeMethods.GainHolo.AUTDGainHoloSDP(backend.Ptr))
                {
                }

                public float_t Alpha
                {
                    set => NativeMethods.GainHolo.AUTDGainHoloSDPAlpha(Ptr, value);
                }

                public float_t Lambda
                {
                    set => NativeMethods.GainHolo.AUTDGainHoloSDPLambda(Ptr, value);
                }

                public uint Repeat
                {
                    set => NativeMethods.GainHolo.AUTDGainHoloSDPRepeat(Ptr, value);
                }

                public void Add(Vector3 focus, float_t amp) => NativeMethods.GainHolo.AUTDGainHoloSDPAdd(Ptr, focus.x, focus.y, focus.z, amp);

                public void SetConstraint(AmplitudeConstraint constraint)
                {
                    switch (constraint)
                    {
                        case DontCare _:
                            NativeMethods.GainHolo.AUTDGainHoloSDPSetDotCareConstraint(Ptr);
                            break;
                        case Normalize _:
                            NativeMethods.GainHolo.AUTDGainHoloSDPSetNormalizeConstraint(Ptr);
                            break;
                        case Uniform c:
                            NativeMethods.GainHolo.AUTDGainHoloSDPSetUniformConstraint(Ptr, c.Value);
                            break;
                        case Clamp c:
                            NativeMethods.GainHolo.AUTDGainHoloSDPSetClampConstraint(Ptr, c.Min, c.Max);
                            break;
                        default:
                            throw new NotImplementedException();
                    }
                }
            }

            public sealed class EVP : Gain
            {
                public EVP(Backend backend)
                    : base(NativeMethods.GainHolo.AUTDGainHoloEVP(backend.Ptr))
                {
                }

                public float_t Gamma
                {
                    set => NativeMethods.GainHolo.AUTDGainHoloEVPGamma(Ptr, value);
                }

                public void Add(Vector3 focus, float_t amp) => NativeMethods.GainHolo.AUTDGainHoloEVPAdd(Ptr, focus.x, focus.y, focus.z, amp);

                public void SetConstraint(AmplitudeConstraint constraint)
                {
                    switch (constraint)
                    {
                        case DontCare _:
                            NativeMethods.GainHolo.AUTDGainHoloEVPSetDotCareConstraint(Ptr);
                            break;
                        case Normalize _:
                            NativeMethods.GainHolo.AUTDGainHoloEVPSetNormalizeConstraint(Ptr);
                            break;
                        case Uniform c:
                            NativeMethods.GainHolo.AUTDGainHoloEVPSetUniformConstraint(Ptr, c.Value);
                            break;
                        case Clamp c:
                            NativeMethods.GainHolo.AUTDGainHoloEVPSetClampConstraint(Ptr, c.Min, c.Max);
                            break;
                        default:
                            throw new NotImplementedException();
                    }
                }
            }

            public sealed class Naive : Gain
            {
                public Naive(Backend backend)
                : base(NativeMethods.GainHolo.AUTDGainHoloNaive(backend.Ptr))
                {
                }

                public void Add(Vector3 focus, float_t amp) => NativeMethods.GainHolo.AUTDGainHoloNaiveAdd(Ptr, focus.x, focus.y, focus.z, amp);

                public void SetConstraint(AmplitudeConstraint constraint)
                {
                    switch (constraint)
                    {
                        case DontCare _:
                            NativeMethods.GainHolo.AUTDGainHoloNaiveSetDotCareConstraint(Ptr);
                            break;
                        case Normalize _:
                            NativeMethods.GainHolo.AUTDGainHoloNaiveSetNormalizeConstraint(Ptr);
                            break;
                        case Uniform c:
                            NativeMethods.GainHolo.AUTDGainHoloNaiveSetUniformConstraint(Ptr, c.Value);
                            break;
                        case Clamp c:
                            NativeMethods.GainHolo.AUTDGainHoloNaiveSetClampConstraint(Ptr, c.Min, c.Max);
                            break;
                        default:
                            throw new NotImplementedException();
                    }
                }
            }

            public sealed class GS : Gain
            {
                public GS(Backend backend) : base(NativeMethods.GainHolo.AUTDGainHoloGS(backend.Ptr))
                {
                }

                public uint Repeat
                {
                    set => NativeMethods.GainHolo.AUTDGainHoloGSRepeat(Ptr, value);
                }

                public void Add(Vector3 focus, float_t amp) => NativeMethods.GainHolo.AUTDGainHoloGSAdd(Ptr, focus.x, focus.y, focus.z, amp);

                public void SetConstraint(AmplitudeConstraint constraint)
                {
                    switch (constraint)
                    {
                        case DontCare _:
                            NativeMethods.GainHolo.AUTDGainHoloGSSetDotCareConstraint(Ptr);
                            break;
                        case Normalize _:
                            NativeMethods.GainHolo.AUTDGainHoloGSSetNormalizeConstraint(Ptr);
                            break;
                        case Uniform c:
                            NativeMethods.GainHolo.AUTDGainHoloGSSetUniformConstraint(Ptr, c.Value);
                            break;
                        case Clamp c:
                            NativeMethods.GainHolo.AUTDGainHoloGSSetClampConstraint(Ptr, c.Min, c.Max);
                            break;
                        default:
                            throw new NotImplementedException();
                    }
                }
            }

            public sealed class GSPAT : Gain
            {
                public GSPAT(Backend backend) : base(NativeMethods.GainHolo.AUTDGainHoloGSPAT(backend.Ptr))
                {
                }

                public uint Repeat
                {
                    set => NativeMethods.GainHolo.AUTDGainHoloGSPATRepeat(Ptr, value);
                }

                public void Add(Vector3 focus, float_t amp) => NativeMethods.GainHolo.AUTDGainHoloGSPATAdd(Ptr, focus.x, focus.y, focus.z, amp);

                public void SetConstraint(AmplitudeConstraint constraint)
                {
                    switch (constraint)
                    {
                        case DontCare _:
                            NativeMethods.GainHolo.AUTDGainHoloGSPATSetDotCareConstraint(Ptr);
                            break;
                        case Normalize _:
                            NativeMethods.GainHolo.AUTDGainHoloGSPATSetNormalizeConstraint(Ptr);
                            break;
                        case Uniform c:
                            NativeMethods.GainHolo.AUTDGainHoloGSPATSetUniformConstraint(Ptr, c.Value);
                            break;
                        case Clamp c:
                            NativeMethods.GainHolo.AUTDGainHoloGSPATSetClampConstraint(Ptr, c.Min, c.Max);
                            break;
                        default:
                            throw new NotImplementedException();
                    }
                }
            }

            public sealed class LM : Gain
            {
                public LM(Backend backend) : base(NativeMethods.GainHolo.AUTDGainHoloLM(backend.Ptr))
                {
                }

                public float_t Eps1
                {
                    set => NativeMethods.GainHolo.AUTDGainHoloLMEps1(Ptr, value);
                }

                public float_t Eps2
                {
                    set => NativeMethods.GainHolo.AUTDGainHoloLMEps2(Ptr, value);
                }

                public float_t Tau
                {
                    set => NativeMethods.GainHolo.AUTDGainHoloLMTau(Ptr, value);
                }

                public uint KMax
                {
                    set => NativeMethods.GainHolo.AUTDGainHoloLMKMax(Ptr, value);
                }

                public float_t[]? Initial
                {
                    set => NativeMethods.GainHolo.AUTDGainHoloLMInitial(Ptr, value, (ulong)(value?.Length ?? 0));
                }

                public void Add(Vector3 focus, float_t amp) => NativeMethods.GainHolo.AUTDGainHoloLMAdd(Ptr, focus.x, focus.y, focus.z, amp);

                public void SetConstraint(AmplitudeConstraint constraint)
                {
                    switch (constraint)
                    {
                        case DontCare _:
                            NativeMethods.GainHolo.AUTDGainHoloLMSetDotCareConstraint(Ptr);
                            break;
                        case Normalize _:
                            NativeMethods.GainHolo.AUTDGainHoloLMSetNormalizeConstraint(Ptr);
                            break;
                        case Uniform c:
                            NativeMethods.GainHolo.AUTDGainHoloLMSetUniformConstraint(Ptr, c.Value);
                            break;
                        case Clamp c:
                            NativeMethods.GainHolo.AUTDGainHoloLMSetClampConstraint(Ptr, c.Min, c.Max);
                            break;
                        default:
                            throw new NotImplementedException();
                    }
                }
            }

            public sealed class Greedy : Gain
            {
                public Greedy() : base(NativeMethods.GainHolo.AUTDGainHoloGreedy())
                {
                }

                public uint PhaseDiv
                {
                    set => NativeMethods.GainHolo.AUTDGainHoloGreedyPhaseDiv(Ptr, value);
                }

                public void Add(Vector3 focus, float_t amp) => NativeMethods.GainHolo.AUTDGainHoloGreedyAdd(Ptr, focus.x, focus.y, focus.z, amp);

                public void SetConstraint(AmplitudeConstraint constraint)
                {
                    switch (constraint)
                    {
                        case DontCare _:
                            NativeMethods.GainHolo.AUTDGainHoloGreedySetDotCareConstraint(Ptr);
                            break;
                        case Normalize _:
                            NativeMethods.GainHolo.AUTDGainHoloGreedySetNormalizeConstraint(Ptr);
                            break;
                        case Uniform c:
                            NativeMethods.GainHolo.AUTDGainHoloGreedySetUniformConstraint(Ptr, c.Value);
                            break;
                        case Clamp c:
                            NativeMethods.GainHolo.AUTDGainHoloGreedySetClampConstraint(Ptr, c.Min, c.Max);
                            break;
                        default:
                            throw new NotImplementedException();
                    }
                }
            }
        }
    }
}
