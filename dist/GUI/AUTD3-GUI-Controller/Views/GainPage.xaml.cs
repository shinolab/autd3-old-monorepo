/*
 * File: GainPage.xaml.cs
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
using AUTD3_GUI_Controller.Contracts.Services;
using AUTD3_GUI_Controller.Models.Gain;
using AUTD3_GUI_Controller.ViewModels;
using Microsoft.UI.Xaml;

namespace AUTD3_GUI_Controller.Views;

public sealed partial class GainPage
{
    private readonly ILocalizer _localizer;

    public GainViewModel ViewModel
    {
        get;
    }

    public GainPage()
    {
        ViewModel = App.GetService<GainViewModel>();
        InitializeComponent();

        ViewModel.NavigationService.Frame = GainNavigationFrame;
        ViewModel.NavigationViewService.Initialize(GainNavigationViewControl);

        _localizer = App.GetService<ILocalizer>();
    }

    private void GainPage_OnLoaded(object sender, RoutedEventArgs e)
    {
        ViewModel.XamlRoot = Root.XamlRoot;
        _localizer.RunLocalization(Root);
        App.GetService<IGainNavigationService>().NavigateTo(ViewModel.GainSelect.GetViewModel().FullName!);
    }
}
