/*
 * File: GeometryTest.cs
 * Project: tests
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

namespace tests;

public class GeometryTest
{
    [Fact]
    public void AUTD3Props()
    {
        Assert.Equal(10.16, AUTD3.TransSpacing);
        Assert.Equal(10.16, AUTD3.TransSpacingMm);

        Assert.Equal(192.0, AUTD3.DeviceWidth);
        Assert.Equal(151.4, AUTD3.DeviceHeight);

        Assert.Equal(18, AUTD3.NumTransInX);
        Assert.Equal(14, AUTD3.NumTransInY);
        Assert.Equal(249, AUTD3.NumTransInUnit);

        Assert.Equal(20.48e6, AUTD3.FPGAClkFreq);
    }

    [Fact]
    public async Task GeometryNumDevices()
    {
        var autd = await AUTDTest.CreateController();
        Assert.Equal(2, autd.Geometry.NumDevices);
    }


    [Fact]
    public async Task GeometryNumTransducers()
    {
        var autd = await AUTDTest.CreateController();
        Assert.Equal(2 * 249, autd.Geometry.NumTransducers);
    }

    [Fact]
    public async Task TestGeometrySoundSpeed()
    {
        var autd = await AUTDTest.CreateController();
        autd.Geometry.SetSoundSpeed(350e3);
        foreach (var dev in autd.Geometry) Assert.Equal(350e3, dev.SoundSpeed);
    }

    [Fact]
    public async Task TestGeometrySetSoundSpeedFromTemp()
    {
        var autd = await AUTDTest.CreateController();
        autd.Geometry.SetSoundSpeedFromTemp(15);
        foreach (var dev in autd.Geometry) Assert.Equal(340.2952640537549e3, dev.SoundSpeed);
    }

    [Fact]
    public async Task GeometryCenter()
    {
        var autd = await AUTDTest.CreateController();
        Assert.Equal(new Vector3d(86.62522088353406, 66.71325301204821, 0), autd.Geometry.Center);
    }

    [Fact]
    public async Task TestDeviceIdx()
    {
        var autd = await AUTDTest.CreateController();
        Assert.Equal(0, autd.Geometry[0].Idx);
        Assert.Equal(1, autd.Geometry[1].Idx);
    }

    [Fact]
    public async Task TestDeviceSoundSpeed()
    {
        var autd = await AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(340e3, dev.SoundSpeed);
            dev.SoundSpeed = 350e3;
            Assert.Equal(350e3, dev.SoundSpeed);
        }
    }

    [Fact]
    public async Task TestDeviceSetSoundSpeedFromTemp()
    {
        var autd = await AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            dev.SetSoundSpeedFromTemp(15);
            Assert.Equal(340.2952640537549e3, dev.SoundSpeed);
        }
    }

    [Fact]
    public async Task TestDeviceAttenuation()
    {
        var autd = await AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(0.0, dev.Attenuation);
            dev.Attenuation = 1.0;
            Assert.Equal(1.0, dev.Attenuation);
        }
    }

    [Fact]
    public async Task TestDeviceEnable()
    {
        var autd = await AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            Assert.True(dev.Enable);
            dev.Enable = false;
            Assert.False(dev.Enable);
        }
    }

    [Fact]
    public async Task TestDeviceNumTransducers()
    {
        var autd = await AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(249, dev.NumTransducers);
        }
    }

    [Fact]
    public async Task TestDeviceCenter()
    {
        var autd = await AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            var center = dev.Center;
            Assert.Equal(86.62522088353406, center.x);
            Assert.Equal(66.71325301204821, center.y);
            Assert.Equal(0.0, center.z);
        }
    }

    [Fact]
    public async Task TestDeviceTranslate()
    {
        var autd = await AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            var originalPos = dev.Select(tr => tr.Position).ToArray();
            var t = new Vector3d(1, 2, 3);
            dev.Translate(t);
            foreach (var tr in dev)
            {
                Assert.Equal(tr.Position, originalPos[tr.Idx] + t);
            }
        }
    }

    [Fact]
    public async Task TestDeviceRotate()
    {
        var autd = await AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            var r = new Quaterniond(0, 0, 0.7071067811865476, 0.7071067811865476);
            dev.Rotate(r);
            foreach (var tr in dev)
            {
                Assert.Equal(r, tr.Rotation);
            }
        }
    }

    [Fact]
    public async Task TestDeviceAffine()
    {
        var autd = await AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            var originalPos = dev.Select(tr => tr.Position).ToArray();
            var t = new Vector3d(1, 2, 3);
            var r = new Quaterniond(0, 0, 0.7071067811865476, 0.7071067811865476);
            dev.Affine(t, r);
            foreach (var tr in dev)
            {
                var op = originalPos[tr.Idx];
                var expected = new Vector3d(-op.y, op.x, op.z) + t;
                Assert.True(Math.Abs(expected.x - tr.Position.x) < 1e-3);
                Assert.True(Math.Abs(expected.y - tr.Position.y) < 1e-3);
                Assert.True(Math.Abs(expected.z - tr.Position.z) < 1e-3);
                Assert.Equal(r, tr.Rotation);
            }
        }
    }

    [Fact]
    public async Task TestTransducerIdx()
    {
        var autd = await AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            foreach (var (tr, i) in dev.Select((tr, i) => (tr, i)))
            {
                Assert.Equal(i, tr.Idx);
            }
        }
    }

    [Fact]
    public async Task TestTransducerPosition()
    {
        var autd = await AUTDTest.CreateController();
        Assert.Equal(autd.Geometry[0][0].Position, new Vector3d(0.0, 0.0, 0.0));
        Assert.Equal(autd.Geometry[0][AUTD3.NumTransInUnit - 1].Position,
            new Vector3d((AUTD3.NumTransInX - 1) * AUTD3.TransSpacing, (AUTD3.NumTransInY - 1) * AUTD3.TransSpacing, 0.0));

        Assert.Equal(autd.Geometry[1][0].Position, new Vector3d(0.0, 0.0, 0.0));
        Assert.Equal(autd.Geometry[1][AUTD3.NumTransInUnit - 1].Position,
            new Vector3d((AUTD3.NumTransInX - 1) * AUTD3.TransSpacing, (AUTD3.NumTransInY - 1) * AUTD3.TransSpacing, 0.0));
    }

    [Fact]
    public async Task TestTransducerRotation()
    {
        var autd = await AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            foreach (var tr in dev)
            {
                Assert.Equal(tr.Rotation, new Quaterniond(0.0, 0.0, 0.0, 1.0));
            }
        }
    }

    [Fact]
    public async Task TestTransducerXDirection()
    {
        var autd = await AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            foreach (var tr in dev)
            {
                Assert.Equal(tr.XDirection, new Vector3d(1.0, 0.0, 0.0));
            }
        }
    }

    [Fact]
    public async Task TestTransducerYDirection()
    {
        var autd = await AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            foreach (var tr in dev)
            {
                Assert.Equal(tr.YDirection, new Vector3d(0.0, 1.0, 0.0));
            }
        }
    }

    [Fact]
    public async Task TestTransducerZDirection()
    {
        var autd = await AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            foreach (var tr in dev)
            {
                Assert.Equal(tr.ZDirection, new Vector3d(0.0, 0.0, 1.0));
            }
        }
    }

    [Fact]
    public async Task TestTransducerWavelength()
    {
        var autd = await AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            foreach (var tr in dev)
            {
                Assert.Equal(340e3 / 40e3, tr.Wavelength(340e3));
            }
        }
    }

    [Fact]
    public async Task TestTransducerWavenum()
    {
        var autd = await AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            foreach (var tr in dev)
            {
                Assert.Equal(2.0 * Math.PI * 40e3 / 340e3, tr.Wavenumber(340e3));
            }
        }
    }

}