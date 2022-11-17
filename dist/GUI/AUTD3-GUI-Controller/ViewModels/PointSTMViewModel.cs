/*
 * File: PointSTMViewModel.cs
 * Project: ViewModels
 * Created Date: 18/08/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 17/11/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

using System.Collections.ObjectModel;
using AUTD3_GUI_Controller.Contracts.Services;
using AUTD3_GUI_Controller.Models;
using AUTD3_GUI_Controller.Services;
using AUTD3Sharp.Utils;
using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml;

namespace AUTD3_GUI_Controller.ViewModels;

[INotifyPropertyChanged]
public partial class PointSTMViewModel
{
    private const string PointSTMKey = "PointSTM";
    private const string PointSTMFreqKey = "PointSTM_Frequency";

    private readonly AUTDService _autdService;
    private readonly ILocalSettingsService _localSettingsService;

    public XamlRoot? XamlRoot
    {
        get;
        set;
    }

    [RelayCommand]
    public void AddItem()
    {
        var item = new PointSTMTarget(Targets.Count);
        Targets.Add(item);
        Selected = item;
    }

    [RelayCommand(CanExecute = "RemoveItemCanExecute")]
    public void RemoveItem()
    {
        var delNo = Selected!.No;
        Targets.RemoveAt(delNo);
        ResetNo();
        Selected = Targets.Count > delNo ? Targets[delNo] : Targets.Count > 0 ? Targets[delNo - 1] : null;
    }
    private bool RemoveItemCanExecute() => Selected != null;

    [RelayCommand(CanExecute = "UpItemCanExecute")]
    public void UpItem()
    {
        var cNo = Selected!.No;
        Targets.Move(cNo, cNo - 1);
        ResetNo();
        Selected = Targets[cNo]; // This should not be necessary, but if not, selected will not be highlighted...
        Selected = Targets[cNo - 1];
    }
    private bool UpItemCanExecute() => Selected != null && Selected.No != 0;

    [RelayCommand(CanExecute = "DownItemCanExecute")]
    public void DownItem()
    {
        var cNo = Selected!.No;
        Targets.Move(cNo, cNo + 1);
        ResetNo();
        Selected = Targets[cNo]; // This should not be necessary, but if not, selected will not be highlighted...
        Selected = Targets[cNo + 1];
    }
    private bool DownItemCanExecute() => Selected != null && Selected.No != Targets.Count - 1;

    [ObservableProperty] private ObservableCollection<PointSTMTarget> _targets;

    [ObservableProperty]
    private PointSTMTarget? _selected;
    partial void OnSelectedChanged(PointSTMTarget? value)
    {
        RemoveItemCommand.NotifyCanExecuteChanged();
        UpItemCommand.NotifyCanExecuteChanged();
        DownItemCommand.NotifyCanExecuteChanged();
    }

    [ObservableProperty] private double _frequency;
    async partial void OnFrequencyChanged(double value) => await _localSettingsService.SaveSettingAsync(PointSTMFreqKey, value);

    public EventHandler CellEditEnded
    {
        get;
        set;
    }

    [RelayCommand(CanExecute = "SendCanExecute")]
    public async void Send()
    {
        var stm = new AUTD3Sharp.STM.PointSTM(App.GetService<AUTDService>().GetSoundSpeed());
        foreach (var t in Targets)
        {
            stm.Add(new Vector3d(t.X, t.Y, t.Z, t.Shift));
        }
        stm.Frequency = Frequency;

        if (App.GetService<AUTDService>().SendPointSTM(stm))
        {
            App.GetService<ShellViewModel>().StartCommand.NotifyCanExecuteChanged();
            App.GetService<ShellViewModel>().PauseCommand.NotifyCanExecuteChanged();
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

    public PointSTMViewModel(ILocalSettingsService localSettingsService)
    {
        _autdService = App.GetService<AUTDService>();
        _localSettingsService = localSettingsService;
        _targets = new ObservableCollection<PointSTMTarget>(_localSettingsService.ReadSetting<PointSTMTarget[]>(PointSTMKey) ?? Array.Empty<PointSTMTarget>());
        _frequency = _localSettingsService.ReadSetting<double?>(PointSTMFreqKey) ?? 1;
        Targets.CollectionChanged += async (_, _) => await Save();
        CellEditEnded += async (_, _) => await Save();
        _selected = null;
    }

    private void ResetNo()
    {
        foreach (var (item, i) in Targets.Select((x, i) => (x, i)))
        {
            item.No = i;
        }
    }

    private async Task Save()
    {
        await _localSettingsService.SaveSettingAsync(PointSTMKey, Targets.ToArray());
    }
}
