/*
 * File: Link.cs
 * Project: src
 * Created Date: 28/04/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021-2023 Shun Suzuki. All rights reserved.
 *
 */

using System.Runtime.InteropServices;
using AUTD3Sharp.NativeMethods;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

namespace AUTD3Sharp.Internal
{
    [ComVisible(false)]
    public interface ILinkBuilder<out T>
    {
        internal LinkBuilderPtr Ptr();
        internal T ResolveLink(LinkPtr ptr);
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
