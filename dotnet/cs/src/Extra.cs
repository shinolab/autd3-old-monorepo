/*
* File: Extra.cs
* Project: src
* Created Date: 11/10/2022
* Author: Shun Suzuki
* -----
* Last Modified: 01/02/2023
* Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
* -----
* Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
* 
*/

using System;
using System.Text;
using System.Runtime.InteropServices;

namespace AUTD3Sharp
{
    namespace Extra
    {
        /// <summary>
        /// Graphical viewer for Geometry
        /// </summary>
        [ComVisible(false)]
        public class GeometryViewer
        {
            private GeometryViewerPtr _handle;

            public GeometryViewer()
            {
                _handle = NativeMethods.GeometryViewer.AUTDGeometryViewer();
            }

            /// <summary>
            /// Set window size
            /// </summary>
            /// <param name="width">Width of window</param>
            /// <param name="height">Height of window</param>
            /// <returns></returns>
            public GeometryViewer WindowSize(uint width, uint height)
            {
                _handle = NativeMethods.GeometryViewer.AUTDGeometryViewerSize(_handle, width, height);
                return this;
            }

            /// <summary>
            /// Set vsync
            /// </summary>
            /// <param name="vsync">vsync</param>
            /// <returns></returns>
            public GeometryViewer Vsync(bool vsync)
            {
                _handle = NativeMethods.GeometryViewer.AUTDGeometryViewerVsync(_handle, vsync);
                return this;
            }

            /// <summary>
            /// Run viewer
            /// </summary>
            /// <param name="geometry"></param>
            /// <returns>0 if success, otherwise error code</returns>
            public int Run(Geometry geometry)
            =>
                NativeMethods.GeometryViewer.AUTDGeometryViewerRun(_handle, geometry.Ptr);

        }

        /// <summary>
        /// AUTD Simulator
        /// </summary>
        [ComVisible(false)]
        public class Simulator
        {
            private IntPtr _handle;

            public Simulator()
            {
                _handle = NativeMethods.Simulator.AUTDSimulator();
            }

            /// <summary>
            /// Set window size
            /// </summary>
            /// <param name="width">Width of window</param>
            /// <param name="height">Height of window</param>
            /// <returns></returns>
            public Simulator WindowSize(uint width, uint height)
            {
                _handle = NativeMethods.Simulator.AUTDSimulatorWindowSize(_handle, width, height);
                return this;
            }

            public Simulator Vsync(bool vsync)
            {
                _handle = NativeMethods.Simulator.AUTDSimulatorVsync(_handle, vsync);
                return this;
            }

            /// <summary>
            /// Set GPU index
            /// </summary>
            /// <param name="idx">GPU index. If -1, use the most suitable GPU.</param>
            /// <returns></returns>
            public Simulator GpuIdx(int idx)
            {
                _handle = NativeMethods.Simulator.AUTDSimulatorGpuIdx(_handle, idx);
                return this;
            }

            /// <summary>
            /// Set simulator port
            /// </summary>
            /// <param name="port">Port</param>
            /// <returns></returns>
            public Simulator Port(ushort port)
            {
                _handle = NativeMethods.Simulator.AUTDSimulatorPort(_handle, port);
                return this;
            }

            /// <summary>
            /// Set settings path
            /// </summary>
            /// <param name="settingsPath">Settings path</param>
            /// <returns></returns>
            public Simulator SettingsPath(string settingsPath)
            {
                var err = new byte[256];
                var handle = NativeMethods.Simulator.AUTDSimulatorSettingsPath(_handle, settingsPath, err);
                if (handle != IntPtr.Zero)
                    _handle = handle;

                return this;
            }

            /// <summary>
            /// Run simulator
            /// </summary>
            /// <returns>0 if success, otherwise error code</returns>
            public int Run()
            =>
                NativeMethods.Simulator.AUTDSimulatorRun(_handle);

            /// <summary>
            /// Save current simulator settings to file
            /// </summary>
            /// <param name="path">File path</param>
            /// <exception cref="AUTDException"></exception>
            public void SaveSettings(string path)
            {
                var err = new byte[256];
                if (!NativeMethods.Simulator.AUTDSimulatorSaveSettings(_handle, path, err))
                    throw new AUTDException(err);
            }
        }
    }
}
