/*
 * File: SquareViewModel.cs
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

public class SquareViewModel : ObservableRecipient
{
    public SquareModel Model
    {
        get;
        set;
    }

    public SquareViewModel(ILocalSettingsService localSettingsService)
    {
        Model = localSettingsService.ReadSetting<SquareModel>(nameof(SquareModel)) ?? new SquareModel(150);
        Model.PropertyChanged += async (_, _) => await localSettingsService.SaveSettingAsync(nameof(SquareModel), Model);
    }
}
