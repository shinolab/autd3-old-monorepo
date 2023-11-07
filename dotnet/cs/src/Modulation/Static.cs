/*
 * File: Static.cs
 * Project: Modulation
 * Created Date: 13/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 07/11/2023
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
        private float_t? _amp;

        /// <summary>
        /// Set amplitude
        /// </summary>
        /// <param name="amp">normalized amplitude (0.0 - 1.0)</param>
        /// <returns></returns>
        public Static WithAmp(float_t amp)
        {
            _amp = amp;
            return this;
        }

        internal override ModulationPtr ModulationPtr()
        {
            var ptr = NativeMethodsBase.AUTDModulationStatic();
            if (_amp != null)
                ptr = NativeMethodsBase.AUTDModulationStaticWithAmp(ptr, _amp.Value);
            return ptr;
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
