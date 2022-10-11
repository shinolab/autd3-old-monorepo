/*
 * File: BesselBeamViewModel.cs
 * Project: Gain
 * Created Date: 24/08/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3_GUI_Controller.Contracts.Services;
using AUTD3_GUI_Controller.Models.Gain;
using CommunityToolkit.Mvvm.ComponentModel;

namespace AUTD3_GUI_Controller.ViewModels.Gain;

public class BesselBeamViewModel : ObservableRecipient
{
    public BesselBeamModel Model
    {
        get;
        set;
    }

    public BesselBeamViewModel(ILocalSettingsService localSettingsService)
    {
        Model = localSettingsService.ReadSetting<BesselBeamModel>(nameof(BesselBeamModel)) ?? new BesselBeamModel(90, 70, 0, 0, 0, 1, 18.0);
        Model.PropertyChanged += async (_, _) => await localSettingsService.SaveSettingAsync(nameof(BesselBeamModel), Model);
    }
}
