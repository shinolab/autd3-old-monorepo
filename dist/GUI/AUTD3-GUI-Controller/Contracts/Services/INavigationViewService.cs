using Microsoft.UI.Xaml.Controls;

namespace AUTD3_GUI_Controller.Contracts.Services;

public interface INavigationViewService
{
    object? SettingsItem
    {
        get;
    }

    void Initialize(NavigationView navigationView);

    NavigationViewItem? GetSelectedItem(Type pageType);
}
