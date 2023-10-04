/*
 * File: AUTDException.cs
 * Project: src
 * Created Date: 26/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 03/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

using System;

namespace AUTD3Sharp
{
    [Serializable]
    public class AUTDException : Exception
    {
        public AUTDException() { }

        public AUTDException(byte[] value)
            : base($"AUTDException: {System.Text.Encoding.UTF8.GetString(value).TrimEnd('\0')}")
        {
        }

        public AUTDException(string msg)
            : base($"AUTDException: {msg}")
        {
        }
    }
}
