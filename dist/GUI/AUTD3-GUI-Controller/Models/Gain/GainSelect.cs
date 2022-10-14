/*
 * File: GainSelect.cs
 * Project: Gain
 * Created Date: 24/08/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/08/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */


using AUTD3_GUI_Controller.ViewModels.Gain;

namespace AUTD3_GUI_Controller.Models.Gain;

public enum GainSelect
{
    Focus,
    BesselBeam,
    PlaneWave,
    Holo
}

public static class GainSelectExtensions
{
    public static Type GetViewModel(this GainSelect value) => value switch
    {
        GainSelect.Focus => typeof(FocusViewModel),
        GainSelect.BesselBeam => typeof(BesselBeamViewModel),
        GainSelect.PlaneWave => typeof(PlaneWaveViewModel),
        GainSelect.Holo => typeof(HoloViewModel),
        _ => throw new ArgumentOutOfRangeException(nameof(value), value, null)
    };

    public static AUTD3Sharp.Gain.Gain GetGain(this GainSelect value) => value switch
    {
        GainSelect.Focus => App.GetService<FocusViewModel>().Model.ToGain(),
        GainSelect.BesselBeam => App.GetService<BesselBeamViewModel>().Model.ToGain(),
        GainSelect.PlaneWave => App.GetService<PlaneWaveViewModel>().Model.ToGain(),
        GainSelect.Holo => App.GetService<HoloViewModel>().Model.ToGain(),
        _ => throw new ArgumentOutOfRangeException(nameof(value), value, null)
    };
}