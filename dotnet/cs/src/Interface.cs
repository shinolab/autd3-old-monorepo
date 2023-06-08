/*
 * File: Interface.cs
 * Project: src
 * Created Date: 17/12/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
* Copyright (c) 2021-2023 Shun Suzuki. All rights reserved.
 * 
 */

using System.Runtime.InteropServices;

namespace AUTD3Sharp
{
    [ComVisible(false)]
    public interface ISpecialData
    {
        public DatagramSpecialPtr Ptr();
    }

    [ComVisible(false)]
    public interface IHeader
    {
        public DatagramHeaderPtr Ptr();
    }

    [ComVisible(false)]
    public interface IBody
    {
        public DatagramBodyPtr Ptr(Geometry geometry);
    }
}
