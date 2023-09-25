/*
 * File: FirmwareInfo.cs
 * Project: src
 * Created Date: 28/04/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 * 
 */

using System;
using System.Text;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

namespace AUTD3Sharp
{
    public readonly struct FirmwareInfo : IEquatable<FirmwareInfo>
    {
        public string Info { get; }

        public static string LatestVersion
        {
            get
            {
                var latest = new byte[256];
                NativeMethods.Base.AUTDFirmwareLatest(latest);
                return Encoding.UTF8.GetString(latest).TrimEnd('\0');
            }
        }

        internal FirmwareInfo(string info)
        {
            Info = info.TrimEnd('\0');
        }

        public override string ToString() => $"{Info}";
        public bool Equals(FirmwareInfo other) => Info.Equals(other.Info);
        public static bool operator ==(FirmwareInfo left, FirmwareInfo right) => left.Equals(right);
        public static bool operator !=(FirmwareInfo left, FirmwareInfo right) => !left.Equals(right);
        public override bool Equals(object? obj) => obj is FirmwareInfo info && Equals(info);
        public override int GetHashCode() => Info.GetHashCode();
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
