/*
 * File: SinePage.xaml.cs
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


using AK.Toolkit.WinUI3.Localization;
using AUTD3_GUI_Controller.ViewModels.Modulation;
using Microsoft.UI.Xaml;

namespace AUTD3_GUI_Controller.Views.Modulation;

public sealed partial class SinePage
{
    private readonly ILocalizer _localizer;
    public SineViewModel ViewModel
    {
        get;
    }

    public SinePage()
    {
        ViewModel = App.GetService<SineViewModel>();
        InitializeComponent(); _localizer = App.GetService<ILocalizer>();

    }

    private void SinePage_OnLoaded(object sender, RoutedEventArgs e)
    {
        _localizer.RunLocalization(Root);
    }
}
