/*
 * File: Log.cs
 * Project: Link
 * Created Date: 13/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 13/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

using System.Runtime.InteropServices;
using AUTD3Sharp.Internal;

namespace AUTD3Sharp.Link
{
    /// <summary>
    /// Link for logging
    /// </summary>
    public sealed class Log : Internal.Link
    {
        public Log(Internal.Link link) : base(NativeMethods.Base.AUTDLinkLog(link.Ptr))
        {
        }

        /// <summary>
        /// Set log function
        /// </summary>
        /// <remarks>By default, the logger will display log messages on the console.</remarks>
        /// <param name="output">output callback</param>
        /// <param name="flush">flush callback</param>
        /// <returns></returns>
        public Log WithLogFunc(OnLogOutputCallback output, OnLogFlushCallback flush)
        {
            Ptr = NativeMethods.Base.AUTDLinkLogWithLogFunc(Ptr, Marshal.GetFunctionPointerForDelegate(output), Marshal.GetFunctionPointerForDelegate(flush));
            return this;
        }

        /// <summary>
        /// Set log level
        /// </summary>
        /// <param name="level"></param>
        /// <returns></returns>
        public Log WithLogLevel(Level level)
        {
            Ptr = NativeMethods.Base.AUTDLinkLogWithLogLevel(Ptr, level);
            return this;
        }
    }

    public static class LogLinkExtensions
    {
        public static Log WithLog(this Internal.Link s)
        {
            return new Log(s);
        }
    }
}
