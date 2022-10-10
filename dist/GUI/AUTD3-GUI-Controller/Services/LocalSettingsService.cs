/*
 * File: LocalSettingsService.cs
 * Project: Services
 * Created Date: 23/08/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 23/08/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3_GUI_Controller.Contracts.Services;
using AUTD3_GUI_Controller.Core.Contracts.Services;
using AUTD3_GUI_Controller.Core.Helpers;
using AUTD3_GUI_Controller.Helpers;
using AUTD3_GUI_Controller.Models;

using Microsoft.Extensions.Options;

using Windows.Storage;

namespace AUTD3_GUI_Controller.Services;

public class LocalSettingsService : ILocalSettingsService
{
    private const string DefaultApplicationDataFolder = "AUTD3-GUI-Controller/ApplicationData";
    private const string DefaultLocalSettingsFile = "LocalSettings.json";

    private readonly IFileService _fileService;

    private readonly string _localApplicationData = Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData);
    private readonly string _applicationDataFolder;
    private readonly string _localSettingsFile;

    private IDictionary<string, object> _settings;

    private bool _isInitialized;

    public LocalSettingsService(IFileService fileService, IOptions<LocalSettingsOptions> options)
    {
        _fileService = fileService;
        var options1 = options.Value;

        _applicationDataFolder = Path.Combine(_localApplicationData, options1.ApplicationDataFolder ?? DefaultApplicationDataFolder);
        _localSettingsFile = options1.LocalSettingsFile ?? DefaultLocalSettingsFile;

        _settings = new Dictionary<string, object>();
    }

    private async Task InitializeAsync()
    {
        if (!_isInitialized)
        {
            _settings = await Task.Run(() => _fileService.Read<IDictionary<string, object>>(_applicationDataFolder, _localSettingsFile)) ?? new Dictionary<string, object>();

            _isInitialized = true;
        }
    }

    public T? ReadSetting<T>(string key)
    {
        return Task.Run(async () => await ReadSettingAsync<T>(key)).Result;
    }

    public async Task<T?> ReadSettingAsync<T>(string key)
    {
        if (RuntimeHelper.IsMsix)
        {
            if (!ApplicationData.Current.LocalSettings.Values.TryGetValue(key, out var obj))
            {
                return default;
            }

            var str = await Json.StringifyAsync(obj);
            return await Json.ToObjectAsync<T>(str);
        }
        else
        {
            await InitializeAsync();

            if (!_settings.TryGetValue(key, out var obj))
            {
                return default;
            }

            var str = await Json.StringifyAsync(obj);
            return await Json.ToObjectAsync<T>(str);
        }
    }

    public async Task SaveSettingAsync<T>(string key, T value) where T : notnull
    {
        if (RuntimeHelper.IsMsix)
        {
            ApplicationData.Current.LocalSettings.Values[key] = value;
        }
        else
        {
            await InitializeAsync();

            _settings[key] = value;

            await Task.Run(() => _fileService.Save(_applicationDataFolder, _localSettingsFile, _settings));
        }
    }
}
