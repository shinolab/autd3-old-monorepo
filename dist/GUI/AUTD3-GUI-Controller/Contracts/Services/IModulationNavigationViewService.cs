/*
 * File: IModulationNavigationViewService.cs
 * Project: Services
 * Created Date: 24/08/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/08/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

using Microsoft.UI.Xaml.Controls;

namespace AUTD3_GUI_Controller.Contracts.Services;

public interface IModulationNavigationViewService
{
    void Initialize(NavigationView navigationView);

    NavigationViewItem? GetSelectedItem(Type pageType);
}
