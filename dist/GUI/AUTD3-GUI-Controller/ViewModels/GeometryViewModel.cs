/*
 * File: GeometryViewModel.cs
 * Project: ViewModels
 * Created Date: 18/08/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

using System.Collections.ObjectModel;
using AUTD3_GUI_Controller.Contracts.Services;
using AUTD3_GUI_Controller.Models;
using AUTD3Sharp;
using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;

namespace AUTD3_GUI_Controller.ViewModels;

[INotifyPropertyChanged]
public partial class GeometryViewModel
{
    private const string GeometryKey = "Geometry";

    private readonly ILocalSettingsService _localSettingsService;

    [RelayCommand]
    public void AddItem()
    {
        var item = new GeometrySetting(Geometries.Count);
        Geometries.Add(item);
        Selected = item;
    }

    [RelayCommand(CanExecute = "RemoveItemCanExecute")]
    public void RemoveItem()
    {
        var delNo = Selected!.No;
        Geometries.RemoveAt(delNo);
        ResetNo();
        Selected = Geometries.Count > delNo ? Geometries[delNo] : Geometries.Count > 0 ? Geometries[delNo - 1] : null;
    }
    private bool RemoveItemCanExecute() => Selected != null;

    [RelayCommand(CanExecute = "UpItemCanExecute")]
    public void UpItem()
    {
        var cNo = Selected!.No;
        Geometries.Move(cNo, cNo - 1);
        ResetNo();
        Selected = Geometries[cNo]; // This should not be necessary, but if not, selected will not be highlighted...
        Selected = Geometries[cNo - 1];
    }
    private bool UpItemCanExecute() => Selected != null && Selected.No != 0;

    [RelayCommand(CanExecute = "DownItemCanExecute")]
    public void DownItem()
    {
        var cNo = Selected!.No;
        Geometries.Move(cNo, cNo + 1);
        ResetNo();
        Selected = Geometries[cNo]; // This should not be necessary, but if not, selected will not be highlighted...
        Selected = Geometries[cNo + 1];
    }
    private bool DownItemCanExecute() => Selected != null && Selected.No != Geometries.Count - 1;


    [RelayCommand]
    public void View()
    {
        var cnt = new Controller();
        foreach (var geo in Geometries)
            cnt.Geometry.AddDevice(new AUTD3Sharp.Utils.Vector3d(geo.X, geo.Y, geo.Z), new AUTD3Sharp.Utils.Vector3d(
                AngleUnitConverter.Instance.ToRadian(
                geo.RotateZ1),
                AngleUnitConverter.Instance.ToRadian(geo.RotateY),
                AngleUnitConverter.Instance.ToRadian(geo.RotateZ2)));
        new AUTD3Sharp.Extra.GeometryViewer().View(cnt.Geometry);
    }

    [ObservableProperty] private ObservableCollection<GeometrySetting> _geometries;

    [ObservableProperty]
    private GeometrySetting? _selected;

    public EventHandler CellEditEnded
    {
        get;
        set;
    }

    partial void OnSelectedChanged(GeometrySetting? value)
    {
        RemoveItemCommand.NotifyCanExecuteChanged();
        UpItemCommand.NotifyCanExecuteChanged();
        DownItemCommand.NotifyCanExecuteChanged();
    }

    public GeometryViewModel(ILocalSettingsService localSettingsService)
    {
        _localSettingsService = localSettingsService;
        _geometries = new ObservableCollection<GeometrySetting>(_localSettingsService.ReadSetting<GeometrySetting[]>(GeometryKey) ?? Array.Empty<GeometrySetting>());
        Geometries.CollectionChanged += async (_, _) => await Save();
        CellEditEnded += async (_, _) => await Save();
        _selected = null;
    }

    private void ResetNo()
    {
        foreach (var (item, i) in Geometries.Select((x, i) => (x, i)))
        {
            item.No = i;
        }
    }

    private async Task Save()
    {
        await _localSettingsService.SaveSettingAsync(GeometryKey, Geometries.ToArray());
    }
}
