/*
 * File: Plane.cs
 * Project: Gain
 * Created Date: 13/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 13/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

#if UNITY_2018_3_OR_NEWER
#define USE_SINGLE
#endif

using AUTD3Sharp.NativeMethods;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

#if UNITY_2018_3_OR_NEWER
using UnityEngine;
using Vector3 = UnityEngine.Vector3;
#else
using Vector3 = AUTD3Sharp.Utils.Vector3d;
#endif

#if USE_SINGLE
using float_t = System.Single;
#else
using float_t = System.Double;
#endif

namespace AUTD3Sharp.Gain
{
    /// <summary>
    /// Gain to produce a plane wave
    /// </summary>
    public sealed class Plane : Internal.Gain
    {
        private readonly Vector3 _dir;
        private float_t? _amp;

        public Plane(Vector3 dir)
        {
            _dir = dir;
            _amp = null;
        }

        /// <summary>
        /// Set amplitude
        /// </summary>
        /// <param name="amp">normalized amplitude (from 0 to 1)</param>
        /// <returns></returns>
        public Plane WithAmp(float_t amp)
        {
            _amp = amp;
            return this;
        }

        public override GainPtr GainPtr(Geometry geometry)
        {
            var ptr = Base.AUTDGainPlane(_dir.x, _dir.y, _dir.z);
            if (_amp != null)
                ptr = Base.AUTDGainPlaneWithAmp(ptr, _amp.Value);
            return ptr;
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
