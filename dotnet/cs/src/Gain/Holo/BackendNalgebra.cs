/*
 * File: BackendNalgebra.cs
 * Project: Holo
 * Created Date: 13/09/2023
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

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

#if USE_SINGLE
using float_t = System.Single;
#else
using float_t = System.Double;
#endif

namespace AUTD3Sharp.Gain.Holo
{
    /// <summary>
    /// Backend using <see href="https://nalgebra.org/">Nalgebra</see>
    /// </summary>
    public sealed class NalgebraBackend : Backend
    {
        public NalgebraBackend()
        {
            Ptr = NativeMethods.GainHolo.AUTDNalgebraBackend();
        }

        ~NalgebraBackend()
        {
            if (Ptr._0 == IntPtr.Zero) return;
            NativeMethods.GainHolo.AUTDDeleteNalgebraBackend(Ptr);
            Ptr._0 = IntPtr.Zero;
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
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
