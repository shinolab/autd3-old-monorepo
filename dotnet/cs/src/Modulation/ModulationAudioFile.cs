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

namespace AUTD3Sharp.Modulation
{
    /// <summary>
    /// Modulation constructed from wav file
    /// <remarks>The wav data is re-sampled to the sampling frequency of Modulation.</remarks>
    /// </summary>
    public sealed class Wav : Internal.Modulation
    {
        private readonly string _filename;
        private uint? _freqDiv;

        /// <summary>
        /// Constructor
        /// </summary>
        /// <param name="filename">Path to wav file</param>
        public Wav(string filename)
        {
            _filename = filename;
        }

        /// <summary>
        /// Set sampling frequency division
        /// </summary>
        /// <param name="div">The sampling frequency is <see cref="AUTD3.FPGASubClkFreq">AUTD3.FPGASubClkFreq</see> / div.</param>
        /// <returns></returns>
        public Wav WithSamplingFrequencyDivision(uint div)
        {
            _freqDiv = div;
            return this;
        }

        /// <summary>
        /// Set sampling frequency
        /// </summary>
        /// <returns></returns>
        public Wav WithSamplingFrequency(float_t freq)
        {
            return WithSamplingFrequencyDivision((uint)((float_t)AUTD3.FPGASubClkFreq / freq));
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
