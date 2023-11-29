/*
 * File: Naive.cs
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
    /// Gain to produce multiple foci with naive linear synthesis
    /// </summary>
    /// <typeparam name="TB">Backend</typeparam>
    public sealed class Naive<TB> : Holo<Naive<TB>>
        where TB : Backend
    {
        private readonly TB _backend;
        private IAmplitudeConstraint? _constraint;


        public Naive(TB backend)
        {
            _backend = backend;

        }

        /// <summary>
        /// Set amplitude constraint
        /// </summary>
        /// <param name="constraint"></param>
        /// <returns></returns>
        public Naive<TB> WithConstraint(IAmplitudeConstraint constraint)
        {
            _constraint = constraint;
            return this;
        }

        internal override GainPtr GainPtr(Geometry geometry)
        {
            var ptr = _backend.Naive(Foci.ToArray(), Amps.ToArray(),
                (ulong)Amps.Count);
            if (_constraint != null) ptr = _backend.NaiveWithConstraint(ptr, _constraint.Ptr());
            return ptr;
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
