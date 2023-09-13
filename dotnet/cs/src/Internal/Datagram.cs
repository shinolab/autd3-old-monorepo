/*
 * File: Datagram.cs
 * Project: Internal
 * Created Date: 20/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

using System;
using System.Runtime.InteropServices;

namespace AUTD3Sharp.Internal
{
    [ComVisible(false)]
    public interface ISpecialDatagram
    {
        public DatagramSpecialPtr Ptr();
    }

    [ComVisible(false)]
    public interface IDatagram
    {
        public DatagramPtr Ptr(Geometry geometry);
    }

    public class NullDatagram : IDatagram
    {
        public DatagramPtr Ptr(Geometry geometry)
        {
            return new DatagramPtr { _0 = IntPtr.Zero };
        }
    }
}
