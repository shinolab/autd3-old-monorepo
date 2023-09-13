/*
 * File: Wav.cs
 * Project: AudioFile
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

using System;
using AUTD3Sharp.NativeMethods;

namespace AUTD3Sharp.Modulation.AudioFile
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
            return WithSamplingFrequencyDivision((uint)(AUTD3.FPGASubClkFreq / freq));
        }

        /// <summary>
        /// Set sampling period
        /// </summary>
        /// <returns></returns>
        public Wav WithSamplingPeriod(TimeSpan period)
        {
            return WithSamplingFrequencyDivision((uint)(Def.FpgaSubClkFreq / 1000000000.0 * (period.TotalMilliseconds * 1000.0 * 1000.0)));
        }

        public override ModulationPtr ModulationPtr()
        {
            var err = new byte[256];
            var ptr = ModulationAudioFile.AUTDModulationWav(_filename, err);
            if (ptr._0 == IntPtr.Zero)
                throw new AUTDException(err);
            if (_freqDiv != null)
                ptr = ModulationAudioFile.AUTDModulationWavWithSamplingFrequencyDivision(ptr, _freqDiv.Value);
            return ptr;
        }
    }
}
