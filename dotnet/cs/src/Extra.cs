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
        [ComVisible(false)]
        public class GeometryViewer
        {
            private IntPtr _handle;

            public GeometryViewer()
            {
                _handle = NativeMethods.GeometryViewer.AUTDGeometryViewer();
            }

            public GeometryViewer WindowSize(uint width, uint height)
            {
                _handle = NativeMethods.GeometryViewer.AUTDGeometryViewerSize(_handle, width, height);
                return this;
            }

            public GeometryViewer Vsync(bool vsync)
            {
                _handle = NativeMethods.GeometryViewer.AUTDGeometryViewerVsync(_handle, vsync);
                return this;
            }


            public int Run(Geometry geometry)
            =>
                NativeMethods.GeometryViewer.AUTDGeometryViewerRun(_handle, geometry.Ptr);

        }


        [ComVisible(false)]
        public class Simulator
        {
            private IntPtr _handle;

            public Simulator()
            {
                _handle = NativeMethods.Simulator.AUTDSimulator();
            }

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

            public Simulator GpuIdx(int idx)
            {
                _handle = NativeMethods.Simulator.AUTDSimulatorGpuIdx(_handle, idx);
                return this;
            }

            public Simulator Port(ushort port)
            {
                _handle = NativeMethods.Simulator.AUTDSimulatorPort(_handle, port);
                return this;
            }

            public Simulator SettingsPath(string settingsPath)
            {
                var err = new byte[256];
                var handle = NativeMethods.Simulator.AUTDSimulatorSettingsPath(_handle, settingsPath, err);
                if (handle != IntPtr.Zero)
                    _handle = handle;

                return this;
            }

            public int Run()
            =>
                NativeMethods.Simulator.AUTDSimulatorRun(_handle);

            public void SaveSettings(string path)
            {
                var err = new byte[256];
                if (!NativeMethods.Simulator.AUTDSimulatorSaveSettings(_handle, path, err))
                    throw new AUTDException(err);
            }
        }
    }
}
