/*
 * File: BackendCUDA.cs
 * Project: src
 * Created Date: 08/06/2023
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

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

using System;
using System.Runtime.InteropServices;

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
            /// <summary>
            /// Backend using CUDA
            /// </summary>
            [ComVisible(false)]
            public class BackendCUDA : Backend
            {
                public BackendCUDA()
                {
                    var err = new byte[256];
                    Ptr = NativeMethods.BackendCUDA.AUTDCUDABackend(err);
                    if (Ptr._0 == IntPtr.Zero)
                        throw new AUTDException(err);
                }

                ~BackendCUDA()
                {
                    if (Ptr._0 != IntPtr.Zero)
                    {
                        NativeMethods.BackendCUDA.AUTDDeleteCUDABackend(Ptr);
                        Ptr._0 = IntPtr.Zero;
                    }
                }

                public override GainPtr SDP(float_t[]? foci, float_t[]? amps, ulong size)
                {
                    return NativeMethods.BackendCUDA.AUTDGainHoloSDPCUDA(Ptr, foci, amps, size);
                }

                public override GainPtr SDPWithAlpha(GainPtr ptr, float_t v)
                {
                    return NativeMethods.BackendCUDA.AUTDGainHoloSDPWithAlphaCUDA(ptr, v);
                }

                public override GainPtr SDPWithRepeat(GainPtr ptr, uint v)
                {
                    return NativeMethods.BackendCUDA.AUTDGainHoloSDPWithRepeatCUDA(ptr, v);
                }

                public override GainPtr SDPWithLambda(GainPtr ptr, float_t v)
                {
                    return NativeMethods.BackendCUDA.AUTDGainHoloSDPWithLambdaCUDA(ptr, v);
                }

                public override GainPtr SDPWithConstraint(GainPtr ptr, ConstraintPtr v)
                {
                    return NativeMethods.BackendCUDA.AUTDGainHoloSDPWithConstraintCUDA(ptr, v);
                }

                public override GainPtr EVP(float_t[]? foci, float_t[]? amps, ulong size)
                {
                    return NativeMethods.BackendCUDA.AUTDGainHoloEVPCUDA(Ptr, foci, amps, size);
                }

                public override GainPtr EVPWithGamma(GainPtr ptr, float_t v)
                {
                    return NativeMethods.BackendCUDA.AUTDGainHoloEVPWithGammaCUDA(ptr, v);
                }

                public override GainPtr EVPWithConstraint(GainPtr ptr, ConstraintPtr v)
                {
                    return NativeMethods.BackendCUDA.AUTDGainHoloEVPWithConstraintCUDA(ptr, v);
                }

                public override GainPtr GS(float_t[]? foci, float_t[]? amps, ulong size)
                {
                    return NativeMethods.BackendCUDA.AUTDGainHoloGSCUDA(Ptr, foci, amps, size);
                }

                public override GainPtr GSWithRepeat(GainPtr ptr, uint v)
                {
                    return NativeMethods.BackendCUDA.AUTDGainHoloGSWithRepeatCUDA(ptr, v);
                }

                public override GainPtr GSWithConstraint(GainPtr ptr, ConstraintPtr v)
                {
                    return NativeMethods.BackendCUDA.AUTDGainHoloGSWithConstraintCUDA(ptr, v);
                }

                public override GainPtr GSPAT(float_t[]? foci, float_t[]? amps, ulong size)
                {
                    return NativeMethods.BackendCUDA.AUTDGainHoloGSPATCUDA(Ptr, foci, amps, size);
                }

                public override GainPtr GSPATWithRepeat(GainPtr ptr, uint v)
                {
                    return NativeMethods.BackendCUDA.AUTDGainHoloGSPATWithRepeatCUDA(ptr, v);
                }

                public override GainPtr GSPATWithConstraint(GainPtr ptr, ConstraintPtr v)
                {
                    return NativeMethods.BackendCUDA.AUTDGainHoloGSPATWithConstraintCUDA(ptr, v);
                }

                public override GainPtr Naive(float_t[]? foci, float_t[]? amps, ulong size)
                {
                    return NativeMethods.BackendCUDA.AUTDGainHoloNaiveCUDA(Ptr, foci, amps, size);
                }

                public override GainPtr NaiveWithConstraint(GainPtr ptr, ConstraintPtr v)
                {
                    return NativeMethods.BackendCUDA.AUTDGainHoloNaiveWithConstraintCUDA(ptr, v);
                }

                public override GainPtr LM(float_t[]? foci, float_t[]? amps, ulong size)
                {
                    return NativeMethods.BackendCUDA.AUTDGainHoloLMCUDA(Ptr, foci, amps, size);
                }

                public override GainPtr LMWithEps1(GainPtr ptr, float_t v)
                {
                    return NativeMethods.BackendCUDA.AUTDGainHoloLMWithEps1CUDA(ptr, v);
                }

                public override GainPtr LMWithEps2(GainPtr ptr, float_t v)
                {
                    return NativeMethods.BackendCUDA.AUTDGainHoloLMWithEps2CUDA(ptr, v);
                }

                public override GainPtr LMWithTau(GainPtr ptr, float_t v)
                {
                    return NativeMethods.BackendCUDA.AUTDGainHoloLMWithTauCUDA(ptr, v);
                }

                public override GainPtr LMWithKMax(GainPtr ptr, uint v)
                {
                    return NativeMethods.BackendCUDA.AUTDGainHoloLMWithKMaxCUDA(ptr, v);
                }

                public override GainPtr LMWithInitial(GainPtr ptr, float_t[]? v,
                                                                                         ulong size)
                {
                    return NativeMethods.BackendCUDA.AUTDGainHoloLMWithInitialCUDA(ptr, v, size);
                }

                public override GainPtr LMWithConstraint(GainPtr ptr, ConstraintPtr v)
                {
                    return NativeMethods.BackendCUDA.AUTDGainHoloLMWithConstraintCUDA(ptr, v);
                }
            }
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
