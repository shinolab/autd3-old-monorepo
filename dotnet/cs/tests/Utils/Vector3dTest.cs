/*
 * File: Vector3dTest.cs
 * Project: Utils
 * Created Date: 06/12/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */


namespace tests;

public class Vector3dTest
{
    [Fact]
    public void Constructor_WithThreeArguments_SetsPropertiesCorrectly()
    {
        var vector = new Vector3d(1, 2, 3);

        Assert.Equal(1, vector.x);
        Assert.Equal(2, vector.y);
        Assert.Equal(3, vector.z);
    }

    [Fact]
    public void Constructor_WithArrayArgument_SetsPropertiesCorrectly()
    {
        var vector = new Vector3d(new double[] { 1, 2, 3 });

        Assert.Equal(1, vector.x);
        Assert.Equal(2, vector.y);
        Assert.Equal(3, vector.z);

        Assert.Throws<InvalidCastException>(() => new Vector3d(new double[] { }));
    }

    [Fact]
    public void Add_AddsVectorsCorrectly()
    {
        var vector1 = new Vector3d(1, 2, 3);
        var vector2 = new Vector3d(4, 5, 6);

        var result = Vector3d.Add(vector1, vector2);

        Assert.Equal(5, result.x);
        Assert.Equal(7, result.y);
        Assert.Equal(9, result.z);
    }

    [Fact]
    public void Subtract_SubtractsVectorsCorrectly()
    {
        var vector1 = new Vector3d(1, 2, 3);
        var vector2 = new Vector3d(4, 5, 6);

        {
            var result = Vector3d.Subtract(vector1, vector2);

            Assert.Equal(-3, result.x);
            Assert.Equal(-3, result.y);
            Assert.Equal(-3, result.z);
        }

        {
            var result = vector1 - vector2;

            Assert.Equal(-3, result.x);
            Assert.Equal(-3, result.y);
            Assert.Equal(-3, result.z);
        }
    }

    [Fact]
    public void GetHashCode_ReturnsConsistentHashCodes()
    {
        var vector1 = new Vector3d(1, 2, 3);
        var vector2 = new Vector3d(1, 2, 3);

        Assert.Equal(vector1.GetHashCode(), vector2.GetHashCode());
    }

    [Fact]
    public void GetEnumerator_ReturnsCorrectValues()
    {
        var vector = new Vector3d(1, 2, 3);

        var enumerator = vector.GetEnumerator();

        Assert.True(enumerator.MoveNext());
        Assert.Equal(1, enumerator.Current);

        Assert.True(enumerator.MoveNext());
        Assert.Equal(2, enumerator.Current);

        Assert.True(enumerator.MoveNext());
        Assert.Equal(3, enumerator.Current);

        Assert.False(enumerator.MoveNext());
    }


    [Fact]
    public void Normalized_ReturnsNormalizedVector()
    {
        var vector = new Vector3d(1, 2, 2);

        var result = vector.Normalized;

        Assert.Equal(1 / 3.0, result.x);
        Assert.Equal(2 / 3.0, result.y);
        Assert.Equal(2 / 3.0, result.z);
    }

    [Fact]
    public void L2Norm_ReturnsCorrectNorm()
    {
        var vector = new Vector3d(1, 2, 2);

        var result = vector.L2Norm;

        Assert.Equal(3, result);
    }

    [Fact]
    public void L2NormSquared_ReturnsCorrectNormSquared()
    {
        var vector = new Vector3d(1, 2, 2);

        var result = vector.L2NormSquared;

        Assert.Equal(9, result);
    }

    [Fact]
    public void Indexer_ReturnsCorrectValues()
    {
        var vector = new Vector3d(1, 2, 3);

        Assert.Equal(1, vector[0]);
        Assert.Equal(2, vector[1]);
        Assert.Equal(3, vector[2]);
        Assert.Throws<ArgumentOutOfRangeException>(() => vector[3]);
    }

    [Fact]
    public void Multiply_MultipliesCorrectly()
    {
        var vector = new Vector3d(1, 2, 3);

        var result = Vector3d.Multiply(2, vector);

        Assert.Equal(2, result.x);
        Assert.Equal(4, result.y);
        Assert.Equal(6, result.z);
    }

    [Fact]
    public void OperatorMultiply_MultipliesCorrectly()
    {
        var vector = new Vector3d(1, 2, 3);

        var result = vector * 2;

        Assert.Equal(2, result.x);
        Assert.Equal(4, result.y);
        Assert.Equal(6, result.z);
    }

    [Fact]
    public void Equals_ReturnsTrueForEqualVectors()
    {
        var vector1 = new Vector3d(1, 2, 3);
        var vector2 = new Vector3d(1, 2, 3);
        var vector3 = new Vector3d(2, 3, 4);

        Assert.True(vector1 == vector2);
        Assert.True(vector1 != vector3);
        Assert.True(vector1.Equals(vector2));
        Assert.True(!vector1.Equals(vector3));
        Assert.True(vector1.Equals((object)vector2));
        Assert.True(!vector1.Equals((object)vector3));
        Assert.True(!vector1.Equals(null));
    }

    [Fact]
    public void Rectify_ReturnsRectifiedVector()
    {
        var vector = new Vector3d(-1, -2, -3);

        var result = vector.Rectify();

        Assert.Equal(0, result.x);
        Assert.Equal(0, result.y);
        Assert.Equal(0, result.z);
    }

    [Fact]
    public void ToArray_ReturnsCorrectArray()
    {
        var vector = new Vector3d(1, 2, 3);

        var result = vector.ToArray();

        Assert.Equal(new double[] { 1, 2, 3 }, result);
    }

    [Fact]
    public void ToString_ReturnsCorrectString()
    {
        var vector = new Vector3d(1, 2, 3);

        var result = vector.ToString();

        Assert.Equal("3D Column Vector:\n1                   \n2                   \n3                   ", result);
    }
}