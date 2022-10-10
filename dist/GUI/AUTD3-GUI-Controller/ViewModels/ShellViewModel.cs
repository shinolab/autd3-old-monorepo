using AUTD3_GUI_Controller.Contracts.Services;
using AUTD3_GUI_Controller.Services;
using AUTD3_GUI_Controller.Views;

using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Navigation;

namespace AUTD3_GUI_Controller.ViewModels;

[INotifyPropertyChanged]
public partial class ShellViewModel
{
    [ObservableProperty]
    private bool _isBackEnabled;
    [ObservableProperty]
    private object? _selected;

    public INavigationService NavigationService
    {
        get;
    }

    public INavigationViewService NavigationViewService
    {
        get;
    }

    [RelayCommand]
    public void MenuFileExit()
    {
        Application.Current.Exit();
    }

    [RelayCommand(CanExecute = "CanStartExecute")]
    public void Start()
    {
        App.GetService<AUTDService>().Resume();
        StartCommand.NotifyCanExecuteChanged();
        PauseCommand.NotifyCanExecuteChanged();
    }
    public bool CanStartExecute()
    {
        return App.GetService<AUTDService>().IsOpened && !App.GetService<AUTDService>().IsStarted;
    }

    [RelayCommand(CanExecute = "CanPauseExecute")]
    public void Pause()
    {
        App.GetService<AUTDService>().Stop();
        StartCommand.NotifyCanExecuteChanged();
        PauseCommand.NotifyCanExecuteChanged();
    }
    public bool CanPauseExecute()
    {
        return App.GetService<AUTDService>().IsOpened && App.GetService<AUTDService>().IsStarted;
    }

    public ShellViewModel(INavigationService navigationService, INavigationViewService navigationViewService)
    {
        NavigationService = navigationService;
        NavigationService.Navigated += OnNavigated;
        NavigationViewService = navigationViewService;
    }

    private void OnNavigated(object sender, NavigationEventArgs e)
    {
        IsBackEnabled = NavigationService.CanGoBack;

        if (e.SourcePageType == typeof(SettingsPage))
        {
            Selected = NavigationViewService.SettingsItem;
            return;
        }

        var selectedItem = NavigationViewService.GetSelectedItem(e.SourcePageType);
        if (selectedItem != null)
        {
            Selected = selectedItem;
        }
    }
}
