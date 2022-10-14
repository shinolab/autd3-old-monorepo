using AUTD3_GUI_Controller.Contracts.Services;
using AUTD3_GUI_Controller.Services;
using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;

namespace AUTD3_GUI_Controller.ViewModels;

[INotifyPropertyChanged]
public partial class SilencerViewModel
{
    private const string CycleKey = "SilencerCycle";
    private const string StepKey = "SilencerStep";

    private readonly ILocalSettingsService _localSettingsService;

    [ObservableProperty] private ushort _cycle;
    async partial void OnCycleChanged(ushort value) => await _localSettingsService.SaveSettingAsync(CycleKey, value);

    [ObservableProperty] private ushort _step;
    async partial void OnStepChanged(ushort value) => await _localSettingsService.SaveSettingAsync(StepKey, value);

    [RelayCommand(CanExecute = "ConfigCanExecute")]
    public void Config()
    {
        App.GetService<AUTDService>().ConfigSilencer(Step, Cycle);
    }
    private bool ConfigCanExecute() => App.GetService<AUTDService>().IsOpened;

    public SilencerViewModel(ILocalSettingsService localSettingsService)
    {
        _localSettingsService = localSettingsService;
        _cycle = localSettingsService.ReadSetting<ushort?>(CycleKey) ?? 4096;
        _step = localSettingsService.ReadSetting<ushort?>(StepKey) ?? 10;
    }
}
