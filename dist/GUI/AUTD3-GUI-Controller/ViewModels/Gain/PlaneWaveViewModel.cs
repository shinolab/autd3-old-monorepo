/*
 * File: PlaneWaveViewModel.cs
 * Project: Gain
 * Created Date: 24/08/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/08/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3_GUI_Controller.Contracts.Services;
using AUTD3_GUI_Controller.Models.Gain;
using CommunityToolkit.Mvvm.ComponentModel;

namespace AUTD3_GUI_Controller.ViewModels.Gain;

public class PlaneWaveViewModel : ObservableRecipient
{
    public PlaneWaveModel Model
    {
        get;
        set;
    }

    public PlaneWaveViewModel(ILocalSettingsService localSettingsService)
    {
        Model = localSettingsService.ReadSetting<PlaneWaveModel>(nameof(PlaneWaveModel)) ?? new PlaneWaveModel(0, 0, 1);
        Model.PropertyChanged += async (_, _) => await localSettingsService.SaveSettingAsync(nameof(PlaneWaveModel), Model);
    }
}
