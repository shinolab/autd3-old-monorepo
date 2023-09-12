/*
 * File: Geometry.cs
 * Project: src
 * Created Date: 08/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

using System.Collections.Generic;
using System.Linq;
using AUTD3Sharp.NativeMethods;

#if UNITY_2018_3_OR_NEWER
using UnityEngine;
using Vector3 = UnityEngine.Vector3;
#else
using Vector3 = AUTD3Sharp.Utils.Vector3d;
#endif

namespace AUTD3Sharp
{
    public sealed class Geometry : IEnumerable<Device>
    {
        internal readonly GeometryPtr Ptr;
        internal readonly TransMode Mode;
        private readonly List<Device> _devices;

        internal Geometry(GeometryPtr ptr, TransMode mode)
        {
            Ptr = ptr;
            Mode = mode;
            _devices = Enumerable.Range(0, (int)Base.AUTDGeometryNumDevices(Ptr)).Select(x => new Device(x, Base.AUTDGetDevice(Ptr, (uint)x))).ToList();
        }

        /// <summary>
        /// Number of devices
        /// </summary>
        public int NumDevices => _devices.Count;


        /// <summary>
        /// Get center position of all transducers
        /// </summary>
        public Vector3 Center
        {
            get
            {
                return _devices.Aggregate(Vector3.zero, (current, device) => current + device.Center) / _devices.Count;
            }
        }

        public Device this[int index] => _devices[index];
        public IEnumerator<Device> GetEnumerator() => _devices.GetEnumerator();
        System.Collections.IEnumerator System.Collections.IEnumerable.GetEnumerator() => GetEnumerator();
    }
}