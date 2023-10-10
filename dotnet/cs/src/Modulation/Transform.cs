/*
 * File: Transform.cs
 * Project: Modulation
 * Created Date: 13/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/10/2023
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

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

#if UNITY_2018_3_OR_NEWER
using Math = UnityEngine.Mathf;
#endif

#if USE_SINGLE
using float_t = System.Single;
#else
using float_t = System.Double;
#endif

namespace AUTD3Sharp.Modulation
{
    using Base = NativeMethods.Base;

    public class Transform : Internal.Modulation
    {
        [UnmanagedFunctionPointer(CallingConvention.Cdecl)] public delegate float_t ModTransformDelegate(IntPtr context, uint i, float_t d);

        private readonly Internal.Modulation _m;
        private readonly ModTransformDelegate _f;

        public Transform(Internal.Modulation m, Func<int, float_t, float_t> f)
        {
            _m = m;
            _f = (context, i, d) => f((int)i, d);
        }

        public override ModulationPtr ModulationPtr()
        {
            return Base.AUTDModulationWithTransform(_m.ModulationPtr(), Marshal.GetFunctionPointerForDelegate(_f), IntPtr.Zero);
        }
    }

    public static class TransformModulationExtensions
    {
        public static Transform WithTransform(this Internal.Modulation s, Func<int, float_t, float_t> f)
        {
            return new Transform(s, f);
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
