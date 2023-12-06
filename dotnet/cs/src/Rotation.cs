/*
 * File: Rotation.cs
 * Project: src
 * Created Date: 26/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

#if UNITY_2018_3_OR_NEWER
#define USE_SINGLE
#endif

using AUTD3Sharp.NativeMethods;

#if UNITY_2018_3_OR_NEWER
using UnityEngine;
using Quaternion = UnityEngine.Quaternion;
#else
using Quaternion = AUTD3Sharp.Utils.Quaterniond;
#endif

#if USE_SINGLE
using float_t = System.Single;
#else
using float_t = System.Double;
#endif

namespace AUTD3Sharp
{
    public struct Angle
    {
        public static Angle FromDegree(float_t degree) => new Angle(degree / 180 * AUTD3.Pi);
        public static Angle FromRadian(float_t radian) => new Angle(radian);

        public float_t Radian { get; }

        public class UnitRadian
        {
            internal UnitRadian() { }
            public static Angle operator *(float_t a, UnitRadian b) => FromRadian(a);
        }
        public class UnitDegree
        {
            internal UnitDegree() { }
            public static Angle operator *(float_t a, UnitDegree b) => FromDegree(a);
        }

        public static class Units
        {
            public static UnitRadian Rad { get; } = new UnitRadian();
            public static UnitDegree Deg { get; } = new UnitDegree();
        }

        private Angle(float_t value)
        {
            Radian = value;
        }
    }

    public static class EulerAngles
    {
        public static Quaternion FromZYZ(Angle z1, Angle y, Angle z2)
        {
            unsafe
            {
                float_t* rot = stackalloc float_t[4];
                NativeMethodsBase.AUTDRotationFromEulerZYZ(z1.Radian, y.Radian, z2.Radian, rot);
                return new Quaternion(rot[1], rot[2], rot[3], rot[0]);
            }
        }
    }
}