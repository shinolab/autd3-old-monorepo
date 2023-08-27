/*
 * File: SimpleAUTDController.cs
 * Project: Example
 * Created Date: 10/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/08/2023
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
    private readonly AUTD3Sharp.Link.OnLogOutputCallback _output = new(LogOutput);
    private readonly AUTD3Sharp.Link.OnLogFlushCallback _flush = new(LogFlush);

    private void Awake()
    {
        try
        {
            _autd = Controller.Builder()
                .AddDevice(new AUTD3(gameObject.transform.position, gameObject.transform.rotation))
                .OpenWith(new AUTD3Sharp.Link.SOEM()
                    .WithOnLost(_onLost)
                    .WithLogFunc(_output, _flush));
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

        _autd!.Send(new Clear());

        _autd!.Send(new Synchronize());

        _autd!.Send(new AUTD3Sharp.Modulation.Sine(150)); // 150 Hz

        if (Target == null) return;
        _autd?.Send(new AUTD3Sharp.Gain.Focus(Target.transform.position));
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
        _autd?.Send(new AUTD3Sharp.Gain.Focus(Target.transform.position));
        _oldPosition = Target.transform.position;
    }

    private void OnApplicationQuit()
    {
        _autd?.Dispose();
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
