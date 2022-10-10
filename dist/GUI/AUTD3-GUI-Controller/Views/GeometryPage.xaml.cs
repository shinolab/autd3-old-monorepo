using AUTD3_GUI_Controller.ViewModels;

namespace AUTD3_GUI_Controller.Views;

public sealed partial class GeometryPage
{
    public GeometryViewModel ViewModel
    {
        get;
    }

    public GeometryPage()
    {
        ViewModel = App.GetService<GeometryViewModel>();
        InitializeComponent();
    }
}
