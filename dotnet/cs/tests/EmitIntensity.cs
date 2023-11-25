/*
 * File: EmitIntensity.cs
 * Project: tests
 * Created Date: 25/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */


namespace tests;

public class EmitIntensityTest
{
    [Fact]
    public void EmitIntensityNew()
    {
        for (int i = 0; i <= 0xFF; i++)
        {
            var intensity = new EmitIntensity((byte)i);
            Assert.Equal(i, intensity.Value);
        }
    }

    [Fact]
    public void EmitIntensityWithCorrection()
    {
        for (int i = 0; i <= 0xFF; i++)
        {
            var intensity = EmitIntensity.NewWithCorrection((byte)i);
            Assert.Equal((int)Math.Round(Math.Asin(Math.Pow((double)i / 255.0, 1.0 / EmitIntensity.DefaultCorrectedAlpha)) / Math.PI * 510.0), intensity.Value);
        }
    }
}