/*
 * File: SimulatorRun.cs
 * Project: Editor
 * Created Date: 05/11/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/11/2022
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
        if (GUILayout.Button("Run"))
        {
            var mono = MonoScript.FromScriptableObject(this);
            var path = AssetDatabase.GetAssetPath(mono);
            var parent = System.IO.Directory.GetParent(path).FullName;
            var simulator_path = System.IO.Path.Combine(parent, "autd_simulator.exe");
            var p = new System.Diagnostics.Process();
            p.StartInfo.FileName = simulator_path;
            p.Start();
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif
