/*
 * File: Link.cs
 * Project: src
 * Created Date: 28/04/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 * 
 */

using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
using Microsoft.Win32.SafeHandles;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

namespace AUTD3Sharp
{
    namespace Link
    {
        [ComVisible(false)]
        public class Link : SafeHandleZeroOrMinusOneIsInvalid
        {
            internal IntPtr LinkPtr => handle;

            internal Link(IntPtr handle) : base(false)
            {
                SetHandle(handle);
            }

            protected override bool ReleaseHandle() => true;
        }

        [UnmanagedFunctionPointer(CallingConvention.Cdecl, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)] public delegate void OnLogOutputCallback(string str);

        [UnmanagedFunctionPointer(CallingConvention.Cdecl)] public delegate void OnLogFlushCallback();

        public sealed class Debug
        {
            private IntPtr _builder = IntPtr.Zero;

            public Debug()
            {
                _builder = NativeMethods.Base.AUTDLinkDebug();
            }

            public Debug LogFunc(NativeMethods.Level level, OnLogOutputCallback output, OnLogFlushCallback flush)
            {
                NativeMethods.Base.AUTDLinkDebugLogFunc(_builder, level, Marshal.GetFunctionPointerForDelegate(output), Marshal.GetFunctionPointerForDelegate(flush));
                return this;
            }

            public Debug LogLevel(NativeMethods.Level level)
            {
                NativeMethods.Base.AUTDLinkDebugLogLevel(_builder, level);
                return this;
            }

            public Debug Timeout(TimeSpan timeout)
            {
                NativeMethods.Base.AUTDLinkDebugTimeout(_builder, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
                return this;
            }


            public Link Build()
            {
                var handle = NativeMethods.Base.AUTDLinkDebugBuild(_builder);
                return new Link(handle);
            }
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif
