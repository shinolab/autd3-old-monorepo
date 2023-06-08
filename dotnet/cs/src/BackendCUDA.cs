/*
 * File: BackendCUDA.cs
 * Project: src
 * Created Date: 08/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

using System;
using System.Runtime.InteropServices;

namespace AUTD3Sharp
{
    namespace Gain
    {
        namespace Holo
        {
            [ComVisible(false)]
            public class BackendCUDA : Backend
            {
                public BackendCUDA()
                {
                    var err = new byte[256];
                    Ptr = NativeMethods.BackendCUDA.AUTDCUDABackend(err);
                    if (Ptr._0 == IntPtr.Zero)
                        throw new AUTDException(err);
                }
            }
        }
    }
}