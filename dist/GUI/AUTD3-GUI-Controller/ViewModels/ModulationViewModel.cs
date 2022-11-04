/*
 * File: ModulationViewModel.cs
 * Project: ViewModels
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
using AUTD3_GUI_Controller.Services;
using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Navigation;

namespace AUTD3_GUI_Controller.ViewModels;

[INotifyPropertyChanged]
public partial class ModulationViewModel
{
    private readonly AUTDService _autdService;
    private readonly ILocalSettingsService _localSettingsService;

    public XamlRoot? XamlRoot
    {
        get;
        set;
    }

    [ObservableProperty]
    private bool _isBackEnabled;
    [ObservableProperty]
    private object? _selected;

    [ObservableProperty] private ModulationSelect _modulationSelect;
    async partial void OnModulationSelectChanged(ModulationSelect value)
    {
        await _localSettingsService.SaveSettingAsync(nameof(ModulationSelect), value);
    }

    public IModulationNavigationService NavigationService
    {
        get;
    }

    public IModulationNavigationViewService NavigationViewService
    {
        get;
    }

    [RelayCommand(CanExecute = "SendCanExecute")]
    public async void Send()
    {
        if (_autdService.SendModulation(ModulationSelect.GetModulation()))
        {
            return;
        }

        var noWifiDialog = new ContentDialog
        {
            Title = "AUTD internal error",
            Content = AUTDService.GetLastError(),
            CloseButtonText = "Ok",
            XamlRoot = XamlRoot!
        };
        await noWifiDialog.ShowAsync();
    }
    private bool SendCanExecute() => _autdService.IsOpened;
    public ModulationViewModel(IModulationNavigationService navigationService, IModulationNavigationViewService navigationViewService, ILocalSettingsService localSettingsService)
    {
        _autdService = App.GetService<AUTDService>();
        _localSettingsService = localSettingsService;
        NavigationService = navigationService;
        NavigationService.Navigated += OnNavigated;
        NavigationViewService = navigationViewService;

        _modulationSelect = _localSettingsService.ReadSetting<ModulationSelect?>(nameof(ModulationSelect)) ?? ModulationSelect.Sine;
    }

    private void OnNavigated(object sender, NavigationEventArgs e)
    {
        IsBackEnabled = NavigationService.CanGoBack;
        if (e.SourcePageType == null)
        {
            return;
        }
        var selectedItem = NavigationViewService.GetSelectedItem(e.SourcePageType);
        if (selectedItem == null)
        {
            return;
        }

        if (Enum.TryParse<ModulationSelect>(e.SourcePageType.FullName!.Split('.').LastOrDefault()?[..^4] ?? string.Empty, true, out var modulationSelect))
        {
            ModulationSelect = modulationSelect;
        }
        Selected = selectedItem;
    }
}
