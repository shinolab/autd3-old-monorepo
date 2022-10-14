using AUTD3_GUI_Controller.Helpers;

namespace AUTD3_GUI_Controller;

public sealed partial class MainWindow
{
    public MainWindow()
    {
        InitializeComponent();

        AppWindow.SetIcon(Path.Combine(AppContext.BaseDirectory, "Assets/WindowIcon.ico"));
        Content = null;
        Title = "AppDisplayName".GetLocalized();
    }
}
