/*
 * File: GSPAT.cs
 * Project: Holo
 * Created Date: 13/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

using AUTD3Sharp.NativeMethods;

namespace AUTD3Sharp.Gain.Holo
{
    /// <summary>
    /// Gain to produce multiple foci with GS-PAT algorithm
    /// </summary>
    /// <remarks>Diego Martinez Plasencia et al. "GS-pat: high-speed multi-point sound-fields for phased arrays of transducers," ACMTrans-actions on Graphics (TOG), 39(4):138–1, 2020.</remarks>
    /// <typeparam name="TB">Backend</typeparam>
    public sealed class GSPAT<TB> : Holo<GSPAT<TB>>
        where TB : Backend
    {
        private readonly TB _backend;
        private uint? _repeat;
        private IAmplitudeConstraint? _constraint;

        public GSPAT(TB backend)
        {
            _backend = backend;
        }

        /// <summary>
        /// Parameter. See the paper for details.
        /// </summary>
        /// <param name="value"></param>
        /// <returns></returns>
        public GSPAT<TB> WithRepeat(uint value)
        {
            _repeat = value;
            return this;
        }

        /// <summary>
        /// Set amplitude constraint
        /// </summary>
        /// <param name="constraint"></param>
        /// <returns></returns>
        public GSPAT<TB> WithConstraint(IAmplitudeConstraint constraint)
        {
            _constraint = constraint;
            return this;
        }

        internal override GainPtr GainPtr(Geometry geometry)
        {
            var ptr = _backend.Gspat(Foci.ToArray(), Amps.ToArray(),
                (ulong)Amps.Count);
            if (_repeat.HasValue) ptr = _backend.GspatWithRepeat(ptr, _repeat.Value);
            if (_constraint != null) ptr = _backend.GspatWithConstraint(ptr, _constraint.Ptr());
            return ptr;
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
