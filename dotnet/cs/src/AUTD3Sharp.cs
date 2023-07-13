/*
 * File: AUTD3Sharp.cs
 * Project: src
 * Created Date: 23/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 13/07/2023
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
using AUTD3Sharp.Gain;

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

    public class AUTD3
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

        internal Vector3 Pos;
        internal Vector3? Rot;
        internal Quaternion? Quat;

        public AUTD3(Vector3 pos, Vector3 rot)
        {
            Pos = pos;
            Rot = rot;
            Quat = null;
        }

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

        public int Idx { get; }

        public Vector3 Position
        {
            get
            {
                var pos = new float_t[3];
                Base.AUTDTransPosition(_ptr, (uint)Idx, pos);
                return new Vector3(pos[0], pos[1], pos[2]);
            }
        }

        public Quaternion Rotation
        {
            get
            {
                var rot = new float_t[4];
                Base.AUTDTransRotation(_ptr, (uint)Idx, rot);
                return new Quaternion(rot[1], rot[2], rot[3], rot[0]);
            }
        }

        public Vector3 XDirection
        {
            get
            {
                var dir = new float_t[3];
                Base.AUTDTransXDirection(_ptr, (uint)Idx, dir);
                return new Vector3(dir[0], dir[1], dir[2]);
            }
        }

        public Vector3 YDirection
        {
            get
            {
                var dir = new float_t[3];
                Base.AUTDTransYDirection(_ptr, (uint)Idx, dir);
                return new Vector3(dir[0], dir[1], dir[2]);
            }
        }

        public Vector3 ZDirection
        {
            get
            {
                var dir = new float_t[3];
                Base.AUTDTransZDirection(_ptr, (uint)Idx, dir);
                return new Vector3(dir[0], dir[1], dir[2]);
            }
        }
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

        public ushort ModDelay
        {
            get => Base.AUTDGetTransModDelay(_ptr, (uint)Idx);
            set => Base.AUTDSetTransModDelay(_ptr, (uint)Idx, value);
        }


        public float_t Wavelength(float_t soundSpeed) => Base.AUTDGetWavelength(_ptr, (uint)Idx, soundSpeed);

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

        public int NumTransducers => (int)Base.AUTDNumTransducers(Ptr);

        public int NumDevices => (int)Base.AUTDNumDevices(Ptr);

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
                var center = new float_t[3];
                Base.AUTDGeometryCenter(Ptr, center);
                return new Vector3(center[0], center[1], center[2]);
            }
        }

        public Transducer this[int index] => _transducers[index];

        public void SetSoundSpeedFromTemp(float_t temp, float_t k = (float_t)1.4, float_t r = (float_t)8.31446261815324, float_t m = (float_t)28.9647e-3)
        {
            Base.AUTDSetSoundSpeedFromTemp(Ptr, temp, k, r, m);
        }

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

        public bool IsThermalAssert => (_info & 0x01) != 0;
    }


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

            public ControllerBuilder AddDevice(AUTD3 device)
            {
                if (device.Rot != null)
                    _ptr = Base.AUTDAddDevice(_ptr, device.Pos.x, device.Pos.y, device.Pos.z, device.Rot.Value.x, device.Rot.Value.y, device.Rot.Value.z);
                else if (device.Quat != null)
                    _ptr = Base.AUTDAddDeviceQuaternion(_ptr, device.Pos.x, device.Pos.y, device.Pos.z, device.Quat.Value.w, device.Quat.Value.x, device.Quat.Value.y, device.Quat.Value.z);
                return this;
            }

            public ControllerBuilder LegacyMode()
            {
                _mode = TransMode.Legacy;
                return this;
            }

            public ControllerBuilder AdvancedMode()
            {
                _mode = TransMode.Advanced;
                return this;
            }

            public ControllerBuilder AdvancedPhaseMode()
            {
                _mode = TransMode.AdvancedPhase;
                return this;
            }

            public
               Controller OpenWith(Link.Link link)
            {
                return Controller.OpenImpl(_ptr, _mode, link.Ptr);
            }

            internal ControllerBuilder()
            {
                _ptr = Base.AUTDCreateControllerBuilder();
                _mode = TransMode.Legacy;
            }
        }

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
            Base.AUTDSetForceFan(Ptr, value);
        }

        public void ReadsFPGAInfo(bool value)
        {
            Base.AUTDSetReadsFPGAInfo(Ptr, value);
        }

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

    public sealed class UpdateFlags : ISpecialData
    {
        public DatagramSpecialPtr Ptr() => Base.AUTDUpdateFlags();
    }

    public sealed class Clear : ISpecialData
    {
        public DatagramSpecialPtr Ptr() => Base.AUTDClear();
    }

    public sealed class Synchronize : ISpecialData
    {
        public DatagramSpecialPtr Ptr() => Base.AUTDSynchronize();
    }

    public sealed class Stop : ISpecialData
    {
        public DatagramSpecialPtr Ptr() => Base.AUTDStop();
    }

    public sealed class ModDelayConfig : ISpecialData
    {
        public DatagramSpecialPtr Ptr() => Base.AUTDModDelayConfig();
    }

    public sealed class SilencerConfig : IHeader
    {
        private readonly ushort _step;

        public SilencerConfig(ushort step = 10)
        {
            _step = step;
        }

        public DatagramHeaderPtr Ptr() => Base.AUTDCreateSilencer(_step);


        public static SilencerConfig None()
        {
            return new SilencerConfig(0xFFFF);
        }
    }

    public sealed class Amplitudes : IBody
    {
        private readonly float_t _amp;

        public Amplitudes(float_t amp = 1)
        {
            _amp = amp;
        }

        public DatagramBodyPtr Ptr(Geometry geometry) => Base.AUTDCreateAmplitudes(_amp);
    }

    namespace Gain
    {

        [ComVisible(false)]
        public abstract class GainBase : IBody
        {
            public DatagramBodyPtr Ptr(Geometry geometry) => Base.AUTDGainIntoDatagram(GainPtr(geometry));

            public abstract GainPtr GainPtr(Geometry geometry);
        }

        public sealed class Focus : GainBase
        {
            private readonly Vector3 _point;
            private float_t? _amp;

            public Focus(Vector3 point)
            {
                _point = point;
                _amp = null;
            }

            public Focus WithAmp(float_t amp)
            {
                _amp = amp;
                return this;
            }

            public override GainPtr GainPtr(Geometry geometry)
            {
                var ptr = Base.AUTDGainFocus(_point.x, _point.y, _point.z);
                if (_amp != null)
                    ptr = Base.AUTDGainFocusWithAmp(ptr, _amp.Value);
                return ptr;
            }
        }

        public sealed class Grouped : GainBase
        {
            private readonly List<(int, GainBase)> _gains;

            public Grouped()
            {
                _gains = new List<(int, GainBase)>();
            }

            public void AddGain(int deviceIdx, GainBase gain)
            {
                _gains.Add((deviceIdx, gain));
            }

            public override GainPtr GainPtr(Geometry geometry)
            {
                return _gains.Aggregate(Base.AUTDGainGrouped(), (current, gain) => Base.AUTDGainGroupedAdd(current, (uint)gain.Item1, gain.Item2.GainPtr(geometry)));
            }
        }

        public sealed class Bessel : GainBase
        {
            private readonly Vector3 _point;
            private readonly Vector3 _dir;
            private readonly float_t _thetaZ;
            private float_t? _amp;

            public Bessel(Vector3 point, Vector3 dir, float_t thetaZ)
            {
                _point = point;
                _dir = dir;
                _thetaZ = thetaZ;
                _amp = null;
            }

            public Bessel WithAmp(float_t amp)
            {
                _amp = amp;
                return this;
            }

            public override GainPtr GainPtr(Geometry geometry)
            {
                var ptr = Base.AUTDGainBessel(_point.x, _point.y, _point.z, _dir.x, _dir.y, _dir.z, _thetaZ);
                if (_amp != null)
                    ptr = Base.AUTDGainBesselWithAmp(ptr, _amp.Value);
                return ptr;
            }
        }

        public sealed class Plane : GainBase
        {
            private readonly Vector3 _dir;
            private float_t? _amp;

            public Plane(Vector3 dir)
            {
                _dir = dir;
                _amp = null;
            }

            public Plane WithAmp(float_t amp)
            {
                _amp = amp;
                return this;
            }

            public override GainPtr GainPtr(Geometry geometry)
            {
                var ptr = Base.AUTDGainPlane(_dir.x, _dir.y, _dir.z);
                if (_amp != null)
                    ptr = Base.AUTDGainPlaneWithAmp(ptr, _amp.Value);
                return ptr;
            }
        }

        public abstract class Gain : GainBase
        {
            sealed public override GainPtr GainPtr(Geometry geometry)
            {
                var drives = Calc(geometry);
                return Base.AUTDGainCustom(drives, (ulong)drives.Length);
            }

            public abstract Drive[] Calc(Geometry geometry);

            public static Drive[] Transform(Geometry geometry, Func<Transducer, Drive> f)
            {
                return geometry.Select(f).ToArray();
            }
        }

        public sealed class Cache : GainBase, IEnumerable<Drive>
        {
            private Drive[] _drives;

            public Cache(GainBase g, Geometry geometry)
            {
                var err = new byte[256];
                _drives = new Drive[geometry.NumTransducers];
                if (Base.AUTDGainCalc(g.GainPtr(geometry), geometry.Ptr, _drives, err) == Def.Autd3Err)
                    throw new AUTDException(err);
            }

            sealed public override GainPtr GainPtr(Geometry geometry)
            {
                return Base.AUTDGainCustom(_drives, (ulong)_drives.Length);
            }

            public Drive this[int index]
            {
                get => _drives[index];
                set => _drives[index] = value;
            }

            public Drive[] Drives => _drives;

            public IEnumerator<Drive> GetEnumerator() => _drives.AsEnumerable().GetEnumerator();

            System.Collections.IEnumerator System.Collections.IEnumerable.GetEnumerator() => GetEnumerator();
        }

        public sealed class Null : GainBase
        {
            public override GainPtr GainPtr(Geometry geometry) => Base.AUTDGainNull();
        }
    }

    namespace Modulation
    {

        [ComVisible(false)]
        public abstract class ModulationBase : IHeader
        {
            public float_t SamplingFrequency => Base.AUTDModulationSamplingFrequency(ModulationPtr());
            public uint SamplingFrequencyDivision => Base.AUTDModulationSamplingFrequencyDivision(ModulationPtr());

            public DatagramHeaderPtr Ptr() => Base.AUTDModulationIntoDatagram(ModulationPtr());

            public abstract ModulationPtr ModulationPtr();
        }

        public sealed class Static : ModulationBase
        {
            private float_t? _amp;

            public Static WithAmp(float_t amp)
            {
                _amp = amp;
                return this;
            }

            public override ModulationPtr ModulationPtr()
            {
                var ptr = Base.AUTDModulationStatic();
                if (_amp != null)
                    ptr = Base.AUTDModulationStaticWithAmp(ptr, _amp.Value);
                return ptr;
            }
        }

        public sealed class Sine : ModulationBase
        {
            private readonly int _freq;
            private float_t? _amp;
            private float_t? _offset;
            private uint? _freq_div;

            public Sine(int freq)
            {
                _freq = freq;
                _amp = null;
                _offset = null;
                _freq_div = null;
            }

            public Sine WithAmp(float_t amp)
            {
                _amp = amp;
                return this;
            }

            public Sine WithOffset(float_t offset)
            {
                _offset = offset;
                return this;
            }

            public Sine with_sampling_frequency_division(uint div)
            {
                _freq_div = div;
                return this;
            }

            public Sine with_sampling_frequency(float_t freq)
            {
                return with_sampling_frequency_division((uint)((float_t)Def.FpgaSubClkFreq / freq));
            }

            public override ModulationPtr ModulationPtr()
            {
                var ptr = Base.AUTDModulationSine((uint)_freq);
                if (_amp != null)
                    ptr = Base.AUTDModulationSineWithAmp(ptr, _amp.Value);
                if (_offset != null)
                    ptr = Base.AUTDModulationSineWithOffset(ptr, _offset.Value);
                if (_freq_div != null)
                    ptr = Base.AUTDModulationSineWithSamplingFrequencyDivision(ptr, _freq_div.Value);
                return ptr;
            }
        }

        public sealed class SineLegacy : ModulationBase
        {
            private readonly float_t _freq;
            private float_t? _amp;
            private float_t? _offset;
            private uint? _freq_div;

            public SineLegacy(float_t freq)
            {
                _freq = freq;
                _amp = null;
                _offset = null;
                _freq_div = null;
            }

            public SineLegacy WithAmp(float_t amp)
            {
                _amp = amp;
                return this;
            }

            public SineLegacy WithOffset(float_t offset)
            {
                _offset = offset;
                return this;
            }

            public SineLegacy with_sampling_frequency_division(uint div)
            {
                _freq_div = div;
                return this;
            }

            public SineLegacy with_sampling_frequency(float_t freq)
            {
                return with_sampling_frequency_division((uint)((float_t)Def.FpgaSubClkFreq / freq));
            }

            public override ModulationPtr ModulationPtr()
            {
                var ptr = Base.AUTDModulationSineLegacy(_freq);
                if (_amp != null)
                    ptr = Base.AUTDModulationSineLegacyWithAmp(ptr, _amp.Value);
                if (_offset != null)
                    ptr = Base.AUTDModulationSineLegacyWithOffset(ptr, _offset.Value);
                if (_freq_div != null)
                    ptr = Base.AUTDModulationSineLegacyWithSamplingFrequencyDivision(ptr, _freq_div.Value);
                return ptr;
            }
        }

        public sealed class Square : ModulationBase
        {
            private readonly int _freq;
            private float_t? _low;
            private float_t? _high;
            private float_t? _duty;
            private uint? _freq_div;

            public Square(int freq)
            {
                _freq = freq;
                _low = null;
                _high = null;
                _duty = null;
                _freq_div = null;
            }

            public Square WithLow(float_t low)
            {
                _low = low;
                return this;
            }

            public Square WithHigh(float_t high)
            {
                _high = high;
                return this;
            }

            public Square WithDuty(float_t duty)
            {
                _duty = duty;
                return this;
            }

            public Square with_sampling_frequency_division(uint div)
            {
                _freq_div = div;
                return this;
            }

            public Square with_sampling_frequency(float_t freq)
            {
                return with_sampling_frequency_division((uint)((float_t)Def.FpgaSubClkFreq / freq));
            }

            public override ModulationPtr ModulationPtr()
            {
                var ptr = Base.AUTDModulationSquare((uint)_freq);
                if (_low != null)
                    ptr = Base.AUTDModulationSquareWithLow(ptr, _low.Value);
                if (_high != null)
                    ptr = Base.AUTDModulationSquareWithHigh(ptr, _high.Value);
                if (_duty != null)
                    ptr = Base.AUTDModulationSquareWithDuty(ptr, _duty.Value);
                if (_freq_div != null)
                    ptr = Base.AUTDModulationSquareWithSamplingFrequencyDivision(ptr, _freq_div.Value);
                return ptr;
            }
        }

        public abstract class Modulation : ModulationBase
        {
            private readonly uint _freqDiv;

            protected Modulation(uint freqDiv)
            {
                _freqDiv = freqDiv;
            }

            protected Modulation(float_t samplingFreq)
            {
                _freqDiv = (uint)(Def.FpgaSubClkFreq / samplingFreq);
            }

            sealed public override ModulationPtr ModulationPtr()
            {
                var data = Calc();
                return Base.AUTDModulationCustom(_freqDiv, data, (ulong)data.Length);
            }

            public abstract float_t[] Calc();
        }


        public class Cache : ModulationBase, IEnumerable<float_t>
        {
            private readonly uint _freqDiv;
            private float_t[] _buffer;

            public Cache(ModulationBase m)
            {
                _freqDiv = m.SamplingFrequencyDivision;

                var err = new byte[256];
                var size = Base.AUTDModulationSize(m.ModulationPtr(), err);
                if (size == Def.Autd3Err) throw new AUTDException(err);
                _buffer = new float_t[size];
                if (Base.AUTDModulationCalc(m.ModulationPtr(), _buffer, err) == Def.Autd3Err)
                    throw new AUTDException(err);
            }

            sealed public override ModulationPtr ModulationPtr()
            {
                return Base.AUTDModulationCustom(_freqDiv, _buffer, (ulong)_buffer.Length);
            }

            public float_t this[int index]
            {
                get => _buffer[index];
                set => _buffer[index] = value;
            }

            public float_t[] Buffer => _buffer;

            public IEnumerator<float_t> GetEnumerator() => _buffer.AsEnumerable().GetEnumerator();

            System.Collections.IEnumerator System.Collections.IEnumerable.GetEnumerator() => GetEnumerator();
        }

        public sealed class RadiationPressure : ModulationBase
        {
            private readonly uint _freqDiv;
            private readonly float_t[] _buffer;

            public RadiationPressure(ModulationBase m)
            {
                _freqDiv = m.SamplingFrequencyDivision;

                var err = new byte[256];
                var size = Base.AUTDModulationSize(m.ModulationPtr(), err);
                if (size == Def.Autd3Err) throw new AUTDException(err);
                var buf = new float_t[size];
                if (Base.AUTDModulationCalc(m.ModulationPtr(), buf, err) == Def.Autd3Err)
                    throw new AUTDException(err);
                _buffer = buf.Select(v => Math.Sqrt(v)).ToArray();
            }

            sealed public override ModulationPtr ModulationPtr()
            {
                return Base.AUTDModulationCustom(_freqDiv, _buffer, (ulong)_buffer.Length);
            }
        }
    }

    namespace STM
    {
        public abstract class STM : IBody
        {
            private readonly float_t? _freq;
            private readonly float_t? _samplFreq;
            private readonly uint? _samplFreqDiv;
            protected int StartIdxV;
            protected int FinishIdxV;

            protected STM(float_t? freq, float_t? samplFreq, uint? sampleFreqDiv)
            {
                _freq = freq;
                _samplFreq = samplFreq;
                _samplFreqDiv = sampleFreqDiv;
                StartIdxV = -1;
                FinishIdxV = -1;
            }

            public DatagramBodyPtr Ptr(Geometry geometry) => STMPtr(geometry);

            public abstract DatagramBodyPtr STMPtr(Geometry geometry);

            public ushort? StartIdx => StartIdxV == -1 ? null : (ushort?)StartIdxV;

            public ushort? FinishIdx => FinishIdxV == -1 ? null : (ushort?)FinishIdxV;

            protected STMPropsPtr Props()
            {
                var ptr = new STMPropsPtr();
                if (_freq != null)
                    ptr = Base.AUTDSTMProps(_freq.Value);
                if (_samplFreq != null)
                    ptr = Base.AUTDSTMPropsWithSamplingFreq(_samplFreq.Value);
                if (_samplFreqDiv != null)
                    ptr = Base.AUTDSTMPropsWithSamplingFreqDiv(_samplFreqDiv.Value);
                ptr = Base.AUTDSTMPropsWithStartIdx(ptr, StartIdxV);
                ptr = Base.AUTDSTMPropsWithFinishIdx(ptr, FinishIdxV);
                return ptr;
            }

            protected float_t FreqFromSize(int size) => Base.AUTDSTMPropsFrequency(Props(), (ulong)size);
            protected float_t SamplFreqFromSize(int size) => Base.AUTDSTMPropsSamplingFrequency(Props(), (ulong)size);
            protected uint SamplFreqDivFromSize(int size) => Base.AUTDSTMPropsSamplingFrequencyDivision(Props(), (ulong)size);
        }

        public sealed class FocusSTM : STM
        {
            private readonly List<float_t> _points;
            private readonly List<byte> _shifts;

            private FocusSTM(float_t? freq, float_t? samplFreq, uint? sampleFreqDiv) : base(freq, samplFreq, sampleFreqDiv)
            {
                _points = new List<float_t>();
                _shifts = new List<byte>();
            }

            public FocusSTM(float_t freq) : this(freq, null, null)
            {
            }

            public static FocusSTM WithSamplingFrequency(float_t freq)
            {
                return new FocusSTM(null, freq, null);
            }

            public static FocusSTM WithSamplingFrequencyDivision(uint freqDiv)
            {
                return new FocusSTM(null, null, freqDiv);
            }

            public FocusSTM AddFocus(Vector3 point, byte shift = 0)
            {
                _points.Add(point.x);
                _points.Add(point.y);
                _points.Add(point.z);
                _shifts.Add(shift);
                return this;
            }

            public FocusSTM AddFociFromIter(IEnumerable<Vector3> iter)
            {
                return iter.Aggregate(this, (stm, point) => stm.AddFocus(point));
            }

            public FocusSTM AddFociFromIter(IEnumerable<(Vector3, byte)> iter)
            {
                return iter.Aggregate(this, (stm, point) => stm.AddFocus(point.Item1, point.Item2));
            }

            public FocusSTM WithStartIdx(ushort? startIdx)
            {
                StartIdxV = startIdx ?? -1;
                return this;
            }

            public FocusSTM WithFinishIdx(ushort? finishIdx)
            {
                FinishIdxV = finishIdx ?? -1;
                return this;
            }

            public float_t Frequency => FreqFromSize(_shifts.Count);
            public float_t SamplingFrequency => SamplFreqFromSize(_shifts.Count);
            public uint SamplingFrequencyDivision => SamplFreqDivFromSize(_shifts.Count);

            public override DatagramBodyPtr STMPtr(Geometry geometry)
            {
                return Base.AUTDFocusSTM(Props(), _points.ToArray(), _shifts.ToArray(), (ulong)_shifts.Count);
            }
        }


        public sealed class GainSTM : STM
        {
            private readonly List<GainBase> _gains;
            private GainSTMMode? _mode;

            private GainSTM(float_t? freq, float_t? samplFreq, uint? sampleFreqDiv) : base(freq, samplFreq, sampleFreqDiv)
            {
                _gains = new List<GainBase>();
                _mode = GainSTMMode.PhaseDutyFull;
            }

            public GainSTM(float_t freq) : this(freq, null, null)
            {
            }

            public static GainSTM WithSamplingFrequency(float_t freq)
            {
                return new GainSTM(null, freq, null);
            }

            public static GainSTM WithSamplingFrequencyDivision(uint freqDiv)
            {
                return new GainSTM(null, null, freqDiv);
            }

            public GainSTM AddGain(GainBase gain)
            {
                _gains.Add(gain);
                return this;
            }

            public GainSTM AddGainsFromIter(IEnumerable<GainBase> iter)
            {
                return iter.Aggregate(this, (stm, gain) => stm.AddGain(gain));
            }

            public GainSTM WithStartIdx(ushort? startIdx)
            {
                StartIdxV = startIdx ?? -1;
                return this;
            }

            public GainSTM WithFinishIdx(ushort? finishIdx)
            {
                FinishIdxV = finishIdx ?? -1;
                return this;
            }

            public GainSTM WithMode(GainSTMMode mode)
            {
                _mode = mode;
                return this;
            }

            public float_t Frequency => FreqFromSize(_gains.Count);
            public float_t SamplingFrequency => SamplFreqFromSize(_gains.Count);
            public uint SamplingFrequencyDivision => SamplFreqDivFromSize(_gains.Count);

            public override DatagramBodyPtr STMPtr(Geometry geometry)
            {
                return _gains.Aggregate(_mode.HasValue ? Base.AUTDGainSTMWithMode(Props(), _mode.Value) : Base.AUTDGainSTM(Props()), (current, gain) => Base.AUTDGainSTMAddGain(current, gain.GainPtr(geometry)));
            }
        }
    }
}
