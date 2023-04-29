/*
 * File: StaticModel.cs
 * Project: Modulation
 * Created Date: 24/08/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/04/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

using CommunityToolkit.Mvvm.ComponentModel;

namespace AUTD3_GUI_Controller.Models.Modulation;

public partial class StaticModel : ObservableObject, IModulation
{
    [ObservableProperty]
    private double _amp;

    public StaticModel(double amp = 1.0)
    {
        Amp = amp;
    }

    public AUTD3Sharp.Modulation.Modulation ToModulation() => new AUTD3Sharp.Modulation.Static(Amp);
}
