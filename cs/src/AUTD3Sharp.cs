﻿/*
 * File: AUTD3Sharp.cs
 * Project: src
 * Created Date: 23/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/03/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

#if UNITY_2018_3_OR_NEWER
#define USE_SINGLE
#define DIMENSION_M
#endif

using System;
using System.Collections.Generic;
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

#if USE_SINGLE
using autd3_float_t = System.Single;
#else
using autd3_float_t = System.Double;
#endif

namespace AUTD3Sharp
{
    using Base = NativeMethods.Base;

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

        [UnmanagedFunctionPointer(CallingConvention.Cdecl, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)] public delegate void OnLogOutputCallback(string str);
        [UnmanagedFunctionPointer(CallingConvention.Cdecl)] public delegate void OnLogFlushCallback();

        public static void SetLogLevel(DebugLevel level)
        {
            Base.AUTDSetLogLevel((int)level);
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
        private readonly IntPtr _cnt;

        internal Transducer(int trId, IntPtr cnt)
        {
            Id = trId;
            _cnt = cnt;
        }

        public int Id { get; }

        public Vector3 Position
        {
            get
            {
                Base.AUTDTransPosition(_cnt, Id, out var x, out var y, out var z);
                return new Vector3(x, y, z);
            }
        }

        public Vector3 XDirection
        {
            get
            {
                Base.AUTDTransXDirection(_cnt, Id, out var x, out var y, out var z);
                return new Vector3(x, y, z);
            }
        }

        public Vector3 YDirection
        {
            get
            {
                Base.AUTDTransYDirection(_cnt, Id, out var x, out var y, out var z);
                return new Vector3(x, y, z);
            }
        }

        public Vector3 ZDirection
        {
            get
            {
                Base.AUTDTransZDirection(_cnt, Id, out var x, out var y, out var z);
                return new Vector3(x, y, z);
            }
        }

        public autd3_float_t Wavelength => Base.AUTDGetWavelength(_cnt, Id);

        public autd3_float_t Frequency
        {
            get => Base.AUTDGetTransFrequency(_cnt, Id);
            set => Base.AUTDSetTransFrequency(_cnt, Id, value);
        }

        public ushort Cycle
        {
            get => Base.AUTDGetTransCycle(_cnt, Id);
            set => Base.AUTDSetTransCycle(_cnt, Id, value);
        }

        public ushort ModDelay
        {
            get => Base.AUTDGetTransModDelay(_cnt, Id);
            set => Base.AUTDSetTransModDelay(_cnt, Id, value);
        }
    }

    public sealed class Geometry : IEnumerable<Transducer>
    {
        internal readonly IntPtr GeometryPtr;
        private bool _isDisposed;

        internal Geometry(IntPtr geometryPtr)
        {
            GeometryPtr = geometryPtr;
        }

        public int NumTransducers => Base.AUTDNumTransducers(GeometryPtr);

        public int NumDevices => Base.AUTDNumDevices(GeometryPtr);

        public autd3_float_t SoundSpeed
        {
            get => Base.AUTDGetSoundSpeed(GeometryPtr);
            set => Base.AUTDSetSoundSpeed(GeometryPtr, value);
        }

        public autd3_float_t Attenuation
        {
            get => Base.AUTDGetAttenuation(GeometryPtr);
            set => Base.AUTDSetAttenuation(GeometryPtr, value);
        }

        public Vector3 Center
        {
            get
            {
                Base.AUTDGeometryCenter(GeometryPtr, out var x, out var y, out var z);
                return new Vector3(x, y, z);
            }
        }

        public Transducer this[int index]
        {
            get
            {
                if (index >= NumTransducers) throw new IndexOutOfRangeException();
                return new Transducer(index, GeometryPtr);
            }
        }

        public void SetSoundSpeedFromTemp(autd3_float_t temp, autd3_float_t k = (autd3_float_t)1.4, autd3_float_t r = (autd3_float_t)8.31446261815324, autd3_float_t m = (autd3_float_t)28.9647e-3)
        {
            Base.AUTDSetSoundSpeedFromTemp(GeometryPtr, temp, k, r, m);
        }

        public Vector3 CenterOf(int devIdx)
        {
            Base.AUTDGeometryCenterOf(GeometryPtr, devIdx, out var x, out var y, out var z);
            return new Vector3(x, y, z);
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

        public IEnumerator<Transducer> GetEnumerator() => new TransducerEnumerator(GeometryPtr);

        System.Collections.IEnumerator System.Collections.IEnumerable.GetEnumerator() => GetEnumerator();

        private void Dispose()
        {
            if (_isDisposed) return;
            Base.AUTDFreeGeometry(GeometryPtr);
            _isDisposed = true;
            GC.SuppressFinalize(this);
        }

        ~Geometry()
        {
            Dispose();
        }
    }

    public sealed class GeometryBuilder
    {
        private readonly IntPtr _builderPtr;

        public GeometryBuilder()
        {
            _builderPtr = new IntPtr();
            Base.AUTDCreateGeometryBuilder(out _builderPtr);
        }

        public GeometryBuilder AddDevice(Vector3 position, Vector3 rotation)
        {
            Base.AUTDAddDevice(_builderPtr, position.x, position.y, position.z, rotation.x, rotation.y, rotation.z);
            return this;
        }

        public GeometryBuilder AddDevice(Vector3 position, Quaternion quaternion)
        {
            Base.AUTDAddDeviceQuaternion(_builderPtr, position.x, position.y, position.z, quaternion.w, quaternion.x, quaternion.y, quaternion.z);
            return this;
        }

        public GeometryBuilder ToLegacy()
        {
            Base.AUTDSetMode(_builderPtr, 0);
            return this;
        }

        public GeometryBuilder ToAdvanced()
        {
            Base.AUTDSetMode(_builderPtr, 1);
            return this;
        }

        public GeometryBuilder ToAdvancedPhase()
        {
            Base.AUTDSetMode(_builderPtr, 2);
            return this;
        }


        public Geometry Build()
        {
            Base.AUTDBuildGeometry(out var geometryPtr, _builderPtr);
            return new Geometry(geometryPtr);
        }
    }

    public sealed class Controller : IDisposable
    {
        #region field

        private bool _isDisposed;
        internal readonly IntPtr CntPtr;

        #endregion

        #region Controller

        public static Controller Open(Geometry geometry, Link.Link link)
        {
            if (!Base.AUTDOpenController(out var cnt, geometry.GeometryPtr, link.LinkPtr))
                throw new Exception("Failed to open controller.");
            return new Controller(cnt, geometry);
        }

        private Controller(IntPtr cnt, Geometry geometry)
        {
            CntPtr = cnt;
            Geometry = geometry;
        }

        public IEnumerable<FirmwareInfo> FirmwareInfoList()
        {
            var size = Base.AUTDGetFirmwareInfoListPointer(CntPtr, out var handle);
            for (var i = 0; i < size; i++)
            {
                var info = new StringBuilder(256);
                Base.AUTDGetFirmwareInfo(handle, i, info, out var matchesVersion, out var isSupported);
                yield return new FirmwareInfo(info.ToString(), matchesVersion, isSupported);
            }

            Base.AUTDFreeFirmwareInfoListPointer(handle);
        }

        public bool Close() => Base.AUTDClose(CntPtr);

        public void Dispose()
        {
            Dispose(true);
            GC.SuppressFinalize(this);
        }

        private void Dispose(bool disposing)
        {
            if (_isDisposed) return;

            if (disposing) Close();

            Base.AUTDFreeController(CntPtr);

            _isDisposed = true;
        }

        ~Controller()
        {
            Dispose(false);
        }

        #endregion

        #region Property
        public Geometry Geometry { get; }

        public bool IsOpen => Base.AUTDIsOpen(CntPtr);

        public void ForceFan(bool value)
        {
            Base.AUTDSetForceFan(CntPtr, value);
        }

        public void ReadsFPGAInfo(bool value)
        {
            Base.AUTDSetReadsFPGAInfo(CntPtr, value);
        }

        public byte[] FPGAInfo
        {
            get
            {
                var infos = new byte[Geometry.NumDevices];
                Base.AUTDGetFPGAInfo(CntPtr, infos);
                return infos;
            }
        }
        #endregion

        public bool Send(SpecialData special, TimeSpan? timeout = null)
        {
            if (special == null) throw new ArgumentNullException(nameof(special));
            return Base.AUTDSendSpecial(CntPtr, special.Ptr, (ulong)(timeout?.TotalMilliseconds * 1000 * 1000 ?? 0));
        }

        public bool Send(Header header, TimeSpan? timeout = null)
        {
            if (header == null) throw new ArgumentNullException(nameof(header));
            return Base.AUTDSend(CntPtr, header.Ptr, IntPtr.Zero, (ulong)(timeout?.TotalMilliseconds * 1000 * 1000 ?? 0));
        }

        public bool Send(Body body, TimeSpan? timeout = null)
        {
            if (body == null) throw new ArgumentNullException(nameof(body));
            return Base.AUTDSend(CntPtr, IntPtr.Zero, body.Ptr, (ulong)(timeout?.TotalMilliseconds * 1000 * 1000 ?? 0));
        }

        public bool Send(Header header, Body body, TimeSpan? timeout = null)
        {
            if (header == null) throw new ArgumentNullException(nameof(header));
            if (body == null) throw new ArgumentNullException(nameof(body));
            return Base.AUTDSend(CntPtr, header.Ptr, body.Ptr, (ulong)(timeout?.TotalMilliseconds * 1000 * 1000 ?? 0));
        }

        public bool Send(Body body, Header header, TimeSpan? timeout = null)
        {
            if (header == null) throw new ArgumentNullException(nameof(header));
            if (body == null) throw new ArgumentNullException(nameof(body));
            return Base.AUTDSend(CntPtr, header.Ptr, body.Ptr, (ulong)(timeout?.TotalMilliseconds * 1000 * 1000 ?? 0));
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
            private readonly List<Gain> _gains;

            public Grouped()
            {
                Base.AUTDGainGrouped(out handle);
                _gains = new List<Gain>();
            }

            public void Add(int deviceIdx, Gain gain)
            {
                Base.AUTDGainGroupedAdd(handle, deviceIdx, gain.Ptr);
                _gains.Add(gain);
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
            public Custom(autd3_float_t[] data, uint freqDiv)
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
            public FocusSTM()
            {
                Base.AUTDFocusSTM(out handle);
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
            private readonly List<AUTD3Sharp.Gain.Gain> _gains;

            public GainSTM(Mode mode = Mode.PhaseDutyFull)
            {
                Base.AUTDGainSTM(out handle, (ushort)mode);
                _gains = new List<AUTD3Sharp.Gain.Gain>();
            }

            public void Add(AUTD3Sharp.Gain.Gain gain)
            {
                Base.AUTDGainSTMAdd(handle, gain.Ptr);
                _gains.Add(gain);
            }
        }
    }
}
