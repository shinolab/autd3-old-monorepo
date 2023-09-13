/*
 * File: Constraint.cs
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

#if USE_SINGLE
using float_t = System.Single;
#else
using float_t = System.Double;
#endif

namespace AUTD3Sharp.Gain.Holo
{
    /// <summary>
    /// Amplitude constraint
    /// </summary>
    public interface IAmplitudeConstraint
    {
        public ConstraintPtr Ptr();
    }

    /// <summary>
    /// Do nothing (this is equivalent to `Clamp(0, 1)`)
    /// </summary>
    public sealed class DontCare : IAmplitudeConstraint
    {
        public ConstraintPtr Ptr() => NativeMethods.GainHolo.AUTDGainHoloDotCareConstraint();
    }

    /// <summary>
    /// Normalize the value by dividing the maximum value
    /// </summary>
    public sealed class Normalize : IAmplitudeConstraint
    {
        public ConstraintPtr Ptr() => NativeMethods.GainHolo.AUTDGainHoloNormalizeConstraint();
    }

    /// <summary>
    /// Set all amplitudes to the specified value
    /// </summary>
    public sealed class Uniform : IAmplitudeConstraint
    {

        internal readonly float_t Value;

        public Uniform(float_t value = 1)
        {
            Value = value;
        }

        public ConstraintPtr Ptr() => NativeMethods.GainHolo.AUTDGainHoloUniformConstraint(Value);
    }

    /// <summary>
    /// Clamp all amplitudes to the specified range
    /// </summary>
    public sealed class Clamp : IAmplitudeConstraint
    {
        internal readonly float_t Min;
        internal readonly float_t Max;

        public Clamp(float_t min = 0, float_t max = 1)
        {
            Min = min;
            Max = max;
        }

        public ConstraintPtr Ptr() => NativeMethods.GainHolo.AUTDGainHoloClampConstraint(Min, Max);
    }
}
