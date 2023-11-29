/*
 * File: Backend.cs
 * Project: Holo
 * Created Date: 13/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

#if UNITY_2018_3_OR_NEWER
#define USE_SINGLE
#endif

using System.Runtime.InteropServices;
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
    [ComVisible(false)]
    public abstract class Backend
    {
        internal BackendPtr Ptr;

        internal abstract GainPtr Sdp(float_t[] foci, Amplitude[] amps, ulong size);
        internal abstract GainPtr SdpWithAlpha(GainPtr ptr, float_t v);
        internal abstract GainPtr SdpWithRepeat(GainPtr ptr, uint v);
        internal abstract GainPtr SdpWithLambda(GainPtr ptr, float_t v);
        internal abstract GainPtr SdpWithConstraint(GainPtr ptr, EmissionConstraintPtr v);

        internal abstract GainPtr Gs(float_t[] foci, Amplitude[] amps, ulong size);
        internal abstract GainPtr GsWithRepeat(GainPtr ptr, uint v);
        internal abstract GainPtr GsWithConstraint(GainPtr ptr, EmissionConstraintPtr v);

        internal abstract GainPtr Gspat(float_t[] foci, Amplitude[] amps, ulong size);
        internal abstract GainPtr GspatWithRepeat(GainPtr ptr, uint v);
        internal abstract GainPtr GspatWithConstraint(GainPtr ptr, EmissionConstraintPtr v);

        internal abstract GainPtr Naive(float_t[] foci, Amplitude[] amps, ulong size);
        internal abstract GainPtr NaiveWithConstraint(GainPtr ptr, EmissionConstraintPtr v);

        internal abstract GainPtr Lm(float_t[] foci, Amplitude[] amps, ulong size);
        internal abstract GainPtr LmWithEps1(GainPtr ptr, float_t v);
        internal abstract GainPtr LmWithEps2(GainPtr ptr, float_t v);
        internal abstract GainPtr LmWithTau(GainPtr ptr, float_t v);
        internal abstract GainPtr LmWithKMax(GainPtr ptr, uint v);
        internal abstract GainPtr LmWithInitial(GainPtr ptr, float_t[] v, ulong size);
        internal abstract GainPtr LmWithConstraint(GainPtr ptr, EmissionConstraintPtr v);
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
