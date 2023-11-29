/*
 * File: GS.cs
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
    /// Gain to produce multiple foci with GS algorithm
    /// </summary>
    /// <remarks>Asier Marzo and Bruce W Drinkwater. Holographic acoustic tweezers.Proceedings of theNational Academy of Sciences, 116(1):84–89, 2019.</remarks>
    /// <typeparam name="TB">Backend</typeparam>
    public sealed class GS<TB> : Holo<GS<TB>>
        where TB : Backend
    {
        private readonly TB _backend;
        private uint? _repeat;
        private IAmplitudeConstraint? _constraint;

        public GS(TB backend)
        {
            _backend = backend;
        }

        /// <summary>
        /// Parameter. See the paper for details.
        /// </summary>
        /// <param name="value"></param>
        /// <returns></returns>
        public GS<TB> WithRepeat(uint value)
        {
            _repeat = value;
            return this;
        }

        /// <summary>
        /// Set amplitude constraint
        /// </summary>
        /// <param name="constraint"></param>
        /// <returns></returns>
        public GS<TB> WithConstraint(IAmplitudeConstraint constraint)
        {
            _constraint = constraint;
            return this;
        }

        internal override GainPtr GainPtr(Geometry geometry)
        {
            var ptr = _backend.Gs(Foci.ToArray(), Amps.ToArray(),
                (ulong)Amps.Count);
            if (_repeat.HasValue) ptr = _backend.GsWithRepeat(ptr, _repeat.Value);
            if (_constraint != null) ptr = _backend.GsWithConstraint(ptr, _constraint.Ptr());
            return ptr;
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
