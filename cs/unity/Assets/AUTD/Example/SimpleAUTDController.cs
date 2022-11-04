/*
 * File: SimpleAUTDController.cs
 * Project: Example
 * Created Date: 10/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3Sharp;
using UnityEngine;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

public class SimpleAUTDController : MonoBehaviour
{
    Controller _autd = new Controller();
    AUTD3Sharp.Link.Link? _link = null;
    public GameObject? Target = null;

    private Vector3 _oldPosition;

    void Awake()
    {
        _autd.Geometry.AddDevice(gameObject.transform.position, gameObject.transform.rotation);

        _link = new AUTD3Sharp.Link.SOEM()
            .HighPrecision(true)
            .FreeRun(true)
            .Build();

        if (!_autd.Open(_link))
        {
            Debug.LogError("Failed to open AUTD3 controller!");
            return;
        }

        _autd.CheckTrials = 50;

        _autd.Clear();

        _autd.Synchronize();

        _autd.Send(new AUTD3Sharp.Modulation.Sine(150)); // 150 Hz

        if (Target != null)
        {
            _autd.Send(new AUTD3Sharp.Gain.Focus(Target.transform.position, 1.0));
            _oldPosition = Target.transform.position;
        }
    }

    void Update()
    {
        if (Target != null && Target.transform.position != _oldPosition)
        {
            _autd.Send(new AUTD3Sharp.Gain.Focus(Target.transform.position, 1.0));
            _oldPosition = Target.transform.position;
        }
    }

    private void OnApplicationQuit()
    {
        _autd.Dispose();
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif
