/*
 * File: RotationTest.cs
 * Project: tests
 * Created Date: 26/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

namespace tests;

public class RotationTest
{
    [Fact]
    public void AngleTest()
    {
        var angle = Angle.FromDegree(90);
        Assert.Equal(Math.PI / 2, angle.Radian);

        angle = Angle.FromRadian(Math.PI / 2);
        Assert.Equal(Math.PI / 2, angle.Radian);
    }

    [Fact]
    public async Task WithRotation()
    {
        var open = async (Quaterniond q) =>
            await new ControllerBuilder().AddDevice(new AUTD3(Vector3d.zero).WithRotation(q))
                .OpenWithAsync(Audit.Builder());

        var assert_near_vec3 = (Vector3d expected, Vector3d x) =>
        {
            Assert.True(Math.Abs(expected.x - x.x) < 1e-6);
            Assert.True(Math.Abs(expected.y - x.y) < 1e-6);
            Assert.True(Math.Abs(expected.z - x.z) < 1e-6);
        };

        {
            var autd = await open(EulerAngles.FromZYZ(Angle.FromDegree(90), Angle.FromDegree(0),
                Angle.FromDegree(0)));
            assert_near_vec3(Vector3d.UnitY, autd.Geometry[0][0].XDirection);
            assert_near_vec3(-Vector3d.UnitX, autd.Geometry[0][0].YDirection);
            assert_near_vec3(Vector3d.UnitZ, autd.Geometry[0][0].ZDirection);
        }
        {
            var autd = await open(EulerAngles.FromZYZ(Angle.FromDegree(0), Angle.FromDegree(90),
                Angle.FromDegree(0)));
            assert_near_vec3(-Vector3d.UnitZ, autd.Geometry[0][0].XDirection);
            assert_near_vec3(Vector3d.UnitY, autd.Geometry[0][0].YDirection);
            assert_near_vec3(Vector3d.UnitX, autd.Geometry[0][0].ZDirection);
        }
        {
            var autd = await open(EulerAngles.FromZYZ(Angle.FromDegree(0), Angle.FromDegree(0),
                Angle.FromDegree(90)));
            assert_near_vec3(Vector3d.UnitY, autd.Geometry[0][0].XDirection);
            assert_near_vec3(-Vector3d.UnitX, autd.Geometry[0][0].YDirection);
            assert_near_vec3(Vector3d.UnitZ, autd.Geometry[0][0].ZDirection);
        }
        {
            var autd = await open(EulerAngles.FromZYZ(Angle.FromDegree(0), Angle.FromDegree(90),
                Angle.FromDegree(90)));
            assert_near_vec3(Vector3d.UnitY, autd.Geometry[0][0].XDirection);
            assert_near_vec3(Vector3d.UnitZ, autd.Geometry[0][0].YDirection);
            assert_near_vec3(Vector3d.UnitX, autd.Geometry[0][0].ZDirection);
        }
        {
            var autd = await open(EulerAngles.FromZYZ(Angle.FromDegree(90), Angle.FromDegree(90),
                Angle.FromDegree(0)));
            assert_near_vec3(-Vector3d.UnitZ, autd.Geometry[0][0].XDirection);
            assert_near_vec3(-Vector3d.UnitX, autd.Geometry[0][0].YDirection);
            assert_near_vec3(Vector3d.UnitY, autd.Geometry[0][0].ZDirection);
        }
    }
}
