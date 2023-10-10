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

using System;
using AUTD3Sharp.NativeMethods;

namespace AUTD3Sharp.Modulation.AudioFile
{
    /// <summary>
    /// Modulation constructed from wav file
    /// <remarks>The wav data is re-sampled to the sampling frequency of Modulation.</remarks>
    /// </summary>
    public sealed class Wav : Internal.ModulationWithFreqDiv<Wav>
    {
        private readonly string _filename;

        /// <summary>
        /// Constructor
        /// </summary>
        /// <param name="filename">Path to wav file</param>
        public Wav(string filename)
        {
            _filename = filename;
        }

        public override ModulationPtr ModulationPtr()
        {
            var err = new byte[256];
            var ptr = ModulationAudioFile.AUTDModulationWav(_filename, err);
            if (ptr._0 == IntPtr.Zero)
                throw new AUTDException(err);
            if (FreqDiv != null)
                ptr = ModulationAudioFile.AUTDModulationWavWithSamplingFrequencyDivision(ptr, FreqDiv.Value);
            return ptr;
        }
    }
}
