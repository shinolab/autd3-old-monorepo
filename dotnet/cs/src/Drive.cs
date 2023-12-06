/*
 * File: Drive.cs
 * Project: src
 * Created Date: 24/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

using System.Runtime.InteropServices;

namespace AUTD3Sharp
{
    [StructLayout(LayoutKind.Sequential)]
    public struct Drive
    {
        public Phase Phase { get; set; }
        public EmitIntensity Intensity { get; set; }

        public Drive(Phase phase, EmitIntensity intensity)
        {
            Phase = phase;
            Intensity = intensity;
        }

        public Drive(Phase phase, byte intensity)
        {
            Phase = phase;
            Intensity = new EmitIntensity(intensity);
        }
    }
}