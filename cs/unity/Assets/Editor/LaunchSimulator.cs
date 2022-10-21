/*
 * File: LaunchSimulator.cs
 * Project: Editor
 * Created Date: 21/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

using UnityEditor;
using UnityEngine;
using System.Linq;

public class LaunchSimulator : EditorWindow
{
    private void OnEnable()
    {
    }

    [MenuItem("AUTD/Launch Simulator")]
    static void Open()
    {
        new AUTD3Sharp.Extra.Simulator().Run();
    }

    void OnGUI()
    {

    }
}
