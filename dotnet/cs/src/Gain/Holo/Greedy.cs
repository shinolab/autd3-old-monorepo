/*
 * File: Greedy.cs
 * Project: Holo
 * Created Date: 13/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2023
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
    /// Gain to produce multiple foci with greedy algorithm
    /// </summary>
    /// <remarks>
    /// Shun Suzuki, Masahiro Fujiwara, Yasutoshi Makino, and Hiroyuki Shinoda, “Radiation Pressure Field Reconstruction for Ultrasound Midair Haptics by Greedy Algorithm with Brute-Force Search,” in IEEE Transactions on Haptics, doi: 10.1109/TOH.2021.3076489
    /// </remarks>
    public sealed class Greedy : Holo<Greedy>
    {
        private uint? _phaseDiv;
        private IAmplitudeConstraint? _constraint;

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

        internal override GainPtr GainPtr(Geometry geometry)
        {
            unsafe
            {
                fixed (float_t* foci = &Foci.ToArray()[0])
                fixed (Amplitude* amps = &Amps.ToArray()[0])
                {
                    var ptr = NativeMethodsGainHolo.AUTDGainHoloGreedy(foci, (float_t*)amps, (ulong)Amps.Count);
                    if (_phaseDiv.HasValue)
                        ptr = NativeMethodsGainHolo.AUTDGainHoloGreedyWithPhaseDiv(ptr, _phaseDiv.Value);
                    if (_constraint != null)
                        ptr = NativeMethodsGainHolo.AUTDGainHoloGreedyWithConstraint(ptr, _constraint.Ptr());
                    return ptr;
                }
            }
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
