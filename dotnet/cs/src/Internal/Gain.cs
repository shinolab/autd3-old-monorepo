/*
 * File: Gain.cs
 * Project: Internal
 * Created Date: 08/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

using System.Collections.Generic;
using System.Runtime.InteropServices;
using AUTD3Sharp.NativeMethods;

namespace AUTD3Sharp.Internal
{
    [ComVisible(false)]
    public abstract class Gain : IDatagram
    {
        public DatagramPtr Ptr(IEnumerable<Device> devices) => Base.AUTDGainIntoDatagram(GainPtr(devices));

        public abstract GainPtr GainPtr(IEnumerable<Device> devices);
    }
}
