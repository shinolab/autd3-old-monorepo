/*
 * File: SamplingConfiguration.cs
 * Project: src
 * Created Date: 24/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/11/2023
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

using System;

namespace AUTD3Sharp
{
    public readonly struct SamplingConfiguration
    {
        internal SamplingConfigurationRaw Internal { get; }

        internal SamplingConfiguration(SamplingConfigurationRaw @internal)
        {
            Internal = @internal;
        }

        public static SamplingConfiguration NewWithFrequencyDivision(uint div)
        {
            return new SamplingConfiguration(NativeMethodsDef.AUTDSamplingConfigNewWithFrequencyDivision(div).Validate());
        }

        public static SamplingConfiguration NewWithFrequency(float_t f)
        {
            return new SamplingConfiguration(NativeMethodsDef.AUTDSamplingConfigNewWithFrequency(f).Validate());
        }

        public static SamplingConfiguration NewWithPeriod(TimeSpan p)
        {
            return new SamplingConfiguration(NativeMethodsDef.AUTDSamplingConfigNewWithPeriod((ulong)(p.TotalSeconds * 1000 *
                1000 * 1000)).Validate());
        }

        public uint FrequencyDivision => NativeMethodsDef.AUTDSamplingConfigFrequencyDivision(Internal);

        public float_t Frequency => NativeMethodsDef.AUTDSamplingConfigFrequency(Internal);

        public TimeSpan Period => TimeSpan.FromSeconds(NativeMethodsDef.AUTDSamplingConfigPeriod(Internal) / 1000.0 / 1000.0 / 1000.0);
    }
}