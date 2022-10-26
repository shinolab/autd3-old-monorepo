/*
 * File: Quaterniond.cs
 * Project: Util
 * Created Date: 02/07/2018
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2019 Shun Suzuki. All rights reserved.
 * 
 */

using System;

namespace AUTD3Sharp.Utils
{
    public readonly struct Quaterniond : IEquatable<Quaterniond>
    {
        #region ctor
        public Quaterniond(double x, double y, double z, double w)
        {
            this.w = w;
            this.x = x;
            this.y = y;
            this.z = z;
        }
        #endregion

        #region property
#pragma warning disable IDE1006
        public double w { get; }
        public double x { get; }
        public double y { get; }
        public double z { get; }
#pragma warning restore IDE1006
        #endregion

        public Quaterniond Normalized => this / L2Norm;
        public double L2Norm => Math.Sqrt(L2NormSquared);
        public double L2NormSquared => w * w + x * x + y * y + z * z;

        #region indexcer
        public double this[int index]
        {
            get
            {
                return index switch
                {
                    3 => w,
                    0 => x,
                    1 => y,
                    2 => z,
                    _ => throw new ArgumentOutOfRangeException(nameof(index))
                };
            }
        }
        #endregion

        #region arithmetic
        public static bool operator ==(Quaterniond left, Quaterniond right) => left.Equals(right);
        public static bool operator !=(Quaterniond left, Quaterniond right) => !left.Equals(right);
        public bool Equals(Quaterniond other) => w.Equals(other.w) && x.Equals(other.x) && y.Equals(other.y) && z.Equals(other.z);
        public override bool Equals(object? obj)
        {
            if (obj is Quaterniond qua) return Equals(qua);
            return false;
        }
        #endregion

        #region util
        public override int GetHashCode() => w.GetHashCode() ^ x.GetHashCode() ^ y.GetHashCode() ^ z.GetHashCode();
        #endregion
    }
}
