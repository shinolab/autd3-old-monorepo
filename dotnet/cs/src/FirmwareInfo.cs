/*
 * File: FirmwareInfo.cs
 * Project: src
 * Created Date: 28/04/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 * 
 */

using System;
using System.Text;
using AUTD3Sharp.NativeMethods;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

namespace AUTD3Sharp
{
    public readonly struct FirmwareInfo
    {
        public string Info { get; }

        public static string LatestVersion
        {
            get
            {
                var latest = new byte[256];
                unsafe
                {
                    fixed (byte* l = &latest[0])
                        NativeMethodsBase.AUTDFirmwareLatest(l);
                }
                return Encoding.UTF8.GetString(latest).TrimEnd('\0');
            }
        }

        internal FirmwareInfo(string info)
        {
            Info = info.TrimEnd('\0');
        }

        public override string ToString() => Info;
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
