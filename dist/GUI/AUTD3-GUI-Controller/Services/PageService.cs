using System.ComponentModel;
using AUTD3_GUI_Controller.Contracts.Services;
using AUTD3_GUI_Controller.ViewModels;
using AUTD3_GUI_Controller.Views;
using Microsoft.UI.Xaml.Controls;

namespace AUTD3_GUI_Controller.Services;

public class PageService : IPageService
{
    private readonly Dictionary<string, Type> _pages = new();

    public PageService()
    {
        Configure<HomeViewModel, HomePage>();
        Configure<GeometryViewModel, GeometryPage>();
        Configure<LinkViewModel, LinkPage>();

        Configure<GainViewModel, GainPage>();
        Configure<ModulationViewModel, ModulationPage>();

        Configure<FocusSTMViewModel, FocusSTMPage>();

        Configure<SilencerViewModel, SilencerPage>();

        Configure<SettingsViewModel, SettingsPage>();
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
