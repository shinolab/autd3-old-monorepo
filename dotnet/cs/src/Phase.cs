/*
 * File: Phase.cs
 * Project: src
 * Created Date: 02/12/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

#if UNITY_2018_3_OR_NEWER
#define USE_SINGLE
#endif

using AUTD3Sharp.NativeMethods;

#if USE_SINGLE
using float_t = System.Single;
#else
using float_t = System.Double;
#endif

using System.Runtime.InteropServices;

namespace AUTD3Sharp
{
    [StructLayout(LayoutKind.Sequential)]
    public readonly struct Phase
    {
        public byte Value { get; }

        public Phase(byte value)
        {
            Value = value;
        }

        public float_t Radian => NativeMethodsDef.AUTDPhaseToRad(Value);

        public static Phase FromRad(float_t value) => new Phase(NativeMethodsDef.AUTDPhaseFromRad(value));
    }
}