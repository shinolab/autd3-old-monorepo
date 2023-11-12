/*
 * File: EmitIntensity.cs
 * Project: src
 * Created Date: 12/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */


#if UNITY_2018_3_OR_NEWER
#define USE_SINGLE
#endif

using System.Collections.Generic;
using System.Linq;

#if UNITY_2018_3_OR_NEWER
using UnityEngine;
using Vector3 = UnityEngine.Vector3;
using Quaternion = UnityEngine.Quaternion;
#else
using Vector3 = AUTD3Sharp.Utils.Vector3d;
using Quaternion = AUTD3Sharp.Utils.Quaterniond;
#endif

#if USE_SINGLE
using float_t = System.Single;
#else
using float_t = System.Double;
#endif

namespace AUTD3Sharp
{
    public sealed class EmitIntensity
    {
        public ushort PulseWidth { get; }

        private EmitIntensity(ushort pulseWidth)
        {
            PulseWidth = pulseWidth;
        }

        public float_t Normalized => NativeMethodsDef.AUTDEmitIntensityNormalizedFrom(PulseWidth);
        public float_t DutyRatio => NativeMethodsDef.AUTDEmitIntensityDutyRatioFrom(PulseWidth);

        public static EmitIntensity NewNormalized(float_t value) => new EmitIntensity((ushort)NativeMethodsDef.AUTDEmitIntensityNormalizedInto(value).Validate());

        public static EmitIntensity NewNormalizedCorrectedWithAlpha(float_t value, float_t alpha) => new EmitIntensity((ushort)NativeMethodsDef.AUTDEmitIntensityNormalizedCorrectedInto(value, alpha).Validate());

        public static EmitIntensity NewNormalizedCorrected(float_t value) => EmitIntensity.NewNormalizedCorrectedWithAlpha(value, NativeMethodsDef.DEFAULT_CORRECTED_ALPHA);

        public static EmitIntensity NewDutyRatio(float_t value) => new EmitIntensity((ushort)NativeMethodsDef.AUTDEmitIntensityDutyRatioInto(value).Validate());

        public static EmitIntensity NewPulseWidth(ushort value) => new EmitIntensity((ushort)NativeMethodsDef.AUTDEmitIntensityPulseWidthInto(value).Validate());
    }
}