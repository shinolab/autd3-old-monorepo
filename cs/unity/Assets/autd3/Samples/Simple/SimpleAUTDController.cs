/*
 * File: SimpleAUTDController.cs
 * Project: Example
 * Created Date: 10/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/03/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

using System;
using AUTD3Sharp;
using UnityEngine;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

public class SimpleAUTDController : MonoBehaviour
{
    private Controller? _autd = null;
    public GameObject? Target = null;

    private Vector3 _oldPosition;

    private static bool _isPlaying = true;

    private static void OnLost(string msg)
    {
        Debug.LogError(msg);
#if UNITY_EDITOR
        _isPlaying = false;  // UnityEditor.EditorApplication.isPlaying can be set only from the main thread
#elif UNITY_STANDALONE
        UnityEngine.Application.Quit();
#endif
    }

    private static void LogOutput(string msg)
    {
        Debug.Log(msg);
    }

    private static void LogFlush()
    {
    }

    private readonly AUTD3Sharp.Link.SOEM.OnLostCallbackDelegate _onLost = new(OnLost);
    private readonly AUTD3Sharp.Link.SOEM.OnLogOutputCallback _output = new(LogOutput);
    private readonly AUTD3Sharp.Link.SOEM.OnLogFlushCallback _flush = new(LogFlush);

    private void Awake()
    {
        var geometry = new GeometryBuilder()
            .AddDevice(gameObject.transform.position, gameObject.transform.rotation)
            .Build();

        var link = new AUTD3Sharp.Link.SOEM()
           .OnLost(_onLost)
           .DebugLogFunc(_output, _flush)
           .Build();

        try
        {
            _autd = Controller.Open(geometry, link);
        }
        catch (Exception)
        {
            Debug.LogError("Failed to open AUTD3 controller!");
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
        _autd?.Send(new AUTD3Sharp.Gain.Focus(Target.transform.position), TimeSpan.FromMilliseconds(20));
        _oldPosition = Target.transform.position;
    }

    private void Update()
    {
#if UNITY_EDITOR
        if (!_isPlaying)
        {
            UnityEditor.EditorApplication.isPlaying = false;
            return;
        }
#endif

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
