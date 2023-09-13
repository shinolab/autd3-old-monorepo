/*
 * File: TransTest.cs
 * Project: Gain
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
            internal int DevIdx;
            internal int TrIdx;
            internal float_t Phase;
            internal float_t Amp;
        }

        private readonly List<Prop> _props = new List<Prop>();

        public TransducerTest Set(int devIdx, int trIdx, float_t phase, float_t amp)
        { 
            _props.Add(new Prop { DevIdx = devIdx, TrIdx = trIdx, Phase = phase, Amp = amp });
            return this;
        }

        public override GainPtr GainPtr(Geometry geometry)
        {
            return _props.Aggregate(Base.AUTDGainTransducerTest(), (gainPtr, prop) => Base.AUTDGainTransducerTestSet(gainPtr, (uint)prop.DevIdx, (uint)prop.TrIdx, prop.Phase, prop.Amp));
        }
    }
}
