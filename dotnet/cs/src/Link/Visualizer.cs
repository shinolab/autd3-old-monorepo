/*
 * File: Visualizer.cs
 * Project: Link
 * Created Date: 13/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 13/10/2023
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
        Backend Backend();
    }

    public sealed class PlottersBackend : IBackend
    {
        public Backend Backend() => AUTD3Sharp.Backend.Plotters;
    }

    public sealed class PythonBackend : IBackend
    {
        public Backend Backend() => AUTD3Sharp.Backend.Python;
    }

    public sealed class NullBackend : IBackend
    {
        public Backend Backend() => AUTD3Sharp.Backend.Null;
    }

    public interface IDirectivity
    {
        Directivity Directivity();
    }

    public sealed class Sphere : IDirectivity
    {
        public Directivity Directivity() => AUTD3Sharp.Directivity.Sphere;
    }

    public sealed class T4010A1 : IDirectivity
    {
        public Directivity Directivity() => AUTD3Sharp.Directivity.T4010A1;
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
        ConfigPtr Ptr();
        Backend Backend();
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

        public ConfigPtr Ptr()
        {
            var ptr = NativeMethods.LinkVisualizer.AUTDLinkVisualizerPlotConfigDefault();
            if (Figsize.HasValue)
                ptr = NativeMethods.LinkVisualizer.AUTDLinkVisualizerPlotConfigWithFigSize(ptr, Figsize.Value.Item1, Figsize.Value.Item2);
            if (CbarSize.HasValue)
                ptr = NativeMethods.LinkVisualizer.AUTDLinkVisualizerPlotConfigWithCBarSize(ptr, CbarSize.Value);
            if (FontSize.HasValue)
                ptr = NativeMethods.LinkVisualizer.AUTDLinkVisualizerPlotConfigWithFontSize(ptr, FontSize.Value);
            if (LabelAreaSize.HasValue)
                ptr = NativeMethods.LinkVisualizer.AUTDLinkVisualizerPlotConfigWithLabelAreaSize(ptr, LabelAreaSize.Value);
            if (Margin.HasValue)
                ptr = NativeMethods.LinkVisualizer.AUTDLinkVisualizerPlotConfigWithMargin(ptr, Margin.Value);
            if (TicksStep.HasValue)
                ptr = NativeMethods.LinkVisualizer.AUTDLinkVisualizerPlotConfigWithTicksStep(ptr, TicksStep.Value);
            if (Cmap.HasValue)
                ptr = NativeMethods.LinkVisualizer.AUTDLinkVisualizerPlotConfigWithCMap(ptr, Cmap.Value);
            if (Fname == null) return new ConfigPtr { _0 = ptr._0 };
            var err = new byte[256];
            ptr = NativeMethods.LinkVisualizer.AUTDLinkVisualizerPlotConfigWithFName(ptr, Fname, err);
            if (ptr._0 == IntPtr.Zero)
                throw new AUTDException(err);
            return new ConfigPtr { _0 = ptr._0 };
        }

        public Backend Backend()
        {
            return AUTD3Sharp.Backend.Plotters;
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

        public ConfigPtr Ptr()
        {
            var err = new byte[256];
            var ptr = NativeMethods.LinkVisualizer.AUTDLinkVisualizerPyPlotConfigDefault();
            if (Figsize.HasValue)
                ptr = NativeMethods.LinkVisualizer.AUTDLinkVisualizerPyPlotConfigWithFigSize(ptr, Figsize.Value.Item1, Figsize.Value.Item2);
            if (DPI.HasValue)
                ptr = NativeMethods.LinkVisualizer.AUTDLinkVisualizerPyPlotConfigWithDPI(ptr, DPI.Value);
            if (CbarPosition != null)
            {
                ptr = NativeMethods.LinkVisualizer.AUTDLinkVisualizerPyPlotConfigWithCBarPosition(ptr, CbarPosition, err);
                if (ptr._0 == IntPtr.Zero)
                    throw new AUTDException(err);
            }
            if (CbarSize != null)
            {
                ptr = NativeMethods.LinkVisualizer.AUTDLinkVisualizerPyPlotConfigWithCBarSize(ptr, CbarSize, err);
                if (ptr._0 == IntPtr.Zero)
                    throw new AUTDException(err);
            }
            if (CbarPad != null)
            {
                ptr = NativeMethods.LinkVisualizer.AUTDLinkVisualizerPyPlotConfigWithCBarPad(ptr, CbarPad, err);
                if (ptr._0 == IntPtr.Zero)
                    throw new AUTDException(err);
            }
            if (FontSize.HasValue)
                ptr = NativeMethods.LinkVisualizer.AUTDLinkVisualizerPyPlotConfigWithFontSize(ptr, FontSize.Value);
            if (TicksStep.HasValue)
                ptr = NativeMethods.LinkVisualizer.AUTDLinkVisualizerPyPlotConfigWithTicksStep(ptr, TicksStep.Value);
            if (Cmap != null)
            {
                ptr = NativeMethods.LinkVisualizer.AUTDLinkVisualizerPyPlotConfigWithCMap(ptr, Cmap, err);
                if (ptr._0 == IntPtr.Zero)
                    throw new AUTDException(err);
            }
            if (Show.HasValue)
                ptr = NativeMethods.LinkVisualizer.AUTDLinkVisualizerPyPlotConfigWithShow(ptr, Show.Value);
            if (Fname == null) return new ConfigPtr { _0 = ptr._0 };
            ptr = NativeMethods.LinkVisualizer.AUTDLinkVisualizerPyPlotConfigWithFName(ptr, Fname, err);
            if (ptr._0 == IntPtr.Zero)
                throw new AUTDException(err);
            return new ConfigPtr { _0 = ptr._0 };
        }

        public Backend Backend()
        {
            return AUTD3Sharp.Backend.Python;
        }
    }

    public sealed class NullPlotConfig : IPlotConfig
    {
        public ConfigPtr Ptr()
        {
            return new ConfigPtr { _0 = NativeMethods.LinkVisualizer.AUTDLinkVisualizerNullPlotConfigDefault()._0 };
        }

        public Backend Backend()
        {
            return AUTD3Sharp.Backend.Null;
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

            public LinkBuilderPtr Ptr()
            {
                return _props.Backend switch
                {
                    Backend.Plotters => _props.Directivity switch
                    {
                        Directivity.Sphere => NativeMethods.LinkVisualizer.AUTDLinkVisualizerSpherePlotters(
                            _gpuIdx.HasValue, _gpuIdx ?? 0),
                        Directivity.T4010A1 => NativeMethods.LinkVisualizer.AUTDLinkVisualizerT4010A1Plotters(
                            _gpuIdx.HasValue, _gpuIdx ?? 0),
                        _ => throw new ArgumentOutOfRangeException()
                    },
                    Backend.Python => _props.Directivity switch
                    {
                        Directivity.Sphere => NativeMethods.LinkVisualizer.AUTDLinkVisualizerSpherePython(
                            _gpuIdx.HasValue, _gpuIdx ?? 0),
                        Directivity.T4010A1 => NativeMethods.LinkVisualizer.AUTDLinkVisualizerT4010A1Python(
                            _gpuIdx.HasValue, _gpuIdx ?? 0),
                        _ => throw new ArgumentOutOfRangeException()
                    },
                    Backend.Null => _props.Directivity switch
                    {
                        Directivity.Sphere => NativeMethods.LinkVisualizer.AUTDLinkVisualizerSphereNull(
                            _gpuIdx.HasValue, _gpuIdx ?? 0),
                        Directivity.T4010A1 => NativeMethods.LinkVisualizer.AUTDLinkVisualizerT4010A1Null(
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

        private LinkPtr _ptr = new LinkPtr { _0 = IntPtr.Zero };
        private Backend _backend;
        private Directivity _directivity;

        public float_t[] PhasesOf(int idx)
        {
            var size = NativeMethods.LinkVisualizer.AUTDLinkVisualizerPhasesOf(_ptr, _backend, _directivity,
                (uint)(idx), null);
            var buf = new float_t[size];
            NativeMethods.LinkVisualizer.AUTDLinkVisualizerPhasesOf(_ptr, _backend, _directivity,
                               (uint)(idx), buf);
            return buf;
        }

        public float_t[] Phases()
        {
            return PhasesOf(0);
        }

        public float_t[] DutiesOf(int idx)
        {
            var size = NativeMethods.LinkVisualizer.AUTDLinkVisualizerDutiesOf(_ptr, _backend, _directivity,
                (uint)(idx), null);
            var buf = new float_t[size];
            NativeMethods.LinkVisualizer.AUTDLinkVisualizerDutiesOf(_ptr, _backend, _directivity,
                (uint)(idx), buf);
            return buf;
        }

        public float_t[] Duties()
        {
            return DutiesOf(0);
        }

        public float_t[] ModulationRaw()
        {
            var size = NativeMethods.LinkVisualizer.AUTDLinkVisualizerModulationRaw(_ptr, _backend, _directivity,
                               null);
            var buf = new float_t[size];
            NativeMethods.LinkVisualizer.AUTDLinkVisualizerModulationRaw(_ptr, _backend, _directivity, buf);
            return buf;
        }

        public float_t[] Modulation()
        {
            var size = NativeMethods.LinkVisualizer.AUTDLinkVisualizerModulation(_ptr, _backend, _directivity,
                                              null);
            var buf = new float_t[size];
            NativeMethods.LinkVisualizer.AUTDLinkVisualizerModulation(_ptr, _backend, _directivity, buf);
            return buf;
        }

        public System.Numerics.Complex[] CalcFieldOf(IEnumerable<Vector3> pointsIter, Geometry geometry, int idx)
        {
            var points = pointsIter as Vector3[] ?? pointsIter.ToArray();
            var pointsLen = points.Count();
            var pointsPtr = points.SelectMany(v => new[] { v.x, v.y, v.z }).ToArray();
            var buf = new float_t[pointsLen * 2];
            NativeMethods.LinkVisualizer.AUTDLinkVisualizerCalcFieldOf(_ptr, _backend, _directivity,
                               pointsPtr, (uint)pointsLen, geometry.Ptr, (uint)(idx), buf);
            return Enumerable.Range(0, pointsLen).Select(i => new System.Numerics.Complex(buf[2 * i], buf[2 * i + 1])).ToArray();
        }

        public System.Numerics.Complex[] CalcField(IEnumerable<Vector3> pointsIter, Geometry geometry)
        {
            return CalcFieldOf(pointsIter, geometry, 0);
        }

        public void PlotFieldOf(IPlotConfig config, PlotRange range, Geometry geometry, int idx)
        {
            if (config.Backend() != _backend) throw new AUTDException("Invalid plot config type.");
            var err = new byte[256];
            var ret = NativeMethods.LinkVisualizer.AUTDLinkVisualizerPlotFieldOf(_ptr, _backend, _directivity, config.Ptr(), NativeMethods.LinkVisualizer.AUTDLinkVisualizerPlotRange(range.XStart, range.XEnd, range.YStart, range.YEnd, range.ZStart, range.ZEnd, range.Resolution), geometry.Ptr, (uint)idx, err);
            if (ret == NativeMethods.Def.Autd3Err)
                throw new AUTDException(err);
        }

        public void PlotField(IPlotConfig config, PlotRange range, Geometry geometry)
        {
            PlotFieldOf(config, range, geometry, 0);
        }

        public void PlotPhaseOf(IPlotConfig config, Geometry geometry, int idx)
        {
            if (config.Backend() != _backend) throw new AUTDException("Invalid plot config type.");
            var err = new byte[256];
            var ret = NativeMethods.LinkVisualizer.AUTDLinkVisualizerPlotPhaseOf(_ptr, _backend, _directivity, config.Ptr(), geometry.Ptr, (uint)idx, err);
            if (ret == NativeMethods.Def.Autd3Err)
                throw new AUTDException(err);
        }

        public void PlotPhase(IPlotConfig config, Geometry geometry)
        {
            PlotPhaseOf(config, geometry, 0);
        }

        public void PlotModulationRaw(IPlotConfig config)
        {
            if (config.Backend() != _backend) throw new AUTDException("Invalid plot config type.");
            var err = new byte[256];
            var ret = NativeMethods.LinkVisualizer.AUTDLinkVisualizerPlotModulationRaw(_ptr, _backend, _directivity, config.Ptr(), err);
            if (ret == NativeMethods.Def.Autd3Err)
                throw new AUTDException(err);
        }

        public void PlotModulation(IPlotConfig config)
        {
            if (config.Backend() != _backend) throw new AUTDException("Invalid plot config type.");
            var err = new byte[256];
            var ret = NativeMethods.LinkVisualizer.AUTDLinkVisualizerPlotModulation(_ptr, _backend, _directivity, config.Ptr(), err);
            if (ret == NativeMethods.Def.Autd3Err)
                throw new AUTDException(err);
        }

        public Visualizer Create(LinkPtr ptr, object? props)
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
