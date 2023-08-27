/*
 * File: AUTD3Sharp.cs
 * Project: src
 * Created Date: 23/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 * 
 */

#if UNITY_2018_3_OR_NEWER
#define USE_SINGLE
#define DIMENSION_M
#endif

using System;
using System.Collections.Generic;
using System.Linq;

using AUTD3Sharp.Internal;

#if UNITY_2018_3_OR_NEWER
using UnityEngine;
using Vector3 = UnityEngine.Vector3;
using Quaternion = UnityEngine.Quaternion;
using Math = UnityEngine.Mathf;
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
    using Base = NativeMethods.Base;
    using Def = NativeMethods.Def;

    /// <summary>
    /// AUTD3 device
    /// </summary>
    public class AUTD3
    {
        #region const

        /// <summary>
        /// Meter
        /// </summary>
#if DIMENSION_M
        public const float_t Meter = 1;
#else
        public const float_t Meter = 1000;
#endif

        /// <summary>
        /// Millimeter
        /// </summary>
        public const float_t Millimeter = Meter / 1000;

        /// <summary>
        /// Mathematical constant pi
        /// </summary>
        public const float_t Pi = Math.PI;

        /// <summary>
        /// Number of transducer in an AUTD3 device
        /// </summary>
        public const uint NumTransInUnit = Def.NumTransInUnit;

        /// <summary>
        /// Spacing between transducers in mm
        /// </summary>
        public const float_t TransSpacingMm = Def.TransSpacingMm;

        /// <summary>
        /// Spacing between transducers in m
        /// </summary>
        public const float_t TransSpacing = Def.TransSpacingMm * Millimeter;

        /// <summary>
        /// Number of transducer in x-axis of AUTD3 device
        /// </summary>
        public const uint NumTransInX = Def.NumTransInX;

        /// <summary>
        /// Number of transducer in y-axis of AUTD3 device
        /// </summary>
        public const uint NumTransInY = Def.NumTransInY;

        /// <summary>
        /// FPGA main clock frequency
        /// </summary>
        public const uint FpgaClkFreq = Def.FpgaClkFreq;

        /// <summary>
        /// Device height including substrate
        /// </summary>
        public const float_t DeviceHeight = Def.DeviceHeightMm * Millimeter;

        /// <summary>
        /// Device width including substrate
        /// </summary>
        public const float_t DeviceWidth = Def.DeviceWidthMm * Millimeter;

        /// <summary>
        /// FPGA sub clock frequency
        /// </summary>
        public const uint FpgaSubClkFreq = Def.FpgaSubClkFreq;

        #endregion

        internal Vector3 Pos;
        internal Vector3? Rot;
        internal Quaternion? Quat;

        /// <summary>
        /// Constructor
        /// </summary>
        /// <param name="pos">Global position</param>
        /// <param name="rot">ZYZ euler angels</param>
        public AUTD3(Vector3 pos, Vector3 rot)
        {
            Pos = pos;
            Rot = rot;
            Quat = null;
        }

        /// <summary>
        /// Constructor
        /// </summary>
        /// <param name="pos">Global position</param>
        /// <param name="quat">Rotation quaternion</param>
        public AUTD3(Vector3 pos, Quaternion quat)
        {
            Pos = pos;
            Rot = null;
            Quat = quat;
        }
    }

    public sealed class Transducer
    {
        private readonly GeometryPtr _ptr;

        internal Transducer(int trId, GeometryPtr ptr)
        {
            Idx = trId;
            _ptr = ptr;
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
                var pos = new float_t[3];
                Base.AUTDTransPosition(_ptr, (uint)Idx, pos);
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
                Base.AUTDTransRotation(_ptr, (uint)Idx, rot);
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
                Base.AUTDTransXDirection(_ptr, (uint)Idx, dir);
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
                Base.AUTDTransYDirection(_ptr, (uint)Idx, dir);
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
                Base.AUTDTransZDirection(_ptr, (uint)Idx, dir);
                return new Vector3(dir[0], dir[1], dir[2]);
            }
        }

        /// <summary>
        /// Frequency of the transducer
        /// </summary>
        public float_t Frequency
        {
            get => Base.AUTDGetTransFrequency(_ptr, (uint)Idx);
            set
            {
                var err = new byte[256];
                if (!Base.AUTDSetTransFrequency(_ptr, (uint)Idx, value, err))
                    throw new AUTDException(err);
            }
        }

        /// <summary>
        /// Cycle of the transducer
        /// </summary>
        public ushort Cycle
        {
            get => Base.AUTDGetTransCycle(_ptr, (uint)Idx);
            set
            {
                var err = new byte[256];
                if (!Base.AUTDSetTransCycle(_ptr, (uint)Idx, value, err))
                    throw new AUTDException(err);
            }
        }

        /// <summary>
        /// Modulation delay of the transducer
        /// </summary>
        public ushort ModDelay
        {
            get => Base.AUTDGetTransModDelay(_ptr, (uint)Idx);
            set => Base.AUTDSetTransModDelay(_ptr, (uint)Idx, value);
        }

        /// <summary>
        /// Wavelength of the transducer
        /// </summary>
        /// <param name="soundSpeed">Speed of sound</param>
        /// <returns></returns>
        public float_t Wavelength(float_t soundSpeed) => Base.AUTDGetWavelength(_ptr, (uint)Idx, soundSpeed);

        /// <summary>
        /// Wavenumber of the transducer
        /// </summary>
        /// <param name="soundSpeed">Speed of sound</param>
        /// <returns></returns>
        public float_t Wavenumber(float_t soundSpeed) => 2 * AUTD3.Pi / Wavelength(soundSpeed);
    }

    public sealed class Geometry : IEnumerable<Transducer>
    {
        internal GeometryPtr Ptr;
        internal readonly TransMode Mode;
        private List<Transducer> _transducers;

        internal Geometry(GeometryPtr ptr, TransMode mode)
        {
            Ptr = ptr;
            Mode = mode;
            _transducers = new List<Transducer>();
        }

        /// <summary>
        /// Number of transducers
        /// </summary>
        public int NumTransducers => (int)Base.AUTDNumTransducers(Ptr);

        /// <summary>
        /// Number of devices
        /// </summary>
        public int NumDevices => (int)Base.AUTDNumDevices(Ptr);

        /// <summary>
        /// Speed of sound
        /// </summary>
        public float_t SoundSpeed
        {
            get => Base.AUTDGetSoundSpeed(Ptr);
            set => Base.AUTDSetSoundSpeed(Ptr, value);
        }

        /// <summary>
        /// Attenuation coefficient
        /// </summary>
        public float_t Attenuation
        {
            get => Base.AUTDGetAttenuation(Ptr);
            set => Base.AUTDSetAttenuation(Ptr, value);
        }

        /// <summary>
        /// Get center position of all transducers
        /// </summary>
        public Vector3 Center
        {
            get
            {
                var center = new float_t[3];
                Base.AUTDGeometryCenter(Ptr, center);
                return new Vector3(center[0], center[1], center[2]);
            }
        }

        public Transducer this[int index] => _transducers[index];

        /// <summary>
        /// Set the sound speed from temperature
        /// </summary>
        /// <param name="temp">Temperature in celsius</param>
        /// <param name="k">Ratio of specific heat</param>
        /// <param name="r">Gas constant</param>
        /// <param name="m">Molar mass</param>
        public void SetSoundSpeedFromTemp(float_t temp, float_t k = (float_t)1.4, float_t r = (float_t)8.31446261815324, float_t m = (float_t)28.9647e-3)
        {
            Base.AUTDSetSoundSpeedFromTemp(Ptr, temp, k, r, m);
        }

        /// <summary>
        /// Get center position of transducers in the specified device
        /// </summary>
        /// <param name="devIdx"></param>
        /// <returns></returns>
        public Vector3 CenterOf(int devIdx)
        {
            var center = new float_t[3];
            Base.AUTDGeometryCenterOf(Ptr, (uint)devIdx, center);
            return new Vector3(center[0], center[1], center[2]);
        }

        public IEnumerator<Transducer> GetEnumerator() => _transducers.GetEnumerator();

        System.Collections.IEnumerator System.Collections.IEnumerable.GetEnumerator() => GetEnumerator();

        internal void Configure()
        {
            _transducers = Enumerable.Range(0, NumTransducers).Select(i => new Transducer(i, Ptr)).ToList();
        }
    }

    public readonly struct FPGAInfo
    {
        private readonly byte _info;

        internal FPGAInfo(byte info)
        {
            _info = info;
        }

        /// <summary>
        /// Check if thermal sensor is asserted
        /// </summary>
        public bool IsThermalAssert => (_info & 0x01) != 0;
    }

    /// <summary>
    /// Controller class for AUTD3
    /// </summary>
    public sealed class Controller : IDisposable
    {
        #region field

        private bool _isDisposed;
        internal ControllerPtr Ptr;
        private readonly TransMode _mode;

        #endregion

        #region Controller

        public class ControllerBuilder
        {

            private ControllerBuilderPtr _ptr;
            private TransMode _mode;

            /// <summary>
            /// Add device
            /// </summary>
            /// <param name="device">AUTD3 device</param>
            /// <returns></returns>
            public ControllerBuilder AddDevice(AUTD3 device)
            {
                if (device.Rot != null)
                    _ptr = Base.AUTDAddDevice(_ptr, device.Pos.x, device.Pos.y, device.Pos.z, device.Rot.Value.x, device.Rot.Value.y, device.Rot.Value.z);
                else if (device.Quat != null)
                    _ptr = Base.AUTDAddDeviceQuaternion(_ptr, device.Pos.x, device.Pos.y, device.Pos.z, device.Quat.Value.w, device.Quat.Value.x, device.Quat.Value.y, device.Quat.Value.z);
                return this;
            }

            /// <summary>
            /// Set legacy mode
            /// </summary>
            /// <returns></returns>
            public ControllerBuilder LegacyMode()
            {
                _mode = TransMode.Legacy;
                return this;
            }

            /// <summary>
            /// Set advanced mode
            /// </summary>
            /// <returns></returns>
            public ControllerBuilder AdvancedMode()
            {
                _mode = TransMode.Advanced;
                return this;
            }

            /// <summary>
            /// Set advanced phase mode
            /// </summary>
            /// <returns></returns>
            public ControllerBuilder AdvancedPhaseMode()
            {
                _mode = TransMode.AdvancedPhase;
                return this;
            }

            /// <summary>
            /// Open controller
            /// </summary>
            /// <param name="link">link</param>
            /// <returns>Controller</returns>
            public Controller OpenWith(Link.Link link)
            {
                return Controller.OpenImpl(_ptr, _mode, link.Ptr);
            }

            internal ControllerBuilder()
            {
                _ptr = Base.AUTDCreateControllerBuilder();
                _mode = TransMode.Legacy;
            }
        }

        /// <summary>
        /// Create Controller builder
        /// </summary>
        /// <returns>ControllerBuilder</returns>
        public static ControllerBuilder Builder() { return new ControllerBuilder(); }

        internal static Controller OpenImpl(ControllerBuilderPtr builder, TransMode mode, LinkPtr link)
        {
            var err = new byte[256];
            var ptr = Base.AUTDControllerOpenWith(builder, link, err);
            if (ptr._0 == IntPtr.Zero)
                throw new AUTDException(err);

            var geometry = new Geometry(Base.AUTDGetGeometry(ptr), mode);

            var cnt = new Controller(geometry, ptr, mode);

            cnt.Geometry.Configure();

            return cnt;
        }

        private Controller(Geometry geometry, ControllerPtr ptr, TransMode mode)
        {
            Ptr = ptr;
            Geometry = geometry;
            _mode = mode;
        }

        /// <summary>
        /// Get list of FPGA information
        /// </summary>
        /// <exception cref="AUTDException"></exception>
        public IEnumerable<FirmwareInfo> FirmwareInfoList()
        {
            var err = new byte[256];
            var handle = Base.AUTDGetFirmwareInfoListPointer(Ptr, err);
            if (handle._0 == IntPtr.Zero)
                throw new AUTDException(err);

            for (uint i = 0; i < Geometry.NumDevices; i++)
            {
                var info = new byte[256];
                var props = new bool[2];
                Base.AUTDGetFirmwareInfo(handle, i, info, props);
                yield return new FirmwareInfo(System.Text.Encoding.UTF8.GetString(info), props[0], props[1]);
            }

            Base.AUTDFreeFirmwareInfoListPointer(handle);
        }

        /// <summary>
        /// Close connection
        /// </summary>
        /// <exception cref="AUTDException"></exception>
        public void Close()
        {
            if (Ptr._0 == IntPtr.Zero) return;
            var err = new byte[256];
            if (!Base.AUTDClose(Ptr, err))
                throw new AUTDException(err);
        }

        public void Dispose()
        {
            if (_isDisposed) return;

            if (Ptr._0 != IntPtr.Zero) Base.AUTDFreeController(Ptr);
            Ptr._0 = IntPtr.Zero;

            _isDisposed = true;
            GC.SuppressFinalize(this);
        }

        ~Controller()
        {
            Dispose();
        }

        #endregion

        #region Property
        public Geometry Geometry { get; }

        /// <summary>
        /// set force fan flag
        /// </summary>
        /// <param name="value"></param>
        public void ForceFan(bool value)
        {
            Base.AUTDSetForceFan(Ptr, value);
        }

        /// <summary>
        /// set reads FPGA info flag
        /// </summary>
        /// <param name="value"></param>
        public void ReadsFPGAInfo(bool value)
        {
            Base.AUTDSetReadsFPGAInfo(Ptr, value);
        }

        /// <summary>
        /// List of FPGA information
        /// </summary>
        public FPGAInfo[] FPGAInfo
        {
            get
            {
                var infos = new byte[Geometry.NumDevices];
                var err = new byte[256];
                if (!Base.AUTDGetFPGAInfo(Ptr, infos, err))
                    throw new AUTDException(err);
                return infos.Select(x => new FPGAInfo(x)).ToArray();
            }
        }
        #endregion

        /// <summary>
        /// Send data to the devices
        /// </summary>
        /// <param name="special">Special data (Clear, Synchronize, Stop, ModDelay, or UpdateFlag)</param>
        /// <param name="timeout"></param>
        /// <returns> If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the data has been sent reliably or not.</returns>
        /// <exception cref="ArgumentNullException"></exception>
        /// <exception cref="AUTDException"></exception>
        public bool Send(ISpecialData special, TimeSpan? timeout = null)
        {
            if (special == null) throw new ArgumentNullException(nameof(special));
            var err = new byte[256];
            var res = Base.AUTDSendSpecial(Ptr, _mode, special.Ptr(), (long)(timeout?.TotalMilliseconds * 1000 * 1000 ?? -1), err);
            if (res == Def.Autd3Err)
            {
                throw new AUTDException(err);
            }
            return res == Def.Autd3True;
        }

        /// <summary>
        /// Send data to the devices
        /// </summary>
        /// <param name="header">Header data (SilencerConfig or Modulation)</param>
        /// <param name="timeout"></param>
        /// <returns> If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the data has been sent reliably or not.</returns>
        /// <exception cref="ArgumentNullException"></exception>
        /// <exception cref="AUTDException"></exception>
        public bool Send(IHeader header, TimeSpan? timeout = null)
        {
            if (header == null) throw new ArgumentNullException(nameof(header));
            var err = new byte[256];
            var res = Base.AUTDSend(Ptr, _mode, header.Ptr(), new DatagramBodyPtr(), (long)(timeout?.TotalMilliseconds * 1000 * 1000 ?? -1), err);
            if (res == Def.Autd3Err)
            {
                throw new AUTDException(err);
            }
            return res == Def.Autd3True;
        }

        /// <summary>
        /// Send data to the devices
        /// </summary>
        /// <param name="body">Body data (Gain, STM, or Amplitudes)</param>
        /// <param name="timeout"></param>
        /// <returns> If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the data has been sent reliably or not.</returns>
        /// <exception cref="ArgumentNullException"></exception>
        /// <exception cref="AUTDException"></exception>
        public bool Send(IBody body, TimeSpan? timeout = null)
        {
            if (body == null) throw new ArgumentNullException(nameof(body));
            var err = new byte[256];
            var res = Base.AUTDSend(Ptr, _mode, new DatagramHeaderPtr(), body.Ptr(Geometry), (long)(timeout?.TotalMilliseconds * 1000 * 1000 ?? -1), err);
            if (res == Def.Autd3Err)
            {
                throw new AUTDException(err);
            }
            return res == Def.Autd3True;
        }

        /// <summary>
        /// Send data to the devices
        /// </summary>
        /// <param name="header">Header data (SilencerConfig or Modulation)</param>
        /// <param name="body">Body data (Gain, STM, or Amplitudes)</param>
        /// <param name="timeout"></param>
        /// <returns> If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the data has been sent reliably or not.</returns>
        /// <exception cref="ArgumentNullException"></exception>
        /// <exception cref="AUTDException"></exception>
        public bool Send(IHeader header, IBody body, TimeSpan? timeout = null)
        {
            if (header == null) throw new ArgumentNullException(nameof(header));
            if (body == null) throw new ArgumentNullException(nameof(body));
            var err = new byte[256];
            var res = Base.AUTDSend(Ptr, _mode, header.Ptr(), body.Ptr(Geometry), (long)(timeout?.TotalMilliseconds * 1000 * 1000 ?? -1), err);
            if (res == Def.Autd3Err)
            {
                throw new AUTDException(err);
            }
            return res == Def.Autd3True;

        }

        /// <summary>
        /// Send data to the devices
        /// </summary>
        /// <param name="body">Body data (Gain, STM, or Amplitudes)</param>
        /// <param name="header">Header data (SilencerConfig or Modulation)</param>
        /// <param name="timeout"></param>
        /// <returns> If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the data has been sent reliably or not.</returns>
        /// <exception cref="ArgumentNullException"></exception>
        /// <exception cref="AUTDException"></exception>
        public bool Send(IBody body, IHeader header, TimeSpan? timeout = null)
        {
            if (header == null) throw new ArgumentNullException(nameof(header));
            if (body == null) throw new ArgumentNullException(nameof(body));
            var err = new byte[256];
            var res = Base.AUTDSend(Ptr, _mode, header.Ptr(), body.Ptr(Geometry), (long)(timeout?.TotalMilliseconds * 1000 * 1000 ?? -1), err);
            if (res == Def.Autd3Err)
            {
                throw new AUTDException(err);
            }
            return res == Def.Autd3True;
        }

        /// <summary>
        /// Send data to the devices
        /// </summary>
        /// <param name="data">Tuple of header data (SilencerConfig or Modulation) and body data (Gain, STM, or Amplitudes)</param>
        /// <param name="header">Header data (SilencerConfig or Modulation)</param>
        /// <param name="timeout"></param>
        /// <returns> If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the data has been sent reliably or not.</returns>
        /// <exception cref="ArgumentNullException"></exception>
        /// <exception cref="AUTDException"></exception>
        public bool Send((IHeader, IBody) data, TimeSpan? timeout = null)
        {
            var (header, body) = data;
            if (header == null) throw new ArgumentNullException(nameof(header));
            if (body == null) throw new ArgumentNullException(nameof(body));
            var err = new byte[256];
            var res = Base.AUTDSend(Ptr, _mode, header.Ptr(), body.Ptr(Geometry), (long)(timeout?.TotalMilliseconds * 1000 * 1000 ?? -1), err);
            if (res == Def.Autd3Err)
            {
                throw new AUTDException(err);
            }
            return res == Def.Autd3True;
        }
    }

    /// <summary>
    /// SpecialData to update flags (Force fan flag and reads FPGA info flag)
    /// </summary>
    public sealed class UpdateFlags : ISpecialData
    {
        public DatagramSpecialPtr Ptr() => Base.AUTDUpdateFlags();
    }

    /// <summary>
    /// SpecialData for clear all data in devices
    /// </summary>
    public sealed class Clear : ISpecialData
    {
        public DatagramSpecialPtr Ptr() => Base.AUTDClear();
    }

    /// <summary>
    /// SpecialData to synchronize devices
    /// </summary>
    public sealed class Synchronize : ISpecialData
    {
        public DatagramSpecialPtr Ptr() => Base.AUTDSynchronize();
    }

    /// <summary>
    /// SpecialData to stop output
    /// </summary>
    public sealed class Stop : ISpecialData
    {
        public DatagramSpecialPtr Ptr() => Base.AUTDStop();
    }

    /// <summary>
    /// SpecialData to set modulation delay
    /// </summary>
    public sealed class ModDelayConfig : ISpecialData
    {
        public DatagramSpecialPtr Ptr() => Base.AUTDModDelayConfig();
    }

    /// <summary>
    /// Header to configure silencer
    /// </summary>
    public sealed class SilencerConfig : IHeader
    {
        private readonly ushort _step;

        /// <summary>
        /// Constructor
        /// </summary>
        /// <param name="step">Update step of silencer. The smaller step is, the quieter the output is.</param>
        public SilencerConfig(ushort step = 10)
        {
            _step = step;
        }

        public DatagramHeaderPtr Ptr() => Base.AUTDCreateSilencer(_step);

        /// <summary>
        /// Disable silencer
        /// </summary>
        /// <returns></returns>
        public static SilencerConfig None()
        {
            return new SilencerConfig(0xFFFF);
        }
    }

    /// <summary>
    /// Amplitudes settings for advanced phase mode
    /// </summary>
    public sealed class Amplitudes : IBody
    {
        private readonly float_t _amp;

        public Amplitudes(float_t amp = 1)
        {
            _amp = amp;
        }

        public DatagramBodyPtr Ptr(Geometry geometry) => Base.AUTDCreateAmplitudes(_amp);
    }
}
