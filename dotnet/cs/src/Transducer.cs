/*
 * File: Transducer.cs
 * Project: Internal
 * Created Date: 08/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 14/09/2023
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
        private readonly TransducerPtr _ptr;

        internal Transducer(int localIdx, DevicePtr ptr)
        {
            LocalIdx = localIdx;
            _ptr = Base.AUTDGetTransducer(ptr, (uint)localIdx);
        }

        /// <summary>
        /// Index of the transducer
        /// </summary>
        public int LocalIdx { get; }

        /// <summary>
        /// Position of the transducer
        /// </summary>
        public Vector3 Position
        {
            get
            {
                var pos = new float_t[3];
                Base.AUTDTransPosition(_ptr, pos);
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
                Base.AUTDTransRotation(_ptr, rot);
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
                Base.AUTDTransXDirection(_ptr, dir);
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
                Base.AUTDTransYDirection(_ptr, dir);
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
                Base.AUTDTransZDirection(_ptr, dir);
                return new Vector3(dir[0], dir[1], dir[2]);
            }
        }

        /// <summary>
        /// Frequency of the transducer
        /// </summary>
        public float_t Frequency
        {
            get => Base.AUTDGetTransFrequency(_ptr);
            set
            {
                var err = new byte[256];
                if (!Base.AUTDSetTransFrequency(_ptr, value, err))
                    throw new AUTDException(err);
            }
        }

        /// <summary>
        /// Cycle of the transducer
        /// </summary>
        public ushort Cycle
        {
            get => Base.AUTDGetTransCycle(_ptr);
            set
            {
                var err = new byte[256];
                if (!Base.AUTDSetTransCycle(_ptr, value, err))
                    throw new AUTDException(err);
            }
        }

        /// <summary>
        /// Modulation delay of the transducer
        /// </summary>
        public ushort ModDelay
        {
            get => Base.AUTDGetTransModDelay(_ptr);
            set => Base.AUTDSetTransModDelay(_ptr, value);
        }

        public float_t AmpFilter
        {
            get => Base.AUTDGetTransAmpFilter(_ptr);
            set => Base.AUTDSetTransAmpFilter(_ptr, value);
        }

        public float_t PhaseFilter
        {
            get => Base.AUTDGetTransPhaseFilter(_ptr);
            set => Base.AUTDSetTransPhaseFilter(_ptr, value);
        }

        /// <summary>
        /// Wavelength of the transducer
        /// </summary>
        /// <param name="soundSpeed">Speed of sound</param>
        /// <returns></returns>
        public float_t Wavelength(float_t soundSpeed) => Base.AUTDGetWavelength(_ptr, soundSpeed);

        /// <summary>
        /// Wavenumber of the transducer
        /// </summary>
        /// <param name="soundSpeed">Speed of sound</param>
        /// <returns></returns>
        public float_t Wavenumber(float_t soundSpeed) => 2 * AUTD3.Pi / Wavelength(soundSpeed);
    }
}
