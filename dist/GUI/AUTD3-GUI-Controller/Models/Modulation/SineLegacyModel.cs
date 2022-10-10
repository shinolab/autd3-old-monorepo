﻿/*
 * File: SineLegacyModel.cs
 * Project: Modulation
 * Created Date: 24/08/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/08/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

using CommunityToolkit.Mvvm.ComponentModel;

namespace AUTD3_GUI_Controller.Models.Modulation;

[INotifyPropertyChanged]
public partial class SineLegacyModel : IModulation
{
    [ObservableProperty]
    private double _freq;
    [ObservableProperty]
    private double _amp;
    [ObservableProperty]
    private double _offset;

    public SineLegacyModel(double freq, double amp = 1.0, double offset = 0.5)
    {
        Freq = freq;
        Amp = amp;
        Offset = offset;
    }

    public AUTD3Sharp.Modulation.Modulation ToModulation() => new AUTD3Sharp.Modulation.SineLegacy(Freq, Amp, Offset);
}
