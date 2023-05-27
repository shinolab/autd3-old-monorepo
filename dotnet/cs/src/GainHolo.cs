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
using Microsoft.Win32.SafeHandles;

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
            public abstract class Backend : SafeHandleZeroOrMinusOneIsInvalid
            {
                internal IntPtr Ptr => handle;

                internal Backend() : base(true)
                {
                    var ptr = new IntPtr();
                    SetHandle(ptr);
                }

                protected override bool ReleaseHandle()
                {
                    return true;
                }
            }

            public sealed class BackendDefault : Backend
            {
                public BackendDefault()
                {
                    handle = NativeMethods.GainHolo.AUTDDefaultBackend();
                }
            }

            public class Holo : Gain
            {
                public Holo(Backend backend)
                {
                    Backend = backend;
                }

                public Holo()
                {
                    Backend = new BackendDefault();
                }

                public Backend Backend { get; set; }

                public AmplitudeConstraint Constraint
                {
                    set => value.Set(handle);
                }

                public void Add(Vector3 focus, float_t amp) => NativeMethods.GainHolo.AUTDGainHoloAdd(handle, focus.x, focus.y, focus.z, amp);
            }


            public abstract class AmplitudeConstraint
            {
                internal abstract void Set(IntPtr handle);
            }

            public sealed class DontCare : AmplitudeConstraint
            {
                public DontCare()
                {
                }

                internal override void Set(IntPtr handle)
                {
                    NativeMethods.GainHolo.AUTDGainHoloSetDotCareConstraint(handle);
                }
            }

            public sealed class Normalize : AmplitudeConstraint
            {
                public Normalize()
                {

                }

                internal override void Set(IntPtr handle)
                {
                    NativeMethods.GainHolo.AUTDGainHoloSetNormalizeConstraint(handle);
                }
            }

            public sealed class Uniform : AmplitudeConstraint
            {

                private readonly float_t _value;

                public Uniform(float_t value = (float_t)1.0)
                {
                    _value = value;
                }

                internal override void Set(IntPtr handle)
                {
                    NativeMethods.GainHolo.AUTDGainHoloSetUniformConstraint(handle, _value);
                }
            }

            public sealed class Clamp : AmplitudeConstraint
            {
                private readonly float_t _min;
                private readonly float_t _max;

                public Clamp(float_t min = (float_t)0.0, float_t max = (float_t)1.0)
                {
                    _min = min;
                    _max = max;
                }

                internal override void Set(IntPtr handle)
                {
                    NativeMethods.GainHolo.AUTDGainHoloSetClampConstraint(handle, _min, _max);
                }
            }

            public sealed class SDP : Holo
            {
                public SDP()
                {
                    handle = NativeMethods.GainHolo.AUTDGainHoloSDP(Backend.Ptr);
                }

                public float_t Alpha
                {
                    set => NativeMethods.GainHolo.AUTDGainHoloSDPAlpha(handle, value);
                }

                public float_t Lambda
                {
                    set => NativeMethods.GainHolo.AUTDGainHoloSDPLambda(handle, value);
                }

                public uint Repeat
                {
                    set => NativeMethods.GainHolo.AUTDGainHoloSDPRepeat(handle, value);
                }
            }

            public sealed class EVP : Holo
            {
                public EVP()
                {
                    handle = NativeMethods.GainHolo.AUTDGainHoloEVP(Backend.Ptr);
                }

                public float_t Gamma
                {
                    set => NativeMethods.GainHolo.AUTDGainHoloEVPGamma(handle, value);
                }
            }

            public sealed class Naive : Holo
            {
                public Naive()
                {
                    handle = NativeMethods.GainHolo.AUTDGainHoloNaive(Backend.Ptr);
                }
            }

            public sealed class GS : Holo
            {
                public GS()
                {
                    handle = NativeMethods.GainHolo.AUTDGainHoloGS(Backend.Ptr);
                }

                public uint Repeat
                {
                    set => NativeMethods.GainHolo.AUTDGainHoloGSRepeat(handle, value);
                }
            }

            public sealed class GSPAT : Holo
            {
                public GSPAT()
                {
                    handle = NativeMethods.GainHolo.AUTDGainHoloGSPAT(Backend.Ptr);
                }

                public uint Repeat
                {
                    set => NativeMethods.GainHolo.AUTDGainHoloGSPATRepeat(handle, value);
                }
            }

            public sealed class LM : Holo
            {
                public LM()
                {
                    handle = NativeMethods.GainHolo.AUTDGainHoloLM(Backend.Ptr);
                }

                public float_t Eps1
                {
                    set => NativeMethods.GainHolo.AUTDGainHoloLMEps1(handle, value);
                }

                public float_t Eps2
                {
                    set => NativeMethods.GainHolo.AUTDGainHoloLMEps2(handle, value);
                }

                public float_t Tau
                {
                    set => NativeMethods.GainHolo.AUTDGainHoloLMTau(handle, value);
                }

                public uint KMax
                {
                    set => NativeMethods.GainHolo.AUTDGainHoloLMKMax(handle, value);
                }

                public float_t[]? Initial
                {
                    set => NativeMethods.GainHolo.AUTDGainHoloLMInitial(handle, value, (ulong)(value?.Length ?? 0));
                }
            }

            public sealed class Greedy : Holo
            {
                public Greedy()
                {
                    handle = NativeMethods.GainHolo.AUTDGainHoloGreedy();
                }

                public uint PhaseDiv
                {
                    set => NativeMethods.GainHolo.AUTDGainHoloGreedyPhaseDiv(handle, value);
                }
            }
        }
    }
}
