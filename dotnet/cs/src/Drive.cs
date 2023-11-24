/*
 * File: Drive.cs
 * Project: src
 * Created Date: 24/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#if UNITY_2018_3_OR_NEWER
#define USE_SINGLE
#endif

#if USE_SINGLE
using float_t = System.Single;
#else
using float_t = System.Double;
#endif

using System.Runtime.InteropServices;

namespace AUTD3Sharp
{
    [StructLayout(LayoutKind.Sequential)]
    public struct Drive
    {
        public float_t Phase { get; set; }
        public EmitIntensity Intensity { get; set; }

        public Drive(float_t phase, EmitIntensity intensity)
        {
            Phase = phase;
            Intensity = intensity;
        }
    }
}