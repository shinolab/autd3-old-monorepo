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

using System;

namespace AUTD3Sharp
{
    namespace Modulation
    {
        public sealed class Wav : ModulationBase
        {
            private readonly string _filename;

            public Wav(string filename)
            {
                _filename = filename;
            }

            public override ModulationPtr ModulationPtr()
            {
                var err = new byte[256];
                var ptr = NativeMethods.ModulationAudioFile.AUTDModulationWav(_filename, err);
                if (ptr._0 == IntPtr.Zero)
                    throw new AUTDException(err);
                return ptr;
            }
        }
    }
}
