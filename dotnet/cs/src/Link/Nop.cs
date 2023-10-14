/*
 * File: Nop.cs
 * Project: Link
 * Created Date: 10/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

namespace AUTD3Sharp.Link
{
    /// <summary>
    /// Link which do nothing
    /// </summary>
    public class Nop
    {
        public sealed class NopBuilder : Internal.ILinkBuilder
        {
            public LinkBuilderPtr Ptr()
            {
                return NativeMethods.Base.AUTDLinkNop();
            }
        }

        public static NopBuilder Builder()
        {
            return new NopBuilder();
        }
    }
}
