/*
 * File: AUTD3Sharp.cs
 * Project: src
 * Created Date: 23/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/12/2022
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
using System.Numerics;
#endif

#if USE_SINGLE
using autd3_float_t = System.Single;
#else
using autd3_float_t = System.Double;
#endif

namespace AUTD3Sharp
{
    using Base = NativeMethods.Base;

    internal class AUTDControllerHandle : SafeHandleZeroOrMinusOneIsInvalid
    {
        internal IntPtr CntPtr => handle;

        private readonly byte _driverVersion;

        public AUTDControllerHandle(bool ownsHandle, byte driverVersion) : base(ownsHandle)
        {
            handle = new IntPtr();
            _driverVersion = driverVersion;
        }

        public bool Create()
        {
            return Base.AUTDCreateController(out handle, _driverVersion);
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

        public const byte DriverLatest = 0x00;
        public const byte DriverV2_2 = 0x82;
        public const byte DriverV2_3 = 0x83;
        public const byte DriverV2_4 = 0x84;
        public const byte DriverV2_5 = 0x85;
        public const byte DriverV2_6 = 0x86;
        public const byte DriverV2_7 = 0x87;

        #endregion

        [UnmanagedFunctionPointer(CallingConvention.Cdecl, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)] public delegate void OnLogOutputCallback(string str);
        [UnmanagedFunctionPointer(CallingConvention.Cdecl)] public delegate void OnLogFlushCallback();
        public static void SetLogLevel(int level)
        {
            Base.AUTDSetLogLevel(level);
        }

        public static void SetLogFunc(OnLogOutputCallback output, OnLogFlushCallback flush)
        {
            var onOutput = Marshal.GetFunctionPointerForDelegate(output);
            var onFlush = Marshal.GetFunctionPointerForDelegate(flush);
            Base.AUTDSetDefaultLogger(onOutput, onFlush);
        }
    }

    public sealed class Transducer
    {
        private readonly int _trId;
        private readonly IntPtr _cnt;

        internal Transducer(int trId, IntPtr cnt)
        {
            _trId = trId;
            _cnt = cnt;
        }

        public int Id => _trId;

        public Vector3 Position
        {
            get
            {
                Base.AUTDTransPosition(_cnt, _trId, out var x, out var y, out var z);
                return new Vector3(x, y, z);
            }
        }

        public Vector3 XDirection
        {
            get
            {
                Base.AUTDTransXDirection(_cnt, _trId, out var x, out var y, out var z);
                return new Vector3(x, y, z);
            }
        }

        public Vector3 YDirection
        {
            get
            {
                Base.AUTDTransYDirection(_cnt, _trId, out var x, out var y, out var z);
                return new Vector3(x, y, z);
            }
        }

        public Vector3 ZDirection
        {
            get
            {
                Base.AUTDTransZDirection(_cnt, _trId, out var x, out var y, out var z);
                return new Vector3(x, y, z);
            }
        }

        public autd3_float_t Wavelength => Base.AUTDGetWavelength(_cnt, _trId);

        public autd3_float_t Frequency
        {
            get => Base.AUTDGetTransFrequency(_cnt, _trId);
            set => Base.AUTDSetTransFrequency(_cnt, _trId, value);
        }

        public ushort Cycle
        {
            get => Base.AUTDGetTransCycle(_cnt, _trId);
            set => Base.AUTDSetTransCycle(_cnt, _trId, value);
        }

        public ushort ModDelay
        {
            get => Base.AUTDGetModDelay(_cnt, _trId);
            set => Base.AUTDSetModDelay(_cnt, _trId, value);
        }
    }

    public sealed class Geometry : IEnumerable<Transducer>
    {
        internal readonly IntPtr CntPtr;

        internal Geometry(IntPtr cntPtr)
        {
            CntPtr = cntPtr;
        }

        public void AddDevice(Vector3 position, Vector3 rotation) => Base.AUTDAddDevice(CntPtr, position.x, position.y, position.z, rotation.x, rotation.y, rotation.z);

        public void AddDevice(Vector3 position, Quaternion quaternion) => Base.AUTDAddDeviceQuaternion(CntPtr, position.x, position.y, position.z, quaternion.w, quaternion.x, quaternion.y, quaternion.z);

        public int NumTransducers => Base.AUTDNumTransducers(CntPtr);

        public int NumDevices => Base.AUTDNumDevices(CntPtr);

        public Vector3 Center
        {
            get
            {
                Base.AUTDGeometryCenter(CntPtr, out var x, out var y, out var z);
                return new Vector3(x, y, z);
            }
        }

        public Transducer this[int index]
        {
            get
            {
                if (index >= NumTransducers) throw new IndexOutOfRangeException();
                return new Transducer(index, CntPtr);
            }
        }

        public sealed class TransducerEnumerator : IEnumerator<Transducer>
        {
            private int _idx;
            private readonly IntPtr _cnt;
            private readonly int _numTrans;

            internal TransducerEnumerator(IntPtr cnt)
            {
                _idx = -1;
                _cnt = cnt;
                _numTrans = Base.AUTDNumTransducers(_cnt);
            }

            public bool MoveNext() => ++_idx < _numTrans;

            public void Reset() => _idx = -1;

            public Transducer Current => new Transducer(_idx, _cnt);

            object System.Collections.IEnumerator.Current => Current;

            public void Dispose() { }
        }

        public IEnumerator<Transducer> GetEnumerator() => new TransducerEnumerator(CntPtr);

        System.Collections.IEnumerator System.Collections.IEnumerable.GetEnumerator() => GetEnumerator();
    }

    public sealed class Controller : IDisposable
    {
        #region field

        private bool _isDisposed;
        internal readonly AUTDControllerHandle AUTDControllerHandle;

        #endregion

        #region Controller

        public Controller(byte driverVersion = AUTD3.DriverLatest)
        {
            AUTDControllerHandle = new AUTDControllerHandle(true, driverVersion);
            if (!AUTDControllerHandle.Create())
                throw new Exception("Failed to create Controller.");
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

        public bool Close() => Base.AUTDClose(AUTDControllerHandle.CntPtr);

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


        public autd3_float_t SoundSpeed
        {
            get => Base.AUTDGetSoundSpeed(AUTDControllerHandle.CntPtr);
            set => Base.AUTDSetSoundSpeed(AUTDControllerHandle.CntPtr, value);
        }

        public autd3_float_t Attenuation
        {
            get => Base.AUTDGetAttenuation(AUTDControllerHandle.CntPtr);
            set => Base.AUTDSetAttenuation(AUTDControllerHandle.CntPtr, value);
        }

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
                var infos = new byte[Geometry.NumTransducers / AUTD3.NumTransInDevice];
                Base.AUTDGetFPGAInfo(AUTDControllerHandle.CntPtr, infos);
                return infos;
            }
        }
        #endregion


        public void SetSoundSpeedFromTemp(autd3_float_t temp, autd3_float_t k = (autd3_float_t)1.4, autd3_float_t r = (autd3_float_t)8.31446261815324, autd3_float_t m = (autd3_float_t)28.9647e-3)
        {
            Base.AUTDSetSoundSpeedFromTemp(AUTDControllerHandle.CntPtr, temp, k, r, m);
        }

        public bool Send(SpecialData special)
        {
            if (special == null) throw new ArgumentNullException(nameof(special));
            return Base.AUTDSendSpecial(AUTDControllerHandle.CntPtr, special.Ptr);
        }

        public bool Send(Header header)
        {
            if (header == null) throw new ArgumentNullException(nameof(header));
            return Base.AUTDSend(AUTDControllerHandle.CntPtr, header.Ptr, IntPtr.Zero);
        }

        public bool Send(Body body)
        {
            if (body == null) throw new ArgumentNullException(nameof(body));
            return Base.AUTDSend(AUTDControllerHandle.CntPtr, IntPtr.Zero, body.Ptr);
        }

        public bool Send(Header header, Body body)
        {
            if (header == null) throw new ArgumentNullException(nameof(header));
            if (body == null) throw new ArgumentNullException(nameof(body));
            return Base.AUTDSend(AUTDControllerHandle.CntPtr, header.Ptr, body.Ptr);
        }

        public bool Send(Body body, Header header)
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
        public Amplitudes(autd3_float_t amp = (autd3_float_t)1.0)
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
            public Focus(Vector3 point, autd3_float_t amp = (autd3_float_t)1.0) => Base.AUTDGainFocus(out handle, point.x, point.y, point.z, amp);
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
            public BesselBeam(Vector3 point, Vector3 dir, autd3_float_t thetaZ, autd3_float_t amp = (autd3_float_t)1.0) => Base.AUTDGainBesselBeam(out handle, point.x, point.y, point.z, dir.x, dir.y, dir.z, thetaZ, amp);
        }

        public sealed class PlaneWave : Gain
        {
            public PlaneWave(Vector3 dir, autd3_float_t amp = (autd3_float_t)1.0) => Base.AUTDGainPlaneWave(out handle, dir.x, dir.y, dir.z, amp);
        }

        public sealed class Custom : Gain
        {
            public Custom(autd3_float_t[] amp, autd3_float_t[] phase)
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

            public autd3_float_t SamplingFrequency => Base.AUTDModulationSamplingFrequency(handle);
            public uint SamplingFrequencyDivision
            {
                get => Base.AUTDModulationSamplingFrequencyDivision(handle);
                set => Base.AUTDModulationSetSamplingFrequencyDivision(handle, value);
            }
        }

        public sealed class Static : Modulation
        {
            public Static(autd3_float_t amp = (autd3_float_t)1.0)
            {
                Base.AUTDModulationStatic(out handle, amp);

            }
        }

        public sealed class Sine : Modulation
        {
            public Sine(int freq, autd3_float_t amp = (autd3_float_t)1.0, autd3_float_t offset = (autd3_float_t)0.5)
            {
                Base.AUTDModulationSine(out handle, freq, amp, offset);
            }
        }

        public sealed class SineSquared : Modulation
        {
            public SineSquared(int freq, autd3_float_t amp = (autd3_float_t)1.0, autd3_float_t offset = (autd3_float_t)0.5)
            {
                Base.AUTDModulationSineSquared(out handle, freq, amp, offset);
            }
        }

        public sealed class SineLegacy : Modulation
        {
            public SineLegacy(autd3_float_t freq, autd3_float_t amp = (autd3_float_t)1.0, autd3_float_t offset = (autd3_float_t)0.5)
            {
                Base.AUTDModulationSineLegacy(out handle, freq, amp, offset);
            }
        }


        public sealed class Square : Modulation
        {
            public Square(int freq, autd3_float_t low = (autd3_float_t)0.0, autd3_float_t high = (autd3_float_t)1.0, autd3_float_t duty = (autd3_float_t)0.5)
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

            public autd3_float_t Frequency
            {
                get => Base.AUTDSTMFrequency(handle);
                set => Base.AUTDSTMSetFrequency(handle, value);
            }

            public int StartIdx
            {
                get => Base.AUTDSTMGetStartIdx(handle);
                set => Base.AUTDSTMSetStartIdx(handle, value);
            }

            public int FinishIdx
            {
                get => Base.AUTDSTMGetFinishIdx(handle);
                set => Base.AUTDSTMSetFinishIdx(handle, value);
            }

            public autd3_float_t SamplingFrequency => Base.AUTDSTMSamplingFrequency(handle);
            public uint SamplingFrequencyDivision
            {
                get => Base.AUTDSTMSamplingFrequencyDivision(handle);
                set => Base.AUTDSTMSetSamplingFrequencyDivision(handle, value);
            }
        }

        public sealed class FocusSTM : STM
        {
            public FocusSTM(autd3_float_t soundSpeed)
            {
                Base.AUTDFocusSTM(out handle, soundSpeed);
            }

            public void Add(Vector3 point, byte shift = 0) => Base.AUTDFocusSTMAdd(handle, point.x, point.y, point.z, shift);
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

            public void Add(AUTD3Sharp.Gain.Gain gain)
            {
                Base.AUTDGainSTMAdd(handle, gain.Ptr);
            }
        }
    }
}
