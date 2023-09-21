/*
 * File: GeometryViewer.cs
 * Project: Extra
 * Created Date: 20/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

using System.Runtime.InteropServices;

namespace AUTD3Sharp.Extra
{
    /// <summary>
    /// Graphical viewer for Geometry
    /// </summary>
    [ComVisible(false)]
    public class GeometryViewer
    {
        private GeometryViewerPtr _handle = NativeMethods.GeometryViewer.AUTDGeometryViewer();

        /// <summary>
        /// Set window size
        /// </summary>
        /// <param name="width">Width of window</param>
        /// <param name="height">Height of window</param>
        /// <returns></returns>
        public GeometryViewer WindowSize(uint width, uint height)
        {
            _handle = NativeMethods.GeometryViewer.AUTDGeometryViewerWithSize(_handle, width, height);
            return this;
        }

        /// <summary>
        /// Set vsync
        /// </summary>
        /// <param name="vsync">vsync</param>
        /// <returns></returns>
        public GeometryViewer Vsync(bool vsync)
        {
            _handle = NativeMethods.GeometryViewer.AUTDGeometryViewerWithVsync(_handle, vsync);
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
}
