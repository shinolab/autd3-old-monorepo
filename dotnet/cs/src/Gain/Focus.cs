/*
 * File: Focus.cs
 * Project: Gain
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

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

using AUTD3Sharp.NativeMethods;

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
    /// Gain to produce single focal point
    /// </summary>
    public sealed class Focus : Internal.Gain
    {
        private readonly Vector3 _point;
        private EmitIntensity? _intensity;

        public Focus(Vector3 point)
        {
            _point = point;
            _intensity = null;
        }

        /// <summary>
        /// Set amplitude
        /// </summary>
        /// <param name="intensity">Emission intensity</param>
        /// <returns></returns>
        public Focus WithIntensity(byte intensity)
        {
            _intensity = new EmitIntensity(intensity);
            return this;
        }

        /// <summary>
        /// Set amplitude
        /// </summary>
        /// <param name="intensity">Emission intensity</param>
        /// <returns></returns>
        public Focus WithIntensity(EmitIntensity intensity)
        {
            _intensity = intensity;
            return this;
        }

        internal override GainPtr GainPtr(Geometry geometry)
        {
            var ptr = NativeMethodsBase.AUTDGainFocus(_point.x, _point.y, _point.z);
            if (_intensity != null)
                ptr = NativeMethodsBase.AUTDGainFocusWithIntensity(ptr, _intensity.Value.Value);
            return ptr;
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
