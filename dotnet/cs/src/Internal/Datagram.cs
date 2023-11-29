/*
 * File: Datagram.cs
 * Project: Internal
 * Created Date: 20/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

using System;
using System.Runtime.InteropServices;
using AUTD3Sharp.NativeMethods;

namespace AUTD3Sharp.Internal
{
    [ComVisible(false)]
    public interface ISpecialDatagram
    {
        internal DatagramSpecialPtr Ptr();
    }

    [ComVisible(false)]
    public interface IDatagram
    {
        internal DatagramPtr Ptr(Geometry geometry);
    }

    internal class NullDatagram : IDatagram
    {
        DatagramPtr IDatagram.Ptr(Geometry geometry)
        {
            return new DatagramPtr { Item1 = IntPtr.Zero };
        }
    }
}
