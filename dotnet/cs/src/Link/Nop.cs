/*
 * File: Nop.cs
 * Project: Link
 * Created Date: 10/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

using AUTD3Sharp.Internal;
using AUTD3Sharp.NativeMethods;

namespace AUTD3Sharp.Link
{
    /// <summary>
    /// Link which do nothing
    /// </summary>
    public class Nop
    {
        public sealed class NopBuilder : ILinkBuilder<Nop>
        {
            LinkBuilderPtr ILinkBuilder<Nop>.Ptr()
            {
                return NativeMethodsBase.AUTDLinkNop();
            }

            Nop ILinkBuilder<Nop>.ResolveLink(LinkPtr ptr)
            {
                return new Nop();
            }
        }

        public static NopBuilder Builder()
        {
            return new NopBuilder();
        }
    }
}
