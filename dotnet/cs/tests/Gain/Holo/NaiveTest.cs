/*
 * File: NaiveTest.cs
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

public class NaiveTest
{
    [Fact]
    public void Naive()
    {
        var autd = Controller.Builder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).OpenWith(new Audit());

        var backend = new NalgebraBackend();
        var g = new Naive<NalgebraBackend>(backend)
            .AddFocus(autd.Geometry.Center + new Vector3d(30, 0, 150), 0.5)
            .AddFociFromIter(new double[] { -40 }.Select(x => (autd.Geometry.Center + new Vector3d(x, 0, 150), 0.5)))
            .WithConstraint(new AUTD3Sharp.Gain.Holo.Uniform(0.5));

        Assert.True(autd.Send(g));

        foreach (var dev in autd.Geometry)
        {
            var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(680, d));
            Assert.Contains(phases, p => p != 0);
        }
    }

    [IgnoreIfCUDAIsNotFoundFact]
    public void NaiveWithCUDA()
    {
        var autd = Controller.Builder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).OpenWith(new Audit());

        var backend = new CUDABackend();
        var g = new Naive<CUDABackend>(backend)
            .AddFocus(autd.Geometry.Center + new Vector3d(30, 0, 150), 0.5)
            .AddFociFromIter(new double[] { -40 }.Select(x => (autd.Geometry.Center + new Vector3d(x, 0, 150), 0.5)))
            .WithConstraint(new AUTD3Sharp.Gain.Holo.Uniform(0.5));

        Assert.True(autd.Send(g));

        foreach (var dev in autd.Geometry)
        {
            var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(680, d));
            Assert.Contains(phases, p => p != 0);
        }
    }
}