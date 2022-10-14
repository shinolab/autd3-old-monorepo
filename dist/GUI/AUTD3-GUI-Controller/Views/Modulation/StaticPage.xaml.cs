/*
 * File: StaticPage.xaml.cs
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

public sealed partial class StaticPage
{
    private readonly ILocalizer _localizer;

    public StaticViewModel ViewModel
    {
        get;
    }

    public StaticPage()
    {
        ViewModel = App.GetService<StaticViewModel>();
        InitializeComponent(); _localizer = App.GetService<ILocalizer>();

    }

    private void StaticPage_OnLoaded(object sender, RoutedEventArgs e)
    {
        _localizer.RunLocalization(Root);
    }
}
