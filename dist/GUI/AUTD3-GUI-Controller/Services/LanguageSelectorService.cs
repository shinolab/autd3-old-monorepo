/*
 * File: LanguageSelectorService.cs
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


using AK.Toolkit.WinUI3.Localization;
using AUTD3_GUI_Controller.Contracts.Services;

namespace AUTD3_GUI_Controller.Services;

public class LanguageSelectorService : ILanguageSelectorService
{
    private const string SettingsKey = "Language";

    public string CurrentLanguage
    {
        get;
        set;
    }

    private readonly ILocalizer _localizer;
    private readonly ILocalSettingsService _localSettingsService;

    public LanguageSelectorService(ILocalSettingsService localSettingsService, ILocalizer localizer)
    {
        _localSettingsService = localSettingsService;
        _localizer = localizer;
        CurrentLanguage = localizer.GetAvailableLanguages().First();
    }

    public async Task InitializeAsync()
    {
        CurrentLanguage = await LoadLanguageFromSettingsAsync();
        await Task.CompletedTask;
    }

    public async Task SetLanguageAsync(string language)
    {
        CurrentLanguage = language;

        await SetRequestedLanguageAsync();
        await SaveLanguageInSettingsAsync(language);
    }

    public async Task SetRequestedLanguageAsync()
    {
        _localizer.SetLanguage(CurrentLanguage);
        _localizer.RunLocalizationOnRegisteredRootElements();
        await Task.CompletedTask;
    }

    private async Task<string> LoadLanguageFromSettingsAsync()
    {
        var languageName = await _localSettingsService.ReadSettingAsync<string>(SettingsKey) ?? System.Globalization.CultureInfo.CurrentCulture.ToString();
        var localizer = App.GetService<ILocalizer>();
        if (!localizer.GetAvailableLanguages().Contains(languageName))
        {
            languageName = localizer.GetAvailableLanguages().First();
        }

        return languageName;
    }

    private async Task SaveLanguageInSettingsAsync(string language)
    {
        await _localSettingsService.SaveSettingAsync(SettingsKey, language);
    }
}
