/*
 * File: SettingsViewModel.cs
 * Project: ViewModels
 * Created Date: 11/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

using System.Collections.ObjectModel;
using System.Reflection;

using AUTD3_GUI_Controller.Contracts.Services;
using AUTD3_GUI_Controller.Helpers;
using AUTD3_GUI_Controller.Models;

using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;

using Microsoft.UI.Xaml;

using Windows.ApplicationModel;
using AK.Toolkit.WinUI3.Localization;
using AUTD3_GUI_Controller.Services;
using AUTD3_GUI_Controller.Models.Modulation;
using Newtonsoft.Json.Linq;

namespace AUTD3_GUI_Controller.ViewModels;

[INotifyPropertyChanged]
public partial class SettingsViewModel
{
    private const string AngleUnitKey = "AngleUnit";

    private readonly IThemeSelectorService _themeSelectorService;
    private readonly ILanguageSelectorService _languageSelectorService;
    private readonly ILocalSettingsService _localSettingsService;

    [ObservableProperty]
    private ElementTheme _currentTheme;

    [ObservableProperty]
    private string _versionDescription;

    [ObservableProperty]
    private AngleUnit _angleUnit;
    async partial void OnAngleUnitChanged(AngleUnit value)
    {
        AngleUnitConverter.Instance.AngleUnit = value;
        await _localSettingsService.SaveSettingAsync(AngleUnitKey, value);
    }

    [RelayCommand]
    public async void SwitchTheme(ElementTheme param)
    {
        if (CurrentTheme == param)
        {
            return;
        }

        CurrentTheme = param;
        await _themeSelectorService.SetThemeAsync(param);
    }

    public ObservableCollection<Tuple<string, string>> AvailableLanguages
    {
        get;
        set;
    }

    private Tuple<string, string>? _currentLanguage;

    public Tuple<string, string>? CurrentLanguage
    {
        get => _currentLanguage;
        set
        {
            if (value?.Item2 is { } language)
            {
                _languageSelectorService.SetLanguageAsync(language);
            }
            SetProperty(ref _currentLanguage, value);
            VersionDescription = GetVersionDescription();
        }
    }

    public SettingsViewModel(IThemeSelectorService themeSelectorService, ILanguageSelectorService languageSelectorService, ILocalSettingsService localSettingsService)
    {
        _localSettingsService = localSettingsService;
        _themeSelectorService = themeSelectorService;
        _languageSelectorService = languageSelectorService;
        _currentTheme = themeSelectorService.Theme;
        _versionDescription = GetVersionDescription();

        AngleUnit = _localSettingsService.ReadSetting<AngleUnit?>(AngleUnitKey) ?? AngleUnit.Degree;
        AngleUnitConverter.Instance.AngleUnit = AngleUnit;

        var localizer = App.GetService<ILocalizer>();
        AvailableLanguages = new ObservableCollection<Tuple<string, string>>(localizer.GetAvailableLanguages().Select(x =>
        {
            var displayName = x;
            if (localizer.GetLocalizedString(x.ToLower()) is { } localizedDisplayName)
            {
                displayName = localizedDisplayName;
            }
            return new Tuple<string, string>(displayName, x);
        }));
        CurrentLanguage = AvailableLanguages.FirstOrDefault(x => x.Item2 == localizer.GetCurrentLanguage());
    }

    private static string GetVersionDescription()
    {
        Version version;

        if (RuntimeHelper.IsMsix)
        {
            var packageVersion = Package.Current.Id.Version;

            version = new Version(packageVersion.Major, packageVersion.Minor, packageVersion.Build);
        }
        else
        {
            version = Assembly.GetExecutingAssembly().GetName().Version!;
        }

        return $"{"AppDisplayName".GetLocalized()} - {version.Major}.{version.Minor}.{version.Build}";
    }
}
