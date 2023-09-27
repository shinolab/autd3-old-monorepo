/*
 * File: GeometryTest.cs
 * Project: tests
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 27/09/2023
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

        Assert.Equal(163.84e6, AUTD3.FPGAClkFreq);
        Assert.Equal(20.48e6, AUTD3.FPGASubClkFreq);
    }

    [Fact]
    public void GeometryNumDevices()
    {
        var autd = AUTDTest.CreateController();
        Assert.Equal(2, autd.Geometry.NumDevices);
    }

    [Fact]
    public void GeometryCenter()
    {
        var autd = AUTDTest.CreateController();
        Assert.Equal(new Vector3d(86.62522088353406, 66.71325301204821, 0), autd.Geometry.Center);
    }

    [Fact]
    public void TestDeviceIdx()
    {
        var autd = AUTDTest.CreateController();
        Assert.Equal(0, autd.Geometry[0].Idx);
        Assert.Equal(1, autd.Geometry[1].Idx);
    }

    [Fact]
    public void TestDeviceSoundSpeed()
    {
        var autd = AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(340e3, dev.SoundSpeed);
            dev.SoundSpeed = 350e3;
            Assert.Equal(350e3, dev.SoundSpeed);
        }
    }

    [Fact]
    public void TestDeviceSetSoundSpeedFromTemp()
    {
        var autd = AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            dev.SetSoundSpeedFromTemp(15);
            Assert.Equal(340.2952640537549e3, dev.SoundSpeed);
        }
    }

    [Fact]
    public void TestDeviceAttenuation()
    {
        var autd = AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(0.0, dev.Attenuation);
            dev.Attenuation = 1.0;
            Assert.Equal(1.0, dev.Attenuation);
        }
    }

    [Fact]
    public void TestDeviceEnable()
    {
        var autd = AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            Assert.True(dev.Enable);
            dev.Enable = false;
            Assert.False(dev.Enable);
        }
    }

    [Fact]
    public void TestDeviceNumTransducers()
    {
        var autd = AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(249, dev.NumTransducers);
        }
    }

    [Fact]
    public void TestDeviceCenter()
    {
        var autd = AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            var center = dev.Center;
            Assert.Equal(86.62522088353406, center.x);
            Assert.Equal(66.71325301204821, center.y);
            Assert.Equal(0.0, center.z);
        }
    }

    [Fact]
    public void TestDeviceForceFan()
    {
        var autd = AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(0, Audit.FpgaFlags(autd, dev.Idx));
        }

        autd.Geometry[0].ForceFan = true;
        autd.Geometry[1].ForceFan = false;

        autd.Send(new UpdateFlags());

        Assert.Equal(1, Audit.FpgaFlags(autd, 0));
        Assert.Equal(0, Audit.FpgaFlags(autd, 1));

        autd.Geometry[0].ForceFan = false;
        autd.Geometry[1].ForceFan = true;

        autd.Send(new UpdateFlags());

        Assert.Equal(0, Audit.FpgaFlags(autd, 0));
        Assert.Equal(1, Audit.FpgaFlags(autd, 1));
    }

    [Fact]
    public void TestDeviceTranslate()
    {
        var autd = AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            var originalPos = dev.Select(tr => tr.Position).ToArray();
            var t = new Vector3d(1, 2, 3);
            dev.Translate(t);
            foreach (var tr in dev)
            {
                Assert.Equal(tr.Position, originalPos[tr.LocalIdx] + t);
            }
        }
    }

    [Fact]
    public void TestDeviceRotate()
    {
        var autd = AUTDTest.CreateController();
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
    public void TestDeviceAffine()
    {
        var autd = AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            var originalPos = dev.Select(tr => tr.Position).ToArray();
            var t = new Vector3d(1, 2, 3);
            var r = new Quaterniond(0, 0, 0.7071067811865476, 0.7071067811865476);
            dev.Affine(t, r);
            foreach (var tr in dev)
            {
                var op = originalPos[tr.LocalIdx];
                var expected = new Vector3d(-op.y, op.x, op.z) + t;
                Assert.True(Math.Abs(expected.x - tr.Position.x) < 1e-3);
                Assert.True(Math.Abs(expected.y - tr.Position.y) < 1e-3);
                Assert.True(Math.Abs(expected.z - tr.Position.z) < 1e-3);
                Assert.Equal(r, tr.Rotation);
            }
        }
    }

    [Fact]
    public void TestTransducerLocalIdx()
    {
        var autd = AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            foreach (var (tr, i) in dev.Select((tr, i) => (tr, i)))
            {
                Assert.Equal(i, tr.LocalIdx);
            }
        }
    }

    [Fact]
    public void TestTransducerPosition()
    {
        var autd = AUTDTest.CreateController();
        Assert.Equal(autd.Geometry[0][0].Position, new Vector3d(0.0, 0.0, 0.0));
        Assert.Equal(autd.Geometry[0][AUTD3.NumTransInUnit - 1].Position,
            new Vector3d((AUTD3.NumTransInX - 1) * AUTD3.TransSpacing, (AUTD3.NumTransInY - 1) * AUTD3.TransSpacing, 0.0));

        Assert.Equal(autd.Geometry[1][0].Position, new Vector3d(0.0, 0.0, 0.0));
        Assert.Equal(autd.Geometry[1][AUTD3.NumTransInUnit - 1].Position,
            new Vector3d((AUTD3.NumTransInX - 1) * AUTD3.TransSpacing, (AUTD3.NumTransInY - 1) * AUTD3.TransSpacing, 0.0));
    }

    [Fact]
    public void TestTransducerRotation()
    {
        var autd = AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            foreach (var tr in dev)
            {
                Assert.Equal(tr.Rotation, new Quaterniond(0.0, 0.0, 0.0, 1.0));
            }
        }
    }

    [Fact]
    public void TestTransducerXDirection()
    {
        var autd = AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            foreach (var tr in dev)
            {
                Assert.Equal(tr.XDirection, new Vector3d(1.0, 0.0, 0.0));
            }
        }
    }

    [Fact]
    public void TestTransducerYDirection()
    {
        var autd = AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            foreach (var tr in dev)
            {
                Assert.Equal(tr.YDirection, new Vector3d(0.0, 1.0, 0.0));
            }
        }
    }

    [Fact]
    public void TestTransducerZDirection()
    {
        var autd = AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            foreach (var tr in dev)
            {
                Assert.Equal(tr.ZDirection, new Vector3d(0.0, 0.0, 1.0));
            }
        }
    }

    [Fact]
    public void TestTransducerFrequency()
    {
        var autd = AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            foreach (var tr in dev)
            {
                Assert.Equal(40e3, tr.Frequency);
                tr.Frequency = 69.98718496369073e3;
                Assert.Equal(69.98718496369073e3, tr.Frequency);
            }
        }
    }

    [Fact]
    public void TestTransducerCycle()
    {
        var autd = AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            foreach (var tr in dev)
            {
                Assert.Equal(4096, tr.Cycle);
                tr.Cycle = 3000;
                Assert.Equal(3000, tr.Cycle);
            }
        }
    }

    [Fact]
    public void TestTransducerModDelay()
    {
        var autd = AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            foreach (var tr in dev)
            {
                Assert.Equal(0, tr.ModDelay);
                tr.ModDelay = 1;
                Assert.Equal(1, tr.ModDelay);
            }
        }
    }

    [Fact]
    public void TestTransducerAmpFilter()
    {
        var autd = AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            foreach (var tr in dev)
            {
                Assert.Equal(0, tr.AmpFilter);
                tr.AmpFilter = -1;
                Assert.Equal(-1, tr.AmpFilter);
            }
        }
    }

    [Fact]
    public void TestTransducerPhaseFilter()
    {
        var autd = AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            foreach (var tr in dev)
            {
                Assert.Equal(0, tr.PhaseFilter);
                tr.PhaseFilter = -1;
                Assert.Equal(-1, tr.PhaseFilter);
            }
        }
    }

    [Fact]
    public void TestTransducerWavelength()
    {
        var autd = AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            foreach (var tr in dev)
            {
                Assert.Equal(340e3 / 40e3, tr.Wavelength(340e3));
            }
        }
    }

    [Fact]
    public void TestTransducerWavenum()
    {
        var autd = AUTDTest.CreateController();
        foreach (var dev in autd.Geometry)
        {
            foreach (var tr in dev)
            {
                Assert.Equal(2.0 * Math.PI * 40e3 / 340e3, tr.Wavenumber(340e3));
            }
        }
    }

}