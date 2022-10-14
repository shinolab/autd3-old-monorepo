/*
 * File: FocusPage.xaml.cs
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

using AK.Toolkit.WinUI3.Localization;
using AUTD3_GUI_Controller.ViewModels.Gain;
using Microsoft.UI.Xaml;

namespace AUTD3_GUI_Controller.Views.Gain;

public sealed partial class FocusPage
{
    private readonly ILocalizer _localizer;

    public FocusViewModel ViewModel
    {
        get;
    }

    public FocusPage()
    {
        ViewModel = App.GetService<FocusViewModel>();
        InitializeComponent(); _localizer = App.GetService<ILocalizer>();

    }

    private void FocusPage_OnLoaded(object sender, RoutedEventArgs e)
    {
        _localizer.RunLocalization(Root);
    }
}
