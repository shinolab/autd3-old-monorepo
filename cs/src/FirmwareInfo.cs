/*
 * File: FirmwareInfo.cs
 * Project: src
 * Created Date: 28/04/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/02/2023
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
        public bool MatchesVersion { get; }
        public bool IsLatest { get; }

        public static string LatestVersion
        {
            get
            {
                var latest = new StringBuilder(256);
                NativeMethods.Base.AUTDGetLatestFirmware(latest);
                return latest.ToString();
            }
        }

        internal FirmwareInfo(string info, bool matchesVersion, bool isLatest)
        {
            Info = info;
            MatchesVersion = matchesVersion;
            IsLatest = isLatest;
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
#nullable disable
#endif
