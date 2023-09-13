/*
 * File: EVP.cs
 * Project: Holo
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

using System.Collections.Generic;
using System.Linq;

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

namespace AUTD3Sharp.Gain.Holo
{
    /// <summary>
    /// Gain to produce multiple foci by solving Eigen Value Problem
    /// </summary>
    /// <remarks>Long, Benjamin, et al. "Rendering volumetric haptic shapes in mid-air using ultrasound." ACM Transactions on Graphics (TOG) 33.6 (2014): 1-10.</remarks>
    /// <typeparam name="TB">Backend</typeparam>
    public sealed class EVP<TB> : Internal.Gain
        where TB : Backend
    {
        private readonly TB _backend;
        private readonly List<float_t> _foci;
        private readonly List<float_t> _amps;
        private float_t? _gamma;
        private IAmplitudeConstraint? _constraint;

        public EVP(TB backend)
        {
            _backend = backend;
            _foci = new List<float_t>();
            _amps = new List<float_t>();
        }

        public EVP<TB> AddFocus(Vector3 focus, float_t amp)
        {
            _foci.Add(focus.x);
            _foci.Add(focus.y);
            _foci.Add(focus.z);
            _amps.Add(amp);
            return this;
        }

        /// <summary>
        /// Add foci
        /// </summary>
        /// <param name="iter">Enumerable of foci and amps</param>
        public EVP<TB> AddFociFromIter(IEnumerable<(Vector3, float_t)> iter)
        {
            return iter.Aggregate(this, (holo, point) => holo.AddFocus(point.Item1, point.Item2));
        }

        /// <summary>
        /// Parameter. See the paper for details.
        /// </summary>
        /// <param name="value"></param>
        /// <returns></returns>
        public EVP<TB> WithGamma(float_t value)
        {
            _gamma = value;
            return this;
        }

        /// <summary>
        /// Set amplitude constraint
        /// </summary>
        /// <param name="constraint"></param>
        /// <returns></returns>
        public EVP<TB> WithConstraint(IAmplitudeConstraint constraint)
        {
            _constraint = constraint;
            return this;
        }

        public override GainPtr GainPtr(Geometry geometry)
        {
            var ptr = _backend.Evp(_foci.ToArray(), _amps.ToArray(),
                (ulong)_amps.Count);
            if (_gamma.HasValue) ptr = _backend.EvpWithGamma(ptr, _gamma.Value);
            if (_constraint != null) ptr = _backend.EvpWithConstraint(ptr, _constraint.Ptr());
            return ptr;
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
