/*
 * File: ModulationPage.xaml.cs
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
using AUTD3_GUI_Controller.Models.Modulation;
using AUTD3_GUI_Controller.ViewModels;
using Microsoft.UI.Xaml;

namespace AUTD3_GUI_Controller.Views;

public sealed partial class ModulationPage
{
    private readonly ILocalizer _localizer;

    public ModulationViewModel ViewModel
    {
        get;
    }

    public ModulationPage()
    {
        ViewModel = App.GetService<ModulationViewModel>();
        InitializeComponent();

        ViewModel.NavigationService.Frame = ModulationNavigationFrame;
        ViewModel.NavigationViewService.Initialize(ModulationNavigationViewControl);

        _localizer = App.GetService<ILocalizer>();
    }

    private void ModulationPage_OnLoaded(object sender, RoutedEventArgs e)
    {
        ViewModel.XamlRoot = Root.XamlRoot;
        _localizer.RunLocalization(Root);
        App.GetService<IModulationNavigationService>().NavigateTo(ViewModel.ModulationSelect.GetViewModel().FullName!);
    }
}
