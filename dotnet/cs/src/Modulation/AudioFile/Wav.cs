/*
 * File: Wav.cs
 * Project: AudioFile
 * Created Date: 13/09/2023
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

using System;

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

        internal override ModulationPtr ModulationPtr()
        {
            var filenameBytes = System.Text.Encoding.UTF8.GetBytes(_filename);
            unsafe
            {
                fixed (byte* fp = filenameBytes)
                {
                    var result = NativeMethodsModulationAudioFile.AUTDModulationWav(fp);
                    if (result.result.Item1 == IntPtr.Zero)
                    {
                        var err = new byte[result.err_len];
                        fixed (byte* p = err)
                            NativeMethodsDef.AUTDGetErr(result.err, p);
                        throw new AUTDException(err);
                    }
                    var ptr = result.result;
                    if (Config != null)
                        ptr = NativeMethodsModulationAudioFile.AUTDModulationRawPCMWithSamplingConfig(ptr, Config.Value.Internal);
                    return ptr;
                }
            }
        }
    }
}
