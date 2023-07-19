/*
* File: ModulationAudioFile.cs
* Project: src
* Created Date: 23/05/2021
* Author: Shun Suzuki
* -----
* Last Modified: 25/04/2023
* Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
* -----
* Copyright (c) 2021-2023 Shun Suzuki. All rights reserved.
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
    namespace Modulation
    {
        public sealed class Wav : ModulationBase
        {
            private readonly string _filename;
            private uint? _freqDiv;

            public Wav(string filename)
            {
                _filename = filename;
            }

            public Wav WithSamplingFrequencyDivision(uint div)
            {
                _freqDiv = div;
                return this;
            }

            public Wav WithSamplingFrequency(float_t freq)
            {
                return WithSamplingFrequencyDivision((uint)((float_t)NativeMethods.Def.FpgaSubClkFreq / freq));
            }

            public override ModulationPtr ModulationPtr()
            {
                var err = new byte[256];
                var ptr = NativeMethods.ModulationAudioFile.AUTDModulationWav(_filename, err);
                if (ptr._0 == IntPtr.Zero)
                    throw new AUTDException(err);
                if (_freqDiv != null)
                    ptr = NativeMethods.ModulationAudioFile.AUTDModulationWavWithSamplingFrequencyDivision(ptr, _freqDiv.Value);
                return ptr;
            }
        }
    }
}
