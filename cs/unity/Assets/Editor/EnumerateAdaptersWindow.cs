/*
 * File: EnumerateAdaptersWindow.cs
 * Project: Editor
 * Created Date: 03/07/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 * 
 */

using UnityEditor;
using UnityEngine;
using System.Linq;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

public class EnumerateAdaptersWindow : EditorWindow
{
    AUTD3Sharp.Link.EtherCATAdapter[]? _adapters = null;
    Vector2 _leftScrollPos = Vector2.zero;

    private void OnEnable()
    {
        _adapters = AUTD3Sharp.Link.SOEM.EnumerateAdapters().ToArray();
    }

    [MenuItem("AUTD/Enumerate Adapters")]
    static void Open()
    {
        GetWindow<EnumerateAdaptersWindow>("Enumerate adapters");
    }

    void OnGUI()
    {
        EditorGUILayout.LabelField("Available ethernet adapters ");
        var default_color = GUI.color;
        using (var sv = new GUILayout.ScrollViewScope(_leftScrollPos))
        {
            _leftScrollPos = sv.scrollPosition;

            foreach (var adapter in _adapters ?? new AUTD3Sharp.Link.EtherCATAdapter[] { })
            {
                using (new GUILayout.HorizontalScope(GUI.skin.box))
                {
                    EditorGUILayout.SelectableLabel(adapter.Name);
                    GUI.color = Color.black;
                    GUILayout.Box("", GUILayout.ExpandHeight(true), GUILayout.MaxHeight(30), GUILayout.Width(1));
                    GUI.color = default_color;
                    EditorGUILayout.SelectableLabel(adapter.Desc);
                }
            }
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif
