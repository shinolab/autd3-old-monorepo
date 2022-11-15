/*
 * File: AUTD3Sharp.cs
 * Project: src
 * Created Date: 23/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/11/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

#if UNITY_2018_3_OR_NEWER
#define USE_SINGLE
#define DIMENSION_M
#endif

using Microsoft.Win32.SafeHandles;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Text;

#if UNITY_2018_3_OR_NEWER
using UnityEngine;
using Vector3 = UnityEngine.Vector3;
using Quaternion = UnityEngine.Quaternion;
using Math = UnityEngine.Mathf;
#else
using Vector3 = AUTD3Sharp.Utils.Vector3d;
using Quaternion = AUTD3Sharp.Utils.Quaterniond;
#endif


namespace AUTD3Sharp
{
    using Base = NativeMethods.Base;

    internal class AUTDControllerHandle : SafeHandleZeroOrMinusOneIsInvalid
    {
        internal IntPtr CntPtr => handle;

        public AUTDControllerHandle(bool ownsHandle) : base(ownsHandle)
        {
            handle = new IntPtr();
            Base.AUTDCreateController(out handle);
        }

        protected override bool ReleaseHandle()
        {
            Base.AUTDFreeController(handle);
            return true;
        }
    }

    public static class AUTD3
    {
        #region const
#if USE_SINGLE
#if DIMENSION_M
        public const float Millimeter = 1e-3f;
#else
        public const float Millimeter = 1;
#endif
        public const float DeviceWidth = 192.0f * Millimeter;
        public const float DeviceHeight = 151.4f * Millimeter;
        public const float TransSpacing = 10.16f * Millimeter;
        public const float Pi = Math.PI;
#else
#if DIMENSION_M
        public const double Millimeter = 1e-3;
#else
        public const double Millimeter = 1;
#endif
        public const double DeviceWidth = 192.0 * Millimeter;
        public const double DeviceHeight = 151.4 * Millimeter;
        public const double TransSpacing = 10.16 * Millimeter;
        public const double Pi = Math.PI;
#endif
        public const int NumTransInDevice = 249;
        public const int NumTransInX = 18;
        public const int NumTransInY = 14;

        #endregion
    }

    public static class TypeHelper
    {
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        internal static (double, double, double) Convert(Vector3 vector)
        {
#if USE_SINGLE
            return ((double)vector.x, (double)vector.y, (double)vector.z);
#else
            return (vector.x, vector.y, vector.z);
#endif
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        internal static Vector3 Convert(double x, double y, double z)
        {
#if USE_SINGLE
            var vector = new Vector3((float)x, (float)y, (float)z);
#else
            var vector = new Vector3(x, y, z);
#endif
            return vector;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        internal static (double, double, double, double) Convert(Quaternion quaternion)
        {
#if USE_SINGLE
            return ((double)quaternion.w, (double)quaternion.x, (double)quaternion.y, (double)quaternion.z);
#else
            return (quaternion.w, quaternion.x, quaternion.y, quaternion.z);
#endif
        }
    }

    public sealed class Transducer
    {
        private readonly int _devId;
        private readonly int _trId;
        private readonly IntPtr _cnt;

        internal Transducer(int devId, int trId, IntPtr cnt)
        {
            _devId = devId;
            _trId = trId;
            _cnt = cnt;
        }

        public int Id => AUTD3.NumTransInDevice * _devId + _trId;

        public Vector3 Position
        {
            get
            {
                Base.AUTDTransPosition(_cnt, _devId, _trId, out var x, out var y, out var z);
                return TypeHelper.Convert(x, y, z);
            }
        }

        public Vector3 XDirection
        {
            get
            {
                Base.AUTDTransXDirection(_cnt, _devId, _trId, out var x, out var y, out var z);
                return TypeHelper.Convert(x, y, z);
            }
        }

        public Vector3 YDirection
        {
            get
            {
                Base.AUTDTransYDirection(_cnt, _devId, _trId, out var x, out var y, out var z);
                return TypeHelper.Convert(x, y, z);
            }
        }

        public Vector3 ZDirection
        {
            get
            {
                Base.AUTDTransZDirection(_cnt, _devId, _trId, out var x, out var y, out var z);
                return TypeHelper.Convert(x, y, z);
            }
        }

        public double Wavelength => Base.AUTDGetWavelength(_cnt, _devId, _trId);

        public double Frequency
        {
            get => Base.AUTDGetTransFrequency(_cnt, _devId, _trId);
            set => Base.AUTDSetTransFrequency(_cnt, _devId, _trId, value);
        }

        public ushort Cycle
        {
            get => Base.AUTDGetTransCycle(_cnt, _devId, _trId);
            set => Base.AUTDSetTransCycle(_cnt, _devId, _trId, value);
        }

        public ushort ModDelay
        {
            get => Base.AUTDGetModDelay(_cnt, _devId, _trId);
            set => Base.AUTDSetModDelay(_cnt, _devId, _trId, value);
        }
    }

    public sealed class Device : IEnumerable<Transducer>
    {
        private readonly int _id;
        private readonly IntPtr _cnt;

        internal Device(int id, IntPtr cnt)
        {
            _id = id;
            _cnt = cnt;
        }

        public Vector3 Origin => new Transducer(_id, 0, _cnt).Position;

        public Vector3 Center => this.Aggregate(Vector3.zero, (current, tr) => current + tr.Position) / AUTD3.NumTransInDevice;

        public Transducer this[int index]
        {
            get
            {
                if (index >= AUTD3.NumTransInDevice) throw new IndexOutOfRangeException();
                return new Transducer(_id, index, _cnt);
            }
        }

        public sealed class TransducerEnumerator : IEnumerator<Transducer>
        {
            private int _idx;
            private readonly int _devId;
            private readonly IntPtr _cnt;

            internal TransducerEnumerator(int devId, IntPtr cnt)
            {
                _idx = -1;
                _devId = devId;
                _cnt = cnt;
            }

            public bool MoveNext() => ++_idx < AUTD3.NumTransInDevice;
            public void Reset() => _idx = -1;

            public Transducer Current => new Transducer(_devId, _idx, _cnt);

            object System.Collections.IEnumerator.Current => Current;

            public void Dispose() { }
        }

        public IEnumerator<Transducer> GetEnumerator() => new TransducerEnumerator(_id, _cnt);

        System.Collections.IEnumerator System.Collections.IEnumerable.GetEnumerator() => GetEnumerator();
    }

    public sealed class Geometry : IEnumerable<Device>
    {
        internal readonly IntPtr CntPtr;

        internal Geometry(IntPtr cntPtr)
        {
            CntPtr = cntPtr;
        }

        public int AddDevice(Vector3 position, Vector3 rotation)
        {
            var (x, y, z) = TypeHelper.Convert(position);
            var (rx, ry, rz) = TypeHelper.Convert(rotation);
            return Base.AUTDAddDevice(CntPtr, x, y, z, rx, ry, rz);
        }

        public int AddDevice(Vector3 position, Quaternion quaternion)
        {
            var (x, y, z) = TypeHelper.Convert(position);
            var (qw, qx, qy, qz) = TypeHelper.Convert(quaternion);
            return Base.AUTDAddDeviceQuaternion(CntPtr, x, y, z, qw, qx, qy, qz);
        }

        public int NumDevices => Base.AUTDNumDevices(CntPtr);

        public int NumTransducers => NumDevices * AUTD3.NumTransInDevice;

        public double SoundSpeed
        {
            get => Base.AUTDGetSoundSpeed(CntPtr);
            set => Base.AUTDSetSoundSpeed(CntPtr, value);
        }

        public double Attenuation
        {
            get => Base.AUTDGetAttenuation(CntPtr);
            set => Base.AUTDSetAttenuation(CntPtr, value);
        }

        public Vector3 Center => this.Aggregate(Vector3.zero, (current, dev) => current + dev.Center) / NumDevices;

        public Device this[int index]
        {
            get
            {
                if (index >= NumDevices) throw new IndexOutOfRangeException();
                return new Device(index, CntPtr);
            }
        }

        public sealed class DeviceEnumerator : IEnumerator<Device>
        {
            private int _idx;
            private readonly int _devLen;
            private readonly IntPtr _cnt;

            internal DeviceEnumerator(int devLen, IntPtr cnt)
            {
                _idx = -1;
                _devLen = devLen;
                _cnt = cnt;
            }

            public bool MoveNext() => ++_idx < _devLen;
            public void Reset() => _idx = -1;

            public Device Current => new Device(_idx, _cnt);

            object System.Collections.IEnumerator.Current => Current;

            public void Dispose() { }
        }

        public IEnumerator<Device> GetEnumerator() => new DeviceEnumerator(NumDevices, CntPtr);

        System.Collections.IEnumerator System.Collections.IEnumerable.GetEnumerator() => GetEnumerator();
    }

    public sealed class Controller : IDisposable
    {
        #region field

        private bool _isDisposed;
        internal readonly AUTDControllerHandle AUTDControllerHandle;

        #endregion

        #region Controller

        public Controller()
        {
            AUTDControllerHandle = new AUTDControllerHandle(true);
            Geometry = new Geometry(AUTDControllerHandle.CntPtr);
        }

        public void ToLegacy()
        {
            Base.AUTDSetMode(AUTDControllerHandle.CntPtr, 0);
        }

        public void ToNormal()
        {
            Base.AUTDSetMode(AUTDControllerHandle.CntPtr, 1);
        }

        public void ToNormalPhase()
        {
            Base.AUTDSetMode(AUTDControllerHandle.CntPtr, 2);
        }

        public bool Open(Link.Link link) => Base.AUTDOpenController(AUTDControllerHandle.CntPtr, link.LinkPtr);

        public IEnumerable<FirmwareInfo> FirmwareInfoList()
        {
            var size = Base.AUTDGetFirmwareInfoListPointer(AUTDControllerHandle.CntPtr, out var handle);
            for (var i = 0; i < size; i++)
            {
                var info = new StringBuilder(256);
                Base.AUTDGetFirmwareInfo(handle, i, info);
                yield return new FirmwareInfo(info.ToString());
            }

            Base.AUTDFreeFirmwareInfoListPointer(handle);
        }

        public int Close() => Base.AUTDClose(AUTDControllerHandle.CntPtr);

        public void Dispose()
        {
            Dispose(true);
            GC.SuppressFinalize(this);
        }

        private void Dispose(bool disposing)
        {
            if (_isDisposed) return;

            if (disposing) Close();

            AUTDControllerHandle.Dispose();

            _isDisposed = true;
        }

        ~Controller()
        {
            Dispose(false);
        }

        #endregion

        #region Property
        public Geometry Geometry { get; }

        public bool IsOpen => Base.AUTDIsOpen(AUTDControllerHandle.CntPtr);

        public bool ForceFan
        {
            get => Base.AUTDGetForceFan(AUTDControllerHandle.CntPtr);
            set => Base.AUTDSetForceFan(AUTDControllerHandle.CntPtr, value);
        }

        public bool ReadsFPGAInfo
        {
            get => Base.AUTDGetReadsFPGAInfo(AUTDControllerHandle.CntPtr);
            set => Base.AUTDSetReadsFPGAInfo(AUTDControllerHandle.CntPtr, value);
        }

        public ulong AckCheckTimeoutMs
        {
            get => Base.AUTDGetAckCheckTimeout(AUTDControllerHandle.CntPtr) / 1000 / 1000;
            set => Base.AUTDSetAckCheckTimeout(AUTDControllerHandle.CntPtr, value * 1000 * 1000);
        }
        public ulong AckCheckTimeoutNs
        {
            get => Base.AUTDGetAckCheckTimeout(AUTDControllerHandle.CntPtr);
            set => Base.AUTDSetAckCheckTimeout(AUTDControllerHandle.CntPtr, value);
        }

        public ulong SendIntervalsMs
        {
            get => Base.AUTDGetSendInterval(AUTDControllerHandle.CntPtr) / 1000 / 1000;
            set => Base.AUTDSetSendInterval(AUTDControllerHandle.CntPtr, value * 1000 * 1000);
        }

        public ulong SendIntervalsNs
        {
            get => Base.AUTDGetSendInterval(AUTDControllerHandle.CntPtr);
            set => Base.AUTDSetSendInterval(AUTDControllerHandle.CntPtr, value);
        }

        public byte[] FPGAInfo
        {
            get
            {
                var infos = new byte[Geometry.NumDevices];
                Base.AUTDGetFPGAInfo(AUTDControllerHandle.CntPtr, infos);
                return infos;
            }
        }

        public static string LastError
        {
            get
            {
                var size = Base.AUTDGetLastError(null);
                var sb = new StringBuilder(size);
                Base.AUTDGetLastError(sb);
                return sb.ToString();
            }
        }
        #endregion

        public int Send(SpecialData special)
        {
            if (special == null) throw new ArgumentNullException(nameof(special));
            return Base.AUTDSendSpecial(AUTDControllerHandle.CntPtr, special.Ptr);
        }

        public int Send(Header header)
        {
            if (header == null) throw new ArgumentNullException(nameof(header));
            return Base.AUTDSend(AUTDControllerHandle.CntPtr, header.Ptr, IntPtr.Zero);
        }

        public int Send(Body body)
        {
            if (body == null) throw new ArgumentNullException(nameof(body));
            return Base.AUTDSend(AUTDControllerHandle.CntPtr, IntPtr.Zero, body.Ptr);
        }

        public int Send(Header header, Body body)
        {
            if (header == null) throw new ArgumentNullException(nameof(header));
            if (body == null) throw new ArgumentNullException(nameof(body));
            return Base.AUTDSend(AUTDControllerHandle.CntPtr, header.Ptr, body.Ptr);
        }

        public int Send(Body body, Header header)
        {
            if (header == null) throw new ArgumentNullException(nameof(header));
            if (body == null) throw new ArgumentNullException(nameof(body));
            return Base.AUTDSend(AUTDControllerHandle.CntPtr, header.Ptr, body.Ptr);
        }

        public void SendAsync(SpecialData special)
        {
            if (special == null) throw new ArgumentNullException(nameof(special));
            Base.AUTDSendSpecialAsync(AUTDControllerHandle.CntPtr, special.Ptr);
            special.Ptr = IntPtr.Zero;
        }

        public void SendAsync(Header header)
        {
            if (header == null) throw new ArgumentNullException(nameof(header));
            Base.AUTDSendAsync(AUTDControllerHandle.CntPtr, header.Ptr, IntPtr.Zero);
            header.Ptr = IntPtr.Zero;
        }

        public void SendAsync(Body body)
        {
            if (body == null) throw new ArgumentNullException(nameof(body));
            Base.AUTDSendAsync(AUTDControllerHandle.CntPtr, IntPtr.Zero, body.Ptr);
            body.Ptr = IntPtr.Zero;
        }

        public void SendAsync(Header header, Body body)
        {
            if (header == null) throw new ArgumentNullException(nameof(header));
            if (body == null) throw new ArgumentNullException(nameof(body));
            Base.AUTDSendAsync(AUTDControllerHandle.CntPtr, header.Ptr, body.Ptr);
            header.Ptr = IntPtr.Zero;
            body.Ptr = IntPtr.Zero;
        }

        public void SendAsync(Body body, Header header)
        {
            if (header == null) throw new ArgumentNullException(nameof(header));
            if (body == null) throw new ArgumentNullException(nameof(body));
            Base.AUTDSendAsync(AUTDControllerHandle.CntPtr, header.Ptr, body.Ptr);
            header.Ptr = IntPtr.Zero;
            body.Ptr = IntPtr.Zero;

        }
    }

    public sealed class UpdateFlag : SpecialData
    {
        public UpdateFlag()
        {
            Base.AUTDUpdateFlags(out handle);
        }

        protected override bool ReleaseHandle()
        {
            Base.AUTDDeleteSpecialData(handle);
            return true;
        }
    }

    public sealed class Clear : SpecialData
    {
        public Clear()
        {
            Base.AUTDClear(out handle);
        }

        protected override bool ReleaseHandle()
        {
            Base.AUTDDeleteSpecialData(handle);
            return true;
        }
    }

    public sealed class Synchronize : SpecialData
    {
        public Synchronize()
        {
            Base.AUTDSynchronize(out handle);
        }

        protected override bool ReleaseHandle()
        {
            Base.AUTDDeleteSpecialData(handle);
            return true;
        }
    }

    public sealed class Stop : SpecialData
    {
        public Stop()
        {
            Base.AUTDStop(out handle);
        }

        protected override bool ReleaseHandle()
        {
            Base.AUTDDeleteSpecialData(handle);
            return true;
        }
    }
    public sealed class ModDelayConfig : SpecialData
    {
        public ModDelayConfig()
        {
            Base.AUTDModDelayConfig(out handle);
        }

        protected override bool ReleaseHandle()
        {
            Base.AUTDDeleteSpecialData(handle);
            return true;
        }
    }

    public sealed class SilencerConfig : Header
    {
        public SilencerConfig(ushort step = 10, ushort cycle = 4096)
        {
            Base.AUTDCreateSilencer(out handle, step, cycle);
        }

        protected override bool ReleaseHandle()
        {
            Base.AUTDDeleteSilencer(handle);
            return true;
        }

        public static SilencerConfig None()
        {
            return new SilencerConfig(0xFFFF);
        }
    }

    public sealed class Amplitudes : Body
    {
        public Amplitudes(double amp = 1.0)
        {
            Base.AUTDCreateAmplitudes(out handle, amp);
        }

        protected override bool ReleaseHandle()
        {
            Base.AUTDDeleteAmplitudes(handle);
            return true;
        }
    }

    namespace Gain
    {

        [ComVisible(false)]
        public abstract class Gain : Body
        {
            internal Gain()
            {
            }

            protected override bool ReleaseHandle()
            {
                Base.AUTDDeleteGain(handle);
                return true;
            }
        }

        public sealed class Focus : Gain
        {
            public Focus(Vector3 point, double amp = 1.0)
            {
                var (x, y, z) = TypeHelper.Convert(point);
                Base.AUTDGainFocus(out handle, x, y, z, amp);
            }
        }

        public sealed class Grouped : Gain
        {
            public Grouped(Controller cnt)
            {
                Base.AUTDGainGrouped(out handle, cnt.AUTDControllerHandle.CntPtr);
            }

            public void Add(int deviceIdx, Gain gain)
            {
                Base.AUTDGainGroupedAdd(handle, deviceIdx, gain.Ptr);
            }
        }

        public sealed class BesselBeam : Gain
        {
            public BesselBeam(Vector3 point, Vector3 dir, double thetaZ, double amp = 1.0)
            {
                var (x, y, z) = TypeHelper.Convert(point);
                var (dx, dy, dz) = TypeHelper.Convert(dir);
                Base.AUTDGainBesselBeam(out handle, x, y, z, dx, dy, dz, thetaZ, amp);
            }
        }

        public sealed class PlaneWave : Gain
        {
            public PlaneWave(Vector3 dir, double amp = 1.0)
            {
                var (dx, dy, dz) = TypeHelper.Convert(dir);
                Base.AUTDGainPlaneWave(out handle, dx, dy, dz, amp);
            }
        }

        public sealed class Custom : Gain
        {
            public Custom(double[] amp, double[] phase)
            {
                if (amp.Length != phase.Length) throw new ArgumentException();
                var length = amp.Length;
                Base.AUTDGainCustom(out handle, amp, phase, (ulong)length);
            }
        }

        public sealed class Null : Gain
        {
            public Null()
            {
                Base.AUTDGainNull(out handle);
            }
        }
    }

    namespace Modulation
    {

        [ComVisible(false)]
        public abstract class Modulation : Header
        {
            internal Modulation()
            {
            }

            protected override bool ReleaseHandle()
            {
                Base.AUTDDeleteModulation(handle);
                return true;
            }

            public double SamplingFrequency => Base.AUTDModulationSamplingFrequency(handle);
            public uint SamplingFrequencyDivision
            {
                get => Base.AUTDModulationSamplingFrequencyDivision(handle);
                set => Base.AUTDModulationSetSamplingFrequencyDivision(handle, value);
            }
        }

        public sealed class Static : Modulation
        {
            public Static(double amp = 1.0)
            {
                Base.AUTDModulationStatic(out handle, amp);

            }
        }

        public sealed class Sine : Modulation
        {
            public Sine(int freq, double amp = 1.0, double offset = 0.5)
            {
                Base.AUTDModulationSine(out handle, freq, amp, offset);
            }
        }

        public sealed class SineSquared : Modulation
        {
            public SineSquared(int freq, double amp = 1.0, double offset = 0.5)
            {
                Base.AUTDModulationSineSquared(out handle, freq, amp, offset);
            }
        }

        public sealed class SineLegacy : Modulation
        {
            public SineLegacy(double freq, double amp = 1.0, double offset = 0.5)
            {
                Base.AUTDModulationSineLegacy(out handle, freq, amp, offset);
            }
        }


        public sealed class Square : Modulation
        {
            public Square(int freq, double low = 0.0, double high = 1.0, double duty = 0.5)
            {
                Base.AUTDModulationSquare(out handle, freq, low, high, duty);
            }
        }

        public sealed class Custom : Modulation
        {
            public Custom(byte[] data, uint freqDiv)
            {
                Base.AUTDModulationCustom(out handle, data, (ulong)data.Length, freqDiv);
            }
        }
    }

    namespace STM
    {
        public abstract class STM : Body
        {
            protected override bool ReleaseHandle()
            {
                Base.AUTDDeleteSTM(handle);
                return true;
            }

            public double Frequency
            {
                get => Base.AUTDSTMFrequency(handle);
                set => Base.AUTDSTMSetFrequency(handle, value);
            }

            public double SamplingFrequency => Base.AUTDSTMSamplingFrequency(handle);
            public uint SamplingFrequencyDivision
            {
                get => Base.AUTDSTMSamplingFrequencyDivision(handle);
                set => Base.AUTDSTMSetSamplingFrequencyDivision(handle, value);
            }
        }

        public sealed class PointSTM : STM
        {
            public PointSTM()
            {
                Base.AUTDPointSTM(out handle);
            }

            public bool Add(Vector3 point, byte shift = 0)
            {
                var (x, y, z) = TypeHelper.Convert(point);
                return Base.AUTDPointSTMAdd(handle, x, y, z, shift);
            }
        }

        public enum Mode : ushort
        {
            PhaseDutyFull = 0x0001,
            PhaseFull = 0x0002,
            PhaseHalf = 0x0004
        }

        public sealed class GainSTM : STM
        {
            public Mode Mode
            {
                get => (Mode)Base.AUTDGetGainSTMMode(handle);
                set => Base.AUTDSetGainSTMMode(handle, (ushort)value);
            }

            public GainSTM(Controller cnt)
            {
                Base.AUTDGainSTM(out handle, cnt.AUTDControllerHandle.CntPtr);
            }

            public bool Add(AUTD3Sharp.Gain.Gain gain)
            {
                return Base.AUTDGainSTMAdd(handle, gain.Ptr);
            }
        }
    }
}
