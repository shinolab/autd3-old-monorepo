/*
 * File: AUTD3Sharp.cs
 * Project: src
 * Created Date: 23/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 07/12/2023
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
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using System.Diagnostics.CodeAnalysis;
using AUTD3Sharp.Internal;
using AUTD3Sharp.NativeMethods;

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
        internal Quaternion? Rot;

        /// <summary>
        /// Constructor
        /// </summary>
        /// <param name="pos">Global position</param>
        public AUTD3(Vector3 pos)
        {
            Pos = pos;
            Rot = null;
        }

        public AUTD3 WithRotation(Quaternion rot)
        {
            Rot = rot;
            return this;
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
    public sealed class Controller<T> : IDisposable
    {
        #region field

        private bool _isDisposed;
        internal ControllerPtr Ptr;

        #endregion

        #region Controller

        internal Controller(Geometry geometry, ControllerPtr ptr, T link)
        {
            Ptr = ptr;
            Geometry = geometry;
            Link = link;
        }

        private static FirmwareInfo GetFirmwareInfo(FirmwareInfoListPtr handle, uint i)
        {
            var info = new byte[256];
            unsafe
            {
                fixed (byte* p = &info[0])
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
            var handle = await Task.Run(() => NativeMethodsBase.AUTDControllerFirmwareInfoListPointer(Ptr).Validate());
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
            var handle = NativeMethodsBase.AUTDControllerFirmwareInfoListPointer(Ptr).Validate();
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
            return await Task.Run(() => NativeMethodsBase.AUTDControllerClose(Ptr).Validate() == NativeMethodsDef.AUTD3_TRUE);
        }

        /// <summary>
        /// Close connection
        /// </summary>
        /// <exception cref="AUTDException"></exception>
        public bool Close()
        {
            return NativeMethodsBase.AUTDControllerClose(Ptr).Validate() == NativeMethodsDef.AUTD3_TRUE;
        }

        public void Dispose()
        {
            if (_isDisposed) return;

            if (Ptr.Item1 != IntPtr.Zero) NativeMethodsBase.AUTDControllerDelete(Ptr);
            Ptr.Item1 = IntPtr.Zero;

            _isDisposed = true;
            GC.SuppressFinalize(this);
        }


        [ExcludeFromCodeCoverage]
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
            await Task.Run(() =>
            {
                unsafe
                {
                    fixed (byte* ptr = &infos[0])
                        return NativeMethodsBase.AUTDControllerFPGAInfo(Ptr, ptr).Validate();
                }
            });
            return infos.Select(x => new FPGAInfo(x)).ToArray();
        }


        /// <summary>
        /// List of FPGA information
        /// </summary>
        public FPGAInfo[] FPGAInfo()
        {
            var infos = new byte[Geometry.NumDevices];
            unsafe
            {
                fixed (byte* ptr = &infos[0])
                {
                    NativeMethodsBase.AUTDControllerFPGAInfo(Ptr, ptr).Validate();
                    return infos.Select(x => new FPGAInfo(x)).ToArray();
                }
            }
        }

        public T Link { get; }

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
            return await Task.Run(() =>
               NativeMethodsBase.AUTDControllerSendSpecial(Ptr, special.Ptr(),
                   (long)(timeout?.TotalMilliseconds * 1000 * 1000 ?? -1)).Validate() == NativeMethodsDef.AUTD3_TRUE);
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
            return NativeMethodsBase.AUTDControllerSendSpecial(Ptr, special.Ptr(),
                    (long)(timeout?.TotalMilliseconds * 1000 * 1000 ?? -1)).Validate() == NativeMethodsDef.AUTD3_TRUE;
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
            return await Task.Run(() => NativeMethodsBase.AUTDControllerSend(Ptr, data1.Ptr(Geometry), data2.Ptr(Geometry),
                (long)(timeout?.TotalMilliseconds * 1000 * 1000 ?? -1)).Validate() == NativeMethodsDef.AUTD3_TRUE);
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
            return NativeMethodsBase.AUTDControllerSend(Ptr, data1.Ptr(Geometry), data2.Ptr(Geometry),
                (long)(timeout?.TotalMilliseconds * 1000 * 1000 ?? -1)).Validate() == NativeMethodsDef.AUTD3_TRUE;
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
            private readonly Controller<T> _controller;
            private readonly Func<Device, object?> _map;
            private GroupKVMapPtr _kvMap;
            private readonly IDictionary<object, int> _keymap;
            private int _k;

            internal GroupGuard(Func<Device, object?> map, Controller<T> controller)
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

                var timeoutNs = (long)(timeout?.TotalMilliseconds * 1000 * 1000 ?? -1);
                var ptr1 = data1.Ptr(_controller.Geometry);
                var ptr2 = data2.Ptr(_controller.Geometry);
                _keymap[key] = _k++;
                _kvMap = NativeMethodsBase.AUTDControllerGroupKVMapSet(_kvMap, _keymap[key], ptr1, ptr2, timeoutNs).Validate();
                return this;
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

                var timeoutNs = (long)(timeout?.TotalMilliseconds * 1000 * 1000 ?? -1);
                var ptr = data.Ptr();
                _keymap[key] = _k++;
                _kvMap = NativeMethodsBase.AUTDControllerGroupKVMapSetSpecial(_kvMap, _keymap[key], ptr, timeoutNs).Validate();
                return this;
            }

            public async Task<bool> SendAsync()
            {
                var map = _controller.Geometry.Select(dev =>
                {
                    if (!dev.Enable) return -1;
                    var k = _map(dev);
                    return k != null ? _keymap[k] : -1;
                }).ToArray();

                return await Task.Run(() =>
                 {
                     unsafe
                     {
                         fixed (int* mp = &map[0])
                             return NativeMethodsBase.AUTDControllerGroup(_controller.Ptr, mp, _kvMap).Validate() == NativeMethodsDef.AUTD3_TRUE;
                     }
                 });
            }

            public bool Send()
            {
                var map = _controller.Geometry.Select(dev =>
                {
                    if (!dev.Enable) return -1;
                    var k = _map(dev);
                    return k != null ? _keymap[k] : -1;
                }).ToArray();
                unsafe
                {
                    fixed (int* mp = &map[0])
                    {
                        return NativeMethodsBase.AUTDControllerGroup(_controller.Ptr, mp, _kvMap).Validate() == NativeMethodsDef.AUTD3_TRUE;
                    }
                }
            }
        }

        public GroupGuard Group(Func<Device, object?> map)
        {
            return new GroupGuard(map, this);
        }
    }

    public class ControllerBuilder
    {
        private ControllerBuilderPtr _ptr = NativeMethodsBase.AUTDControllerBuilder();

        /// <summary>
        /// Add device
        /// </summary>
        /// <param name="device">AUTD3 device</param>
        /// <returns></returns>
        public ControllerBuilder AddDevice(AUTD3 device)
        {
            var rot = device.Rot ?? Quaternion.identity;
            _ptr = NativeMethodsBase.AUTDControllerBuilderAddDevice(_ptr, device.Pos.x, device.Pos.y, device.Pos.z,
                rot.w, rot.x, rot.y, rot.z);
            return this;
        }

        /// <summary>
        /// Open controller
        /// </summary>
        /// <param name="linkBuilder">link</param>
        /// <returns>Controller</returns>
        public async Task<Controller<T>> OpenWithAsync<T>(ILinkBuilder<T> linkBuilder)
        {
            var ptr = await Task.Run(() => NativeMethodsBase.AUTDControllerOpenWith(_ptr, linkBuilder.Ptr()).Validate());
            var geometry = new Geometry(NativeMethodsBase.AUTDGeometry(ptr));
            var link = linkBuilder.ResolveLink(NativeMethodsBase.AUTDLinkGet(ptr));
            return new Controller<T>(geometry, ptr, link);
        }

        /// <summary>
        /// Open controller
        /// </summary>
        /// <param name="linkBuilder">link</param>
        /// <returns>Controller</returns>
        public Controller<T> OpenWith<T>(ILinkBuilder<T> linkBuilder)
        {
            var ptr = NativeMethodsBase.AUTDControllerOpenWith(_ptr, linkBuilder.Ptr()).Validate();
            var geometry = new Geometry(NativeMethodsBase.AUTDGeometry(ptr));
            var link = linkBuilder.ResolveLink(NativeMethodsBase.AUTDLinkGet(ptr));
            return new Controller<T>(geometry, ptr, link);
        }
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
        [UnmanagedFunctionPointer(CallingConvention.Cdecl)] public delegate ushort ConfigureModDelayDelegate(IntPtr context, GeometryPtr geometryPtr, uint devIdx, byte trIdx);

        private readonly ConfigureModDelayDelegate _f;

        public ConfigureModDelay(Func<Device, Transducer, ushort> f)
        {
            _f = (context, geometryPtr, devIdx, trIdx) =>
            {
                var dev = new Device((int)devIdx, NativeMethodsBase.AUTDDevice(geometryPtr, devIdx));
                var tr = new Transducer(trIdx, dev.Ptr);
                return f(dev, tr);
            };
        }

        DatagramPtr IDatagram.Ptr(Geometry geometry) => NativeMethodsBase.AUTDDatagramConfigureModDelay(Marshal.GetFunctionPointerForDelegate(_f), IntPtr.Zero, geometry.Ptr);
    }

    /// <summary>
    /// Datagram to configure debug output
    /// </summary>
    public sealed class ConfigureDebugOutputIdx : IDatagram
    {
        [UnmanagedFunctionPointer(CallingConvention.Cdecl)] public delegate byte DebugOutputDelegate(IntPtr context, GeometryPtr geometryPtr, uint devIdx);

        private readonly DebugOutputDelegate _f;

        public ConfigureDebugOutputIdx(Func<Device, Transducer?> f)
        {
            _f = (context, geometryPtr, devIdx) =>
            {
                var tr = f(new Device((int)devIdx, NativeMethodsBase.AUTDDevice(geometryPtr, devIdx)));
                return (byte)(tr?.Idx ?? 0xFF);
            };
        }

        DatagramPtr IDatagram.Ptr(Geometry geometry) => NativeMethodsBase.AUTDDatagramConfigureDebugOutputIdx(Marshal.GetFunctionPointerForDelegate(_f), IntPtr.Zero, geometry.Ptr);
    }

    /// <summary>
    /// Datagram to configure force fan
    /// </summary>
    public sealed class ConfigureForceFan : IDatagram
    {
        [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        [return: MarshalAs(UnmanagedType.U1)]
        public delegate bool ConfigureForceFanDelegate(IntPtr context, GeometryPtr geometryPtr, uint devIdx);

        private readonly ConfigureForceFanDelegate _f;

        public ConfigureForceFan(Func<Device, bool> f)
        {
            _f = (context, geometryPtr, devIdx) =>
            {
                return f(new Device((int)devIdx, NativeMethodsBase.AUTDDevice(geometryPtr, devIdx)));
            };
        }

        DatagramPtr IDatagram.Ptr(Geometry geometry) => NativeMethodsBase.AUTDDatagramConfigureForceFan(Marshal.GetFunctionPointerForDelegate(_f), IntPtr.Zero, geometry.Ptr);
    }

    /// <summary>
    /// Datagram to configure reads FPGA Info
    /// </summary>
    public sealed class ConfigureReadsFPGAInfo : IDatagram
    {
        [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        [return: MarshalAs(UnmanagedType.U1)]
        public delegate bool ConfigureReadsFPGAInfoDelegate(IntPtr context, GeometryPtr geometryPtr, uint devIdx);

        private readonly ConfigureReadsFPGAInfoDelegate _f;

        public ConfigureReadsFPGAInfo(Func<Device, bool> f)
        {
            _f = (context, geometryPtr, devIdx) =>
            {
                return f(new Device((int)devIdx, NativeMethodsBase.AUTDDevice(geometryPtr, devIdx)));
            };
        }

        DatagramPtr IDatagram.Ptr(Geometry geometry) => NativeMethodsBase.AUTDDatagramConfigureReadsFPGAInfo(Marshal.GetFunctionPointerForDelegate(_f), IntPtr.Zero, geometry.Ptr);
    }


    /// <summary>
    /// Datagram to configure silencer
    /// </summary>
    public sealed class Silencer : IDatagram
    {
        private readonly ushort _stepIntensity;
        private readonly ushort _stepPhase;

        /// <summary>
        /// Constructor
        /// </summary>
        /// <param name="stepIntensity">Intensity update step of silencer. The smaller step is, the quieter the output is.</param>
        /// <param name="stepPhase">Phase update step of silencer. The smaller step is, the quieter the output is.</param>
        public Silencer(ushort stepIntensity = 256, ushort stepPhase = 256)
        {
            _stepIntensity = stepIntensity;
            _stepPhase = stepPhase;
        }

        DatagramPtr IDatagram.Ptr(Geometry geometry) => NativeMethodsBase.AUTDDatagramSilencer(_stepIntensity, _stepPhase).Validate();

        /// <summary>
        /// Disable silencer
        /// </summary>
        /// <returns></returns>
        public static Silencer Disable()
        {
            return new Silencer(0xFFFF, 0xFFFF);
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
