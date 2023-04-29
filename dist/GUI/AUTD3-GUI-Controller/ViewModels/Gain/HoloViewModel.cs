/*
 * File: PlaneWaveViewModel.cs
 * Project: Gain
 * Created Date: 24/08/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/04/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3_GUI_Controller.Contracts.Services;
using AUTD3_GUI_Controller.Models.Gain;
using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;

namespace AUTD3_GUI_Controller.ViewModels.Gain;

public partial class HoloViewModel : ObservableObject
{
    public HoloModel Model
    {
        get;
        set;
    }

    [RelayCommand]
    public void AddItem()
    {
        var item = new Target(Model.Targets.Count);
        Model.Targets.Add(item);
        Selected = item;
    }

    [RelayCommand(CanExecute = "RemoveItemCanExecute")]
    public void RemoveItem()
    {
        var delNo = Selected!.No;
        Model.Targets.RemoveAt(delNo);
        ResetNo();
        Selected = Model.Targets.Count > delNo ? Model.Targets[delNo] : Model.Targets.Count > 0 ? Model.Targets[delNo - 1] : null;
    }
    private bool RemoveItemCanExecute() => Selected != null;

    [RelayCommand(CanExecute = "UpItemCanExecute")]
    public void UpItem()
    {
        var cNo = Selected!.No;
        Model.Targets.Move(cNo, cNo - 1);
        ResetNo();
        Selected = Model.Targets[cNo]; // This should not be necessary, but if not, selected will not be highlighted...
        Selected = Model.Targets[cNo - 1];
    }
    private bool UpItemCanExecute() => Selected != null && Selected.No != 0;

    [RelayCommand(CanExecute = "DownItemCanExecute")]
    public void DownItem()
    {
        var cNo = Selected!.No;
        Model.Targets.Move(cNo, cNo + 1);
        ResetNo();
        Selected = Model.Targets[cNo]; // This should not be necessary, but if not, selected will not be highlighted...
        Selected = Model.Targets[cNo + 1];
    }
    private bool DownItemCanExecute() => Selected != null && Selected.No != Model.Targets.Count - 1;

    public EventHandler CellEditEnded
    {
        get;
        set;
    }

    [ObservableProperty]
    private Target? _selected;
    partial void OnSelectedChanged(Target? value)
    {
        RemoveItemCommand.NotifyCanExecuteChanged();
        UpItemCommand.NotifyCanExecuteChanged();
        DownItemCommand.NotifyCanExecuteChanged();
    }

    public HoloViewModel(ILocalSettingsService localSettingsService)
    {
        Model = localSettingsService.ReadSetting<HoloModel>(nameof(HoloModel)) ?? new HoloModel();
        Model.PropertyChanged += async (_, _) => await localSettingsService.SaveSettingAsync(nameof(HoloModel), Model);

        Model.Targets.CollectionChanged += async (_, _) => await localSettingsService.SaveSettingAsync(nameof(HoloModel), Model);
        CellEditEnded += async (_, _) => await localSettingsService.SaveSettingAsync(nameof(HoloModel), Model);
    }

    private void ResetNo()
    {
        foreach (var (item, i) in Model.Targets.Select((x, i) => (x, i)))
        {
            item.No = i;
        }
    }

}
