/*
 * File: SquareModel.cs
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
public partial class SquareModel : IModulation
{
    [ObservableProperty]
    private int _freq;
    [ObservableProperty]
    private double _high;
    [ObservableProperty]
    private double _low;
    [ObservableProperty]
    private double _duty;
    public SquareModel(int freq, double high = 1.0, double low = 0.0, double duty = 0.5)
    {
        Freq = freq;
        High = high;
        Low = low;
        Duty = duty;
    }

    public AUTD3Sharp.Modulation.Modulation ToModulation() => new AUTD3Sharp.Modulation.Square(Freq, Low, High, Duty);
}
