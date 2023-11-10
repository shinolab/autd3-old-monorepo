/*
 * File: Visualizer.cs
 * Project: Link
 * Created Date: 13/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 07/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */



#if UNITY_2018_3_OR_NEWER
#define USE_SINGLE
#endif

using System;
using System.Collections.Generic;
using System.Linq;
using AUTD3Sharp.Internal;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

#if UNITY_2018_3_OR_NEWER
using UnityEngine;
using Vector3 = UnityEngine.Vector3;
#else
using Vector3 = AUTD3Sharp.Utils.Vector3d;
#endif

#if USE_SINGLE
using float_t = System.Single;
#else
using float_t = System.Double;
#endif

namespace AUTD3Sharp.Link
{
    public interface IBackend
    {
        internal Backend Backend();
    }

    public sealed class PlottersBackend : IBackend
    {
        Backend IBackend.Backend() => Backend.Plotters;
    }

    public sealed class PythonBackend : IBackend
    {
        Backend IBackend.Backend() => Backend.Python;
    }

    public sealed class NullBackend : IBackend
    {
        Backend IBackend.Backend() => Backend.Null;
    }

    public interface IDirectivity
    {
        internal Directivity Directivity();
    }

    public sealed class Sphere : IDirectivity
    {
        Directivity IDirectivity.Directivity() => Directivity.Sphere;
    }

    public sealed class T4010A1 : IDirectivity
    {
        Directivity IDirectivity.Directivity() => Directivity.T4010A1;
    }

    public struct PlotRange
    {
        public float_t XStart { get; set; }
        public float_t XEnd { get; set; }
        public float_t YStart { get; set; }
        public float_t YEnd { get; set; }
        public float_t ZStart { get; set; }
        public float_t ZEnd { get; set; }
        public float_t Resolution { get; set; }
    }

    public interface IPlotConfig
    {
        internal ConfigPtr Ptr();
        internal Backend Backend();
    }

    public sealed class PlotConfig : IPlotConfig
    {
        public (uint, uint)? Figsize { get; set; }
        public float_t? CbarSize { get; set; }
        public uint? FontSize { get; set; }
        public uint? LabelAreaSize { get; set; }
        public uint? Margin { get; set; }
        public float_t? TicksStep { get; set; }
        public CMap? Cmap { get; set; }
        public string? Fname { get; set; }

        ConfigPtr IPlotConfig.Ptr()
        {

            var ptr = NativeMethodsLinkVisualizer.AUTDLinkVisualizerPlotConfigDefault();
            if (Figsize.HasValue)
                ptr = NativeMethodsLinkVisualizer.AUTDLinkVisualizerPlotConfigWithFigSize(ptr, Figsize.Value.Item1,
                    Figsize.Value.Item2);
            if (CbarSize.HasValue)
                ptr = NativeMethodsLinkVisualizer.AUTDLinkVisualizerPlotConfigWithCBarSize(ptr, CbarSize.Value);
            if (FontSize.HasValue)
                ptr = NativeMethodsLinkVisualizer.AUTDLinkVisualizerPlotConfigWithFontSize(ptr, FontSize.Value);
            if (LabelAreaSize.HasValue)
                ptr = NativeMethodsLinkVisualizer.AUTDLinkVisualizerPlotConfigWithLabelAreaSize(ptr, LabelAreaSize.Value);
            if (Margin.HasValue)
                ptr = NativeMethodsLinkVisualizer.AUTDLinkVisualizerPlotConfigWithMargin(ptr, Margin.Value);
            if (TicksStep.HasValue)
                ptr = NativeMethodsLinkVisualizer.AUTDLinkVisualizerPlotConfigWithTicksStep(ptr, TicksStep.Value);
            if (Cmap.HasValue)
                ptr = NativeMethodsLinkVisualizer.AUTDLinkVisualizerPlotConfigWithCMap(ptr, Cmap.Value);
            if (Fname == null) return new ConfigPtr { Item1 = ptr.Item1 };

            var fnameBytes = System.Text.Encoding.UTF8.GetBytes(Fname);
            unsafe
            {
                fixed (byte* fp = fnameBytes)
                {
                    var res = NativeMethodsLinkVisualizer.AUTDLinkVisualizerPlotConfigWithFName(ptr, fp);
                    if (res.result.Item1 == IntPtr.Zero)
                    {
                        var err = new byte[res.err_len];
                        fixed (byte* ep = err)
                            NativeMethodsDef.AUTDGetErr(res.err, ep);
                        throw new AUTDException(err);
                    }
                    ptr = res.result;
                }
            }
            return new ConfigPtr { Item1 = ptr.Item1 };
        }


        Backend IPlotConfig.Backend()
        {
            return Backend.Plotters;
        }
    }

    public sealed class PyPlotConfig : IPlotConfig
    {
        public (int, int)? Figsize { get; set; }
        public int? DPI { get; set; }
        public string? CbarPosition { get; set; }
        public string? CbarSize { get; set; }
        public string? CbarPad { get; set; }
        public int? FontSize { get; set; }
        public float_t? TicksStep { get; set; }
        public string? Cmap { get; set; }
        public bool? Show { get; set; }
        public string? Fname { get; set; }

        ConfigPtr IPlotConfig.Ptr()
        {
            var ptr = NativeMethodsLinkVisualizer.AUTDLinkVisualizerPyPlotConfigDefault();
            if (Figsize.HasValue)
                ptr = NativeMethodsLinkVisualizer.AUTDLinkVisualizerPyPlotConfigWithFigSize(ptr, Figsize.Value.Item1,
                    Figsize.Value.Item2);
            if (DPI.HasValue)
                ptr = NativeMethodsLinkVisualizer.AUTDLinkVisualizerPyPlotConfigWithDPI(ptr, DPI.Value);
            if (CbarPosition != null)
            {
                var cbarPositionBytes = System.Text.Encoding.UTF8.GetBytes(CbarPosition);
                unsafe
                {
                    fixed (byte* fp = cbarPositionBytes)
                    {
                        var res = NativeMethodsLinkVisualizer.AUTDLinkVisualizerPyPlotConfigWithCBarPosition(ptr, fp);
                        if (res.result.Item1 == IntPtr.Zero)
                        {
                            var err = new byte[res.err_len];
                            fixed (byte* ep = err)
                                NativeMethodsDef.AUTDGetErr(res.err, ep);
                            throw new AUTDException(err);
                        }
                        ptr = res.result;
                    }
                }
            }

            if (CbarSize != null)
            {
                var cbarPSizeBytes = System.Text.Encoding.UTF8.GetBytes(CbarSize);
                unsafe
                {
                    fixed (byte* fp = cbarPSizeBytes)
                    {
                        var res = NativeMethodsLinkVisualizer.AUTDLinkVisualizerPyPlotConfigWithCBarSize(ptr, fp);
                        if (res.result.Item1 == IntPtr.Zero)
                        {
                            var err = new byte[res.err_len];
                            fixed (byte* ep = err)
                                NativeMethodsDef.AUTDGetErr(res.err, ep);
                            throw new AUTDException(err);
                        }
                        ptr = res.result;
                    }
                }
            }
            if (CbarPad != null)
            {
                var cbarPadBytes = System.Text.Encoding.UTF8.GetBytes(CbarPad);
                unsafe
                {
                    fixed (byte* fp = cbarPadBytes)
                    {
                        var res = NativeMethodsLinkVisualizer.AUTDLinkVisualizerPyPlotConfigWithCBarPad(ptr, fp);
                        if (res.result.Item1 == IntPtr.Zero)
                        {
                            var err = new byte[res.err_len];
                            fixed (byte* ep = err)
                                NativeMethodsDef.AUTDGetErr(res.err, ep);
                            throw new AUTDException(err);
                        }
                        ptr = res.result;
                    }
                }
            }
            if (FontSize.HasValue)
                ptr = NativeMethodsLinkVisualizer.AUTDLinkVisualizerPyPlotConfigWithFontSize(ptr, FontSize.Value);
            if (TicksStep.HasValue)
                ptr = NativeMethodsLinkVisualizer.AUTDLinkVisualizerPyPlotConfigWithTicksStep(ptr, TicksStep.Value);
            if (Cmap != null)
            {
                var cmapBytes = System.Text.Encoding.UTF8.GetBytes(Cmap);
                unsafe
                {
                    fixed (byte* fp = cmapBytes)
                    {
                        var res = NativeMethodsLinkVisualizer.AUTDLinkVisualizerPyPlotConfigWithCMap(ptr, fp);
                        if (res.result.Item1 == IntPtr.Zero)
                        {
                            var err = new byte[res.err_len];
                            fixed (byte* ep = err)
                                NativeMethodsDef.AUTDGetErr(res.err, ep);
                            throw new AUTDException(err);
                        }
                        ptr = res.result;
                    }
                }
            }
            if (Show.HasValue)
                ptr = NativeMethodsLinkVisualizer.AUTDLinkVisualizerPyPlotConfigWithShow(ptr, Show.Value);
            if (Fname == null) return new ConfigPtr { Item1 = ptr.Item1 };

            var fnameBytes = System.Text.Encoding.UTF8.GetBytes(Fname);
            unsafe
            {
                fixed (byte* fp = fnameBytes)
                {
                    var res = NativeMethodsLinkVisualizer.AUTDLinkVisualizerPyPlotConfigWithFName(ptr, fp);
                    if (res.result.Item1 == IntPtr.Zero)
                    {
                        var err = new byte[res.err_len];
                        fixed (byte* ep = err)
                            NativeMethodsDef.AUTDGetErr(res.err, ep);
                        throw new AUTDException(err);
                    }
                    ptr = res.result;
                }
            }
            return new ConfigPtr { Item1 = ptr.Item1 };
        }

        Backend IPlotConfig.Backend()
        {
            return Backend.Python;
        }
    }

    public sealed class NullPlotConfig : IPlotConfig
    {
        ConfigPtr IPlotConfig.Ptr()
        {
            return new ConfigPtr { Item1 = NativeMethodsLinkVisualizer.AUTDLinkVisualizerNullPlotConfigDefault().Item1 };
        }

        Backend IPlotConfig.Backend()
        {
            return Backend.Null;
        }
    }

    /// <summary>
    /// Link for visualizing
    /// </summary>
    public sealed class Visualizer : ILink<Visualizer>
    {
        internal struct Props
        {
            internal Backend Backend;
            internal Directivity Directivity;
        }


        public sealed class VisualizerBuilder : ILinkBuilder
        {
            private Props _props;
            private int? _gpuIdx;

            internal VisualizerBuilder(Backend backend = Backend.Plotters, Directivity directivity = Directivity.Sphere)
            {
                _props = new Props
                {
                    Backend = backend,
                    Directivity = directivity
                };
            }

            LinkBuilderPtr ILinkBuilder.Ptr()
            {
                return _props.Backend switch
                {
                    Backend.Plotters => _props.Directivity switch
                    {
                        Directivity.Sphere => NativeMethodsLinkVisualizer.AUTDLinkVisualizerSpherePlotters(
                            _gpuIdx.HasValue, _gpuIdx ?? 0),
                        Directivity.T4010A1 => NativeMethodsLinkVisualizer.AUTDLinkVisualizerT4010A1Plotters(
                            _gpuIdx.HasValue, _gpuIdx ?? 0),
                        _ => throw new ArgumentOutOfRangeException()
                    },
                    Backend.Python => _props.Directivity switch
                    {
                        Directivity.Sphere => NativeMethodsLinkVisualizer.AUTDLinkVisualizerSpherePython(
                            _gpuIdx.HasValue, _gpuIdx ?? 0),
                        Directivity.T4010A1 => NativeMethodsLinkVisualizer.AUTDLinkVisualizerT4010A1Python(
                            _gpuIdx.HasValue, _gpuIdx ?? 0),
                        _ => throw new ArgumentOutOfRangeException()
                    },
                    Backend.Null => _props.Directivity switch
                    {
                        Directivity.Sphere => NativeMethodsLinkVisualizer.AUTDLinkVisualizerSphereNull(
                            _gpuIdx.HasValue, _gpuIdx ?? 0),
                        Directivity.T4010A1 => NativeMethodsLinkVisualizer.AUTDLinkVisualizerT4010A1Null(
                            _gpuIdx.HasValue, _gpuIdx ?? 0),
                        _ => throw new ArgumentOutOfRangeException()
                    },
                    _ => throw new ArgumentOutOfRangeException()
                };
            }

            object ILinkBuilder.Props()
            {
                return _props;
            }

            public VisualizerBuilder WithGpu(int gpuIdx)
            {
                _gpuIdx = gpuIdx;
                return this;
            }

            public VisualizerBuilder WithBackend<TB>()
            where TB : IBackend, new()
            {
                _props.Backend = new TB().Backend();
                return this;
            }

            public VisualizerBuilder WithDirectivity<TD>()
                where TD : IDirectivity, new()
            {
                _props.Directivity = new TD().Directivity();
                return this;
            }
        }

        public static VisualizerBuilder Builder()
        {
            return new VisualizerBuilder();
        }

        public static VisualizerBuilder Plotters()
        {
            return new VisualizerBuilder(Backend.Plotters);
        }

        public static VisualizerBuilder Python()
        {
            return new VisualizerBuilder(Backend.Python);
        }

        public static VisualizerBuilder Null()
        {
            return new VisualizerBuilder(Backend.Null);
        }

        private LinkPtr _ptr = new LinkPtr { Item1 = IntPtr.Zero };
        private Backend _backend;
        private Directivity _directivity;

        public float_t[] PhasesOf(int idx)
        {
            unsafe
            {
                var size = NativeMethodsLinkVisualizer.AUTDLinkVisualizerPhasesOf(_ptr, _backend, _directivity,
                    (uint)(idx), null);
                var buf = new float_t[size];
                fixed (float_t* bp = buf)
                    NativeMethodsLinkVisualizer.AUTDLinkVisualizerPhasesOf(_ptr, _backend, _directivity,
                        (uint)(idx), bp);
                return buf;
            }
        }

        public float_t[] Phases()
        {
            return PhasesOf(0);
        }

        public float_t[] DutiesOf(int idx)
        {
            unsafe
            {
                var size = NativeMethodsLinkVisualizer.AUTDLinkVisualizerDutiesOf(_ptr, _backend, _directivity,
                    (uint)(idx), null);
                var buf = new float_t[size];
                fixed (float_t* bp = buf)
                    NativeMethodsLinkVisualizer.AUTDLinkVisualizerDutiesOf(_ptr, _backend, _directivity,
                    (uint)(idx), bp);
                return buf;
            }
        }

        public float_t[] Duties()
        {
            return DutiesOf(0);
        }

        public float_t[] ModulationRaw()
        {
            unsafe
            {
                var size = NativeMethodsLinkVisualizer.AUTDLinkVisualizerModulationRaw(_ptr, _backend, _directivity,
                    null);
                var buf = new float_t[size];
                fixed (float_t* bp = buf)
                    NativeMethodsLinkVisualizer.AUTDLinkVisualizerModulationRaw(_ptr, _backend, _directivity, bp);
                return buf;
            }
        }

        public float_t[] Modulation()
        {
            unsafe
            {
                var size = NativeMethodsLinkVisualizer.AUTDLinkVisualizerModulation(_ptr, _backend, _directivity,
                    null);
                var buf = new float_t[size];
                fixed (float_t* bp = buf)
                    NativeMethodsLinkVisualizer.AUTDLinkVisualizerModulation(_ptr, _backend, _directivity, bp);
                return buf;
            }
        }

        public System.Numerics.Complex[] CalcFieldOf(IEnumerable<Vector3> pointsIter, Geometry geometry, int idx)
        {
            var points = pointsIter as Vector3[] ?? pointsIter.ToArray();
            var pointsLen = points.Length;
            var pointsPtr = points.SelectMany(v => new[] { v.x, v.y, v.z }).ToArray();
            var buf = new float_t[pointsLen * 2];
            unsafe
            {
                fixed (float_t* pp = pointsPtr)
                fixed (float_t* bp = buf)
                {
                    var res = NativeMethodsLinkVisualizer.AUTDLinkVisualizerCalcFieldOf(_ptr, _backend, _directivity,
                        pp, (uint)pointsLen, geometry.Ptr, (uint)idx, bp);
                    if (res.result != NativeMethodsDef.AUTD3_ERR)
                        return Enumerable.Range(0, pointsLen)
                            .Select(i => new System.Numerics.Complex(buf[2 * i], buf[2 * i + 1])).ToArray();
                    var err = new byte[res.errLen];
                    fixed (byte* ep = err)
                        NativeMethodsDef.AUTDGetErr(res.err, ep);
                    throw new AUTDException(err);
                }
            }
        }

        public System.Numerics.Complex[] CalcField(IEnumerable<Vector3> pointsIter, Geometry geometry)
        {
            return CalcFieldOf(pointsIter, geometry, 0);
        }

        public void PlotFieldOf(IPlotConfig config, PlotRange range, Geometry geometry, int idx)
        {
            if (config.Backend() != _backend) throw new AUTDException("Invalid plot config type.");
            var ret = NativeMethodsLinkVisualizer.AUTDLinkVisualizerPlotFieldOf(_ptr, _backend, _directivity,
                config.Ptr(),
                NativeMethodsLinkVisualizer.AUTDLinkVisualizerPlotRange(range.XStart, range.XEnd, range.YStart,
                    range.YEnd,
                    range.ZStart, range.ZEnd, range.Resolution), geometry.Ptr, (uint)idx);
            if (ret.result != NativeMethodsDef.AUTD3_ERR) return;
            var err = new byte[ret.errLen];
            unsafe
            {
                fixed (byte* ep = err)
                    NativeMethodsDef.AUTDGetErr(ret.err, ep);
                throw new AUTDException(err);
            }
        }

        public void PlotField(IPlotConfig config, PlotRange range, Geometry geometry)
        {
            PlotFieldOf(config, range, geometry, 0);
        }

        public void PlotPhaseOf(IPlotConfig config, Geometry geometry, int idx)
        {
            if (config.Backend() != _backend) throw new AUTDException("Invalid plot config type.");
            var ret = NativeMethodsLinkVisualizer.AUTDLinkVisualizerPlotPhaseOf(_ptr, _backend, _directivity,
                config.Ptr(),
                geometry.Ptr, (uint)idx);
            if (ret.result != NativeMethodsDef.AUTD3_ERR) return;
            var err = new byte[ret.errLen];
            unsafe
            {
                fixed (byte* ep = err)
                    NativeMethodsDef.AUTDGetErr(ret.err, ep);
                throw new AUTDException(err);
            }
        }

        public void PlotPhase(IPlotConfig config, Geometry geometry)
        {
            PlotPhaseOf(config, geometry, 0);
        }

        public void PlotModulationRaw(IPlotConfig config)
        {
            if (config.Backend() != _backend) throw new AUTDException("Invalid plot config type.");
            var ret = NativeMethodsLinkVisualizer.AUTDLinkVisualizerPlotModulationRaw(_ptr, _backend, _directivity,
                config.Ptr());
            if (ret.result != NativeMethodsDef.AUTD3_ERR) return;
            var err = new byte[ret.errLen];
            unsafe
            {
                fixed (byte* ep = err)
                    NativeMethodsDef.AUTDGetErr(ret.err, ep);
                throw new AUTDException(err);
            }
        }

        public void PlotModulation(IPlotConfig config)
        {
            if (config.Backend() != _backend) throw new AUTDException("Invalid plot config type.");
            var ret = NativeMethodsLinkVisualizer.AUTDLinkVisualizerPlotModulation(_ptr, _backend, _directivity, config.Ptr());
            if (ret.result != NativeMethodsDef.AUTD3_ERR) return;
            var err = new byte[ret.errLen];
            unsafe
            {
                fixed (byte* ep = err)
                    NativeMethodsDef.AUTDGetErr(ret.err, ep);
                throw new AUTDException(err);
            }
        }

        Visualizer ILink<Visualizer>.Create(LinkPtr ptr, object? props)
        {
            var p = (Props)props!;
            return new Visualizer
            {
                _ptr = ptr,
                _backend = p.Backend,
                _directivity = p.Directivity
            };
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
