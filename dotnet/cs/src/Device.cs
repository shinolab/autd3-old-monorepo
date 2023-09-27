/*
 * File: Device.cs
 * Project: src
 * Created Date: 08/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 27/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

#if UNITY_2018_3_OR_NEWER
#define USE_SINGLE
#endif

using System.Collections.Generic;
using System.Linq;
using System.Numerics;
using AUTD3Sharp.NativeMethods;

#if UNITY_2018_3_OR_NEWER
using UnityEngine;
using Vector3 = UnityEngine.Vector3;
using Quaternion = UnityEngine.Quaternion;
#else
using Vector3 = AUTD3Sharp.Utils.Vector3d;
using Quaternion = AUTD3Sharp.Utils.Quaterniond;
#endif

#if USE_SINGLE
using float_t = System.Single;
#else
using float_t = System.Double;
#endif

namespace AUTD3Sharp
{
    public sealed class Device : IEnumerable<Transducer>
    {
        internal readonly DevicePtr Ptr;
        private readonly List<Transducer> _transducers;

        internal Device(int idx, DevicePtr ptr)
        {
            Idx = idx;
            Ptr = ptr;
            _transducers = Enumerable.Range(0, (int)Base.AUTDDeviceNumTransducers(Ptr)).Select(i => new Transducer(i, Ptr)).ToList();
        }

        /// <summary>
        /// Index of the transducer
        /// </summary>
        public int Idx { get; }

        /// <summary>
        /// Number of transducers
        /// </summary>
        public int NumTransducers => _transducers.Count;

        /// <summary>
        /// Speed of sound
        /// </summary>
        public float_t SoundSpeed
        {
            get => Base.AUTDDeviceGetSoundSpeed(Ptr);
            set => Base.AUTDDeviceSetSoundSpeed(Ptr, value);
        }

        /// <summary>
        /// Attenuation coefficient
        /// </summary>
        public float_t Attenuation
        {
            get => Base.AUTDDeviceGetAttenuation(Ptr);
            set => Base.AUTDDeviceSetAttenuation(Ptr, value);
        }

        public bool Enable
        {
            get => Base.AUTDDeviceEnableGet(Ptr);
            set => Base.AUTDDeviceEnableSet(Ptr, value);
        }

        /// <summary>
        /// Get center position of all transducers
        /// </summary>
        public Vector3 Center
        {
            get
            {
                var center = new float_t[3];
                Base.AUTDDeviceCenter(Ptr, center);
                return new Vector3(center[0], center[1], center[2]);
            }
        }


        /// <summary>
        /// set force fan flag
        /// </summary>
        /// <param name="value"></param>
        public bool ForceFan
        {
            set => Base.AUTDDeviceSetForceFan(Ptr, value);
        }

        /// <summary>
        /// set reads FPGA info flag
        /// </summary>
        /// <param name="value"></param>
        public bool ReadsFPGAInfo
        {
            set => Base.AUTDDeviceSetReadsFPGAInfo(Ptr, value);
        }

        public void Translate(Vector3 t)
        {
            Base.AUTDDeviceTranslate(Ptr, t.x, t.y, t.z);
        }

        public void Rotate(Quaternion r)
        {
            Base.AUTDDeviceRotate(Ptr, r.w, r.x, r.y, r.z);
        }

        public void Affine(Vector3 t, Quaternion r)
        {
            Base.AUTDDeviceAffine(Ptr, t.x, t.y, t.z, r.w, r.x, r.y, r.z);
        }

        /// <summary>
        /// Set the sound speed from temperature
        /// </summary>
        /// <param name="temp">Temperature in celsius</param>
        /// <param name="k">Ratio of specific heat</param>
        /// <param name="r">Gas constant</param>
        /// <param name="m">Molar mass</param>
        public void SetSoundSpeedFromTemp(float_t temp, float_t k = (float_t)1.4, float_t r = (float_t)8.31446261815324, float_t m = (float_t)28.9647e-3)
        {
            Base.AUTDDeviceSetSoundSpeedFromTemp(Ptr, temp, k, r, m);
        }

        public Transducer this[int index] => _transducers[index];

        public IEnumerator<Transducer> GetEnumerator() => _transducers.GetEnumerator();

        System.Collections.IEnumerator System.Collections.IEnumerable.GetEnumerator() => GetEnumerator();
    }
}
