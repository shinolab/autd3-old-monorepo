/*
 * File: FocusViewModel.cs
 * Project: Gain
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
using AUTD3_GUI_Controller.Models.Gain;
using CommunityToolkit.Mvvm.ComponentModel;

namespace AUTD3_GUI_Controller.ViewModels.Gain;

public class FocusViewModel : ObservableRecipient
{
    public FocusModel Model
    {
        get;
        set;
    }

    public FocusViewModel(ILocalSettingsService localSettingsService)
    {
        Model = localSettingsService.ReadSetting<FocusModel>(nameof(FocusModel)) ?? new FocusModel(90, 70, 150);
        Model.PropertyChanged += async (_, _) => await localSettingsService.SaveSettingAsync(nameof(FocusModel), Model);
    }
}
