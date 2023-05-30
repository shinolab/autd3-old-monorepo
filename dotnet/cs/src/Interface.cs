/*
 * File: Interface.cs
 * Project: src
 * Created Date: 17/12/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 30/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
* Copyright (c) 2021-2023 Shun Suzuki. All rights reserved.
 * 
 */

using System;
using System.Runtime.InteropServices;

namespace AUTD3Sharp
{
    [ComVisible(false)]
    public abstract class SpecialData
    {
        internal IntPtr Ptr { get; set; }

        internal SpecialData(IntPtr ptr)
        {
            Ptr = ptr;
        }

        ~SpecialData()
        {
            NativeMethods.Base.AUTDDeleteSpecialData(Ptr);
        }
    }

    [ComVisible(false)]
    public abstract class Header
    {
        internal IntPtr Ptr { get; set; }

        internal Header(IntPtr ptr)
        {
            Ptr = ptr;
        }
    }

    [ComVisible(false)]
    public abstract class Body
    {
        internal IntPtr Ptr { get; set; }

        internal Body(IntPtr ptr)
        {
            Ptr = ptr;
        }
    }
}
