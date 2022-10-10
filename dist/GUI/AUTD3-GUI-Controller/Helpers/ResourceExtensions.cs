/*
 * File: ResourceExtensions.cs
 * Project: Helpers
 * Created Date: 18/08/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/08/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

using AK.Toolkit.WinUI3.Localization;

namespace AUTD3_GUI_Controller.Helpers;

public static class ResourceExtensions
{
    private static readonly ILocalizer Localizer = App.GetService<ILocalizer>();
    public static string GetLocalized(this string resourceKey) => Localizer.GetLocalizedString(resourceKey) ?? "";
}
