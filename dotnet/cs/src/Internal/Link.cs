/*
 * File: Link.cs
 * Project: src
 * Created Date: 28/04/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021-2023 Shun Suzuki. All rights reserved.
 *
 */

using System.Runtime.InteropServices;

namespace AUTD3Sharp.Internal
{
    [ComVisible(false)]
    public interface ILinkBuilder
    {
        LinkBuilderPtr Ptr();
    }

    [ComVisible(false)]
    public interface ILink<out T>
    {
        T Create(LinkPtr ptr);
    }
}
