using AK.Toolkit.WinUI3.Localization;
using AUTD3_GUI_Controller.ViewModels;
using Microsoft.UI.Xaml;

namespace AUTD3_GUI_Controller.Views;

public sealed partial class PointSTMPage
{
    private readonly ILocalizer _localizer;

    public PointSTMViewModel ViewModel
    {
        get;
    }

    public PointSTMPage()
    {
        ViewModel = App.GetService<PointSTMViewModel>();
        InitializeComponent();

        _localizer = App.GetService<ILocalizer>();
    }

    private void PointSTMPage_OnLoaded(object sender, RoutedEventArgs e)
    {
        ViewModel.XamlRoot = Root.XamlRoot;
        _localizer.RunLocalization(Root);
    }
}
