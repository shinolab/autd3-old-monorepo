/*
 * File: TransTest.cs
 * Project: Gain
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

using System.Collections.Generic;
using System.Linq;
using AUTD3Sharp.NativeMethods;

#if USE_SINGLE
using float_t = System.Single;
#else
using float_t = System.Double;
#endif

namespace AUTD3Sharp.Gain
{
    /// <summary>
    /// Gain to set amp and phase uniformly
    /// </summary>
    public sealed class TransducerTest : Internal.Gain
    {
        internal struct Prop
        {
            internal Transducer Tr;
            internal float_t Phase;
            internal EmitIntensity Intensity;
        }

        private readonly List<Prop> _props = new List<Prop>();

        public TransducerTest Set(Transducer tr, float_t phase, byte intensity)
        {
            _props.Add(new Prop { Tr = tr, Phase = phase, Intensity = new EmitIntensity(intensity) });
            return this;
        }

        public TransducerTest Set(Transducer tr, float_t phase, EmitIntensity intensity)
        {
            _props.Add(new Prop { Tr = tr, Phase = phase, Intensity = intensity });
            return this;
        }

        internal override GainPtr GainPtr(Geometry geometry)
        {
            return _props.Aggregate(NativeMethodsBase.AUTDGainTransducerTest(), (gainPtr, prop) => NativeMethodsBase.AUTDGainTransducerTestSet(gainPtr, prop.Tr.Ptr, prop.Phase, prop.Intensity.Value));
        }
    }
}
