/*
 * File: AUTDService.cs
 * Project: Services
 * Created Date: 23/08/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/11/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */


using AUTD3_GUI_Controller.Helpers;
using AUTD3_GUI_Controller.Models;
using AUTD3_GUI_Controller.ViewModels;
using AUTD3Sharp;
using AUTD3Sharp.Link;
using AUTD3Sharp.Modulation;
using AUTD3Sharp.STM;
using AUTD3Sharp.Utils;
using Gain = AUTD3Sharp.Gain.Gain;

namespace AUTD3_GUI_Controller.Services;

public class AUTDService
{
    private Controller? _autd;

    private Body? _lastBody;

    public bool IsOpened => _autd?.IsOpen ?? false;


    public bool IsStarted
    {
        get;
        private set;
    }
    public bool Open()
    {
        _autd?.Close();
        _autd?.Dispose();
        _autd = new Controller();
        foreach (var geometry in App.GetService<GeometryViewModel>().Geometries)
        {
            _autd.Geometry.AddDevice(new Vector3d(geometry.X, geometry.Y, geometry.Z), new Vector3d(AngleUnitConverter.Instance.ToRadian(geometry.RotateZ1), AngleUnitConverter.Instance.ToRadian(geometry.RotateY), AngleUnitConverter.Instance.ToRadian(geometry.RotateZ2)));
        }

        var linkVm = App.GetService<LinkViewModel>();
        Link BuildSOEM()
        {
            var soem = new SOEM().FreeRun(linkVm.FreeRun)
                .HighPrecision(linkVm.HighPrecision)
                .SendCycle(linkVm.SendCycle)
                .Sync0Cycle(linkVm.Sync0Cycle)
                .CheckInterval(linkVm.CheckInterval);
            if (linkVm.InterfaceName != "SOEM_Link_AUTO".GetLocalized())
            {
                soem = soem.Ifname(linkVm.InterfaceName);
            }

            return soem.Build();
        }
        var link = linkVm.Selected switch
        {
            LinkType.SOEM => BuildSOEM(),
            LinkType.TwinCAT => new TwinCAT().Build(),
            LinkType.RemoteTwinCAT => new RemoteTwinCAT(linkVm.RemoteAmsNetId).RemoteIp(linkVm.RemoteIp).LocalAmsNetId(linkVm.LocalAmsNetId).Build(),
            LinkType.Simulator => new Simulator().Build(),
            LinkType.RemoteSOEM => new RemoteSOEM().Ip(linkVm.RemoteSOEMIp).Port(linkVm.RemoteSOEMPort).Build(),
            _ => throw new ArgumentOutOfRangeException()
        };

        if (!_autd.Open(link))
            return false;

        if (!_autd.Send(new Clear()))
            return false;

        if (!_autd.Send(new Synchronize()))
            return false;

        _autd.AckCheckTimeoutMs = linkVm.AckCheckTimeoutMs;
        _autd.SendIntervalsMs = linkVm.SendIntervalsMs;

        if (!_autd.Send(new Static()))
            return false;

        _lastBody = null;
        IsStarted = false;

        return true;
    }

    public bool ConfigSilencer(ushort step, ushort cycle)
    {
        return _autd?.Send(new SilencerConfig(step, cycle)) ?? false;
    }

    public bool SendGain(Gain gain)
    {
        IsStarted = true;
        _lastBody = gain;
        return _autd?.Send(gain) ?? false;
    }

    public bool SendPointSTM(PointSTM stm)
    {
        IsStarted = true;
        _lastBody = stm;
        return _autd?.Send(stm) ?? false;
    }

    public bool SendModulation(Modulation modulation)
    {
        return _autd?.Send(modulation) ?? false;
    }

    public bool Stop()
    {
        IsStarted = false;
        return _autd?.Send(new Stop()) ?? false;
    }

    public bool Resume()
    {
        if (_lastBody == null)
        {
            return true;
        }

        IsStarted = true;
        return _autd?.Send(_lastBody) ?? false;
    }

    public bool Close()
    {
        _lastBody = null;
        IsStarted = false;
        return _autd?.Close() ?? false;
    }

    public double GetSoundSpeed()
    {
        return _autd?.SoundSpeed ?? 340e3;
    }
}
