/*
 * File: Amplitude.cs
 * Project: Holo
 * Created Date: 24/11/2023
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
    [StructLayout(LayoutKind.Sequential)]
    public readonly struct Amplitude
    {
        internal Amplitude(float_t value)
        {
            Pascal = value;
        }

        public float_t Pascal { get; }

        public float_t SPL => NativeMethodsGainHolo.AUTDGainHoloPascalToSPL(Pascal);

        public static Amplitude NewPascal(float_t pascal) => new Amplitude(pascal);
        public static Amplitude NewSPL(float_t spl) => new Amplitude(NativeMethodsGainHolo.AUTDGainHoloSPLToPascal(spl));

        public class UnitPascal
        {
            internal UnitPascal() { }
            public static Amplitude operator *(float_t a, UnitPascal _) => NewPascal(a);
        }
        public class UnitSPL
        {
            internal UnitSPL() { }
            public static Amplitude operator *(float_t a, UnitSPL _) => NewSPL(a);
        }

        public static class Units
        {
            public static UnitPascal Pascal { get; } = new UnitPascal();
            public static UnitSPL dB { get; } = new UnitSPL();
        }
    }
}
