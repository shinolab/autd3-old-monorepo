/*
 * File: Transform.cs
 * Project: Modulation
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

using System;
using System.Runtime.InteropServices;
using AUTD3Sharp.NativeMethods;


#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

namespace AUTD3Sharp.Modulation
{
    public class Transform : Internal.Modulation
    {
        [UnmanagedFunctionPointer(CallingConvention.Cdecl)] public delegate byte ModTransformDelegate(IntPtr context, uint i, byte d);

        private readonly Internal.Modulation _m;
        private readonly ModTransformDelegate _f;

        public Transform(Internal.Modulation m, Func<int, EmitIntensity, EmitIntensity> f)
        {
            _m = m;
            _f = (context, i, d) => f((int)i, new EmitIntensity(d)).Value;
        }

        internal override ModulationPtr ModulationPtr()
        {
            return NativeMethodsBase.AUTDModulationWithTransform(_m.ModulationPtr(), Marshal.GetFunctionPointerForDelegate(_f), IntPtr.Zero);
        }
    }

    public static class TransformModulationExtensions
    {
        public static Transform WithTransform(this Internal.Modulation s, Func<int, EmitIntensity, EmitIntensity> f)
        {
            return new Transform(s, f);
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
