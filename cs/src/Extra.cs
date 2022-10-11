/*
 * File: Extra.cs
 * Project: src
 * Created Date: 11/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */


namespace AUTD3Sharp
{
    namespace Extra
    {
        public class GeometryViewer
        {

            private int _width;
            private int _height;
            private bool _vsync;
            private string _model;
            private string _font;
            private int _gpuIdx;


            public GeometryViewer(int width = 800, int height = 600)
            {
                _width = width;
                _height = height;
                _vsync = true;
                _model = "models/AUTD.glb";
                _font = "";
                _gpuIdx = 0;
            }

            public GeometryViewer Vsync(bool vsync)
            {
                _vsync = vsync;
                return this;
            }

            public GeometryViewer Model(string model)
            {
                _model = model;
                return this;
            }

            public GeometryViewer Font(string font)
            {
                _font = font;
                return this;
            }

            public GeometryViewer GpuIdx(int idx)
            {
                _gpuIdx = idx;
                return this;
            }

            public void View(Controller cnt)
            {
                NativeMethods.ExtraGeometryViewer.AUTDExtraGeometryViewer(cnt.AUTDControllerHandle.CntPtr, _width, _height, _vsync, _model, _font, _gpuIdx);
            }
        }

        public class Simulator
        {
            private string _settingsPath;

            private bool _vsync;
            private ushort _port;
            private string _ip;
            private int _gpuIdx;

            public Simulator()
            {
                _settingsPath = "settings.json";
                _port = 50632;
                _vsync = true;
                _ip = "127.0.0.1";
                _gpuIdx = 0;
            }

            public Simulator SettingsPath(string settingsPath)
            {
                _settingsPath = settingsPath;
                return this;
            }

            public Simulator GpuIdx(int idx)
            {
                _gpuIdx = idx;
                return this;
            }

            public Simulator Vsync(bool vsync)
            {
                _vsync = vsync;
                return this;
            }

            public Simulator Port(ushort port)
            {
                _port = port;
                return this;
            }
            public Simulator Ip(string ip)
            {
                _ip = ip;
                return this;
            }

            public void Run()
            {
                NativeMethods.ExtraSimulator.AUTDExtraSimulator(_settingsPath, _port, _ip, _vsync, _gpuIdx);
            }
        }
    }
}
