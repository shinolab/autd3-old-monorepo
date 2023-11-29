/*
 * File: Static.cs
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

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

using AUTD3Sharp.NativeMethods;

#if USE_SINGLE
using float_t = System.Single;
#else
using float_t = System.Double;
#endif

namespace AUTD3Sharp.Modulation
{
    /// <summary>
    /// Without modulation
    /// </summary>
    public sealed class Static : Internal.Modulation
    {
        private EmitIntensity? _intensity;

        /// <summary>
        /// Set amplitude
        /// </summary>
        /// <param name="intensity">Emission intensity</param>
        /// <returns></returns>
        public Static WithIntensity(byte intensity)
        {
            _intensity = new EmitIntensity(intensity);
            return this;
        }

        /// <summary>
        /// Set amplitude
        /// </summary>
        /// <param name="intensity">Emission intensity</param>
        /// <returns></returns>
        public Static WithIntensity(EmitIntensity intensity)
        {
            _intensity = intensity;
            return this;
        }

        internal override ModulationPtr ModulationPtr()
        {
            var ptr = NativeMethodsBase.AUTDModulationStatic();
            if (_intensity != null)
                ptr = NativeMethodsBase.AUTDModulationStaticWithIntensity(ptr, _intensity.Value.Value);
            return ptr;
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
