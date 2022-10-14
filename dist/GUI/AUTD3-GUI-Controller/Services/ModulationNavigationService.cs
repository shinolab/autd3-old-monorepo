/*
 * File: ModulationNavigationService.cs
 * Project: Services
 * Created Date: 24/08/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/08/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

using System.Diagnostics.CodeAnalysis;

using AUTD3_GUI_Controller.Contracts.Services;
using AUTD3_GUI_Controller.Contracts.ViewModels;
using AUTD3_GUI_Controller.Helpers;

using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Navigation;

namespace AUTD3_GUI_Controller.Services;

public class ModulationNavigationService : IModulationNavigationService
{
    private readonly IModulationPageService _pageService;
    private object? _lastParameterUsed;
    private Frame? _frame;

    public event NavigatedEventHandler? Navigated;

    public Frame? Frame
    {
        get
        {
            if (_frame != null)
            {
                return _frame;
            }

            _frame = App.MainWindow.Content as Frame;
            RegisterFrameEvents();
            return _frame;
        }

        set
        {
            UnregisterFrameEvents();
            _frame = value;
            RegisterFrameEvents();
        }
    }

    [MemberNotNullWhen(true, nameof(Frame), nameof(_frame))]
    public bool CanGoBack => Frame != null && Frame.CanGoBack;

    public ModulationNavigationService(IModulationPageService pageService)
    {
        _pageService = pageService;
    }

    private void RegisterFrameEvents()
    {
        if (_frame != null)
        {
            _frame.Navigated += OnNavigated;
        }
    }

    private void UnregisterFrameEvents()
    {
        if (_frame != null)
        {
            _frame.Navigated -= OnNavigated;
        }
    }

    public bool GoBack()
    {
        if (!CanGoBack)
        {
            return false;
        }

        var vmBeforeNavigation = _frame.GetPageViewModel();
        _frame.GoBack();
        if (vmBeforeNavigation is INavigationAware navigationAware)
        {
            navigationAware.OnNavigatedFrom();
        }

        return true;
    }

    public bool NavigateTo(string pageKey, object? parameter = null, bool clearNavigation = false)
    {
        var pageType = _pageService.GetPageType(pageKey);

        if (_frame == null || (_frame.Content?.GetType() == pageType &&
                               (parameter == null || parameter.Equals(_lastParameterUsed))))
        {
            return false;
        }

        _frame.Tag = clearNavigation;
        var vmBeforeNavigation = _frame.GetPageViewModel();
        var navigated = _frame.Navigate(pageType, parameter);
        if (!navigated)
        {
            return navigated;
        }

        _lastParameterUsed = parameter;
        if (vmBeforeNavigation is INavigationAware navigationAware)
        {
            navigationAware.OnNavigatedFrom();
        }

        return navigated;

    }

    private void OnNavigated(object sender, NavigationEventArgs e)
    {
        if (sender is not Frame frame)
        {
            return;
        }

        var clearNavigation = (bool)frame.Tag;
        if (clearNavigation)
        {
            frame.BackStack.Clear();
        }

        if (frame.GetPageViewModel() is INavigationAware navigationAware)
        {
            navigationAware.OnNavigatedTo(e.Parameter);
        }

        Navigated?.Invoke(sender, e);
    }
}
