/*
 * File: AUTDException.cs
 * Project: src
 * Created Date: 26/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 30/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

using System;

namespace AUTD3Sharp
{
    [Serializable]
    internal class AUTDException : Exception
    {
        public AUTDException() { }

        public AUTDException(byte[] value)
            : base($"AUTDException: {System.Text.Encoding.UTF8.GetString(value)}")
        {
        }
        public AUTDException(string msg)
            : base($"AUTDException: {msg}")
        {
        }
    }
}
