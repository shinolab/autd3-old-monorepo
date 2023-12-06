/*
 * File: Quaterniond.cs
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

public class QuaterniondTests
{
    [Fact]
    public void Constructor_WithThreeArguments_SetsPropertiesCorrectly()
    {
        var q = new Quaterniond(1, 2, 3, 4);

        Assert.Equal(1, q.x);
        Assert.Equal(2, q.y);
        Assert.Equal(3, q.z);
        Assert.Equal(4, q.w);
    }

    [Fact]
    public void Construct_Identity()
    {
        var q = Quaterniond.identity;

        Assert.Equal(0, q.x);
        Assert.Equal(0, q.y);
        Assert.Equal(0, q.z);
        Assert.Equal(1, q.w);
    }

    [Fact]
    public void GetHashCode_ReturnsConsistentHashCodes()
    {
        var q1 = new Quaterniond(1, 2, 3, 4);
        var q2 = new Quaterniond(1, 2, 3, 4);

        Assert.Equal(q1.GetHashCode(), q2.GetHashCode());
    }

    [Fact]
    public void Normalized_ReturnsNormalizedVector()
    {
        var q = new Quaterniond(1, 2, 2, 4);

        var result = q.Normalized;

        Assert.Equal(1 / 5.0, result.x);
        Assert.Equal(2 / 5.0, result.y);
        Assert.Equal(2 / 5.0, result.z);
        Assert.Equal(4 / 5.0, result.w);
    }

    [Fact]
    public void L2Norm_ReturnsCorrectNorm()
    {
        var q = new Quaterniond(1, 2, 2, 4);

        var result = q.L2Norm;

        Assert.Equal(5, result);
    }

    [Fact]
    public void L2NormSquared_ReturnsCorrectNormSquared()
    {
        var q = new Quaterniond(1, 2, 2, 4);

        var result = q.L2NormSquared;

        Assert.Equal(25, result);
    }

    [Fact]
    public void Indexer_ReturnsCorrectValues()
    {
        var q = new Quaterniond(1, 2, 3, 4);

        Assert.Equal(1, q[0]);
        Assert.Equal(2, q[1]);
        Assert.Equal(3, q[2]);
        Assert.Equal(4, q[3]);
        Assert.Throws<ArgumentOutOfRangeException>(() => q[4]);
    }

    [Fact]
    public void Equals_ReturnsTrueForEqualVectors()
    {
        var q1 = new Quaterniond(1, 2, 3, 4);
        var q2 = new Quaterniond(1, 2, 3, 4);
        var q3 = new Quaterniond(1, 2, 3, 5);

        Assert.True(q1 == q2);
        Assert.True(q1 != q3);
        Assert.True(q1.Equals(q2));
        Assert.True(!q1.Equals(q3));
        Assert.True(q1.Equals((object)q2));
        Assert.True(!q1.Equals((object)q3));
        Assert.True(!q1.Equals(null));
    }

    [Fact]
    public void ToString_ReturnsCorrectString()
    {
        var q = new Quaterniond(1, 2, 3, 4);

        var result = q.ToString();

        Assert.Equal("(4, 1, 2, 3)", result);
    }
}