/*
 * File: LMTest.cs
 * Project: Holo
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 14/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3Sharp.Gain.Holo;

namespace tests.Gain.Holo;

public class LMTest
{
    [Fact]
    public async Task LM()
    {
        var autd = await new ControllerBuilder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).OpenWithAsync(Audit.Builder());

        var backend = new NalgebraBackend();
        var g = new LM<NalgebraBackend>(backend)
            .AddFocus(autd.Geometry.Center + new Vector3d(30, 0, 150), 0.5)
            .AddFociFromIter(new double[] { -40 }.Select(x => (autd.Geometry.Center + new Vector3d(x, 0, 150), 0.5)))
            .WithEps1(1e-3)
            .WithEps2(1e-3)
            .WithTau(1e-3)
            .WithKMax(5)
            .WithInitial(new[] { 1.0 })
            .WithConstraint(new AUTD3Sharp.Gain.Holo.Uniform(0.5));

        Assert.True(await autd.SendAsync(g));

        foreach (var dev in autd.Geometry)
        {
            var (duties, phases) = autd.Link.DutiesAndPhases(dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(85, d));
            Assert.Contains(phases, p => p != 0);
        }
    }

    [IgnoreIfCUDAIsNotFoundFact]
    public async Task GSPATWithCUDA()
    {
        var autd = await new ControllerBuilder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).OpenWithAsync(Audit.Builder());

        var backend = new CUDABackend();
        var g = new LM<CUDABackend>(backend)
            .AddFocus(autd.Geometry.Center + new Vector3d(30, 0, 150), 0.5)
            .AddFociFromIter(new double[] { -40 }.Select(x => (autd.Geometry.Center + new Vector3d(x, 0, 150), 0.5)))
            .WithEps1(1e-3)
            .WithEps2(1e-3)
            .WithTau(1e-3)
            .WithKMax(5)
            .WithInitial(new[] { 1.0 })
            .WithConstraint(new AUTD3Sharp.Gain.Holo.Uniform(0.5));

        Assert.True(await autd.SendAsync(g));

        foreach (var dev in autd.Geometry)
        {
            var (duties, phases) = autd.Link.DutiesAndPhases(dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(85, d));
            Assert.Contains(phases, p => p != 0);
        }
    }
}