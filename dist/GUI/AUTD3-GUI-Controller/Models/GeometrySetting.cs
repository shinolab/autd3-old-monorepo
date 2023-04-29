﻿/*
 * File: GeometrySetting.cs
 * Project: Models
 * Created Date: 19/08/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/04/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

using CommunityToolkit.Mvvm.ComponentModel;

namespace AUTD3_GUI_Controller.Models;

public partial class GeometrySetting : ObservableObject
{
    [ObservableProperty]
    private int _no;
    [ObservableProperty]
    private double _x;
    [ObservableProperty]
    private double _y;
    [ObservableProperty]
    private double _z;
    [ObservableProperty]
    private double _rotateZ1;
    [ObservableProperty]
    private double _rotateY;
    [ObservableProperty]
    private double _rotateZ2;

    public GeometrySetting(int id)
    {
        _no = id;
    }
}
