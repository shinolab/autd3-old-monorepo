/*
 * File: GroupTest.cs
 * Project: Gain
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

namespace tests.Gain;

public class GroupTest
{
    [Fact]
    public async Task Group()
    {
        var autd = await AUTDTest.CreateController();

        var cx = autd.Geometry.Center.x;

        Assert.True(await autd.SendAsync(new Group((_, tr) => tr.Position.x switch
        {
            var x when x < cx => "uniform",
            _ => "null"
        }).Set("uniform", new Uniform(new EmitIntensity(0x80)).WithPhase(new Phase(0x90))).Set("null", new Null())));
        foreach (var dev in autd.Geometry)
        {
            var (intensities, phases) = autd.Link.IntensitiesAndPhases(dev.Idx, 0);
            foreach (var tr in dev)
            {
                if (tr.Position.x < cx)
                {
                    Assert.Equal(0x80, intensities[tr.Idx]);
                    Assert.Equal(0x90, phases[tr.Idx]);
                }
                else
                {
                    Assert.Equal(0, intensities[tr.Idx]);
                    Assert.Equal(0, phases[tr.Idx]);
                }
            }
        }

        Assert.True(await autd.SendAsync(new Group((_, tr) => tr.Position.x switch
        {
            var x when x > cx => "uniform",
            _ => null
        }).Set("uniform", new Uniform(new EmitIntensity(0x81)).WithPhase(new Phase(0x91)))));
        foreach (var dev in autd.Geometry)
        {
            var (intensities, phases) = autd.Link.IntensitiesAndPhases(dev.Idx, 0);
            foreach (var tr in dev)
            {
                if (tr.Position.x > cx)
                {
                    Assert.Equal(0x81, intensities[tr.Idx]);
                    Assert.Equal(0x91, phases[tr.Idx]);
                }
                else
                {
                    Assert.Equal(0, intensities[tr.Idx]);
                    Assert.Equal(0, phases[tr.Idx]);
                }
            }
        }
    }

    [Fact]
    public async Task GroupUnknownKey()
    {
        var autd = await AUTDTest.CreateController();

        var exception = await Record.ExceptionAsync(async () =>
        {
            await autd.SendAsync(new Group((_, _) => "null").Set("uniform", new Uniform(new EmitIntensity(0x80)).WithPhase(new Phase(0x90))).Set("null", new Null()));
        });

        if (exception == null) Assert.Fail("Exception is expected");
        Assert.Equal(typeof(AUTDException), exception.GetType());
        Assert.Equal("AUTDException: Unknown group key", exception.Message);
    }

    [Fact]
    public async Task GroupUnspecifiedKey()
    {
        var autd = await AUTDTest.CreateController();

        var exception = await Record.ExceptionAsync(async () =>
        {
            await autd.SendAsync(new Group((_, _) => "null"));
        });

        if (exception == null) Assert.Fail("Exception is expected");
        Assert.Equal(typeof(AUTDException), exception.GetType());
        Assert.Equal("AUTDException: Unspecified group key", exception.Message);
    }

    [Fact]
    public async Task GroupCheckOnlyForEnabled()
    {
        var autd = await AUTDTest.CreateController();
        autd.Geometry[0].Enable = false;

        var check = new bool[autd.Geometry.NumDevices];
        Assert.True(await autd.SendAsync(new Group((dev, _) =>
        {
            check[dev.Idx] = true;
            return "uniform";
        }).Set("uniform", new Uniform(new EmitIntensity(0x80)).WithPhase(new Phase(0x90)))));

        Assert.False(check[0]);
        Assert.True(check[1]);

        {
            var (intensities, phases) = autd.Link.IntensitiesAndPhases(0, 0);
            Assert.All(intensities, d => Assert.Equal(0, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }
        {
            var (intensities, phases) = autd.Link.IntensitiesAndPhases(1, 0);
            Assert.All(intensities, d => Assert.Equal(0x80, d));
            Assert.All(phases, p => Assert.Equal(0x90, p));
        }
    }
}
