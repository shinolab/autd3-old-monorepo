/*
 * File: Vector3d.cs
 * Project: Util
 * Created Date: 02/07/2018
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/06/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2018-2019 Shun Suzuki. All rights reserved.
 * 
 */

using System;
using System.Collections;
using System.Collections.Generic;
using System.Globalization;

namespace AUTD3Sharp.Utils
{
    public readonly struct Vector3d : IEquatable<Vector3d>, IEnumerable<double>
    {
        #region ctor
        public Vector3d(double x, double y, double z)
        {
            this.x = x;
            this.y = y;
            this.z = z;
        }

        public Vector3d(params double[] vector)
        {
            if (vector == null) throw new ArgumentNullException(nameof(vector));
            if (vector.Length != 3) throw new InvalidCastException();

            x = vector[0];
            y = vector[1];
            z = vector[2];
        }
        #endregion

        #region property
        public static Vector3d UnitX => new Vector3d(1, 0, 0);
        public static Vector3d UnitY => new Vector3d(0, 1, 0);
        public static Vector3d UnitZ => new Vector3d(0, 0, 1);
        public static Vector3d Zero => new Vector3d(0, 0, 0);
        public Vector3d Normalized => this / L2Norm;
        public double L2Norm => Math.Sqrt(L2NormSquared);
        public double L2NormSquared => x * x + y * y + z * z;
#pragma warning disable IDE1006
        public double x { get; }
        public double y { get; }
        public double z { get; }
#pragma warning restore IDE1006
        #endregion

        #region indexcer
        public double this[int index]
        {
            get
            {
                return index switch
                {
                    0 => x,
                    1 => y,
                    2 => z,
                    _ => throw new ArgumentOutOfRangeException(nameof(index))
                };
            }
        }
        #endregion

        #region arithmetic
        public static Vector3d Negate(Vector3d operand) => new Vector3d(-operand.x, -operand.y, -operand.z);

        public static Vector3d Add(Vector3d left, Vector3d right)
        {
            var v1 = left.x + right.x;
            var v2 = left.y + right.y;
            var v3 = left.z + right.z;
            return new Vector3d(v1, v2, v3);
        }
        public static Vector3d Subtract(Vector3d left, Vector3d right)
        {
            var v1 = left.x - right.x;
            var v2 = left.y - right.y;
            var v3 = left.z - right.z;
            return new Vector3d(v1, v2, v3);
        }

        public static Vector3d Divide(Vector3d left, double right)
        {
            var v1 = left.x / right;
            var v2 = left.y / right;
            var v3 = left.z / right;
            return new Vector3d(v1, v2, v3);
        }

        public static Vector3d Multiply(Vector3d left, double right)
        {
            var v1 = left.x * right;
            var v2 = left.y * right;
            var v3 = left.z * right;
            return new Vector3d(v1, v2, v3);
        }

        public static Vector3d Multiply(double left, Vector3d right) => Multiply(right, left);
        public static Vector3d operator -(Vector3d operand) => Negate(operand);
        public static Vector3d operator +(Vector3d left, Vector3d right) => Add(left, right);
        public static Vector3d operator -(Vector3d left, Vector3d right) => Subtract(left, right);
        public static Vector3d operator *(Vector3d left, double right) => Multiply(left, right);
        public static Vector3d operator *(double left, Vector3d right) => Multiply(right, left);
        public static Vector3d operator /(Vector3d left, double right) => Divide(left, right);
        public static bool operator ==(Vector3d left, Vector3d right) => left.Equals(right);
        public static bool operator !=(Vector3d left, Vector3d right) => !left.Equals(right);
        public bool Equals(Vector3d other) => x.Equals(other.x) && y.Equals(other.y) && z.Equals(other.z);
        public override bool Equals(object? obj)
        {
            if (obj is Vector3d vec) return Equals(vec);
            return false;
        }
        #endregion

        #region public methods
        public Vector3d Rectify() => new Vector3d(Math.Max(x, 0), Math.Max(y, 0), Math.Max(z, 0));
        public double[] ToArray() => new[] { x, y, z };
        #endregion

        #region util
        public override int GetHashCode() => x.GetHashCode() ^ y.GetHashCode() ^ z.GetHashCode();
        IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();
        public string ToString(string format) => $"3D Column Vector:\n{string.Format(CultureInfo.CurrentCulture, format, x)}\n{string.Format(CultureInfo.CurrentCulture, format, y)}\n{string.Format(CultureInfo.CurrentCulture, format, z)}";

        public IEnumerator<double> GetEnumerator()
        {
            yield return x;
            yield return y;
            yield return z;
        }

        public override string ToString() => ToString("{0,-20}");
        #endregion
    }
}
