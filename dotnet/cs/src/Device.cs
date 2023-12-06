/*
 * File: Device.cs
 * Project: src
 * Created Date: 08/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2023
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
using System.Diagnostics.CodeAnalysis;
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
            _transducers = Enumerable.Range(0, (int)NativeMethodsBase.AUTDDeviceNumTransducers(Ptr)).Select(i => new Transducer(i, Ptr)).ToList();
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
            get => NativeMethodsBase.AUTDDeviceGetSoundSpeed(Ptr);
            set => NativeMethodsBase.AUTDDeviceSetSoundSpeed(Ptr, value);
        }

        /// <summary>
        /// Attenuation coefficient
        /// </summary>
        public float_t Attenuation
        {
            get => NativeMethodsBase.AUTDDeviceGetAttenuation(Ptr);
            set => NativeMethodsBase.AUTDDeviceSetAttenuation(Ptr, value);
        }

        public bool Enable
        {
            get => NativeMethodsBase.AUTDDeviceEnableGet(Ptr);
            set => NativeMethodsBase.AUTDDeviceEnableSet(Ptr, value);
        }

        /// <summary>
        /// Get center position of all transducers
        /// </summary>
        public Vector3 Center
        {
            get
            {
                unsafe
                {
                    float_t* center = stackalloc float_t[3];
                    NativeMethodsBase.AUTDDeviceCenter(Ptr, center);
                    return new Vector3(center[0], center[1], center[2]);
                }
            }
        }

        public void Translate(Vector3 t)
        {
            NativeMethodsBase.AUTDDeviceTranslate(Ptr, t.x, t.y, t.z);
        }

        public void Rotate(Quaternion r)
        {
            NativeMethodsBase.AUTDDeviceRotate(Ptr, r.w, r.x, r.y, r.z);
        }

        public void Affine(Vector3 t, Quaternion r)
        {
            NativeMethodsBase.AUTDDeviceAffine(Ptr, t.x, t.y, t.z, r.w, r.x, r.y, r.z);
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
            NativeMethodsBase.AUTDDeviceSetSoundSpeedFromTemp(Ptr, temp, k, r, m);
        }

        public Transducer this[int index] => _transducers[index];

        public IEnumerator<Transducer> GetEnumerator() => _transducers.GetEnumerator();
        [ExcludeFromCodeCoverage] System.Collections.IEnumerator System.Collections.IEnumerable.GetEnumerator() => GetEnumerator();
    }
}
