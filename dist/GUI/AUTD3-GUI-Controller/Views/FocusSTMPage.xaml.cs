using AK.Toolkit.WinUI3.Localization;
using AUTD3_GUI_Controller.ViewModels;
using Microsoft.UI.Xaml;

namespace AUTD3_GUI_Controller.Views;

public sealed partial class FocusSTMPage
{
    private readonly ILocalizer _localizer;

    public FocusSTMViewModel ViewModel
    {
        get;
    }

    public FocusSTMPage()
    {
        ViewModel = App.GetService<FocusSTMViewModel>();
        InitializeComponent();

        _localizer = App.GetService<ILocalizer>();
    }

    private void FocusSTMPage_OnLoaded(object sender, RoutedEventArgs e)
    {
        ViewModel.XamlRoot = Root.XamlRoot;
        _localizer.RunLocalization(Root);
    }
}
