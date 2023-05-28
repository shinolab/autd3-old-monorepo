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
using System.Text;

namespace AUTD3Sharp
{
    namespace Modulation
    {
        public sealed class Wav : Modulation
        {
            public Wav(string filename) : base(IntPtr.Zero)
            {
                var err = new byte[256];
                Ptr = NativeMethods.ModulationAudioFile.AUTDModulationWav(filename, err);
                if (Ptr == IntPtr.Zero)
                    throw new AUTDException(err);
                ;
            }
        }
    }
}
