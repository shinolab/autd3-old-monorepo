/*
 * File: SineViewModel.cs
 * Project: Modulation
 * Created Date: 24/08/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/08/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3_GUI_Controller.Contracts.Services;
using AUTD3_GUI_Controller.Models.Modulation;
using CommunityToolkit.Mvvm.ComponentModel;

namespace AUTD3_GUI_Controller.ViewModels.Modulation;

public class SineViewModel : ObservableRecipient
{
    public SineModel Model
    {
        get;
        set;
    }

    public SineViewModel(ILocalSettingsService localSettingsService)
    {
        Model = localSettingsService.ReadSetting<SineModel>(nameof(SineModel)) ?? new SineModel(150);
        Model.PropertyChanged += async (_, _) => await localSettingsService.SaveSettingAsync(nameof(SineModel), Model);
    }
}
