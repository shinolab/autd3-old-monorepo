/*
 * File: Debug.cs
 * Project: Link
 * Created Date: 20/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */


using System;
using System.Runtime.InteropServices;
using AUTD3Sharp.Internal;

namespace AUTD3Sharp.Link
{
    /// <summary>
    /// Link for debugging
    /// </summary>
    public sealed class Debug : Internal.Link
    {
        public Debug() : base(NativeMethods.Base.AUTDLinkDebug())
        {
        }

        /// <summary>
        /// Set log function
        /// </summary>
        /// <remarks>By default, the logger will display log messages on the console.</remarks>
        /// <param name="output">output callback</param>
        /// <param name="flush">flush callback</param>
        /// <returns></returns>
        public Debug WithLogFunc(OnLogOutputCallback output, OnLogFlushCallback flush)
        {
            Ptr = NativeMethods.Base.AUTDLinkDebugWithLogFunc(Ptr, Marshal.GetFunctionPointerForDelegate(output), Marshal.GetFunctionPointerForDelegate(flush));
            return this;
        }

        /// <summary>
        /// Set log level
        /// </summary>
        /// <param name="level"></param>
        /// <returns></returns>
        public Debug WithLogLevel(Level level)
        {
            Ptr = NativeMethods.Base.AUTDLinkDebugWithLogLevel(Ptr, level);
            return this;
        }

        public Debug WithTimeout(TimeSpan timeout)
        {
            Ptr = NativeMethods.Base.AUTDLinkDebugWithTimeout(Ptr, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
            return this;
        }
    }
}
