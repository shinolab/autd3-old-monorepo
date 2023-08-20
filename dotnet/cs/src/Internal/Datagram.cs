/*
 * File: Datagram.cs
 * Project: Internal
 * Created Date: 20/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

using System.Runtime.InteropServices;

namespace AUTD3Sharp
{
    namespace Internal
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
}