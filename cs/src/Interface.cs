/*
 * File: Interface.cs
 * Project: src
 * Created Date: 17/12/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/06/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 * 
 */

using System;
using System.Runtime.InteropServices;
using Microsoft.Win32.SafeHandles;

namespace AUTD3Sharp
{
    [ComVisible(false)]
    public abstract class Header : SafeHandleZeroOrMinusOneIsInvalid
    {
        internal IntPtr Ptr => handle;

        internal Header() : base(true)
        {
            var ptr = new IntPtr();
            SetHandle(ptr);
        }
    }

    [ComVisible(false)]
    public abstract class Body : SafeHandleZeroOrMinusOneIsInvalid
    {
        internal IntPtr Ptr => handle;

        internal Body() : base(true)
        {
            var ptr = new IntPtr();
            SetHandle(ptr);
        }
    }
}
