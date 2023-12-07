/*
 * File: BackendNalgebra.cs
 * Project: Holo
 * Created Date: 13/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 07/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

#if UNITY_2018_3_OR_NEWER
#define USE_SINGLE
#endif

using System;
using System.Diagnostics.CodeAnalysis;
using AUTD3Sharp.NativeMethods;

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
            Ptr = NativeMethodsGainHolo.AUTDNalgebraBackend();
        }

        [ExcludeFromCodeCoverage]
        ~NalgebraBackend()
        {
            NativeMethodsGainHolo.AUTDDeleteNalgebraBackend(Ptr);
            Ptr.Item1 = IntPtr.Zero;
        }

        internal override GainPtr Sdp(float_t[] foci, Amplitude[] amps, ulong size)
        {
            unsafe
            {
                fixed (float_t* pf = &foci[0])
                fixed (Amplitude* pa = &amps[0])
                {
                    return NativeMethodsGainHolo.AUTDGainHoloSDP(Ptr, pf, (float_t*)pa, size);
                }
            }
        }

        internal override GainPtr SdpWithAlpha(GainPtr ptr, float_t v)
        {
            return NativeMethodsGainHolo.AUTDGainHoloSDPWithAlpha(ptr, v);
        }

        internal override GainPtr SdpWithRepeat(GainPtr ptr, uint v)
        {
            return NativeMethodsGainHolo.AUTDGainHoloSDPWithRepeat(ptr, v);
        }

        internal override GainPtr SdpWithLambda(GainPtr ptr, float_t v)
        {
            return NativeMethodsGainHolo.AUTDGainHoloSDPWithLambda(ptr, v);
        }

        internal override GainPtr SdpWithConstraint(GainPtr ptr, EmissionConstraintPtr v)
        {
            return NativeMethodsGainHolo.AUTDGainHoloSDPWithConstraint(ptr, v);
        }

        internal override GainPtr Gs(float_t[] foci, Amplitude[] amps, ulong size)
        {
            unsafe
            {
                fixed (float_t* pf = &foci[0])
                fixed (Amplitude* pa = &amps[0])
                {
                    return NativeMethodsGainHolo.AUTDGainHoloGS(Ptr, pf, (float_t*)pa, size);
                }
            }
        }

        internal override GainPtr GsWithRepeat(GainPtr ptr, uint v)
        {
            return NativeMethodsGainHolo.AUTDGainHoloGSWithRepeat(ptr, v);
        }

        internal override GainPtr GsWithConstraint(GainPtr ptr, EmissionConstraintPtr v)
        {
            return NativeMethodsGainHolo.AUTDGainHoloGSWithConstraint(ptr, v);
        }

        internal override GainPtr Gspat(float_t[] foci, Amplitude[] amps, ulong size)
        {
            unsafe
            {
                fixed (float_t* pf = &foci[0])
                fixed (Amplitude* pa = &amps[0])
                {
                    return NativeMethodsGainHolo.AUTDGainHoloGSPAT(Ptr, pf, (float_t*)pa, size);
                }
            }
        }

        internal override GainPtr GspatWithRepeat(GainPtr ptr, uint v)
        {
            return NativeMethodsGainHolo.AUTDGainHoloGSPATWithRepeat(ptr, v);
        }

        internal override GainPtr GspatWithConstraint(GainPtr ptr, EmissionConstraintPtr v)
        {
            return NativeMethodsGainHolo.AUTDGainHoloGSPATWithConstraint(ptr, v);
        }

        internal override GainPtr Naive(float_t[] foci, Amplitude[] amps, ulong size)
        {
            unsafe
            {
                fixed (float_t* pf = &foci[0])
                fixed (Amplitude* pa = &amps[0])
                {
                    return NativeMethodsGainHolo.AUTDGainHoloNaive(Ptr, pf, (float_t*)pa, size);
                }
            }
        }

        internal override GainPtr NaiveWithConstraint(GainPtr ptr, EmissionConstraintPtr v)
        {
            return NativeMethodsGainHolo.AUTDGainHoloNaiveWithConstraint(ptr, v);
        }

        internal override GainPtr Lm(float_t[] foci, Amplitude[] amps, ulong size)
        {
            unsafe
            {
                fixed (float_t* pf = &foci[0])
                fixed (Amplitude* pa = &amps[0])
                {
                    return NativeMethodsGainHolo.AUTDGainHoloLM(Ptr, pf, (float_t*)pa, size);
                }
            }
        }

        internal override GainPtr LmWithEps1(GainPtr ptr, float_t v)
        {
            return NativeMethodsGainHolo.AUTDGainHoloLMWithEps1(ptr, v);
        }

        internal override GainPtr LmWithEps2(GainPtr ptr, float_t v)
        {
            return NativeMethodsGainHolo.AUTDGainHoloLMWithEps2(ptr, v);
        }

        internal override GainPtr LmWithTau(GainPtr ptr, float_t v)
        {
            return NativeMethodsGainHolo.AUTDGainHoloLMWithTau(ptr, v);
        }

        internal override GainPtr LmWithKMax(GainPtr ptr, uint v)
        {
            return NativeMethodsGainHolo.AUTDGainHoloLMWithKMax(ptr, v);
        }

        internal override GainPtr LmWithInitial(GainPtr ptr, float_t[] v, ulong size)
        {
            unsafe
            {
                fixed (float_t* p = &v[0])
                {
                    return NativeMethodsGainHolo.AUTDGainHoloLMWithInitial(ptr, p, size);
                }
            }
        }

        internal override GainPtr LmWithConstraint(GainPtr ptr, EmissionConstraintPtr v)
        {
            return NativeMethodsGainHolo.AUTDGainHoloLMWithConstraint(ptr, v);
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
