/*
 * File: ConstraintTest.cs
 * Project: Holo
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3Sharp.Gain.Holo;

namespace tests.Gain.Holo;

public class ConstraintTest
{
    [Fact]
    public void Uniform()
    {
        var autd = AUTDTest.CreateController();

        var backend = new NalgebraBackend();
        var g = new Naive<NalgebraBackend>(backend)
            .AddFocus(autd.Geometry.Center + new Vector3d(30, 0, 150), 0.5)
            .AddFocus(autd.Geometry.Center + new Vector3d(30, 0, 150), 0.5)
            .WithConstraint(new AUTD3Sharp.Gain.Holo.Uniform(0.5));

        Assert.True(autd.Send(g));

        foreach (var dev in autd.Geometry)
        {
            var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(680, d));
            Assert.Contains(phases, p => p != 0);
        }
    }

    [Fact]
    public void Normalize()
    {
        var autd = AUTDTest.CreateController();

        var backend = new NalgebraBackend();
        var g = new Naive<NalgebraBackend>(backend)
            .AddFocus(autd.Geometry.Center + new Vector3d(30, 0, 150), 0.5)
            .AddFocus(autd.Geometry.Center + new Vector3d(30, 0, 150), 0.5)
            .WithConstraint(new Normalize());

        Assert.True(autd.Send(g));

        foreach (var dev in autd.Geometry)
        {
            var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 0);
            Assert.Contains(duties, d => d != 0);
            Assert.Contains(phases, p => p != 0);
        }
    }

    [Fact]
    public void Clamp()
    {
        var autd = AUTDTest.CreateController();

        var backend = new NalgebraBackend();
        var g = new Naive<NalgebraBackend>(backend)
            .AddFocus(autd.Geometry.Center + new Vector3d(30, 0, 150), 0.5)
            .AddFocus(autd.Geometry.Center + new Vector3d(30, 0, 150), 0.5)
            .WithConstraint(new Clamp(0.4, 0.5));

        Assert.True(autd.Send(g));

        foreach (var dev in autd.Geometry)
        {
            var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 0);
            Assert.All(duties, d => Assert.True(536 <= d));
            Assert.All(duties, d => Assert.True(d <= 680));
            Assert.Contains(phases, p => p != 0);
        }
    }

    [Fact]
    public void DontCare()
    {
        var autd = AUTDTest.CreateController();

        var backend = new NalgebraBackend();
        var g = new Naive<NalgebraBackend>(backend)
            .AddFocus(autd.Geometry.Center + new Vector3d(30, 0, 150), 0.5)
            .AddFocus(autd.Geometry.Center + new Vector3d(30, 0, 150), 0.5)
            .WithConstraint(new DontCare());

        Assert.True(autd.Send(g));

        foreach (var dev in autd.Geometry)
        {
            var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 0);
            Assert.Contains(duties, d => d != 0);
            Assert.Contains(phases, p => p != 0);
        }
    }
}