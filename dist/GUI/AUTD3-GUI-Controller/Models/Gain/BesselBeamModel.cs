/*
 * File: BesselBeamModel.cs
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

using AUTD3Sharp.Utils;
using CommunityToolkit.Mvvm.ComponentModel;

namespace AUTD3_GUI_Controller.Models.Gain;

public partial class BesselBeamModel : ObservableObject, IGain
{
    [ObservableProperty]
    private double _x;
    [ObservableProperty]
    private double _y;
    [ObservableProperty]
    private double _z;
    [ObservableProperty]
    private double _dirX;
    [ObservableProperty]
    private double _dirY;
    [ObservableProperty]
    private double _dirZ;
    [ObservableProperty]
    private double _theta;
    [ObservableProperty]
    private double _amp;

    public BesselBeamModel(double x, double y, double z, double dx, double dy, double dz, double theta, double amp = 1.0)
    {
        X = x;
        Y = y;
        Z = z;
        DirX = dx;
        DirY = dy;
        DirZ = dz;
        Theta = theta;
        Amp = amp;
    }

    public AUTD3Sharp.Gain.Gain ToGain() => new AUTD3Sharp.Gain.BesselBeam(new Vector3d(X, Y, Z), new Vector3d(DirX, DirY, DirZ), AngleUnitConverter.Instance.ToRadian(Theta), Amp);
}
