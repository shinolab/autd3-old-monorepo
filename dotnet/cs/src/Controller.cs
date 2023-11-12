/*
 * File: AUTD3Sharp.cs
 * Project: src
 * Created Date: 23/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 * 
 */

#if UNITY_2018_3_OR_NEWER
#define USE_SINGLE
#define DIMENSION_M
#endif

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
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
        public const int NumTransInUnit = (int)NativeMethodsDef.NUM_TRANS_IN_UNIT;

        /// <summary>
        /// Spacing between transducers in mm
        /// </summary>
        public const float_t TransSpacingMm = NativeMethodsDef.TRANS_SPACING_MM;

        /// <summary>
        /// Spacing between transducers in m
        /// </summary>
        public const float_t TransSpacing = NativeMethodsDef.TRANS_SPACING_MM * Millimeter;

        /// <summary>
        /// Number of transducer in x-axis of AUTD3 device
        /// </summary>
        public const int NumTransInX = (int)NativeMethodsDef.NUM_TRANS_IN_X;

        /// <summary>
        /// Number of transducer in y-axis of AUTD3 device
        /// </summary>
        public const int NumTransInY = (int)NativeMethodsDef.NUM_TRANS_IN_Y;

        /// <summary>
        /// FPGA clock frequency
        /// </summary>
        public const uint FPGAClkFreq = NativeMethodsDef.FPGA_CLK_FREQ;

        /// <summary>
        /// Device height including substrate
        /// </summary>
        public const float_t DeviceHeight = NativeMethodsDef.DEVICE_HEIGHT_MM * Millimeter;

        /// <summary>
        /// Device width including substrate
        /// </summary>
        public const float_t DeviceWidth = NativeMethodsDef.DEVICE_WIDTH_MM * Millimeter;

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
        private readonly object? _linkProps;

        #endregion

        #region Controller

        public class ControllerBuilder
        {

            private ControllerBuilderPtr _ptr;

            /// <summary>
            /// Add device
            /// </summary>
            /// <param name="device">AUTD3 device</param>
            /// <returns></returns>
            public ControllerBuilder AddDevice(AUTD3 device)
            {
                if (device.Rot != null)
                    _ptr = NativeMethodsBase.AUTDControllerBuilderAddDevice(_ptr, device.Pos.x, device.Pos.y, device.Pos.z, device.Rot.Value.x,
                        device.Rot.Value.y, device.Rot.Value.z);
                else if (device.Quat != null)
                    _ptr = NativeMethodsBase.AUTDControllerBuilderAddDeviceQuaternion(_ptr, device.Pos.x, device.Pos.y, device.Pos.z,
                        device.Quat.Value.w, device.Quat.Value.x, device.Quat.Value.y, device.Quat.Value.z);
                return this;
            }

            /// <summary>
            /// Open controller
            /// </summary>
            /// <param name="link">link</param>
            /// <returns>Controller</returns>
            public async Task<Controller> OpenWithAsync(ILinkBuilder link)
            {
                var result = await Task.Run(() => NativeMethodsBase.AUTDControllerOpenWith(_ptr, link.Ptr()));
                if (result.result.Item1 == IntPtr.Zero)
                {
                    var err = new byte[result.err_len];
                    unsafe
                    {
                        fixed (byte* ep = err) NativeMethodsDef.AUTDGetErr(result.err, ep);
                        throw new AUTDException(err);
                    }
                }
                var ptr = result.result;
                var geometry = new Geometry(NativeMethodsBase.AUTDGeometry(ptr));
                return new Controller(geometry, ptr, link.Props());
            }

            /// <summary>
            /// Open controller
            /// </summary>
            /// <param name="link">link</param>
            /// <returns>Controller</returns>
            public Controller OpenWith(ILinkBuilder link)
            {
                var result = NativeMethodsBase.AUTDControllerOpenWith(_ptr, link.Ptr());
                if (result.result.Item1 == IntPtr.Zero)
                {
                    var err = new byte[result.err_len];
                    unsafe
                    {
                        fixed (byte* ep = err) NativeMethodsDef.AUTDGetErr(result.err, ep);
                        throw new AUTDException(err);
                    }
                }
                var ptr = result.result;
                var geometry = new Geometry(NativeMethodsBase.AUTDGeometry(ptr));
                return new Controller(geometry, ptr, link.Props());
            }

            internal ControllerBuilder()
            {
                _ptr = NativeMethodsBase.AUTDControllerBuilder();
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

        private Controller(Geometry geometry, ControllerPtr ptr, object? linkProps)
        {
            Ptr = ptr;
            Geometry = geometry;
            _linkProps = linkProps;
        }

        private ResultFirmwareInfoList GetFirmwareInfoListPtr()
        {

            var res = NativeMethodsBase.AUTDControllerFirmwareInfoListPointer(Ptr);
            if (res.result != IntPtr.Zero) return res;
            unsafe
            {
                var err = new byte[res.err_len];
                fixed (byte* ep = err) NativeMethodsDef.AUTDGetErr(res.err, ep);
                throw new AUTDException(err);
            }
        }

        private static FirmwareInfo GetFirmwareInfo(ResultFirmwareInfoList handle, uint i)
        {
            var info = new byte[256];
            unsafe
            {
                fixed (byte* p = info)
                {
                    NativeMethodsBase.AUTDControllerFirmwareInfoGet(handle, i, p);
                    return new FirmwareInfo(System.Text.Encoding.UTF8.GetString(info));
                }
            }
        }

        /// <summary>
        /// Get list of FPGA information
        /// </summary>
        /// <exception cref="AUTDException"></exception>
        public async Task<FirmwareInfo[]> FirmwareInfoListAsync()
        {
            var handle = await Task.Run(GetFirmwareInfoListPtr);
            var result = Enumerable.Range(0, Geometry.NumDevices).Select(i => GetFirmwareInfo(handle, (uint)i)).ToArray();
            NativeMethodsBase.AUTDControllerFirmwareInfoListPointerDelete(handle);
            return result;
        }

        /// <summary>
        /// Get list of FPGA information
        /// </summary>
        /// <exception cref="AUTDException"></exception>
        public FirmwareInfo[] FirmwareInfoList()
        {
            var handle = GetFirmwareInfoListPtr();
            var result = Enumerable.Range(0, Geometry.NumDevices).Select(i => GetFirmwareInfo(handle, (uint)i)).ToArray();
            NativeMethodsBase.AUTDControllerFirmwareInfoListPointerDelete(handle);
            return result;
        }

        /// <summary>
        /// Close connection
        /// </summary>
        /// <exception cref="AUTDException"></exception>
        public async Task<bool> CloseAsync()
        {
            if (Ptr.Item1 == IntPtr.Zero) return false;
            var res = await Task.Run(() => NativeMethodsBase.AUTDControllerClose(Ptr));
            if (res.result != NativeMethodsDef.AUTD3_ERR) return res.result == NativeMethodsDef.AUTD3_TRUE;
            var err = new byte[res.err_len];
            unsafe
            {
                fixed (byte* ep = err) NativeMethodsDef.AUTDGetErr(res.err, ep);
                throw new AUTDException(err);
            }
        }

        /// <summary>
        /// Close connection
        /// </summary>
        /// <exception cref="AUTDException"></exception>
        public bool Close()
        {
            if (Ptr.Item1 == IntPtr.Zero) return false;
            var res = NativeMethodsBase.AUTDControllerClose(Ptr);
            if (res.result != NativeMethodsDef.AUTD3_ERR) return res.result == NativeMethodsDef.AUTD3_TRUE;
            var err = new byte[res.err_len];
            unsafe
            {
                fixed (byte* ep = err) NativeMethodsDef.AUTDGetErr(res.err, ep);
                throw new AUTDException(err);
            }
        }

        public void Dispose()
        {
            if (_isDisposed) return;

            if (Ptr.Item1 != IntPtr.Zero) NativeMethodsBase.AUTDControllerDelete(Ptr);
            Ptr.Item1 = IntPtr.Zero;

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
        public async Task<FPGAInfo[]> FPGAInfoAsync()
        {
            var infos = new byte[Geometry.NumDevices];
            var res = await Task.Run(() =>
            {
                unsafe
                {
                    fixed (byte* ptr = infos)
                        return NativeMethodsBase.AUTDControllerFPGAInfo(Ptr, ptr);
                }
            });
            if (res.result != NativeMethodsDef.AUTD3_ERR) return infos.Select(x => new FPGAInfo(x)).ToArray();
            var err = new byte[res.err_len];
            unsafe
            {
                fixed (byte* ep = err) NativeMethodsDef.AUTDGetErr(res.err, ep);
                throw new AUTDException(err);
            }
        }


        /// <summary>
        /// List of FPGA information
        /// </summary>
        public FPGAInfo[] FPGAInfo()
        {
            var infos = new byte[Geometry.NumDevices];
            unsafe
            {
                fixed (byte* ptr = infos)
                {
                    var res = NativeMethodsBase.AUTDControllerFPGAInfo(Ptr, ptr);
                    if (res.result != NativeMethodsDef.AUTD3_ERR) return infos.Select(x => new FPGAInfo(x)).ToArray();
                    var err = new byte[res.err_len];
                    fixed (byte* ep = err) NativeMethodsDef.AUTDGetErr(res.err, ep);
                    throw new AUTDException(err);
                }
            }
        }

        public T Link<T>()
            where T : ILink<T>, new()
        {
            return new T().Create(NativeMethodsBase.AUTDLinkGet(Ptr), _linkProps);
        }

        #endregion

        /// <summary>
        /// Send data to the devices
        /// </summary>
        /// <param name="special">Special data (Stop)</param>
        /// <param name="timeout"></param>
        /// <returns> If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the data has been sent reliably or not.</returns>
        /// <exception cref="ArgumentNullException"></exception>
        /// <exception cref="AUTDException"></exception>
        public async Task<bool> SendAsync(ISpecialDatagram special, TimeSpan? timeout = null)
        {
            if (special == null) throw new ArgumentNullException(nameof(special));

            var res = await Task.Run(() =>
                NativeMethodsBase.AUTDControllerSendSpecial(Ptr, special.Ptr(),
                    (long)(timeout?.TotalMilliseconds * 1000 * 1000 ?? -1)));
            if (res.result != NativeMethodsDef.AUTD3_ERR) return res.result == NativeMethodsDef.AUTD3_TRUE;
            var err = new byte[res.err_len];
            unsafe
            {
                fixed (byte* ep = err) NativeMethodsDef.AUTDGetErr(res.err, ep);
                throw new AUTDException(err);
            }
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

            var res = NativeMethodsBase.AUTDControllerSendSpecial(Ptr, special.Ptr(),
                    (long)(timeout?.TotalMilliseconds * 1000 * 1000 ?? -1));
            if (res.result != NativeMethodsDef.AUTD3_ERR) return res.result == NativeMethodsDef.AUTD3_TRUE;
            var err = new byte[res.err_len];
            unsafe
            {
                fixed (byte* ep = err) NativeMethodsDef.AUTDGetErr(res.err, ep);
                throw new AUTDException(err);
            }
        }

        /// <summary>
        /// Send data to the devices
        /// </summary>
        /// <param name="data">Data</param>
        /// <param name="timeout"></param>
        /// <returns> If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the data has been sent reliably or not.</returns>
        /// <exception cref="ArgumentNullException"></exception>
        /// <exception cref="AUTDException"></exception>
        public async Task<bool> SendAsync(IDatagram data, TimeSpan? timeout = null)
        {
            return await SendAsync(data, new NullDatagram(), timeout);
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
        public async Task<bool> SendAsync(IDatagram data1, IDatagram data2, TimeSpan? timeout = null)
        {
            if (data1 == null) throw new ArgumentNullException(nameof(data1));
            if (data2 == null) throw new ArgumentNullException(nameof(data2));

            var res = await Task.Run(() => NativeMethodsBase.AUTDControllerSend(Ptr, data1.Ptr(Geometry), data2.Ptr(Geometry),
                (long)(timeout?.TotalMilliseconds * 1000 * 1000 ?? -1)));
            if (res.result != NativeMethodsDef.AUTD3_ERR) return res.result == NativeMethodsDef.AUTD3_TRUE;
            var err = new byte[res.err_len];
            unsafe
            {
                fixed (byte* ep = err) NativeMethodsDef.AUTDGetErr(res.err, ep);
                throw new AUTDException(err);
            }
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

            var res = NativeMethodsBase.AUTDControllerSend(Ptr, data1.Ptr(Geometry), data2.Ptr(Geometry),
                (long)(timeout?.TotalMilliseconds * 1000 * 1000 ?? -1));
            if (res.result != NativeMethodsDef.AUTD3_ERR) return res.result == NativeMethodsDef.AUTD3_TRUE;
            var err = new byte[res.err_len];
            unsafe
            {
                fixed (byte* ep = err) NativeMethodsDef.AUTDGetErr(res.err, ep);
                throw new AUTDException(err);
            }
        }

        /// <summary>
        /// Send data to the devices
        /// </summary>
        /// <param name="data">Tuple of data</param>
        /// <param name="timeout"></param>
        /// <returns> If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the data has been sent reliably or not.</returns>
        /// <exception cref="ArgumentNullException"></exception>
        /// <exception cref="AUTDException"></exception>
        public async Task<bool> SendAsync((IDatagram, IDatagram) data, TimeSpan? timeout = null)
        {
            var (data1, data2) = data;
            return await SendAsync(data1, data2, timeout);
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

        public sealed class GroupGuard
        {
            private readonly Controller _controller;
            private readonly Func<Device, object?> _map;
            private ResultGroupKVMap _kvMap;
            private readonly IDictionary<object, int> _keymap;
            private int _k;

            internal GroupGuard(Func<Device, object?> map, Controller controller)
            {
                _controller = controller;
                _map = map;
                _kvMap = NativeMethodsBase.AUTDControllerGroupCreateKVMap();
                _keymap = new Dictionary<object, int>();
                _k = 0;
            }

            public GroupGuard Set(object key, IDatagram data1, IDatagram data2, TimeSpan? timeout = null)
            {
                if (_keymap.ContainsKey(key)) throw new AUTDException("Key already exists");
                if (data1 == null) throw new ArgumentNullException(nameof(data1));
                if (data2 == null) throw new ArgumentNullException(nameof(data2));

                var timeoutNs = (long)(timeout?.TotalMilliseconds * 1000 * 1000 ?? -1);
                var ptr1 = data1.Ptr(_controller.Geometry);
                var ptr2 = data2.Ptr(_controller.Geometry);
                _keymap[key] = _k++;
                _kvMap = NativeMethodsBase.AUTDControllerGroupKVMapSet(_kvMap, _keymap[key], ptr1, ptr2, timeoutNs);
                if (_kvMap.result != IntPtr.Zero) return this;
                var err = new byte[_kvMap.err_len];
                unsafe
                {
                    fixed (byte* ep = err) NativeMethodsDef.AUTDGetErr(_kvMap.err, ep);
                    throw new AUTDException(err);
                }
            }

            public GroupGuard Set(object key, IDatagram data, TimeSpan? timeout = null)
            {
                return Set(key, data, new NullDatagram(), timeout);
            }

            public GroupGuard Set(object key, (IDatagram, IDatagram) data, TimeSpan? timeout = null)
            {
                return Set(key, data.Item1, data.Item2, timeout);
            }

            public GroupGuard Set(object key, ISpecialDatagram data, TimeSpan? timeout = null)
            {
                if (_keymap.ContainsKey(key)) throw new AUTDException("Key already exists");
                if (data == null) throw new ArgumentNullException(nameof(data));

                var timeoutNs = (long)(timeout?.TotalMilliseconds * 1000 * 1000 ?? -1);
                var ptr = data.Ptr();
                _keymap[key] = _k++;
                _kvMap = NativeMethodsBase.AUTDControllerGroupKVMapSetSpecial(_kvMap, _keymap[key], ptr, timeoutNs);
                if (_kvMap.result != IntPtr.Zero) return this;
                var err = new byte[_kvMap.err_len];
                unsafe
                {
                    fixed (byte* ep = err) NativeMethodsDef.AUTDGetErr(_kvMap.err, ep);
                    throw new AUTDException(err);
                }
            }

            public async Task SendAsync()
            {
                var map = _controller.Geometry.Select(dev =>
                {
                    if (!dev.Enable) return -1;
                    var k = _map(dev);
                    return k != null ? _keymap[k] : -1;
                }).ToArray();

                var res = await Task.Run(() =>
                {
                    unsafe
                    {
                        fixed (int* mp = map)
                            return NativeMethodsBase.AUTDControllerGroup(_controller.Ptr, mp, _kvMap);
                    }
                });
                if (res.result != NativeMethodsDef.AUTD3_ERR) return;
                var err = new byte[res.err_len];
                unsafe
                {
                    fixed (byte* ep = err) NativeMethodsDef.AUTDGetErr(res.err, ep);
                    throw new AUTDException(err);
                }
            }

            public void Send()
            {
                var map = _controller.Geometry.Select(dev =>
                {
                    if (!dev.Enable) return -1;
                    var k = _map(dev);
                    return k != null ? _keymap[k] : -1;
                }).ToArray();
                unsafe
                {
                    fixed (int* mp = map)
                    {
                        var res = NativeMethodsBase.AUTDControllerGroup(_controller.Ptr, mp, _kvMap);
                        if (res.result != NativeMethodsDef.AUTD3_ERR) return;
                        var err = new byte[res.err_len];
                        fixed (byte* ep = err) NativeMethodsDef.AUTDGetErr(res.err, ep);
                        throw new AUTDException(err);
                    }
                }
            }
        }

        public GroupGuard Group(Func<Device, object?> map)
        {
            return new GroupGuard(map, this);
        }
    }

    /// <summary>
    /// Datagram to update flags (Force fan flag and reads FPGA info flag)
    /// </summary>
    public sealed class UpdateFlags : IDatagram
    {
        DatagramPtr IDatagram.Ptr(Geometry geometry) => NativeMethodsBase.AUTDDatagramUpdateFlags();
    }

    /// <summary>
    /// Datagram for clear all data in devices
    /// </summary>
    public sealed class Clear : IDatagram
    {
        DatagramPtr IDatagram.Ptr(Geometry geometry) => NativeMethodsBase.AUTDDatagramClear();
    }

    /// <summary>
    /// Datagram to synchronize devices
    /// </summary>
    public sealed class Synchronize : IDatagram
    {
        DatagramPtr IDatagram.Ptr(Geometry geometry) => NativeMethodsBase.AUTDDatagramSynchronize();
    }

    /// <summary>
    /// SpecialData to stop output
    /// </summary>
    public sealed class Stop : ISpecialDatagram
    {
        DatagramSpecialPtr ISpecialDatagram.Ptr() => NativeMethodsBase.AUTDDatagramStop();
    }

    /// <summary>
    /// Datagram to set modulation delay
    /// </summary>
    public sealed class ConfigureModDelay : IDatagram
    {
        DatagramPtr IDatagram.Ptr(Geometry geometry) => NativeMethodsBase.AUTDDatagramConfigureModDelay();
    }

    /// <summary>
    /// Datagram to configure amp filter
    /// </summary>
    public sealed class ConfigureAmpFilter : IDatagram
    {
        DatagramPtr IDatagram.Ptr(Geometry geometry) => NativeMethodsBase.AUTDDatagramConfigureAmpFilter();
    }


    /// <summary>
    /// Datagram to configure phase filter
    /// </summary>
    public sealed class ConfigurePhaseFilter : IDatagram
    {
        DatagramPtr IDatagram.Ptr(Geometry geometry) => NativeMethodsBase.AUTDDatagramConfigurePhaseFilter();
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

        DatagramPtr IDatagram.Ptr(Geometry geometry) => NativeMethodsBase.AUTDDatagramSilencer(_step);

        /// <summary>
        /// Disable silencer
        /// </summary>
        /// <returns></returns>
        public static Silencer Disable()
        {
            return new Silencer(0xFFFF);
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
