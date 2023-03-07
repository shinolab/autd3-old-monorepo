/*
 * File: AUTD3Controller.cs
 * Project: MultiDevice
 * Created Date: 27/12/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/03/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

using System;
using System.Linq;
using AUTD3Sharp;
using UnityEngine;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

public class MultiAUTD3Controller : MonoBehaviour
{
    private Controller? _autd = null;
    public GameObject? Target = null;
    private Vector3 _oldPosition;

    void Awake()
    {
        var builder = new GeometryBuilder();
        foreach (var obj in FindObjectsOfType<AUTD3Device>(false).OrderBy(obj => obj.ID))
            builder.AddDevice(obj.transform.position, obj.transform.rotation);
        var geometry = builder.Build();

        var link = new AUTD3Sharp.Link.Simulator().Build();

        try
        {
            _autd = Controller.Open(geometry, link);
        }
        catch (Exception)
        {
            Debug.LogError("Before running this sample, open simulator from \"AUTD -> Simulator -> Run\" in menubar.");
#if UNITY_EDITOR
            UnityEditor.EditorApplication.isPlaying = false;
#elif UNITY_STANDALONE
            UnityEngine.Application.Quit();
#endif
        }

        _autd!.Send(new Clear(), TimeSpan.FromMilliseconds(20));

        _autd!.Send(new Synchronize(), TimeSpan.FromMilliseconds(20));

        _autd!.Send(new AUTD3Sharp.Modulation.Sine(150), TimeSpan.FromMilliseconds(20)); // 150 Hz

        if (Target == null) return;
        _autd!.Send(new AUTD3Sharp.Gain.Focus(Target.transform.position, 1.0f), TimeSpan.FromMilliseconds(20));
        _oldPosition = Target.transform.position;
    }

    private void Update()
    {
        if (Target == null || Target.transform.position == _oldPosition) return;
        _autd?.Send(new AUTD3Sharp.Gain.Focus(Target.transform.position), TimeSpan.FromMilliseconds(20));
        _oldPosition = Target.transform.position;
    }

    private void OnApplicationQuit()
    {
        _autd?.Dispose();
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif
