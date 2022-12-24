/*
 * File: ModulationAudioFile.cs
 * Project: src
 * Created Date: 23/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/12/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

#if UNITY_2018_3_OR_NEWER
#define USE_SINGLE
#endif

#if USE_SINGLE
using autd3_float_t = System.Single;
#else
using autd3_float_t = System.Double;
#endif

namespace AUTD3Sharp
{
    namespace Modulation
    {
        public sealed class RawPCM : Modulation
        {
            public RawPCM(string filename, autd3_float_t samplingFreq, uint modSamplingFreqDiv)
            {
                NativeMethods.ModulationAudioFile.AUTDModulationRawPCM(out handle, filename, samplingFreq, modSamplingFreqDiv);
            }
        }
        public sealed class Wav : Modulation
        {
            public Wav(string filename, uint modSamplingFreqDiv)
            {
                NativeMethods.ModulationAudioFile.AUTDModulationWav(out handle, filename, modSamplingFreqDiv);
            }
        }
    }
}
