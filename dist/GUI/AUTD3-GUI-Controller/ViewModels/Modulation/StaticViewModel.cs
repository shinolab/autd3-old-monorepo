/*
 * File: SineViewModel.cs
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

using AUTD3_GUI_Controller.Contracts.Services;
using AUTD3_GUI_Controller.Models.Modulation;
using CommunityToolkit.Mvvm.ComponentModel;

namespace AUTD3_GUI_Controller.ViewModels.Modulation;

public class StaticViewModel : ObservableRecipient
{
    public StaticModel Model
    {
        get;
        set;
    }

    public StaticViewModel(ILocalSettingsService localSettingsService)
    {
        Model = localSettingsService.ReadSetting<StaticModel>(nameof(StaticModel)) ?? new StaticModel();
        Model.PropertyChanged += async (_, _) => await localSettingsService.SaveSettingAsync(nameof(StaticModel), Model);
    }
}
