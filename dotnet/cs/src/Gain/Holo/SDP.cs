/*
 * File: SDP.cs
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

namespace AUTD3Sharp.Gain.Holo
{
    /// <summary>
    /// Gain to produce multiple foci by solving Semi-Definite Programming
    /// </summary>
    /// <remarks>Inoue, Seki, Yasutoshi Makino, and Hiroyuki Shinoda. "Active touch perception produced by airborne ultrasonic haptic hologram." 2015 IEEE World Haptics Conference (WHC). IEEE, 2015.</remarks>
    /// <typeparam name="TB">Backend</typeparam>
    public sealed class SDP<TB> : Holo<SDP<TB>>
        where TB : Backend
    {
        private readonly TB _backend;
        private float_t? _alpha;
        private float_t? _lambda;
        private uint? _repeat;
        private IAmplitudeConstraint? _constraint;

        public SDP(TB backend)
        {
            _backend = backend;
        }

        /// <summary>
        /// Parameter. See the paper for details.
        /// </summary>
        /// <param name="value"></param>
        /// <returns></returns>
        public SDP<TB> WithAlpha(float_t value)
        {
            _alpha = value;
            return this;
        }

        /// <summary>
        /// Parameter. See the paper for details.
        /// </summary>
        /// <param name="value"></param>
        /// <returns></returns>
        public SDP<TB> WithLambda(float_t value)
        {
            _lambda = value;
            return this;
        }

        /// <summary>
        /// Parameter. See the paper for details.
        /// </summary>
        /// <param name="value"></param>
        /// <returns></returns>
        public SDP<TB> WithRepeat(uint value)
        {
            _repeat = value;
            return this;
        }

        /// <summary>
        /// Set amplitude constraint
        /// </summary>
        /// <param name="constraint"></param>
        /// <returns></returns>
        public SDP<TB> WithConstraint(IAmplitudeConstraint constraint)
        {
            _constraint = constraint;
            return this;
        }

        internal override GainPtr GainPtr(Geometry geometry)
        {
            var ptr = _backend.Sdp(Foci.ToArray(), Amps.ToArray(),
                (ulong)Amps.Count);
            if (_alpha.HasValue) ptr = _backend.SdpWithAlpha(ptr, _alpha.Value);
            if (_lambda.HasValue) ptr = _backend.SdpWithLambda(ptr, _lambda.Value);
            if (_repeat.HasValue) ptr = _backend.SdpWithRepeat(ptr, _repeat.Value);
            if (_constraint != null) ptr = _backend.SdpWithConstraint(ptr, _constraint.Ptr());
            return ptr;
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
