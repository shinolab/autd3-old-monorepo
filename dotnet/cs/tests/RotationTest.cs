/*
 * File: RotationTest.cs
 * Project: tests
 * Created Date: 26/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 27/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

namespace tests;

using static Angle.Units;

public class RotationTest
{
    [Fact]
    public void AngleTest()
    {

        var angle = 90 * Deg;
        Assert.Equal(Math.PI / 2, angle.Radian);

        angle = Math.PI / 2 * Rad;
        Assert.Equal(Math.PI / 2, angle.Radian);
    }

    [Fact]
    public async Task WithRotation()
    {
        {
            var autd = await Open(EulerAngles.FromZYZ(90 * Deg, 0 * Deg, 0 * Deg));
            AssertNearVec3(Vector3d.UnitY, autd.Geometry[0][0].XDirection);
            AssertNearVec3(-Vector3d.UnitX, autd.Geometry[0][0].YDirection);
            AssertNearVec3(Vector3d.UnitZ, autd.Geometry[0][0].ZDirection);
        }
        {
            var autd = await Open(EulerAngles.FromZYZ(0 * Deg, 90 * Deg, 0 * Deg));
            AssertNearVec3(-Vector3d.UnitZ, autd.Geometry[0][0].XDirection);
            AssertNearVec3(Vector3d.UnitY, autd.Geometry[0][0].YDirection);
            AssertNearVec3(Vector3d.UnitX, autd.Geometry[0][0].ZDirection);
        }
        {
            var autd = await Open(EulerAngles.FromZYZ(0 * Deg, 0 * Deg, 90 * Deg));
            AssertNearVec3(Vector3d.UnitY, autd.Geometry[0][0].XDirection);
            AssertNearVec3(-Vector3d.UnitX, autd.Geometry[0][0].YDirection);
            AssertNearVec3(Vector3d.UnitZ, autd.Geometry[0][0].ZDirection);
        }
        {
            var autd = await Open(EulerAngles.FromZYZ(0 * Deg, 90 * Deg, 90 * Deg));
            AssertNearVec3(Vector3d.UnitY, autd.Geometry[0][0].XDirection);
            AssertNearVec3(Vector3d.UnitZ, autd.Geometry[0][0].YDirection);
            AssertNearVec3(Vector3d.UnitX, autd.Geometry[0][0].ZDirection);
        }
        {
            var autd = await Open(EulerAngles.FromZYZ(90 * Deg, 90 * Deg, 0 * Deg));
            AssertNearVec3(-Vector3d.UnitZ, autd.Geometry[0][0].XDirection);
            AssertNearVec3(-Vector3d.UnitX, autd.Geometry[0][0].YDirection);
            AssertNearVec3(Vector3d.UnitY, autd.Geometry[0][0].ZDirection);
        }
        return;

        async Task<Controller<Audit>> Open(Quaterniond q) =>
            await new ControllerBuilder().AddDevice(new AUTD3(Vector3d.zero).WithRotation(q))
                .OpenWithAsync(Audit.Builder());

        void AssertNearVec3(Vector3d expected, Vector3d x)
        {
            Assert.True(Math.Abs(expected.x - x.x) < 1e-6);
            Assert.True(Math.Abs(expected.y - x.y) < 1e-6);
            Assert.True(Math.Abs(expected.z - x.z) < 1e-6);
        }
    }
}
