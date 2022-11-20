/*
 * File: GainViewModel.cs
 * Project: ViewModels
 * Created Date: 24/08/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/11/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3_GUI_Controller.Contracts.Services;
using AUTD3_GUI_Controller.Models.Gain;
using AUTD3_GUI_Controller.Services;
using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Navigation;

namespace AUTD3_GUI_Controller.ViewModels;

[INotifyPropertyChanged]
public partial class GainViewModel
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

    [ObservableProperty] private GainSelect _gainSelect;
    async partial void OnGainSelectChanged(GainSelect value) => await _localSettingsService.SaveSettingAsync(nameof(GainSelect), value);

    [RelayCommand(CanExecute = "SendCanExecute")]
    public async void Send()
    {
        if (_autdService.SendGain(GainSelect.GetGain()))
        {
            App.GetService<ShellViewModel>().StartCommand.NotifyCanExecuteChanged();
            App.GetService<ShellViewModel>().PauseCommand.NotifyCanExecuteChanged();
            return;
        }

        var noWifiDialog = new ContentDialog
        {
            Title = "AUTD internal error",
            Content = "Failed to send data",
            CloseButtonText = "Ok",
            XamlRoot = XamlRoot!
        };
        await noWifiDialog.ShowAsync();
    }
    private bool SendCanExecute() => _autdService.IsOpened;


    public IGainNavigationService NavigationService
    {
        get;
    }

    public IGainNavigationViewService NavigationViewService
    {
        get;
    }

    public GainViewModel(IGainNavigationService navigationService, IGainNavigationViewService navigationViewService, ILocalSettingsService localSettingsService)
    {
        _autdService = App.GetService<AUTDService>();
        _localSettingsService = localSettingsService;
        NavigationService = navigationService;
        NavigationService.Navigated += OnNavigated;
        NavigationViewService = navigationViewService;

        _gainSelect = _localSettingsService.ReadSetting<GainSelect?>(nameof(GainSelect)) ?? GainSelect.Focus;
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

        if (Enum.TryParse<GainSelect>(e.SourcePageType.FullName!.Split('.').LastOrDefault()?[..^4] ?? string.Empty, true, out var gainSelect))
        {
            GainSelect = gainSelect;
        }
        Selected = selectedItem;
    }
}
