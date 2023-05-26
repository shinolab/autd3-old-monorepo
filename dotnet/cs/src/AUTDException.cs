/*
 * File: AUTDException.cs
 * Project: src
 * Created Date: 26/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

using System;
using System.Text;
using System.Runtime.Serialization;

namespace AUTD3Sharp
{
    [Serializable]
    class AUTDException : Exception
    {
        public AUTDException() { }

        public AUTDException(StringBuilder value)
            : base(String.Format("AUTDException: {0}", value.ToString()))
        {
        }
    }
}
