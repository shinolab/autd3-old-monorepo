/*
 * File: PlaneWaveModel.cs
 * Project: Gain
 * Created Date: 25/08/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/04/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3Sharp.Utils;
using CommunityToolkit.Mvvm.ComponentModel;

namespace AUTD3_GUI_Controller.Models.Gain;

public partial class PlaneWaveModel : ObservableObject, IGain
{
    [ObservableProperty]
    private double _dirX;
    [ObservableProperty]
    private double _dirY;
    [ObservableProperty]
    private double _dirZ;
    [ObservableProperty]
    private double _amp;

    public PlaneWaveModel(double dx, double dy, double dz, double amp = 1.0)
    {
        DirX = dx;
        DirY = dy;
        DirZ = dz;
        Amp = amp;
    }

    public AUTD3Sharp.Gain.Gain ToGain() => new AUTD3Sharp.Gain.PlaneWave(new Vector3d(DirX, DirY, DirZ), Amp);
}
