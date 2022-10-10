using Microsoft.UI.Xaml.Controls;

namespace AUTD3_GUI_Controller.Helpers;

public static class FrameExtensions
{
    public static object? GetPageViewModel(this Frame? frame) => frame?.Content?.GetType().GetProperty("ViewModel")?.GetValue(frame.Content, null);
}
