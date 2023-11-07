/*
 * File: CUDABackend.cs
 * Project: src
 * Created Date: 08/06/2023
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
            unsafe
            {
                fixed (byte* ep = err)
                {
                    Ptr = NativeMethodsBackendCUDA.AUTDCUDABackend(ep);
                }
            }
            if (Ptr.Item1 == IntPtr.Zero)
                throw new AUTDException(err);
        }

        ~CUDABackend()
        {
            if (Ptr.Item1 == IntPtr.Zero) return;
            NativeMethodsBackendCUDA.AUTDCUDABackendDelete(Ptr);
            Ptr.Item1 = IntPtr.Zero;
        }

        internal override GainPtr Sdp(float_t[] foci, float_t[] amps, ulong size)
        {
            unsafe
            {
                fixed (float_t* fp = foci)
                fixed (float_t* ap = amps)
                    return NativeMethodsBackendCUDA.AUTDGainHoloCUDASDP(Ptr, fp, ap, size);
            }
        }

        internal override GainPtr SdpWithAlpha(GainPtr ptr, float_t v)
        {
            return NativeMethodsBackendCUDA.AUTDGainHoloCUDASDPWithAlpha(ptr, v);
        }

        internal override GainPtr SdpWithRepeat(GainPtr ptr, uint v)
        {
            return NativeMethodsBackendCUDA.AUTDGainHoloCUDASDPWithRepeat(ptr, v);
        }

        internal override GainPtr SdpWithLambda(GainPtr ptr, float_t v)
        {
            return NativeMethodsBackendCUDA.AUTDGainHoloCUDASDPWithLambda(ptr, v);
        }

        internal override GainPtr SdpWithConstraint(GainPtr ptr, ConstraintPtr v)
        {
            return NativeMethodsBackendCUDA.AUTDGainHoloCUDASDPWithConstraint(ptr, v);
        }

        internal override GainPtr Evp(float_t[] foci, float_t[] amps, ulong size)
        {
            unsafe
            {
                fixed (float_t* fp = foci)
                fixed (float_t* ap = amps)
                    return NativeMethodsBackendCUDA.AUTDGainHoloCUDAEVP(Ptr, fp, ap, size);
            }
        }

        internal override GainPtr EvpWithGamma(GainPtr ptr, float_t v)
        {
            return NativeMethodsBackendCUDA.AUTDGainHoloCUDAEVPWithGamma(ptr, v);
        }

        internal override GainPtr EvpWithConstraint(GainPtr ptr, ConstraintPtr v)
        {
            return NativeMethodsBackendCUDA.AUTDGainHoloCUDAEVPWithConstraint(ptr, v);
        }

        internal override GainPtr Gs(float_t[] foci, float_t[] amps, ulong size)
        {
            unsafe
            {
                fixed (float_t* fp = foci)
                fixed (float_t* ap = amps)
                    return NativeMethodsBackendCUDA.AUTDGainHoloCUDAGS(Ptr, fp, ap, size);
            }
        }

        internal override GainPtr GsWithRepeat(GainPtr ptr, uint v)
        {
            return NativeMethodsBackendCUDA.AUTDGainHoloCUDAGSWithRepeat(ptr, v);
        }

        internal override GainPtr GsWithConstraint(GainPtr ptr, ConstraintPtr v)
        {
            return NativeMethodsBackendCUDA.AUTDGainHoloCUDAGSWithConstraint(ptr, v);
        }

        internal override GainPtr Gspat(float_t[] foci, float_t[] amps, ulong size)
        {
            unsafe
            {
                fixed (float_t* fp = foci)
                fixed (float_t* ap = amps)
                    return NativeMethodsBackendCUDA.AUTDGainHoloCUDAGSPAT(Ptr, fp, ap, size);
            }
        }

        internal override GainPtr GspatWithRepeat(GainPtr ptr, uint v)
        {
            return NativeMethodsBackendCUDA.AUTDGainHoloCUDAGSPATWithRepeat(ptr, v);
        }

        internal override GainPtr GspatWithConstraint(GainPtr ptr, ConstraintPtr v)
        {
            return NativeMethodsBackendCUDA.AUTDGainHoloCUDAGSPATWithConstraint(ptr, v);
        }

        internal override GainPtr Naive(float_t[] foci, float_t[] amps, ulong size)
        {
            unsafe
            {
                fixed (float_t* fp = foci)
                fixed (float_t* ap = amps)
                    return NativeMethodsBackendCUDA.AUTDGainHoloCUDANaive(Ptr, fp, ap, size);
            }
        }

        internal override GainPtr NaiveWithConstraint(GainPtr ptr, ConstraintPtr v)
        {
            return NativeMethodsBackendCUDA.AUTDGainHoloCUDANaiveWithConstraint(ptr, v);
        }

        internal override GainPtr Lm(float_t[] foci, float_t[] amps, ulong size)
        {
            unsafe
            {
                fixed (float_t* fp = foci)
                fixed (float_t* ap = amps)
                    return NativeMethodsBackendCUDA.AUTDGainHoloCUDALM(Ptr, fp, ap, size);
            }
        }

        internal override GainPtr LmWithEps1(GainPtr ptr, float_t v)
        {
            return NativeMethodsBackendCUDA.AUTDGainHoloCUDALMWithEps1(ptr, v);
        }

        internal override GainPtr LmWithEps2(GainPtr ptr, float_t v)
        {
            return NativeMethodsBackendCUDA.AUTDGainHoloCUDALMWithEps2(ptr, v);
        }

        internal override GainPtr LmWithTau(GainPtr ptr, float_t v)
        {
            return NativeMethodsBackendCUDA.AUTDGainHoloCUDALMWithTau(ptr, v);
        }

        internal override GainPtr LmWithKMax(GainPtr ptr, uint v)
        {
            return NativeMethodsBackendCUDA.AUTDGainHoloCUDALMWithKMax(ptr, v);
        }

        internal override GainPtr LmWithInitial(GainPtr ptr, float_t[] v, ulong size)
        {
            unsafe
            {
                fixed (float_t* vp = v)
                    return NativeMethodsBackendCUDA.AUTDGainHoloCUDALMWithInitial(ptr, vp, size);
            }
        }

        internal override GainPtr LmWithConstraint(GainPtr ptr, ConstraintPtr v)
        {
            return NativeMethodsBackendCUDA.AUTDGainHoloCUDALMWithConstraint(ptr, v);
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
