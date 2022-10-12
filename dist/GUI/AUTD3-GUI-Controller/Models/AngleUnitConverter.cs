/*
 * File: AngleUnitConverter.cs
 * Project: Models
 * Created Date: 11/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

namespace AUTD3_GUI_Controller.Models;

internal class AngleUnitConverter
{
    public AngleUnit AngleUnit { get; set; } = AngleUnit.Degree;

    public static AngleUnitConverter Instance { get; } = new();

    private AngleUnitConverter()
    {
    }

    public double ToRadian(double v)
    {
        return AngleUnit == AngleUnit.Radian ? v : v / 180.0 * Math.PI;
    }
}