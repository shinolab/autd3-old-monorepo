/*
 * File: GainPageService.cs
 * Project: Services
 * Created Date: 24/08/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/08/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

using System.ComponentModel;
using AUTD3_GUI_Controller.Contracts.Services;
using AUTD3_GUI_Controller.ViewModels.Gain;
using AUTD3_GUI_Controller.Views.Gain;
using Microsoft.UI.Xaml.Controls;

namespace AUTD3_GUI_Controller.Services;

public class GainPageService : IGainPageService
{
    private readonly Dictionary<string, Type> _pages = new();

    public GainPageService()
    {
        Configure<FocusViewModel, FocusPage>();
        Configure<BesselBeamViewModel, BesselBeamPage>();
        Configure<PlaneWaveViewModel, PlaneWavePage>();
        Configure<HoloViewModel, HoloPage>();
    }

    public Type GetPageType(string key)
    {
        Type? pageType;
        lock (_pages)
        {
            if (!_pages.TryGetValue(key, out pageType))
            {
                throw new ArgumentException($"Page not found: {key}. Did you forget to call GainPageService.Configure?");
            }
        }

        return pageType;
    }

    private void Configure<TVm, TV>()
        where TVm : INotifyPropertyChanged
        where TV : Page
    {
        lock (_pages)
        {
            var key = typeof(TVm).FullName!;
            if (_pages.ContainsKey(key))
            {
                throw new ArgumentException($"The key {key} is already configured in GainPageService");
            }

            var type = typeof(TV);
            if (_pages.Any(p => p.Value == type))
            {
                throw new ArgumentException($"This type is already configured with key {_pages.First(p => p.Value == type).Key}");
            }

            _pages.Add(key, type);
        }
    }
}
