/*
 * File: ConstraintTest.cs
 * Project: Holo
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 27/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3Sharp.Gain.Holo;
using static AUTD3Sharp.Gain.Holo.Amplitude.Units;

namespace tests.Gain.Holo;

public class ConstraintTest
{
    [Fact]
    public async Task Uniform()
    {
        var autd = await AUTDTest.CreateController();

        var backend = new NalgebraBackend();
        var g = new Naive<NalgebraBackend>(backend)
            .AddFocus(autd.Geometry.Center + new Vector3d(30, 0, 150), 5e3 * Pascal)
            .AddFocus(autd.Geometry.Center + new Vector3d(30, 0, 150), 5e3 * Pascal)
            .WithConstraint(new AUTD3Sharp.Gain.Holo.Uniform(0x80));

        Assert.True(await autd.SendAsync(g));

        foreach (var dev in autd.Geometry)
        {
            var (intensities, phases) = autd.Link.IntensitiesAndPhases(dev.Idx, 0);
            Assert.All(intensities, d => Assert.Equal(0x80, d));
            Assert.Contains(phases, p => p != 0);
        }
    }

    [Fact]
    public async Task Normalize()
    {
        var autd = await AUTDTest.CreateController();

        var backend = new NalgebraBackend();
        var g = new Naive<NalgebraBackend>(backend)
            .AddFocus(autd.Geometry.Center + new Vector3d(30, 0, 150), 5e3 * Pascal)
            .AddFocus(autd.Geometry.Center + new Vector3d(30, 0, 150), 5e3 * Pascal)
            .WithConstraint(new Normalize());

        Assert.True(await autd.SendAsync(g));

        foreach (var dev in autd.Geometry)
        {
            var (intensities, phases) = autd.Link.IntensitiesAndPhases(dev.Idx, 0);
            Assert.Contains(intensities, d => d != 0);
            Assert.Contains(phases, p => p != 0);
        }
    }

    [Fact]
    public async Task Clamp()
    {
        var autd = await AUTDTest.CreateController();

        var backend = new NalgebraBackend();
        var g = new Naive<NalgebraBackend>(backend)
            .AddFocus(autd.Geometry.Center + new Vector3d(30, 0, 150), 5e3 * Pascal)
            .AddFocus(autd.Geometry.Center + new Vector3d(30, 0, 150), 5e3 * Pascal)
            .WithConstraint(new Clamp(new EmitIntensity(67), new EmitIntensity(85)));

        Assert.True(await autd.SendAsync(g));

        foreach (var dev in autd.Geometry)
        {
            var (intensities, phases) = autd.Link.IntensitiesAndPhases(dev.Idx, 0);
            Assert.All(intensities, d => Assert.True(67 <= d));
            Assert.All(intensities, d => Assert.True(d <= 85));
            Assert.Contains(phases, p => p != 0);
        }
    }

    [Fact]
    public async Task DontCare()
    {
        var autd = await AUTDTest.CreateController();

        var backend = new NalgebraBackend();
        var g = new Naive<NalgebraBackend>(backend)
            .AddFocus(autd.Geometry.Center + new Vector3d(30, 0, 150), 5e3 * Pascal)
            .AddFocus(autd.Geometry.Center + new Vector3d(30, 0, 150), 5e3 * Pascal)
            .WithConstraint(new DontCare());

        Assert.True(await autd.SendAsync(g));

        foreach (var dev in autd.Geometry)
        {
            var (intensities, phases) = autd.Link.IntensitiesAndPhases(dev.Idx, 0);
            Assert.Contains(intensities, d => d != 0);
            Assert.Contains(phases, p => p != 0);
        }
    }
}