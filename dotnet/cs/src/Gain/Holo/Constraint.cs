/*
 * File: Constraint.cs
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

using AUTD3Sharp.NativeMethods;

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
        internal EmissionConstraintPtr Ptr();
    }

    /// <summary>
    /// Do nothing (this is equivalent to `Clamp(0, 1)`)
    /// </summary>
    public sealed class DontCare : IAmplitudeConstraint
    {
        EmissionConstraintPtr IAmplitudeConstraint.Ptr() => NativeMethodsGainHolo.AUTDGainHoloConstraintDotCare();
    }

    /// <summary>
    /// Normalize the value by dividing the maximum value
    /// </summary>
    public sealed class Normalize : IAmplitudeConstraint
    {
        EmissionConstraintPtr IAmplitudeConstraint.Ptr() => NativeMethodsGainHolo.AUTDGainHoloConstraintNormalize();
    }

    /// <summary>
    /// Set all amplitudes to the specified value
    /// </summary>
    public sealed class Uniform : IAmplitudeConstraint
    {

        internal readonly EmitIntensity Value;

        public Uniform(byte value)
        {
            Value = new EmitIntensity(value);
        }
        public Uniform(EmitIntensity value)
        {
            Value = value;
        }

        EmissionConstraintPtr IAmplitudeConstraint.Ptr() => NativeMethodsGainHolo.AUTDGainHoloConstraintUniform(Value.Value);
    }

    /// <summary>
    /// Clamp all amplitudes to the specified range
    /// </summary>
    public sealed class Clamp : IAmplitudeConstraint
    {
        internal readonly EmitIntensity Min;
        internal readonly EmitIntensity Max;

        public Clamp(EmitIntensity min, EmitIntensity max)
        {
            Min = min;
            Max = max;
        }

        EmissionConstraintPtr IAmplitudeConstraint.Ptr() => NativeMethodsGainHolo.AUTDGainHoloConstraintClamp(Min.Value, Max.Value);
    }
}
