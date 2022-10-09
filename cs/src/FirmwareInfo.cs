/*
 * File: FirmwareInfo.cs
 * Project: src
 * Created Date: 28/04/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/06/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 * 
 */

using System;

namespace AUTD3Sharp
{
    public readonly struct FirmwareInfo : IEquatable<FirmwareInfo>
    {
        public string Info { get; }

        internal FirmwareInfo(string info)
        {
            Info = info;
        }

        public override string ToString() => $"{Info}";
        public bool Equals(FirmwareInfo other) => Info.Equals(other.Info);
        public static bool operator ==(FirmwareInfo left, FirmwareInfo right) => left.Equals(right);
        public static bool operator !=(FirmwareInfo left, FirmwareInfo right) => !left.Equals(right);
        public override bool Equals(object? obj) => obj is FirmwareInfo info && Equals(info);
        public override int GetHashCode() => Info.GetHashCode();
    }
}
