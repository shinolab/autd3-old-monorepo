/*
 * File: Greedy.cs
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
    /// Gain to produce multiple foci with greedy algorithm
    /// </summary>
    /// <remarks>
    /// Shun Suzuki, Masahiro Fujiwara, Yasutoshi Makino, and Hiroyuki Shinoda, “Radiation Pressure Field Reconstruction for Ultrasound Midair Haptics by Greedy Algorithm with Brute-Force Search,” in IEEE Transactions on Haptics, doi: 10.1109/TOH.2021.3076489
    /// </remarks>
    public sealed class Greedy : Internal.Gain
    {
        private readonly List<float_t> _foci = new List<float_t>();
        private readonly List<float_t> _amps = new List<float_t>();
        private uint? _phaseDiv;
        private IAmplitudeConstraint? _constraint;

        public Greedy AddFocus(Vector3 focus, float_t amp)
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
        public Greedy AddFociFromIter(IEnumerable<(Vector3, float_t)> iter)
        {
            return iter.Aggregate(this, (holo, point) => holo.AddFocus(point.Item1, point.Item2));
        }

        /// <summary>
        /// Parameter. See the paper for details.
        /// </summary>
        /// <param name="value"></param>
        /// <returns></returns>
        public Greedy WithPhaseDiv(uint value)
        {
            _phaseDiv = value;
            return this;
        }

        /// <summary>
        /// Set amplitude constraint
        /// </summary>
        /// <param name="constraint"></param>
        /// <returns></returns>
        public Greedy WithConstraint(IAmplitudeConstraint constraint)
        {
            _constraint = constraint;
            return this;
        }

        public override GainPtr GainPtr(Geometry geometry)
        {
            var ptr = NativeMethods.GainHolo.AUTDGainHoloGreedy(_foci.ToArray(), _amps.ToArray(),
                (ulong)_amps.Count);
            if (_phaseDiv.HasValue)
                ptr = NativeMethods.GainHolo.AUTDGainHoloGreedyWithPhaseDiv(ptr, _phaseDiv.Value);
            if (_constraint != null) ptr = NativeMethods.GainHolo.AUTDGainHoloGreedyWithConstraint(ptr, _constraint.Ptr());
            return ptr;
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
