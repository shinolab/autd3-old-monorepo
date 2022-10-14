/*
 * File: ModulationSelect.cs
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


using AUTD3_GUI_Controller.ViewModels.Modulation;

namespace AUTD3_GUI_Controller.Models.Modulation;

public enum ModulationSelect
{
    Static,
    Sine,
    SineLegacy,
    SineSquared,
    Square
}

public static class ModulationSelectExtensions
{
    public static Type GetViewModel(this ModulationSelect value) => value switch
    {
        ModulationSelect.Static => typeof(StaticViewModel),
        ModulationSelect.Sine => typeof(SineViewModel),
        ModulationSelect.SineSquared => typeof(SineSquaredViewModel),
        ModulationSelect.SineLegacy => typeof(SineLegacyViewModel),
        ModulationSelect.Square => typeof(SquareViewModel),
        _ => throw new ArgumentOutOfRangeException(nameof(value), value, null)
    };
    public static AUTD3Sharp.Modulation.Modulation GetModulation(this ModulationSelect value) => value switch
    {
        ModulationSelect.Sine => App.GetService<SineViewModel>().Model.ToModulation(),
        ModulationSelect.Static => App.GetService<StaticViewModel>().Model.ToModulation(),
        ModulationSelect.SineSquared => App.GetService<SineSquaredViewModel>().Model.ToModulation(),
        ModulationSelect.SineLegacy => App.GetService<SineLegacyViewModel>().Model.ToModulation(),
        ModulationSelect.Square => App.GetService<SquareViewModel>().Model.ToModulation(),
        _ => throw new ArgumentOutOfRangeException(nameof(value), value, null)
    };
}