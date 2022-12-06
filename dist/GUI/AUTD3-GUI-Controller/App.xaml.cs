/*
 * File: App.xaml.cs
 * Project: AUTD3-GUI-Controller
 * Created Date: 18/08/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/11/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

using AK.Toolkit.WinUI3.Localization;

using AUTD3_GUI_Controller.Activation;
using AUTD3_GUI_Controller.Contracts.Services;
using AUTD3_GUI_Controller.Core.Contracts.Services;
using AUTD3_GUI_Controller.Core.Services;
using AUTD3_GUI_Controller.Models;
using AUTD3_GUI_Controller.Services;
using AUTD3_GUI_Controller.ViewModels;
using AUTD3_GUI_Controller.ViewModels.Gain;
using AUTD3_GUI_Controller.ViewModels.Modulation;
using AUTD3_GUI_Controller.Views;
using AUTD3_GUI_Controller.Views.Gain;
using AUTD3_GUI_Controller.Views.Modulation;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using Microsoft.UI.Xaml;

namespace AUTD3_GUI_Controller;

// To learn more about WinUI 3, see https://docs.microsoft.com/windows/apps/winui/winui3/.
public partial class App
{
    // The .NET Generic Host provides dependency injection, configuration, logging, and other services.
    // https://docs.microsoft.com/dotnet/core/extensions/generic-host
    // https://docs.microsoft.com/dotnet/core/extensions/dependency-injection
    // https://docs.microsoft.com/dotnet/core/extensions/configuration
    // https://docs.microsoft.com/dotnet/core/extensions/logging
    public IHost Host
    {
        get;
    }

    public static T GetService<T>()
        where T : class
    {
        if ((Current as App)!.Host.Services.GetService(typeof(T)) is not T service)
        {
            throw new ArgumentException($"{typeof(T)} needs to be registered in ConfigureServices within App.xaml.cs.");
        }

        return service;
    }

    public static WindowEx MainWindow { get; } = new MainWindow();

    public App()
    {
        InitializeComponent();

        Host = Microsoft.Extensions.Hosting.Host.
        CreateDefaultBuilder().
        UseContentRoot(AppContext.BaseDirectory).
        ConfigureServices((context, services) =>
        {
            // Default Activation Handler
            services.AddTransient<ActivationHandler<LaunchActivatedEventArgs>, DefaultActivationHandler>();

            // Other Activation Handlers

            // Services
            services.AddSingleton<ILocalSettingsService, LocalSettingsService>();
            services.AddSingleton<IThemeSelectorService, ThemeSelectorService>();
            services.AddTransient<INavigationViewService, NavigationViewService>();

            services.AddSingleton<IActivationService, ActivationService>();
            services.AddSingleton<IPageService, PageService>();
            services.AddSingleton<INavigationService, NavigationService>();

            services.AddSingleton<ILanguageSelectorService, LanguageSelectorService>();

            services.AddSingleton<AUTDService>();

            // Core Services
            services.AddSingleton<IFileService, FileService>();

            // Views and ViewModels
            services.AddSingleton<SettingsViewModel>();
            services.AddTransient<SettingsPage>();
            services.AddSingleton<LinkViewModel>();
            services.AddTransient<LinkPage>();
            services.AddSingleton<GeometryViewModel>();
            services.AddTransient<GeometryPage>();
            services.AddSingleton<HomeViewModel>();
            services.AddTransient<HomePage>();
            services.AddSingleton<SilencerViewModel>();
            services.AddSingleton<SilencerPage>();
            services.AddSingleton<ShellViewModel>();
            services.AddTransient<ShellPage>();

            services.AddSingleton<IGainNavigationService, GainNavigationService>();
            services.AddSingleton<IGainNavigationViewService, GainNavigationViewService>();
            services.AddSingleton<IGainPageService, GainPageService>();
            services.AddSingleton<GainViewModel>();
            services.AddSingleton<GainPage>();
            services.AddSingleton<FocusViewModel>();
            services.AddTransient<FocusPage>();
            services.AddSingleton<BesselBeamViewModel>();
            services.AddTransient<BesselBeamPage>();
            services.AddSingleton<PlaneWaveViewModel>();
            services.AddTransient<PlaneWavePage>();
            services.AddSingleton<HoloViewModel>();
            services.AddTransient<HoloPage>();

            services.AddSingleton<IModulationNavigationService, ModulationNavigationService>();
            services.AddSingleton<IModulationNavigationViewService, ModulationNavigationViewService>();
            services.AddSingleton<IModulationPageService, ModulationPageService>();
            services.AddSingleton<ModulationViewModel>();
            services.AddSingleton<ModulationPage>();
            services.AddSingleton<SineViewModel>();
            services.AddTransient<SinePage>();
            services.AddSingleton<StaticViewModel>();
            services.AddTransient<StaticPage>();
            services.AddSingleton<SineViewModel>();
            services.AddTransient<SineLegacyPage>();
            services.AddSingleton<SineLegacyViewModel>();
            services.AddTransient<SineSquaredPage>();
            services.AddSingleton<SineSquaredViewModel>();
            services.AddTransient<SquarePage>();
            services.AddSingleton<SquareViewModel>();

            services.AddTransient<FocusSTMPage>();
            services.AddSingleton<FocusSTMViewModel>();

            // Configuration
            services.Configure<LocalSettingsOptions>(context.Configuration.GetSection(nameof(LocalSettingsOptions)));

            services.AddSingleton(_ =>
            {
                var resourcesFolder = Path.Combine(Directory.GetCurrentDirectory(), "Strings");
                var localizer = new LocalizerBuilder()
                    .AddResourcesStringsFolder(new LocalizerResourcesStringsFolder(resourcesFolder))
                    .Build();
                return localizer;
            });
        }).
        Build();

        UnhandledException += App_UnhandledException;
    }

    private static void App_UnhandledException(object sender, Microsoft.UI.Xaml.UnhandledExceptionEventArgs e)
    {
        // TODO: Log and handle exceptions as appropriate.
        // https://docs.microsoft.com/windows/windows-app-sdk/api/winrt/microsoft.ui.xaml.application.unhandledexception.
        Console.WriteLine(e.Message);
    }

    protected override async void OnLaunched(LaunchActivatedEventArgs args)
    {
        base.OnLaunched(args);

        await GetService<IActivationService>().ActivateAsync(args);
    }
}
