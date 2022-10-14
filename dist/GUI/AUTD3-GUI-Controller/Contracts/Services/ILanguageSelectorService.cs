/*
 * File: ILanguageSelectorService.cs
 * Project: Services
 * Created Date: 23/08/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 23/08/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */


namespace AUTD3_GUI_Controller.Contracts.Services;

public interface ILanguageSelectorService
{
    Task InitializeAsync();

    Task SetLanguageAsync(string language);

    Task SetRequestedLanguageAsync();
}
