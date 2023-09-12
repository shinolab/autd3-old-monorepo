/*
 * File: SimulatorRun.cs
 * Project: Editor
 * Created Date: 05/11/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

using UnityEditor;
using UnityEngine;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

public class SimualtorWindow : EditorWindow
{
    [MenuItem("AUTD/Simulator")]
    static void Open()
    {
        var window = GetWindow<SimualtorWindow>();
    }

    void OnGUI()
    {
        if (!GUILayout.Button("Run")) return;
        var path = System.IO.Path.GetFullPath("Packages/com.shinolab.autd3/Editor/autd_simulator.exe");
        var p = new System.Diagnostics.Process();
        p.StartInfo.FileName = path;
        p.StartInfo.Arguments = "run";
        p.Start();
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
