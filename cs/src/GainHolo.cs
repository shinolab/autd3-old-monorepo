/*
 * File: GainHolo.cs
 * Project: src
 * Created Date: 23/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/12/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
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
using autd3_float_t = System.Single;
#else
using autd3_float_t = System.Double;
#endif

#if UNITY_2020_2_OR_NEWER
#nullable enable
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
                    NativeMethods.GainHolo.AUTDDeleteBackend(handle);
                    return true;
                }
            }

            public sealed class BackendEigen : Backend
            {
                public BackendEigen()
                {
                    NativeMethods.GainHolo.AUTDEigenBackend(out handle);
                }
            }

            public abstract class AmplitudeConstraint : SafeHandleZeroOrMinusOneIsInvalid
            {
                internal IntPtr Ptr => handle;

                internal AmplitudeConstraint() : base(true)
                {
                    var ptr = new IntPtr();
                    SetHandle(ptr);
                }

                protected override bool ReleaseHandle()
                {
                    return true;
                }
            }

            public sealed class DontCare : AmplitudeConstraint
            {
                public DontCare()
                {
                    NativeMethods.GainHolo.AUTDConstraintDontCare(out handle);
                }
            }

            public sealed class Normalize : AmplitudeConstraint
            {
                public Normalize()
                {
                    NativeMethods.GainHolo.AUTDConstraintNormalize(out handle);

                }
            }

            public sealed class Uniform : AmplitudeConstraint
            {

                public Uniform(autd3_float_t value = (autd3_float_t)1.0)
                {
                    NativeMethods.GainHolo.AUTDConstraintUniform(out handle, value);
                }
            }

            public sealed class Clamp : AmplitudeConstraint
            {
                public Clamp()
                {
                    NativeMethods.GainHolo.AUTDConstraintClamp(out handle);
                }
            }

            public class Holo : Gain
            {
                public Holo()
                {
                    Backend = new BackendEigen();
                }

                public Backend Backend { get; set; }
                public AmplitudeConstraint Constraint
                {
                    set => NativeMethods.GainHolo.AUTDSetConstraint(handle, value.Ptr);
                }

                public void Add(Vector3 focus, autd3_float_t amp) => NativeMethods.GainHolo.AUTDGainHoloAdd(handle, focus.x, focus.y, focus.z, amp);
            }

            public sealed class SDP : Holo
            {
                public SDP(autd3_float_t alpha = (autd3_float_t)1e-3, autd3_float_t lambda = (autd3_float_t)0.9, ulong repeat = 100)
                {
                    NativeMethods.GainHolo.AUTDGainHoloSDP(out handle, Backend.Ptr, alpha, lambda, repeat);
                }
            }
            public sealed class EVD : Holo
            {
                public EVD(autd3_float_t gamma = (autd3_float_t)1.0)
                {
                    NativeMethods.GainHolo.AUTDGainHoloEVD(out handle, Backend.Ptr, gamma);
                }
            }

            public sealed class Naive : Holo
            {
                public Naive()
                {
                    NativeMethods.GainHolo.AUTDGainHoloNaive(out handle, Backend.Ptr);
                }
            }

            public sealed class GS : Holo
            {
                public GS(ulong repeat = 100)
                {
                    NativeMethods.GainHolo.AUTDGainHoloGS(out handle, Backend.Ptr, repeat);
                }
            }

            public sealed class GSPAT : Holo
            {
                public GSPAT(ulong repeat = 100)
                {
                    NativeMethods.GainHolo.AUTDGainHoloGSPAT(out handle, Backend.Ptr, repeat);
                }
            }
            public sealed class LM : Holo
            {
                public LM(autd3_float_t eps1 = (autd3_float_t)1e-8, autd3_float_t eps2 = (autd3_float_t)1e-8, autd3_float_t tau = (autd3_float_t)1e-3, ulong kMax = 5, autd3_float_t[]? initial = null)
                {
                    NativeMethods.GainHolo.AUTDGainHoloLM(out handle, Backend.Ptr, eps1, eps2, tau, kMax, initial, initial?.Length ?? 0);
                }
            }

            public sealed class LSSGreedy : Holo
            {
                public LSSGreedy(int phaseDiv = 16)
                {
                    NativeMethods.GainHolo.AUTDGainHoloLSSGreedy(out handle, Backend.Ptr, phaseDiv);
                }
            }

            public sealed class APO : Holo
            {
                public APO(autd3_float_t eps = (autd3_float_t)1e-8, autd3_float_t lambda = (autd3_float_t)1.0, int kMax = 200, int lineSearchMax = 100)
                {
                    NativeMethods.GainHolo.AUTDGainHoloAPO(out handle, Backend.Ptr, eps, lambda, kMax, lineSearchMax);
                }
            }

            public sealed class Greedy : Holo
            {
                public Greedy(int phaseDiv = 16)
                {
                    NativeMethods.GainHolo.AUTDGainHoloGreedy(out handle, Backend.Ptr, phaseDiv);
                }
            }
        }
    }
}


#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif
