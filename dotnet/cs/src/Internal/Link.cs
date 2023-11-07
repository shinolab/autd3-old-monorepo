﻿/*
 * File: Link.cs
 * Project: src
 * Created Date: 28/04/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 27/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021-2023 Shun Suzuki. All rights reserved.
 *
 */

using System.Runtime.InteropServices;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

namespace AUTD3Sharp.Internal
{
    [ComVisible(false)]
    public interface ILinkBuilder
    {
        internal LinkBuilderPtr Ptr();

        internal object? Props()
        {
            return null;
        }
    }

    [ComVisible(false)]
    public interface ILink<out T>
    {
        internal T Create(LinkPtr ptr, object? props);
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
