/*
 * File: Transducer.cs
 * Project: Internal
 * Created Date: 08/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */


#if UNITY_2018_3_OR_NEWER
#define USE_SINGLE
#endif

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

        internal Transducer(int devIdx, int trIdx, DevicePtr ptr)
        {
            TrIdx = trIdx;
            DevIdx = devIdx;
            Ptr = NativeMethodsBase.AUTDTransducer(ptr, (uint)trIdx);
        }

        /// <summary>
        /// Index of the transducer
        /// </summary>
        public int TrIdx { get; }

        /// <summary>
        /// Index of the device
        /// </summary>
        public int DevIdx { get; }

        /// <summary>
        /// Position of the transducer
        /// </summary>
        public Vector3 Position
        {
            get
            {
                var pos = new float_t[3];
                unsafe
                {
                    fixed (float_t* p = pos)
                    {
                        NativeMethodsBase.AUTDTransducerPosition(Ptr, p);
                    }
                }
                return new Vector3(pos[0], pos[1], pos[2]);
            }
        }

        /// <summary>
        /// Rotation of the transducer
        /// </summary>
        public Quaternion Rotation
        {
            get
            {
                var rot = new float_t[4];
                unsafe
                {
                    fixed (float_t* p = rot)
                    {
                        NativeMethodsBase.AUTDTransducerRotation(Ptr, p);
                    }
                }
                return new Quaternion(rot[1], rot[2], rot[3], rot[0]);
            }
        }

        /// <summary>
        /// X-direction of the transducer
        /// </summary>
        public Vector3 XDirection
        {
            get
            {
                var dir = new float_t[3];
                unsafe
                {
                    fixed (float_t* p = dir)
                    {
                        NativeMethodsBase.AUTDTransducerDirectionX(Ptr, p);
                    }
                }
                return new Vector3(dir[0], dir[1], dir[2]);
            }
        }

        /// <summary>
        /// Y-direction of the transducer
        /// </summary>
        public Vector3 YDirection
        {
            get
            {
                var dir = new float_t[3];
                unsafe
                {
                    fixed (float_t* p = dir)
                    {
                        NativeMethodsBase.AUTDTransducerDirectionY(Ptr, p);
                    }
                }
                return new Vector3(dir[0], dir[1], dir[2]);
            }
        }

        /// <summary>
        /// Z-direction of the transducer
        /// </summary>
        public Vector3 ZDirection
        {
            get
            {
                var dir = new float_t[3];
                unsafe
                {
                    fixed (float_t* p = dir)
                    {
                        NativeMethodsBase.AUTDTransducerDirectionZ(Ptr, p);
                    }
                }
                return new Vector3(dir[0], dir[1], dir[2]);
            }
        }

        /// <summary>
        /// Modulation delay of the transducer
        /// </summary>
        public ushort ModDelay
        {
            get => NativeMethodsBase.AUTDTransducerModDelayGet(Ptr);
            set => NativeMethodsBase.AUTDTransducerModDelaySet(Ptr, value);
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
