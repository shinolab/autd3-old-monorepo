/*
 * File: SimulatorRun.cs
 * Project: Editor
 * Created Date: 05/11/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

#if UNITY_EDITOR

using UnityEditor;
using UnityEngine;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

public class SimualtorWindow : EditorWindow
{
    private string _configPath = "Assets";

    [MenuItem("AUTD/Simulator")]
    static void Open()
    {
        var window = GetWindow<SimualtorWindow>();
    }

    void Awake()
    {
        var path = EditorUserSettings.GetConfigValue("AUTD3SimulatorConfigPath");
        if (!string.IsNullOrEmpty(path))
        {
            _configPath = path;
        }
    }

    void OnGUI()
    {
        EditorGUILayout.BeginHorizontal();
        EditorGUILayout.TextField("Config files path: ", _configPath);
        if (GUILayout.Button("Select"))
        {
            var filePath = EditorUtility.SaveFolderPanel("Save config files to folder", _configPath, "");

            if (!string.IsNullOrEmpty(filePath))
            {
                _configPath = filePath;
            }
        }
        EditorGUILayout.EndHorizontal();

        EditorGUILayout.Space();

        if (!GUILayout.Button("Run")) return;
        var path = System.IO.Path.GetFullPath("Packages/com.shinolab.autd3/Editor/autd_simulator.exe");
        var p = new System.Diagnostics.Process();
        p.StartInfo.WorkingDirectory = System.IO.Path.GetFullPath("Packages/com.shinolab.autd3/Editor");
        p.StartInfo.FileName = path;
        p.StartInfo.Arguments = $"run --config_path {_configPath}";
        p.Start();
    }

    void OnDestroy()
    {
        EditorUserSettings.SetConfigValue("AUTD3SimulatorConfigPath", _configPath);
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif

#endif
