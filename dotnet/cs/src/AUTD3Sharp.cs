/*
 * File: AUTD3Sharp.cs
 * Project: src
 * Created Date: 23/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 30/05/2023
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
using System.Runtime.InteropServices;
using System.Linq;

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

    public static class AUTD3
    {
        #region const

#if DIMENSION_M
        public const float_t Meter = 1;
#else
        public const float_t Meter = 1000;
#endif

        public const float_t Millimeter = Meter / 1000;

        public const float_t Pi = Math.PI;

        public const uint NumTransInUnit = Def.NumTransInUnit;
        public const float_t TransSpacingMm = Def.TransSpacingMm;
        public const float_t TransSpacing = Def.TransSpacingMm * Millimeter;
        public const uint NumTransInX = Def.NumTransInX;
        public const uint FpgaClkFreq = Def.FpgaClkFreq;
        public const float_t DeviceHeight = Def.DeviceHeightMm * Millimeter;
        public const float_t DeviceWidth = Def.DeviceWidthMm * Millimeter;
        public const uint NumTransInY = Def.NumTransInY;
        public const uint FpgaSubClkFreq = Def.FpgaSubClkFreq;

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
                var err = new byte[256];
                if (!Base.AUTDSetTransFrequency(_cnt, Id, value, err))
                    throw new AUTDException(err);
            }
        }

        public ushort Cycle
        {
            get => Base.AUTDGetTransCycle(_cnt, Id);
            set
            {
                var err = new byte[256];
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
        internal readonly TransMode Mode;

        internal Geometry(IntPtr cnt, TransMode mode)
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
            private TransMode _mode;

            public Builder()
            {
                _builderPtr = Base.AUTDCreateGeometryBuilder();
                _mode = TransMode.Legacy;
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
                _mode = TransMode.Legacy;
                return this;
            }

            public Builder AdvancedMode()
            {
                _mode = TransMode.Advanced;
                return this;
            }

            public Builder AdvancedPhaseMode()
            {
                _mode = TransMode.AdvancedPhase;
                return this;
            }

            public Geometry Build()
            {
                var err = new byte[256];
                var geometryPtr = Base.AUTDBuildGeometry(_builderPtr, err);
                if (geometryPtr == IntPtr.Zero) throw new AUTDException(err);
                return new Geometry(geometryPtr, _mode);
            }
        }
    }

    public readonly struct FPGAInfo
    {
        private readonly byte _info;

        internal FPGAInfo(byte info)
        {
            _info = info;
        }

        public bool IsThermalAssert => (_info & 0x01) != 0;
    }


    public sealed class Controller : IDisposable
    {
        #region field

        private bool _isDisposed;
        internal IntPtr CntPtr;

        #endregion

        #region Controller

        public static Controller Open(Geometry geometry, Link.Link link)
        {
            var err = new byte[256];
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
            var err = new byte[256];
            var handle = Base.AUTDGetFirmwareInfoListPointer(CntPtr, err);
            if (handle == IntPtr.Zero)
                throw new AUTDException(err);

            for (uint i = 0; i < Geometry.NumDevices; i++)
            {
                var info = new byte[256];
                Base.AUTDGetFirmwareInfo(handle, i, info, out var isValid, out var isSupported);
                yield return new FirmwareInfo(System.Text.Encoding.UTF8.GetString(info), isValid, isSupported);
            }

            Base.AUTDFreeFirmwareInfoListPointer(handle);
        }

        public void Close()
        {
            if (CntPtr == IntPtr.Zero) return;
            var err = new byte[256];
            if (!Base.AUTDClose(CntPtr, err))
                throw new AUTDException(err);
        }

        public void Dispose()
        {
            if (_isDisposed) return;

            if (CntPtr != IntPtr.Zero) Base.AUTDFreeController(CntPtr);
            CntPtr = IntPtr.Zero;

            _isDisposed = true; GC.SuppressFinalize(this);
        }

        ~Controller()
        {
            Dispose();
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

        public FPGAInfo[] FPGAInfo
        {
            get
            {
                var infos = new byte[Geometry.NumDevices];
                var err = new byte[256];
                if (!Base.AUTDGetFPGAInfo(CntPtr, infos, err))
                    throw new AUTDException(err);
                return infos.Select(x => new FPGAInfo(x)).ToArray();
            }
        }
        #endregion

        public bool Send(SpecialData special, TimeSpan? timeout = null)
        {
            if (special == null) throw new ArgumentNullException(nameof(special));
            var err = new byte[256];
            var res = Base.AUTDSendSpecial(CntPtr, Geometry.Mode, special.Ptr, (long)(timeout?.TotalMilliseconds * 1000 * 1000 ?? -1), err);
            if (res == Def.Err)
            {
                throw new AUTDException(err);
            }
            return res == Def.True;
        }

        public bool Send(Header header, TimeSpan? timeout = null)
        {
            if (header == null) throw new ArgumentNullException(nameof(header));
            var err = new byte[256];
            var res = Base.AUTDSend(CntPtr, Geometry.Mode, header.Ptr, IntPtr.Zero, (long)(timeout?.TotalMilliseconds * 1000 * 1000 ?? -1), err);
            if (res == Def.Err)
            {
                throw new AUTDException(err);
            }
            return res == Def.True;
        }

        public bool Send(Body body, TimeSpan? timeout = null)
        {
            if (body == null) throw new ArgumentNullException(nameof(body));
            var err = new byte[256];
            var res = Base.AUTDSend(CntPtr, Geometry.Mode, IntPtr.Zero, body.Ptr, (long)(timeout?.TotalMilliseconds * 1000 * 1000 ?? -1), err);
            if (res == Def.Err)
            {
                throw new AUTDException(err);
            }
            return res == Def.True;
        }

        public bool Send(Header header, Body body, TimeSpan? timeout = null)
        {
            if (header == null) throw new ArgumentNullException(nameof(header));
            if (body == null) throw new ArgumentNullException(nameof(body));
            var err = new byte[256];
            var res = Base.AUTDSend(CntPtr, Geometry.Mode, header.Ptr, body.Ptr, (long)(timeout?.TotalMilliseconds * 1000 * 1000 ?? -1), err);
            if (res == Def.Err)
            {
                throw new AUTDException(err);
            }
            return res == Def.True;

        }

        public bool Send(Body body, Header header, TimeSpan? timeout = null)
        {
            if (header == null) throw new ArgumentNullException(nameof(header));
            if (body == null) throw new ArgumentNullException(nameof(body));
            var err = new byte[256];
            var res = Base.AUTDSend(CntPtr, Geometry.Mode, header.Ptr, body.Ptr, (long)(timeout?.TotalMilliseconds * 1000 * 1000 ?? -1), err);
            if (res == Def.Err)
            {
                throw new AUTDException(err);
            }
            return res == Def.True;
        }
    }

    public sealed class UpdateFlag : SpecialData
    {
        public UpdateFlag() : base(Base.AUTDUpdateFlags())
        {
        }
    }

    public sealed class Clear : SpecialData
    {
        public Clear() : base(Base.AUTDClear())
        {
        }
    }

    public sealed class Synchronize : SpecialData
    {
        public Synchronize() : base(Base.AUTDSynchronize())
        {
        }
    }

    public sealed class Stop : SpecialData
    {
        public Stop() : base(Base.AUTDStop())
        {
        }
    }
    public sealed class ModDelayConfig : SpecialData
    {
        public ModDelayConfig() : base(Base.AUTDModDelayConfig())
        {
        }
    }

    public sealed class SilencerConfig : Header
    {
        public SilencerConfig(ushort step = 10) : base(Base.AUTDCreateSilencer(step))
        {
        }

        ~SilencerConfig()
        {
            Base.AUTDDeleteSilencer(Ptr);
        }

        public static SilencerConfig None()
        {
            return new SilencerConfig(0xFFFF);
        }
    }

    public sealed class Amplitudes : Body
    {
        public Amplitudes(float_t amp = 1) : base(Base.AUTDCreateAmplitudes(amp))
        {
        }

        ~Amplitudes()
        {
            Base.AUTDDeleteAmplitudes(Ptr);
        }
    }

    namespace Gain
    {

        [ComVisible(false)]
        public abstract class Gain : Body
        {
            internal Gain(IntPtr ptr) : base(ptr)
            {
            }

            ~Gain()
            {
                if (Ptr == IntPtr.Zero) return;
                Base.AUTDDeleteGain(Ptr);
            }
        }

        public sealed class Focus : Gain
        {
            public Focus(Vector3 point, float_t amp = 1) : base(Base.AUTDGainFocus(point.x, point.y, point.z, amp)) { }
        }

        public sealed class Grouped : Gain
        {
            public Grouped() : base(Base.AUTDGainGrouped())
            {
            }

            public void Add(int deviceIdx, Gain gain)
            {
                Base.AUTDGainGroupedAdd(Ptr, (uint)deviceIdx, gain.Ptr);
                gain.Ptr = IntPtr.Zero;
            }
        }

        public sealed class BesselBeam : Gain
        {
            public BesselBeam(Vector3 point, Vector3 dir, float_t thetaZ, float_t amp = 1) : base(Base.AUTDGainBesselBeam(point.x, point.y, point.z, dir.x, dir.y, dir.z, thetaZ, amp)) { }
        }

        public sealed class PlaneWave : Gain
        {
            public PlaneWave(Vector3 dir, float_t amp = 1) : base(Base.AUTDGainPlaneWave(dir.x, dir.y, dir.z, amp)) { }
        }

        public sealed class Custom : Gain
        {
            public Custom(float_t[] amp, float_t[] phase) : base(Base.AUTDGainCustom(amp, phase, (ulong)amp.Length))
            {
            }
        }

        public sealed class Null : Gain
        {
            public Null() : base(Base.AUTDGainNull())
            {
            }
        }
    }

    namespace Modulation
    {

        [ComVisible(false)]
        public abstract class Modulation : Header
        {
            internal Modulation(IntPtr ptr) : base(ptr)
            {
            }

            ~Modulation()
            {
                Base.AUTDDeleteModulation(Ptr);
            }

            public float_t SamplingFrequency => Base.AUTDModulationSamplingFrequency(Ptr);
            public uint SamplingFrequencyDivision
            {
                get => Base.AUTDModulationSamplingFrequencyDivision(Ptr);
                set => Base.AUTDModulationSetSamplingFrequencyDivision(Ptr, value);
            }
        }

        public sealed class Static : Modulation
        {
            public Static(float_t amp = 1) : base(Base.AUTDModulationStatic(amp))
            {
            }
        }

        public sealed class Sine : Modulation
        {
            public Sine(int freq, float_t amp = 1, float_t offset = (float_t)0.5) : base(Base.AUTDModulationSine((uint)freq, amp, offset))
            {
            }
        }

        public sealed class SineSquared : Modulation
        {
            public SineSquared(int freq, float_t amp = 1, float_t offset = (float_t)0.5) : base(Base.AUTDModulationSineSquared((uint)freq, amp, offset))
            {
            }
        }

        public sealed class SineLegacy : Modulation
        {
            public SineLegacy(float_t freq, float_t amp = 1, float_t offset = (float_t)0.5) : base(Base.AUTDModulationSineLegacy(freq, amp, offset))
            {
            }
        }


        public sealed class Square : Modulation
        {
            public Square(int freq, float_t low = 0, float_t high = 1, float_t duty = (float_t)0.5) : base(Base.AUTDModulationSquare((uint)freq, low, high, duty))
            {
            }
        }

        public sealed class Custom : Modulation
        {
            public Custom(float_t[] data, uint freqDiv) : base(Base.AUTDModulationCustom(data, (ulong)data.Length, freqDiv))
            {
            }
        }
    }

    namespace STM
    {
        public sealed class FocusSTM : Body
        {
            public FocusSTM() : base(Base.AUTDFocusSTM())
            {
            }

            public void Add(Vector3 point, byte shift = 0) => Base.AUTDFocusSTMAdd(Ptr, point.x, point.y, point.z, shift);

            ~FocusSTM()
            {
                Base.AUTDDeleteFocusSTM(Ptr);
            }

            public float_t Frequency
            {
                get => Base.AUTDFocusSTMFrequency(Ptr);
                set => Base.AUTDFocusSTMSetFrequency(Ptr, value);
            }

            public int? StartIdx
            {
                get
                {
                    var idx = Base.AUTDFocusSTMGetStartIdx(Ptr);
                    if (idx < 0) return null;
                    return idx;
                }
                set => Base.AUTDFocusSTMSetStartIdx(Ptr, value ?? -1);
            }

            public int? FinishIdx
            {
                get
                {
                    var idx = Base.AUTDFocusSTMGetFinishIdx(Ptr);
                    if (idx < 0) return null;
                    return idx;
                }
                set => Base.AUTDFocusSTMSetFinishIdx(Ptr, value ?? -1);
            }

            public float_t SamplingFrequency => Base.AUTDFocusSTMSamplingFrequency(Ptr);
            public uint SamplingFrequencyDivision
            {
                get => Base.AUTDFocusSTMSamplingFrequencyDivision(Ptr);
                set => Base.AUTDFocusSTMSetSamplingFrequencyDivision(Ptr, value);
            }
        }

        public sealed class GainSTM : Body
        {
            public GainSTM() : base(Base.AUTDGainSTM())
            {
            }

            public void Add(AUTD3Sharp.Gain.Gain gain)
            {
                Base.AUTDGainSTMAdd(Ptr, gain.Ptr);
                gain.Ptr = IntPtr.Zero;
            }

            ~GainSTM()
            {
                Base.AUTDDeleteGainSTM(Ptr);
            }

            public GainSTMMode Mode
            {
                set => Base.AUTDGainSTMSetMode(Ptr, value);
            }

            public float_t Frequency
            {
                get => Base.AUTDGainSTMFrequency(Ptr);
                set => Base.AUTDGainSTMSetFrequency(Ptr, value);
            }

            public int? StartIdx
            {
                get
                {
                    var idx = Base.AUTDGainSTMGetStartIdx(Ptr);
                    if (idx < 0) return null;
                    return idx;
                }
                set => Base.AUTDGainSTMSetStartIdx(Ptr, value ?? -1);
            }

            public int? FinishIdx
            {
                get
                {
                    var idx = Base.AUTDGainSTMGetFinishIdx(Ptr);
                    if (idx < 0) return null;
                    return idx;
                }
                set => Base.AUTDGainSTMSetFinishIdx(Ptr, value ?? -1);
            }

            public float_t SamplingFrequency => Base.AUTDGainSTMSamplingFrequency(Ptr);
            public uint SamplingFrequencyDivision
            {
                get => Base.AUTDGainSTMSamplingFrequencyDivision(Ptr);
                set => Base.AUTDGainSTMSetSamplingFrequencyDivision(Ptr, value);
            }
        }
    }
}
