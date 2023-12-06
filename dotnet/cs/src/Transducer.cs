/*
 * File: Transducer.cs
 * Project: Internal
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
    public sealed class Transducer
    {
        internal readonly TransducerPtr Ptr;

        internal Transducer(int trIdx, DevicePtr ptr)
        {
            Idx = trIdx;
            Ptr = NativeMethodsBase.AUTDTransducer(ptr, (uint)trIdx);
        }

        /// <summary>
        /// Index of the transducer
        /// </summary>
        public int Idx { get; }

        /// <summary>
        /// Position of the transducer
        /// </summary>
        public Vector3 Position
        {
            get
            {
                unsafe
                {
                    float_t* pos = stackalloc float_t[3];
                    NativeMethodsBase.AUTDTransducerPosition(Ptr, pos);
                    return new Vector3(pos[0], pos[1], pos[2]);
                }
            }
        }

        /// <summary>
        /// Rotation of the transducer
        /// </summary>
        public Quaternion Rotation
        {
            get
            {
                unsafe
                {
                    float_t* rot = stackalloc float_t[3];
                    NativeMethodsBase.AUTDTransducerRotation(Ptr, rot);
                    return new Quaternion(rot[1], rot[2], rot[3], rot[0]);
                }
            }
        }

        /// <summary>
        /// X-direction of the transducer
        /// </summary>
        public Vector3 XDirection
        {
            get
            {
                unsafe
                {
                    float_t* dir = stackalloc float_t[3];
                    NativeMethodsBase.AUTDTransducerDirectionX(Ptr, dir);
                    return new Vector3(dir[0], dir[1], dir[2]);
                }
            }
        }

        /// <summary>
        /// Y-direction of the transducer
        /// </summary>
        public Vector3 YDirection
        {
            get
            {
                unsafe
                {
                    float_t* dir = stackalloc float_t[3];
                    NativeMethodsBase.AUTDTransducerDirectionY(Ptr, dir);
                    return new Vector3(dir[0], dir[1], dir[2]);
                }
            }
        }

        /// <summary>
        /// Z-direction of the transducer
        /// </summary>
        public Vector3 ZDirection
        {
            get
            {
                unsafe
                {
                    float_t* dir = stackalloc float_t[3];
                    NativeMethodsBase.AUTDTransducerDirectionZ(Ptr, dir);
                    return new Vector3(dir[0], dir[1], dir[2]);
                }
            }
        }

        /// <summary>
        /// Wavelength of the transducer
        /// </summary>
        /// <param name="soundSpeed">Speed of sound</param>
        /// <returns></returns>
        public float_t Wavelength(float_t soundSpeed) => NativeMethodsBase.AUTDTransducerWavelength(Ptr, soundSpeed);

        /// <summary>
        /// Wavenumber of the transducer
        /// </summary>
        /// <param name="soundSpeed">Speed of sound</param>
        /// <returns></returns>
        public float_t Wavenumber(float_t soundSpeed) => 2 * AUTD3.Pi / Wavelength(soundSpeed);
    }
}
