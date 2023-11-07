/*
 * File: RawPCM.cs
 * Project: AudioFile
 * Created Date: 13/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 07/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

#if UNITY_2018_3_OR_NEWER
#define USE_SINGLE
#endif

using System;

namespace AUTD3Sharp.Modulation.AudioFile
{
    /// <summary>
    /// Modulation constructed from raw pcm data file
    /// <remarks>The wav data is re-sampled to the sampling frequency of Modulation.</remarks>
    /// </summary>
    public sealed class RawPCM : Internal.ModulationWithFreqDiv<RawPCM>
    {
        private readonly string _filename;
        private readonly uint _sampleRate;

        /// <summary>
        /// Constructor
        /// </summary>
        /// <param name="filename">Path to raw pcm file</param>
        /// <param name="sampleRate">Sampling rate of raw pcm data</param>
        public RawPCM(string filename, uint sampleRate)
        {
            _filename = filename;
            _sampleRate = sampleRate;
        }

        internal override ModulationPtr ModulationPtr()
        {
            var err = new byte[256];
            var filenameBytes = System.Text.Encoding.ASCII.GetBytes(_filename);
            unsafe
            {
                fixed (byte* ep = err)
                fixed (byte* fp = filenameBytes)
                {
                    var ptr = NativeMethodsModulationAudioFile.AUTDModulationRawPCM(fp, _sampleRate, ep);
                    if (ptr.Item1 == IntPtr.Zero)
                        throw new AUTDException(err);
                    if (FreqDiv != null)
                        ptr = NativeMethodsModulationAudioFile.AUTDModulationRawPCMWithSamplingFrequencyDivision(ptr, FreqDiv.Value);
                    return ptr;
                }
            }
        }
    }
}
