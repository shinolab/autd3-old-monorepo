/*
 * File: HoloModel.cs
 * Project: Gain
 * Created Date: 25/08/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/08/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */


using System.Collections.ObjectModel;
using AUTD3Sharp;
using AUTD3Sharp.Gain.Holo;
using AUTD3Sharp.Utils;
using CommunityToolkit.Mvvm.ComponentModel;

namespace AUTD3_GUI_Controller.Models.Gain;

public enum OptMethod
{
    SDP,
    EVD,
    Naive,
    GS,
    GSPAT,
    LM,
    Greedy,
    LSSGreedy,
    APO
}

public enum Constraint
{
    Normalize,
    Uniform,
    Clamp,
    DontCare
}

[INotifyPropertyChanged]
public partial class Target
{
    [ObservableProperty] private int _no;
    [ObservableProperty] private double _x;
    [ObservableProperty] private double _y;
    [ObservableProperty] private double _z;
    [ObservableProperty] private double _amp;

    public Target(int no)
    {
        No = no;
    }
}

[INotifyPropertyChanged]
public partial class HoloModel : IGain
{
    [ObservableProperty] private OptMethod _opt;
    [ObservableProperty] private Constraint _constraint;
    [ObservableProperty] private double _uniformAmp = 1;

    [ObservableProperty] private ObservableCollection<Target> _targets;

    [ObservableProperty] private double _SDPAlpha = 1e-3;
    [ObservableProperty] private ulong _SDPRepeat = 100;
    [ObservableProperty] private double _SDPLambda = 0.9;

    [ObservableProperty] private double _LMEps1 = 1e-2;
    [ObservableProperty] private double _LMEps2 = 1e-2;
    [ObservableProperty] private ulong _LMKMax = 5;
    [ObservableProperty] private double _LMTau = 1e-3;

    [ObservableProperty] private double _APOEps = 1e-2;
    [ObservableProperty] private double _APOLambda = 1;
    [ObservableProperty] private int _APOKMax = 200;
    [ObservableProperty] private int _APOLineSearchMax = 100;

    [ObservableProperty] private double _EVDGamma = 1.0;
    [ObservableProperty] private uint _GSRepeat = 100;
    [ObservableProperty] private uint _GSPATRepeat = 100;
    [ObservableProperty] private int _GreedyPhaseDiv = 16;
    [ObservableProperty] private int _LSSGreedyPhaseDiv = 16;


    private readonly Backend _backend;

    public HoloModel()
    {
        _targets = new ObservableCollection<Target>();
        _backend = new BackendEigen();
    }

    public AUTD3Sharp.Gain.Gain ToGain()
    {
        Holo gain = _opt switch
        {
            OptMethod.SDP => new SDP(SDPAlpha, SDPLambda, SDPRepeat),
            OptMethod.EVD => new EVD(EVDGamma),
            OptMethod.Naive => new Naive(),
            OptMethod.GS => new GS(GSRepeat),
            OptMethod.GSPAT => new GSPAT(GSPATRepeat),
            OptMethod.LM => new LM(LMEps1 * 1e-6, LMEps2 * 1e-6, LMTau, LMKMax),
            OptMethod.Greedy => new Greedy(GreedyPhaseDiv),
            OptMethod.LSSGreedy => new LSSGreedy(LSSGreedyPhaseDiv),
            OptMethod.APO => new APO(APOEps * 1e-6, APOLambda, APOKMax, APOLineSearchMax),
            _ => throw new ArgumentOutOfRangeException()
        };
        gain.Backend = _backend;
        gain.Constraint = _constraint switch
        {
            Constraint.Normalize => new Normalize(),
            Constraint.Uniform => new Uniform(UniformAmp),
            Constraint.Clamp => new Clamp(),
            Constraint.DontCare => new DontCare(),
            _ => throw new ArgumentOutOfRangeException()
        };
        foreach (var t in _targets)
        {
            gain.Add(new Vector3d(t.X, t.Y, t.Z), t.Amp);
        }
        return gain;
    }
}
