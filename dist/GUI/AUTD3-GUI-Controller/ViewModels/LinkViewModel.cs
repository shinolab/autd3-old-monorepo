/*
 * File: LinkViewModel.cs
 * Project: ViewModels
 * Created Date: 18/08/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

using System.Collections.ObjectModel;
using AUTD3_GUI_Controller.Contracts.Services;
using AUTD3_GUI_Controller.Helpers;
using AUTD3_GUI_Controller.Models;
using AUTD3_GUI_Controller.Services;
using AUTD3Sharp.Link;
using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;

namespace AUTD3_GUI_Controller.ViewModels;

[INotifyPropertyChanged]
public partial class LinkViewModel
{
    private readonly ILocalSettingsService _localSettingsService;
    private readonly AUTDService _autdService;

    public XamlRoot? XamlRoot
    {
        get;
        set;
    }

    [ObservableProperty] private LinkType _selected;

    async partial void OnSelectedChanged(LinkType value) => await _localSettingsService.SaveSettingAsync(nameof(Selected), value);


    [ObservableProperty] private ObservableCollection<string> _interfaces;

    [ObservableProperty] private string _interfaceName;

    async partial void OnInterfaceNameChanged(string value) => await _localSettingsService.SaveSettingAsync(nameof(InterfaceName), value);

    [RelayCommand]
    private void UpdateInterfaces()
    {
        var current = _interfaceName;
        Interfaces.Clear();
        Interfaces.Add("SOEM_Link_AUTO".GetLocalized());
        foreach (var adapter in SOEM.EnumerateAdapters())
        {
            Interfaces.Add($"{adapter.Desc}, {adapter.Name}");
        }
        InterfaceName = Interfaces.Contains(current) ? current : "SOEM_Link_AUTO".GetLocalized();
    }

    [ObservableProperty]
    private bool _highPrecision;
    async partial void OnHighPrecisionChanged(bool value) => await _localSettingsService.SaveSettingAsync(nameof(HighPrecision), value);

    [ObservableProperty]
    private bool _freeRun;
    async partial void OnFreeRunChanged(bool value) => await _localSettingsService.SaveSettingAsync(nameof(FreeRun), value);

    [ObservableProperty]
    private ushort _sendCycle;
    async partial void OnSendCycleChanged(ushort value) => await _localSettingsService.SaveSettingAsync(nameof(SendCycle), value);

    [ObservableProperty]
    private ushort _sync0Cycle;
    async partial void OnSync0CycleChanged(ushort value) => await _localSettingsService.SaveSettingAsync(nameof(Sync0Cycle), value);

    [ObservableProperty]
    private string _remoteIp;
    async partial void OnRemoteIpChanged(string value) => await _localSettingsService.SaveSettingAsync(nameof(RemoteIp), value);

    [ObservableProperty]
    private string _remoteAmsNetId;
    async partial void OnRemoteAmsNetIdChanged(string value) => await _localSettingsService.SaveSettingAsync(nameof(RemoteAmsNetId), value);

    [ObservableProperty]
    private string _localAmsNetId;
    async partial void OnLocalAmsNetIdChanged(string value) => await _localSettingsService.SaveSettingAsync(nameof(LocalAmsNetId), value);

    [ObservableProperty]
    private int _checkTrials;
    async partial void OnCheckTrialsChanged(int value) => await _localSettingsService.SaveSettingAsync(nameof(CheckTrials), value);

    [ObservableProperty]
    private int _sendIntervals;
    async partial void OnSendIntervalsChanged(int value) => await _localSettingsService.SaveSettingAsync(nameof(SendIntervals), value);


    [RelayCommand(CanExecute = "OpenCanExecute")]
    public async void Open()
    {
        if (!_autdService.Open())
        {
            var noWifiDialog = new ContentDialog
            {
                Title = "AUTD internal error",
                Content = _autdService.GetLastError(),
                CloseButtonText = "Ok",
                XamlRoot = XamlRoot!
            };
            await noWifiDialog.ShowAsync();

        }
        OpenCommand.NotifyCanExecuteChanged();
        CloseCommand.NotifyCanExecuteChanged();
    }
    private bool OpenCanExecute() => !IsOpened;

    [RelayCommand(CanExecute = "CloseCanExecute")]
    public async void Close()
    {
        if (!_autdService.Close())
        {
            var noWifiDialog = new ContentDialog
            {
                Title = "AUTD internal error",
                Content = _autdService.GetLastError(),
                CloseButtonText = "Ok",
                XamlRoot = XamlRoot!
            };
            await noWifiDialog.ShowAsync();
        }
        OpenCommand.NotifyCanExecuteChanged();
        CloseCommand.NotifyCanExecuteChanged();
    }
    private bool CloseCanExecute() => IsOpened;

    private bool IsOpened => _autdService.IsOpened;

    [RelayCommand]
    public async void RunSimulator()
    {
        await Task.Run(() =>
          {
              new AUTD3Sharp.Extra.Simulator().SettingsPath("simulator_settings.json").Run();
          });
    }


    public LinkViewModel(ILocalSettingsService localSettingsService)
    {
        _autdService = App.GetService<AUTDService>();
        _localSettingsService = localSettingsService;

        XamlRoot = null;

        Selected = _localSettingsService.ReadSetting<LinkType?>(nameof(Selected)) ?? LinkType.SOEM;

        _interfaces = new ObservableCollection<string>();
        _interfaceName = _localSettingsService.ReadSetting<string>(nameof(InterfaceName)) ?? "";
        UpdateInterfaces();

        _highPrecision = _localSettingsService.ReadSetting<bool?>(nameof(HighPrecision)) ?? true;
        _freeRun = _localSettingsService.ReadSetting<bool?>(nameof(FreeRun)) ?? false;
        _sendCycle = _localSettingsService.ReadSetting<ushort?>(nameof(SendCycle)) ?? 1;
        _sync0Cycle = _localSettingsService.ReadSetting<ushort?>(nameof(Sync0Cycle)) ?? 1;

        _remoteIp = _localSettingsService.ReadSetting<string>(nameof(RemoteIp)) ?? "";
        _remoteAmsNetId = _localSettingsService.ReadSetting<string>(nameof(RemoteAmsNetId)) ?? "";
        _localAmsNetId = _localSettingsService.ReadSetting<string>(nameof(LocalAmsNetId)) ?? "";

        _checkTrials = _localSettingsService.ReadSetting<int?>(nameof(CheckTrials)) ?? 0;
        _sendIntervals = _localSettingsService.ReadSetting<int?>(nameof(SendIntervals)) ?? 1;
    }
}
