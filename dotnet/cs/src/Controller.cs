/*
 * File: AUTD3Sharp.cs
 * Project: src
 * Created Date: 23/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/10/2023
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
using System.Runtime.InteropServices;
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
        public const int NumTransInUnit = (int)Def.NumTransInUnit;

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
        public const int NumTransInX = (int)Def.NumTransInX;

        /// <summary>
        /// Number of transducer in y-axis of AUTD3 device
        /// </summary>
        public const int NumTransInY = (int)Def.NumTransInY;

        /// <summary>
        /// FPGA main clock frequency
        /// </summary>
        public const uint FPGAClkFreq = Def.FpgaClkFreq;

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
        public const uint FPGASubClkFreq = Def.FpgaSubClkFreq;

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

        public override string ToString()
        {
            return $"Thermal assert = {IsThermalAssert}";
        }
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
                    _ptr = Base.AUTDControllerBuilderAddDevice(_ptr, device.Pos.x, device.Pos.y, device.Pos.z, device.Rot.Value.x,
                        device.Rot.Value.y, device.Rot.Value.z);
                else if (device.Quat != null)
                    _ptr = Base.AUTDControllerBuilderAddDeviceQuaternion(_ptr, device.Pos.x, device.Pos.y, device.Pos.z,
                        device.Quat.Value.w, device.Quat.Value.x, device.Quat.Value.y, device.Quat.Value.z);
                return this;
            }


            /// <summary>
            /// Create Controller builder (Legacy mode)
            /// </summary>
            /// <returns>ControllerBuilder</returns>
            public ControllerBuilder Legacy()
            {
                _mode = TransMode.Legacy;
                return this;
            }

            /// <summary>
            /// Create Controller builder (Advanced mode)
            /// </summary>
            /// <returns>ControllerBuilder</returns>
            public ControllerBuilder Advanced()
            {
                _mode = TransMode.Advanced;
                return this;
            }



            /// <summary>
            /// Create Controller builder (AdvancedPhase mode)
            /// </summary>
            /// <returns>ControllerBuilder</returns>
            public ControllerBuilder AdvancedPhase()
            {
                _mode = TransMode.AdvancedPhase;
                return this;
            }

            /// <summary>
            /// Open controller
            /// </summary>
            /// <param name="link">link</param>
            /// <returns>Controller</returns>
            public Controller OpenWith(Internal.Link link)
            {
                return OpenImpl(_ptr, _mode, link.Ptr);
            }

            internal ControllerBuilder()
            {
                _ptr = Base.AUTDControllerBuilder();
                _mode = TransMode.Legacy;
            }
        }

        /// <summary>
        /// Create Controller builder
        /// </summary>
        /// <returns>ControllerBuilder</returns>
        public static ControllerBuilder Builder()
        {
            return new ControllerBuilder();
        }

        internal static Controller OpenImpl(ControllerBuilderPtr builder, TransMode mode, LinkPtr link)
        {
            var err = new byte[256];
            var ptr = Base.AUTDControllerOpenWith(builder, link, err);
            if (ptr._0 == IntPtr.Zero)
                throw new AUTDException(err);

            var geometry = new Geometry(Base.AUTDGeometry(ptr), mode);

            return new Controller(geometry, ptr, mode);
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
            var handle = Base.AUTDControllerFirmwareInfoListPointer(Ptr, err);
            System.Diagnostics.Debug.WriteLine(handle._0);
            if (handle._0 == IntPtr.Zero)
                throw new AUTDException(err);

            for (uint i = 0; i < Geometry.NumDevices; i++)
            {
                var info = new byte[256];
                Base.AUTDControllerFirmwareInfoGet(handle, i, info);
                yield return new FirmwareInfo(System.Text.Encoding.UTF8.GetString(info));
            }

            Base.AUTDControllerFirmwareInfoListPointerDelete(handle);
        }

        /// <summary>
        /// Close connection
        /// </summary>
        /// <exception cref="AUTDException"></exception>
        public void Close()
        {
            if (Ptr._0 == IntPtr.Zero) return;
            var err = new byte[256];
            if (!Base.AUTDControllerClose(Ptr, err))
                throw new AUTDException(err);
        }

        public void Dispose()
        {
            if (_isDisposed) return;

            if (Ptr._0 != IntPtr.Zero) Base.AUTDControllerDelete(Ptr);
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
        /// List of FPGA information
        /// </summary>
        public FPGAInfo[] FPGAInfo
        {
            get
            {
                var infos = new byte[Geometry.NumDevices];
                var err = new byte[256];
                if (!Base.AUTDControllerFPGAInfo(Ptr, infos, err))
                    throw new AUTDException(err);
                return infos.Select(x => new FPGAInfo(x)).ToArray();
            }
        }

        #endregion


        public void NotifyLinkGeometryUpdated()
        {
            var err = new byte[256];
            if (!Base.AUTDControllerNotifyLinkGeometryUpdated(Ptr, err))
                throw new AUTDException(err);
        }

        /// <summary>
        /// Send data to the devices
        /// </summary>
        /// <param name="special">Special data (Stop)</param>
        /// <param name="timeout"></param>
        /// <returns> If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the data has been sent reliably or not.</returns>
        /// <exception cref="ArgumentNullException"></exception>
        /// <exception cref="AUTDException"></exception>
        public bool Send(ISpecialDatagram special, TimeSpan? timeout = null)
        {
            if (special == null) throw new ArgumentNullException(nameof(special));
            var err = new byte[256];
            var res = Base.AUTDControllerSendSpecial(Ptr, _mode, special.Ptr(),
                (long)(timeout?.TotalMilliseconds * 1000 * 1000 ?? -1), err);
            if (res == Def.Autd3Err)
            {
                throw new AUTDException(err);
            }

            return res == Def.Autd3True;
        }

        /// <summary>
        /// Send data to the devices
        /// </summary>
        /// <param name="data">Data</param>
        /// <param name="timeout"></param>
        /// <returns> If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the data has been sent reliably or not.</returns>
        /// <exception cref="ArgumentNullException"></exception>
        /// <exception cref="AUTDException"></exception>
        public bool Send(IDatagram data, TimeSpan? timeout = null)
        {
            return Send(data, new NullDatagram(), timeout);
        }

        /// <summary>
        /// Send data to the devices
        /// </summary>
        /// <param name="data1">First data</param>
        /// <param name="data2">Second data</param>
        /// <param name="timeout"></param>
        /// <returns> If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the data has been sent reliably or not.</returns>
        /// <exception cref="ArgumentNullException"></exception>
        /// <exception cref="AUTDException"></exception>
        public bool Send(IDatagram data1, IDatagram data2, TimeSpan? timeout = null)
        {
            if (data1 == null) throw new ArgumentNullException(nameof(data1));
            if (data2 == null) throw new ArgumentNullException(nameof(data2));
            var err = new byte[256];
            var res = Base.AUTDControllerSend(Ptr, _mode, data1.Ptr(Geometry), data2.Ptr(Geometry),
                (long)(timeout?.TotalMilliseconds * 1000 * 1000 ?? -1), err);
            if (res == Def.Autd3Err)
            {
                throw new AUTDException(err);
            }

            return res == Def.Autd3True;

        }

        /// <summary>
        /// Send data to the devices
        /// </summary>
        /// <param name="data">Tuple of data</param>
        /// <param name="timeout"></param>
        /// <returns> If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the data has been sent reliably or not.</returns>
        /// <exception cref="ArgumentNullException"></exception>
        /// <exception cref="AUTDException"></exception>
        public bool Send((IDatagram, IDatagram) data, TimeSpan? timeout = null)
        {
            var (data1, data2) = data;
            return Send(data1, data2, timeout);
        }

        public sealed class SoftwareSTMHandler
        {
            [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
            public delegate bool SoftwareSTMCallbackDelegate(IntPtr ptr, ulong i, ulong elapsed);

            internal struct Context
            {
                internal readonly Controller Controller;
                internal readonly Func<Controller, int, TimeSpan, bool> Callback;

                public Context(Controller controller, Func<Controller, int, TimeSpan, bool> callback)
                {
                    Controller = controller;
                    Callback = callback;
                }
            }

            private readonly ControllerPtr _ptr;
            private readonly Context _context;
            private TimerStrategy _strategy;

            public void Start(TimeSpan interval)
            {
                var intervalNs = (ulong)(interval.TotalMilliseconds * 1000 * 1000);
                SoftwareSTMCallbackDelegate callbackNative = (ptr, i, elapsed) =>
                {
                    var gch = GCHandle.FromIntPtr(ptr);
                    var context = (Context)gch.Target;
                    return context.Callback(context.Controller, (int)i,
                        TimeSpan.FromMilliseconds(elapsed / 1000.0 / 1000.0));
                };
                var err = new byte[256];
                var gch = GCHandle.Alloc(_context);
                if (Base.AUTDControllerSoftwareSTM(_ptr, Marshal.GetFunctionPointerForDelegate(callbackNative),
                        GCHandle.ToIntPtr(gch), _strategy, intervalNs, err) == Def.Autd3Err)
                {
                    gch.Free();
                    throw new AUTDException(err);
                }

                gch.Free();
            }

            public SoftwareSTMHandler WithTimerStrategy(TimerStrategy strategy)
            {
                _strategy = strategy;
                return this;
            }

            internal SoftwareSTMHandler(
                ControllerPtr ptr,
                Context context
            )
            {
                _ptr = ptr;
                _context = context;
                _strategy = TimerStrategy.Sleep;
            }
        }

        public SoftwareSTMHandler SoftwareSTM(Func<Controller, int, TimeSpan, bool> callback)
        {
            return new SoftwareSTMHandler(Ptr, new SoftwareSTMHandler.Context(this, callback));
        }

        public sealed class GroupGuard<TK>
            where TK : class
        {

            private Controller _controller;
            private readonly Func<Device, TK?> _map;
            private GroupKVMapPtr _kvMap;
            private IDictionary<TK, int> _keymap;
            private int _k;

            internal GroupGuard(Func<Device, TK?> map, Controller controller)
            {
                _controller = controller;
                _map = map;
                _kvMap = Base.AUTDControllerGroupCreateKVMap();
                _keymap = new Dictionary<TK, int>();
                _k = 0;
            }

            public GroupGuard<TK> Set(TK key, IDatagram data1, IDatagram data2, TimeSpan? timeout = null)
            {
                if (_keymap.ContainsKey(key)) throw new AUTDException("Key already exists");
                if (data1 == null) throw new ArgumentNullException(nameof(data1));
                if (data2 == null) throw new ArgumentNullException(nameof(data2));

                var timeoutNs = (long)(timeout?.TotalMilliseconds * 1000 * 1000 ?? -1);
                var ptr1 = data1.Ptr(_controller.Geometry);
                var ptr2 = data2.Ptr(_controller.Geometry);
                _keymap[key] = _k++;
                var err = new byte[256];
                _kvMap = Base.AUTDControllerGroupKVMapSet(_kvMap, _keymap[key], ptr1, ptr2, _controller._mode, timeoutNs, err);
                if (_kvMap._0 == IntPtr.Zero) throw new AUTDException(err);
                return this;
            }

            public GroupGuard<TK> Set(TK key, IDatagram data, TimeSpan? timeout = null)
            {
                return Set(key, data, new NullDatagram(), timeout);
            }

            public GroupGuard<TK> Set(TK key, (IDatagram, IDatagram) data, TimeSpan? timeout = null)
            {
                return Set(key, data.Item1, data.Item2, timeout);
            }

            public GroupGuard<TK> Set(TK key, ISpecialDatagram data, TimeSpan? timeout = null)
            {
                if (_keymap.ContainsKey(key)) throw new AUTDException("Key already exists");
                if (data == null) throw new ArgumentNullException(nameof(data));

                var timeoutNs = (long)(timeout?.TotalMilliseconds * 1000 * 1000 ?? -1);
                var ptr = data.Ptr();
                _keymap[key] = _k++;
                var err = new byte[256];
                _kvMap = Base.AUTDControllerGroupKVMapSetSpecial(_kvMap, _keymap[key], ptr, _controller._mode, timeoutNs, err);
                if (_kvMap._0 == IntPtr.Zero) throw new AUTDException(err);
                return this;
            }

            public void Send()
            {
                var map = _controller.Geometry.Select(dev =>
                {
                    if (!dev.Enable) return -1;
                    var k = _map(dev);
                    return k != null ? _keymap[k] : -1;
                }).ToArray();
                var err = new byte[256];
                if (Base.AUTDControllerGroup(_controller.Ptr, map, _kvMap, err) == Def.Autd3Err)
                    throw new AUTDException(err);
            }
        }

        public GroupGuard<TK> Group<TK>(Func<Device, TK?> map)
            where TK : class
        {
            return new GroupGuard<TK>(map, this);
        }
    }

    /// <summary>
    /// Datagram to update flags (Force fan flag and reads FPGA info flag)
    /// </summary>
    public sealed class UpdateFlags : IDatagram
    {
        public DatagramPtr Ptr(Geometry geometry) => Base.AUTDDatagramUpdateFlags();
    }

    /// <summary>
    /// Datagram for clear all data in devices
    /// </summary>
    public sealed class Clear : IDatagram
    {
        public DatagramPtr Ptr(Geometry geometry) => Base.AUTDDatagramClear();
    }

    /// <summary>
    /// Datagram to synchronize devices
    /// </summary>
    public sealed class Synchronize : IDatagram
    {
        public DatagramPtr Ptr(Geometry geometry) => Base.AUTDDatagramSynchronize();
    }

    /// <summary>
    /// SpecialData to stop output
    /// </summary>
    public sealed class Stop : ISpecialDatagram
    {
        public DatagramSpecialPtr Ptr() => Base.AUTDDatagramStop();
    }

    /// <summary>
    /// Datagram to set modulation delay
    /// </summary>
    public sealed class ConfigureModDelay : IDatagram
    {
        public DatagramPtr Ptr(Geometry geometry) => Base.AUTDDatagramConfigureModDelay();
    }

    /// <summary>
    /// Datagram to configure amp filter
    /// </summary>
    public sealed class ConfigureAmpFilter : IDatagram
    {
        public DatagramPtr Ptr(Geometry geometry) => Base.AUTDDatagramConfigureAmpFilter();
    }


    /// <summary>
    /// Datagram to configure phase filter
    /// </summary>
    public sealed class ConfigurePhaseFilter : IDatagram
    {
        public DatagramPtr Ptr(Geometry geometry) => Base.AUTDDatagramConfigurePhaseFilter();
    }

    /// <summary>
    /// Datagram to configure silencer
    /// </summary>
    public sealed class Silencer : IDatagram
    {
        private readonly ushort _step;

        /// <summary>
        /// Constructor
        /// </summary>
        /// <param name="step">Update step of silencer. The smaller step is, the quieter the output is.</param>
        public Silencer(ushort step = 10)
        {
            _step = step;
        }

        public DatagramPtr Ptr(Geometry geometry) => Base.AUTDDatagramSilencer(_step);

        /// <summary>
        /// Disable silencer
        /// </summary>
        /// <returns></returns>
        public static Silencer Disable()
        {
            return new Silencer(0xFFFF);
        }
    }

    /// <summary>
    /// Amplitudes settings for advanced phase mode
    /// </summary>
    public sealed class Amplitudes : IDatagram
    {
        private readonly float_t _amp;

        public static Amplitudes Uniform(float_t amp = 1) => new Amplitudes(amp);

        public Amplitudes(float_t amp = 1)
        {
            _amp = amp;
        }

        public DatagramPtr Ptr(Geometry geometry) => Base.AUTDDatagramAmplitudes(_amp);
    }
}
