using System.Collections.ObjectModel;
using System.Reflection;

using AUTD3_GUI_Controller.Contracts.Services;
using AUTD3_GUI_Controller.Helpers;

using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;

using Microsoft.UI.Xaml;

using Windows.ApplicationModel;
using AK.Toolkit.WinUI3.Localization;

namespace AUTD3_GUI_Controller.ViewModels;

[INotifyPropertyChanged]
public partial class SettingsViewModel
{
    private readonly IThemeSelectorService _themeSelectorService;
    private readonly ILanguageSelectorService _languageSelectorService;

    [ObservableProperty]
    private ElementTheme _currentTheme;

    [ObservableProperty]
    private string _versionDescription;


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

    public SettingsViewModel(IThemeSelectorService themeSelectorService, ILanguageSelectorService languageSelectorService)
    {
        _themeSelectorService = themeSelectorService;
        _languageSelectorService = languageSelectorService;
        _currentTheme = themeSelectorService.Theme;
        _versionDescription = GetVersionDescription();

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

            version = new Version(packageVersion.Major, packageVersion.Minor, packageVersion.Build, packageVersion.Revision);
        }
        else
        {
            version = Assembly.GetExecutingAssembly().GetName().Version!;
        }

        return $"{"AppDisplayName".GetLocalized()} - {version.Major}.{version.Minor}.{version.Build}.{version.Revision}";
    }
}
