/*
 * File: HomePage.xaml.cs
 * Project: Views
 * Created Date: 18/08/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/08/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

using AK.Toolkit.WinUI3.Localization;
using AUTD3_GUI_Controller.ViewModels;
using Microsoft.UI.Xaml;

namespace AUTD3_GUI_Controller.Views;

public sealed partial class SilencerPage
{
    private readonly ILocalizer _localizer;

    public SilencerViewModel ViewModel
    {
        get;
    }

    public SilencerPage()
    {
        ViewModel = App.GetService<SilencerViewModel>();
        InitializeComponent();

        _localizer = App.GetService<ILocalizer>();
    }

    private void SilencerPage_OnLoaded(object sender, RoutedEventArgs e)
    {
        _localizer.RunLocalization(Root);
    }
}
