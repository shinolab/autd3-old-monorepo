/*
 * File: Bessel.cs
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
    /// Gain to produce a Bessel beam
    /// </summary>
    public sealed class Bessel : Internal.Gain
    {
        private readonly Vector3 _point;
        private readonly Vector3 _dir;
        private readonly float_t _thetaZ;
        private float_t? _amp;

        public Bessel(Vector3 point, Vector3 dir, float_t thetaZ)
        {
            _point = point;
            _dir = dir;
            _thetaZ = thetaZ;
            _amp = null;
        }

        /// <summary>
        /// Set amplitude
        /// </summary>
        /// <param name="amp">normalized amplitude (from 0 to 1)</param>
        /// <returns></returns>
        public Bessel WithAmp(float_t amp)
        {
            _amp = amp;
            return this;
        }

        public override GainPtr GainPtr(Geometry geometry)
        {
            var ptr = Base.AUTDGainBessel(_point.x, _point.y, _point.z, _dir.x, _dir.y, _dir.z, _thetaZ);
            if (_amp != null)
                ptr = Base.AUTDGainBesselWithAmp(ptr, _amp.Value);
            return ptr;
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
