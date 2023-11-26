/*
 * File: SimpleAUTDController.cs
 * Project: Example
 * Created Date: 10/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/11/2023
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
    private Controller<AUTD3Sharp.Link.SOEM>? _autd = null;
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

    private readonly AUTD3Sharp.Link.SOEM.OnErrCallbackDelegate _onLost = new(OnLost);

    private async void Awake()
    {
        try
        {
            _autd = await new ControllerBuilder()
                .AddDevice(new AUTD3(gameObject.transform.position).WithRotation(gameObject.transform.rotation))
                .OpenWithAsync(AUTD3Sharp.Link.SOEM.Builder().WithOnLost(_onLost));
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

        await _autd!.SendAsync(new AUTD3Sharp.Modulation.Sine(150)); // 150 Hz

        if (Target == null) return;
        await _autd!.SendAsync(new AUTD3Sharp.Gain.Focus(Target.transform.position));
        _oldPosition = Target.transform.position;
    }

    private async void Update()
    {
#if UNITY_EDITOR
        if (!_isPlaying)
        {
            UnityEditor.EditorApplication.isPlaying = false;
            return;
        }
#endif
        if (_autd == null) return;

        if (Target == null || Target.transform.position == _oldPosition) return;
        await _autd.SendAsync(new AUTD3Sharp.Gain.Focus(Target.transform.position));
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
