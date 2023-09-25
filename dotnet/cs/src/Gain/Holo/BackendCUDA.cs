/*
 * File: CUDABackend.cs
 * Project: src
 * Created Date: 08/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/09/2023
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

#if USE_SINGLE
using float_t = System.Single;
#else
using float_t = System.Double;
#endif
using System;
using System.Runtime.InteropServices;

namespace AUTD3Sharp.Gain.Holo
{
    /// <summary>
    /// Backend using CUDA
    /// </summary>
    [ComVisible(false)]
    public class CUDABackend : Backend
    {
        public CUDABackend()
        {
            var err = new byte[256];
            Ptr = NativeMethods.BackendCUDA.AUTDCUDABackend(err);
            if (Ptr._0 == IntPtr.Zero)
                throw new AUTDException(err);
        }

        ~CUDABackend()
        {
            if (Ptr._0 == IntPtr.Zero) return;
            NativeMethods.BackendCUDA.AUTDCUDABackendDelete(Ptr);
            Ptr._0 = IntPtr.Zero;
        }

        public override GainPtr Sdp(float_t[]? foci, float_t[]? amps, ulong size)
        {
            return NativeMethods.BackendCUDA.AUTDGainHoloCUDASDP(Ptr, foci, amps, size);
        }

        public override GainPtr SdpWithAlpha(GainPtr ptr, float_t v)
        {
            return NativeMethods.BackendCUDA.AUTDGainHoloCUDASDPWithAlpha(ptr, v);
        }

        public override GainPtr SdpWithRepeat(GainPtr ptr, uint v)
        {
            return NativeMethods.BackendCUDA.AUTDGainHoloCUDASDPWithRepeat(ptr, v);
        }

        public override GainPtr SdpWithLambda(GainPtr ptr, float_t v)
        {
            return NativeMethods.BackendCUDA.AUTDGainHoloCUDASDPWithLambda(ptr, v);
        }

        public override GainPtr SdpWithConstraint(GainPtr ptr, ConstraintPtr v)
        {
            return NativeMethods.BackendCUDA.AUTDGainHoloCUDASDPWithConstraint(ptr, v);
        }

        public override GainPtr Evp(float_t[]? foci, float_t[]? amps, ulong size)
        {
            return NativeMethods.BackendCUDA.AUTDGainHoloCUDAEVP(Ptr, foci, amps, size);
        }

        public override GainPtr EvpWithGamma(GainPtr ptr, float_t v)
        {
            return NativeMethods.BackendCUDA.AUTDGainHoloCUDAEVPWithGamma(ptr, v);
        }

        public override GainPtr EvpWithConstraint(GainPtr ptr, ConstraintPtr v)
        {
            return NativeMethods.BackendCUDA.AUTDGainHoloCUDAEVPWithConstraint(ptr, v);
        }

        public override GainPtr Gs(float_t[]? foci, float_t[]? amps, ulong size)
        {
            return NativeMethods.BackendCUDA.AUTDGainHoloCUDAGS(Ptr, foci, amps, size);
        }

        public override GainPtr GsWithRepeat(GainPtr ptr, uint v)
        {
            return NativeMethods.BackendCUDA.AUTDGainHoloCUDAGSWithRepeat(ptr, v);
        }

        public override GainPtr GsWithConstraint(GainPtr ptr, ConstraintPtr v)
        {
            return NativeMethods.BackendCUDA.AUTDGainHoloCUDAGSWithConstraint(ptr, v);
        }

        public override GainPtr Gspat(float_t[]? foci, float_t[]? amps, ulong size)
        {
            return NativeMethods.BackendCUDA.AUTDGainHoloCUDAGSPAT(Ptr, foci, amps, size);
        }

        public override GainPtr GspatWithRepeat(GainPtr ptr, uint v)
        {
            return NativeMethods.BackendCUDA.AUTDGainHoloCUDAGSPATWithRepeat(ptr, v);
        }

        public override GainPtr GspatWithConstraint(GainPtr ptr, ConstraintPtr v)
        {
            return NativeMethods.BackendCUDA.AUTDGainHoloCUDAGSPATWithConstraint(ptr, v);
        }

        public override GainPtr Naive(float_t[]? foci, float_t[]? amps, ulong size)
        {
            return NativeMethods.BackendCUDA.AUTDGainHoloCUDANaive(Ptr, foci, amps, size);
        }

        public override GainPtr NaiveWithConstraint(GainPtr ptr, ConstraintPtr v)
        {
            return NativeMethods.BackendCUDA.AUTDGainHoloCUDANaiveWithConstraint(ptr, v);
        }

        public override GainPtr Lm(float_t[]? foci, float_t[]? amps, ulong size)
        {
            return NativeMethods.BackendCUDA.AUTDGainHoloCUDALM(Ptr, foci, amps, size);
        }

        public override GainPtr LmWithEps1(GainPtr ptr, float_t v)
        {
            return NativeMethods.BackendCUDA.AUTDGainHoloCUDALMWithEps1(ptr, v);
        }

        public override GainPtr LmWithEps2(GainPtr ptr, float_t v)
        {
            return NativeMethods.BackendCUDA.AUTDGainHoloCUDALMWithEps2(ptr, v);
        }

        public override GainPtr LmWithTau(GainPtr ptr, float_t v)
        {
            return NativeMethods.BackendCUDA.AUTDGainHoloCUDALMWithTau(ptr, v);
        }

        public override GainPtr LmWithKMax(GainPtr ptr, uint v)
        {
            return NativeMethods.BackendCUDA.AUTDGainHoloCUDALMWithKMax(ptr, v);
        }

        public override GainPtr LmWithInitial(GainPtr ptr, float_t[]? v,
                                                                                 ulong size)
        {
            return NativeMethods.BackendCUDA.AUTDGainHoloCUDALMWithInitial(ptr, v, size);
        }

        public override GainPtr LmWithConstraint(GainPtr ptr, ConstraintPtr v)
        {
            return NativeMethods.BackendCUDA.AUTDGainHoloCUDALMWithConstraint(ptr, v);
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
