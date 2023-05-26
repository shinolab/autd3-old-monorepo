/*
 * File: AUTD3Sharp.cs
 * Project: src
 * Created Date: 23/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/05/2023
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
using float_t = System.Single;
#else
using float_t = System.Double;
#endif

namespace AUTD3Sharp
{
    using Base = NativeMethods.Base;

    public static class AUTD3
    {
        #region const

#if DIMENSION_M
        public const float_t Millimeter = 1 / 1000;
#else
        public const float_t Millimeter = 1;
#endif

        public const float_t Pi = Math.PI;

        public const uint NumTransInUnit = Base.NumTransInUnit;
        public const float_t TransSpacingMm = Base.TransSpacingMm;
        public const float_t TransSpacing = Base.TransSpacingMm * Millimeter;
        public const uint NumTransInX = Base.NumTransInX;
        public const uint FpgaClkFreq = Base.FpgaClkFreq;
        public const float_t DeviceHeight = Base.DeviceHeight * Millimeter;
        public const float_t DeviceWidth = Base.DeviceWidth * Millimeter;
        public const uint NumTransInY = Base.NumTransInY;
        public const uint FpgaSubClkFreq = Base.FpgaSubClkFreq;

        #endregion
    }

    public sealed class Transducer
    {
        private readonly IntPtr _cnt;

        internal Transducer(uint trId, IntPtr cnt)
        {
            Id = trId;
            _cnt = cnt;
        }

        public uint Id { get; }

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

        public float_t Wavelength => Base.AUTDGetWavelength(_cnt, Id);

        public float_t Frequency
        {
            get => Base.AUTDGetTransFrequency(_cnt, Id);
            set
            {
                var err = new StringBuilder(256);
                if (!Base.AUTDSetTransFrequency(_cnt, Id, value, err))
                    throw new AUTDException(err);
            }
        }

        public ushort Cycle
        {
            get => Base.AUTDGetTransCycle(_cnt, Id);
            set
            {
                var err = new StringBuilder(256);
                if (!Base.AUTDSetTransCycle(_cnt, Id, value, err))
                    throw new AUTDException(err);
            }
        }

        public ushort ModDelay
        {
            get => Base.AUTDGetTransModDelay(_cnt, Id);
            set => Base.AUTDSetTransModDelay(_cnt, Id, value);
        }
    }

    public sealed class Geometry : IEnumerable<Transducer>
    {
        internal IntPtr Ptr;
        internal readonly NativeMethods.TransMode Mode;

        internal Geometry(IntPtr cnt, NativeMethods.TransMode mode)
        {
            Ptr = cnt;
            Mode = mode;
        }

        public uint NumTransducers => Base.AUTDNumTransducers(Ptr);

        public uint NumDevices => Base.AUTDNumDevices(Ptr);

        public float_t SoundSpeed
        {
            get => Base.AUTDGetSoundSpeed(Ptr);
            set => Base.AUTDSetSoundSpeed(Ptr, value);
        }

        public float_t Attenuation
        {
            get => Base.AUTDGetAttenuation(Ptr);
            set => Base.AUTDSetAttenuation(Ptr, value);
        }

        public Vector3 Center
        {
            get
            {
                Base.AUTDGeometryCenter(Ptr, out var x, out var y, out var z);
                return new Vector3(x, y, z);
            }
        }

        public Transducer this[int index]
        {
            get
            {
                if (index >= NumTransducers) throw new IndexOutOfRangeException();
                return new Transducer((uint)index, Ptr);
            }
        }

        public void SetSoundSpeedFromTemp(float_t temp, float_t k = (float_t)1.4, float_t r = (float_t)8.31446261815324, float_t m = (float_t)28.9647e-3)
        {
            Base.AUTDSetSoundSpeedFromTemp(Ptr, temp, k, r, m);
        }

        public Vector3 CenterOf(int devIdx)
        {
            Base.AUTDGeometryCenterOf(Ptr, (uint)devIdx, out var x, out var y, out var z);
            return new Vector3(x, y, z);
        }

        public sealed class TransducerEnumerator : IEnumerator<Transducer>
        {
            private int _idx;
            private readonly IntPtr _cnt;
            private readonly uint _numTrans;

            internal TransducerEnumerator(IntPtr cnt)
            {
                _idx = -1;
                _cnt = cnt;
                _numTrans = Base.AUTDNumTransducers(_cnt);
            }

            public bool MoveNext() => ++_idx < _numTrans;

            public void Reset() => _idx = -1;

            public Transducer Current => new Transducer((uint)_idx, _cnt);

            object System.Collections.IEnumerator.Current => Current;

            public void Dispose() { }
        }

        public IEnumerator<Transducer> GetEnumerator() => new TransducerEnumerator(Ptr);

        System.Collections.IEnumerator System.Collections.IEnumerable.GetEnumerator() => GetEnumerator();

        public sealed class Builder
        {
            private readonly IntPtr _builderPtr;
            private NativeMethods.TransMode _mode;

            public Builder()
            {
                _builderPtr = Base.AUTDCreateGeometryBuilder();
                _mode = NativeMethods.TransMode.Legacy;
            }

            public Builder AddDevice(Vector3 position, Vector3 rotation)
            {
                Base.AUTDAddDevice(_builderPtr, position.x, position.y, position.z, rotation.x, rotation.y, rotation.z);
                return this;
            }

            public Builder AddDevice(Vector3 position, Quaternion quaternion)
            {
                Base.AUTDAddDeviceQuaternion(_builderPtr, position.x, position.y, position.z, quaternion.w, quaternion.x, quaternion.y, quaternion.z);
                return this;
            }

            public Builder LegacyMode()
            {
                _mode = NativeMethods.TransMode.Legacy;
                return this;
            }

            public Builder AdvancedMode()
            {
                _mode = NativeMethods.TransMode.Advanced;
                return this;
            }

            public Builder AdvancedPhaseMode()
            {
                _mode = NativeMethods.TransMode.AdvancedPhase;
                return this;
            }

            public Geometry Build()
            {
                var err = new StringBuilder(256);
                var geometryPtr = Base.AUTDBuildGeometry(_builderPtr, err);
                if (geometryPtr == IntPtr.Zero) throw new AUTDException(err);
                return new Geometry(geometryPtr, _mode);
            }
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
            var err = new StringBuilder(256);
            var cnt = Base.AUTDOpenController(geometry.Ptr, link.LinkPtr, err);
            if (cnt == IntPtr.Zero)
                throw new AUTDException(err);
            return new Controller(cnt, new Geometry(cnt, geometry.Mode));
        }

        private Controller(IntPtr cnt, Geometry geometry)
        {
            CntPtr = cnt;
            Geometry = geometry;
        }

        public IEnumerable<FirmwareInfo> FirmwareInfoList()
        {
            var err = new StringBuilder(256);
            var handle = Base.AUTDGetFirmwareInfoListPointer(CntPtr, err);
            if (handle == IntPtr.Zero)
                throw new AUTDException(err);

            for (uint i = 0; i < Geometry.NumDevices; i++)
            {
                var info = new StringBuilder(256);
                Base.AUTDGetFirmwareInfo(handle, i, info, out var matchesVersion, out var isSupported);
                yield return new FirmwareInfo(info.ToString(), matchesVersion, isSupported);
            }

            Base.AUTDFreeFirmwareInfoListPointer(handle);
        }

        public void Close()
        {
            var err = new StringBuilder(256);
            if (!Base.AUTDClose(CntPtr, err))
                throw new AUTDException(err);
        }

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
                var err = new StringBuilder(256);
                if (!Base.AUTDGetFPGAInfo(CntPtr, infos, err))
                    throw new AUTDException(err);
                return infos;
            }
        }
        #endregion

        public bool Send(SpecialData special, TimeSpan? timeout = null)
        {
            if (special == null) throw new ArgumentNullException(nameof(special));
            var err = new StringBuilder(256);
            var res = Base.AUTDSendSpecial(CntPtr, Geometry.Mode, special.Ptr, (long)(timeout?.TotalMilliseconds * 1000 * 1000 ?? -1), err);
            if (res == Base.Err)
            {
                throw new AUTDException(err);
            }
            return res == Base.True;
        }

        public bool Send(Header header, TimeSpan? timeout = null)
        {
            if (header == null) throw new ArgumentNullException(nameof(header));
            var err = new StringBuilder(256);
            var res = Base.AUTDSend(CntPtr, Geometry.Mode, header.Ptr, IntPtr.Zero, (long)(timeout?.TotalMilliseconds * 1000 * 1000 ?? -1), err);
            if (res == Base.Err)
            {
                throw new AUTDException(err);
            }
            return res == Base.True;
        }

        public bool Send(Body body, TimeSpan? timeout = null)
        {
            if (body == null) throw new ArgumentNullException(nameof(body));
            var err = new StringBuilder(256);
            var res = Base.AUTDSend(CntPtr, Geometry.Mode, IntPtr.Zero, body.Ptr, (long)(timeout?.TotalMilliseconds * 1000 * 1000 ?? -1), err);
            if (res == Base.Err)
            {
                throw new AUTDException(err);
            }
            return res == Base.True;
        }

        public bool Send(Header header, Body body, TimeSpan? timeout = null)
        {
            if (header == null) throw new ArgumentNullException(nameof(header));
            if (body == null) throw new ArgumentNullException(nameof(body));
            var err = new StringBuilder(256);
            var res = Base.AUTDSend(CntPtr, Geometry.Mode, header.Ptr, body.Ptr, (long)(timeout?.TotalMilliseconds * 1000 * 1000 ?? -1), err);
            if (res == Base.Err)
            {
                throw new AUTDException(err);
            }
            return res == Base.True;

        }

        public bool Send(Body body, Header header, TimeSpan? timeout = null)
        {
            if (header == null) throw new ArgumentNullException(nameof(header));
            if (body == null) throw new ArgumentNullException(nameof(body));
            var err = new StringBuilder(256);
            var res = Base.AUTDSend(CntPtr, Geometry.Mode, header.Ptr, body.Ptr, (long)(timeout?.TotalMilliseconds * 1000 * 1000 ?? -1), err);
            if (res == Base.Err)
            {
                throw new AUTDException(err);
            }
            return res == Base.True;
        }
    }

    public sealed class UpdateFlag : SpecialData
    {
        public UpdateFlag()
        {
            handle = Base.AUTDUpdateFlags();
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
            handle = Base.AUTDClear();
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
            handle = Base.AUTDSynchronize();
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
            handle = Base.AUTDStop();
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
            handle = Base.AUTDModDelayConfig();
        }

        protected override bool ReleaseHandle()
        {
            Base.AUTDDeleteSpecialData(handle);
            return true;
        }
    }

    public sealed class SilencerConfig : Header
    {
        public SilencerConfig(ushort step = 10)
        {
            handle = Base.AUTDCreateSilencer(step);
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
        public Amplitudes(float_t amp = (float_t)1.0)
        {
            handle = Base.AUTDCreateAmplitudes(amp);
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
                if (handle == IntPtr.Zero) return true;
                Base.AUTDDeleteGain(handle);
                return true;
            }
        }

        public sealed class Focus : Gain
        {
            public Focus(Vector3 point, float_t amp = (float_t)1.0) => handle = Base.AUTDGainFocus(point.x, point.y, point.z, amp);
        }

        public sealed class Grouped : Gain
        {
            public Grouped()
            {
                handle = Base.AUTDGainGrouped();
            }

            public void Add(int deviceIdx, Gain gain)
            {
                Base.AUTDGainGroupedAdd(handle, (uint)deviceIdx, gain.Ptr);
                gain.SetHandleAsInvalid();
            }
        }

        public sealed class BesselBeam : Gain
        {
            public BesselBeam(Vector3 point, Vector3 dir, float_t thetaZ, float_t amp = (float_t)1.0) => handle = Base.AUTDGainBesselBeam(point.x, point.y, point.z, dir.x, dir.y, dir.z, thetaZ, amp);
        }

        public sealed class PlaneWave : Gain
        {
            public PlaneWave(Vector3 dir, float_t amp = (float_t)1.0) => handle = Base.AUTDGainPlaneWave(dir.x, dir.y, dir.z, amp);
        }

        public sealed class Custom : Gain
        {
            public Custom(float_t[] amp, float_t[] phase)
            {
                if (amp.Length != phase.Length) throw new ArgumentException();
                var length = amp.Length;
                handle = Base.AUTDGainCustom(amp, phase, (ulong)length);
            }
        }

        public sealed class Null : Gain
        {
            public Null()
            {
                handle = Base.AUTDGainNull();
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

            public float_t SamplingFrequency => Base.AUTDModulationSamplingFrequency(handle);
            public uint SamplingFrequencyDivision
            {
                get => Base.AUTDModulationSamplingFrequencyDivision(handle);
                set => Base.AUTDModulationSetSamplingFrequencyDivision(handle, value);
            }
        }

        public sealed class Static : Modulation
        {
            public Static(float_t amp = (float_t)1.0)
            {
                handle = Base.AUTDModulationStatic(amp);

            }
        }

        public sealed class Sine : Modulation
        {
            public Sine(int freq, float_t amp = (float_t)1.0, float_t offset = (float_t)0.5)
            {
                handle = Base.AUTDModulationSine((uint)freq, amp, offset);
            }
        }

        public sealed class SineSquared : Modulation
        {
            public SineSquared(int freq, float_t amp = (float_t)1.0, float_t offset = (float_t)0.5)
            {
                handle = Base.AUTDModulationSineSquared((uint)freq, amp, offset);
            }
        }

        public sealed class SineLegacy : Modulation
        {
            public SineLegacy(float_t freq, float_t amp = (float_t)1.0, float_t offset = (float_t)0.5)
            {
                handle = Base.AUTDModulationSineLegacy(freq, amp, offset);
            }
        }


        public sealed class Square : Modulation
        {
            public Square(int freq, float_t low = (float_t)0.0, float_t high = (float_t)1.0, float_t duty = (float_t)0.5)
            {
                handle = Base.AUTDModulationSquare((uint)freq, low, high, duty);
            }
        }

        public sealed class Custom : Modulation
        {
            public Custom(float_t[] data, uint freqDiv)
            {
                handle = Base.AUTDModulationCustom(data, (ulong)data.Length, freqDiv);
            }
        }
    }

    namespace STM
    {
        public sealed class FocusSTM : Body
        {
            public FocusSTM()
            {
                handle = Base.AUTDFocusSTM();
            }

            public void Add(Vector3 point, byte shift = 0) => Base.AUTDFocusSTMAdd(handle, point.x, point.y, point.z, shift);

            protected override bool ReleaseHandle()
            {
                Base.AUTDDeleteFocusSTM(handle);
                return true;
            }

            public float_t Frequency
            {
                get => Base.AUTDFocusSTMFrequency(handle);
                set => Base.AUTDFocusSTMSetFrequency(handle, value);
            }

            public int? StartIdx
            {
                get
                {
                    var idx = Base.AUTDFocusSTMGetStartIdx(handle);
                    if (idx < 0) return null;
                    return idx;
                }
                set => Base.AUTDFocusSTMSetStartIdx(handle, value == null ? -1 : value.Value);
            }

            public int? FinishIdx
            {
                get
                {
                    var idx = Base.AUTDFocusSTMGetFinishIdx(handle);
                    if (idx < 0) return null;
                    return idx;
                }
                set => Base.AUTDFocusSTMSetFinishIdx(handle, value == null ? -1 : value.Value);
            }

            public float_t SamplingFrequency => Base.AUTDFocusSTMSamplingFrequency(handle);
            public uint SamplingFrequencyDivision
            {
                get => Base.AUTDFocusSTMSamplingFrequencyDivision(handle);
                set => Base.AUTDFocusSTMSetSamplingFrequencyDivision(handle, value);
            }
        }

        public sealed class GainSTM : Body
        {
            public GainSTM()
            {
                handle = Base.AUTDGainSTM();
            }

            public void Add(AUTD3Sharp.Gain.Gain gain)
            {
                Base.AUTDGainSTMAdd(handle, gain.Ptr);
                gain.SetHandleAsInvalid();
            }

            protected override bool ReleaseHandle()
            {
                Base.AUTDDeleteGainSTM(handle);
                return true;
            }

            public NativeMethods.GainSTMMode Mode
            {
                set => Base.AUTDGainSTMSetMode(handle, value);
            }

            public float_t Frequency
            {
                get => Base.AUTDGainSTMFrequency(handle);
                set => Base.AUTDGainSTMSetFrequency(handle, value);
            }

            public int? StartIdx
            {
                get
                {
                    var idx = Base.AUTDGainSTMGetStartIdx(handle);
                    if (idx < 0) return null;
                    return idx;
                }
                set => Base.AUTDGainSTMSetStartIdx(handle, value == null ? -1 : value.Value);
            }

            public int? FinishIdx
            {
                get
                {
                    var idx = Base.AUTDGainSTMGetFinishIdx(handle);
                    if (idx < 0) return null;
                    return idx;
                }
                set => Base.AUTDGainSTMSetFinishIdx(handle, value == null ? -1 : value.Value);
            }

            public float_t SamplingFrequency => Base.AUTDGainSTMSamplingFrequency(handle);
            public uint SamplingFrequencyDivision
            {
                get => Base.AUTDGainSTMSamplingFrequencyDivision(handle);
                set => Base.AUTDGainSTMSetSamplingFrequencyDivision(handle, value);
            }
        }
    }
}
