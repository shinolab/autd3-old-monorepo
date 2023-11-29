/*
 * File: Gain.cs
 * Project: Internal
 * Created Date: 08/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

using System.Runtime.InteropServices;
using AUTD3Sharp.NativeMethods;

namespace AUTD3Sharp.Internal
{
    [ComVisible(false)]
    public abstract class Gain : IDatagram
    {
        DatagramPtr IDatagram.Ptr(Geometry geometry) => NativeMethodsBase.AUTDGainIntoDatagram(GainPtr(geometry));

        internal abstract GainPtr GainPtr(Geometry geometry);
    }
}
