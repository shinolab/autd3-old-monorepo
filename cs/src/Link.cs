/*
 * File: Link.cs
 * Project: src
 * Created Date: 28/04/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 17/04/2023
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


        public sealed class Log
        {
            [UnmanagedFunctionPointer(CallingConvention.Cdecl, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)] public delegate void OnLogOutputCallback(string str);

            [UnmanagedFunctionPointer(CallingConvention.Cdecl)] public delegate void OnLogFlushCallback();

            IntPtr _output = IntPtr.Zero;
            IntPtr _flush = IntPtr.Zero;
            DebugLevel _level = DebugLevel.Info;

            Link _link;

            public Log(Link link)
            {
                _link = link;
            }

            public Log LogFunc(OnLogOutputCallback output, OnLogFlushCallback flush)
            {
                _output = Marshal.GetFunctionPointerForDelegate(output);
                _flush = Marshal.GetFunctionPointerForDelegate(flush);
                return this;
            }

            public Log Level(DebugLevel level)
            {
                _level = level;
                return this;
            }

            public Link Build()
            {
                NativeMethods.Base.AUTDLinkLog(out var handle, _link.LinkPtr, (int)_level, _output, _flush);
                return new Link(handle);
            }
        }


        public sealed class Debug
        {
            [UnmanagedFunctionPointer(CallingConvention.Cdecl, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)] public delegate void OnLogOutputCallback(string str);

            [UnmanagedFunctionPointer(CallingConvention.Cdecl)] public delegate void OnLogFlushCallback();

            IntPtr _output = IntPtr.Zero;
            IntPtr _flush = IntPtr.Zero;
            DebugLevel _level = DebugLevel.Debug;

            public Debug LogFunc(OnLogOutputCallback output, OnLogFlushCallback flush)
            {
                _output = Marshal.GetFunctionPointerForDelegate(output);
                _flush = Marshal.GetFunctionPointerForDelegate(flush);
                return this;
            }

            public Debug Level(DebugLevel level)
            {
                _level = level;
                return this;
            }

            public Link Build()
            {
                NativeMethods.Base.AUTDLinkDebug(out var handle, (int)_level, _output, _flush);
                return new Link(handle);
            }
        }

        public sealed class SOEM
        {
            [UnmanagedFunctionPointer(CallingConvention.Cdecl, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)] public delegate void OnLostCallbackDelegate(string str);

            [UnmanagedFunctionPointer(CallingConvention.Cdecl, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)] public delegate void OnLogOutputCallback(string str);

            [UnmanagedFunctionPointer(CallingConvention.Cdecl)] public delegate void OnLogFlushCallback();

            IntPtr _soem;

            public SOEM()
            {
                NativeMethods.LinkSOEM.AUTDLinkSOEM(out _soem);
            }

            public SOEM Ifname(string ifname)
            {
                NativeMethods.LinkSOEM.AUTDLinkSOEMIfname(_soem, ifname);
                return this;
            }

            public SOEM BufSize(ulong size)
            {
                NativeMethods.LinkSOEM.AUTDLinkSOEMBufSize(_soem, size);
                return this;
            }

            public SOEM SendCycle(ushort sendCycle)
            {
                NativeMethods.LinkSOEM.AUTDLinkSOEMSendCycle(_soem, sendCycle);
                return this;
            }

            public SOEM Sync0Cycle(ushort sync0Cycle)
            {
                NativeMethods.LinkSOEM.AUTDLinkSOEMSync0Cycle(_soem, sync0Cycle);
                return this;
            }

            public SOEM SyncMode(SyncMode syncMode)
            {
                NativeMethods.LinkSOEM.AUTDLinkSOEMFreerun(_soem, syncMode == AUTD3Sharp.SyncMode.FreeRun);
                return this;
            }

            public SOEM TimerStrategy(TimerStrategy timerStrategy)
            {
                NativeMethods.LinkSOEM.AUTDLinkSOEMTimerStrategy(_soem, (byte)timerStrategy);
                return this;
            }

            public SOEM OnLost(OnLostCallbackDelegate onLost)
            {
                NativeMethods.LinkSOEM.AUTDLinkSOEMOnLost(_soem, Marshal.GetFunctionPointerForDelegate(onLost));
                return this;
            }

            public SOEM StateCheckInterval(TimeSpan interval)
            {
                NativeMethods.LinkSOEM.AUTDLinkSOEMStateCheckInterval(_soem, (ulong)interval.TotalMilliseconds);
                return this;
            }

            public SOEM DebugLogFunc(OnLogOutputCallback output, OnLogFlushCallback flush)
            {
                NativeMethods.LinkSOEM.AUTDLinkSOEMLogFunc(_soem, Marshal.GetFunctionPointerForDelegate(output), Marshal.GetFunctionPointerForDelegate(flush));
                return this;
            }

            public SOEM DebugLevel(AUTD3Sharp.DebugLevel level)
            {
                NativeMethods.LinkSOEM.AUTDLinkSOEMLogLevel(_soem, (int)level);
                return this;
            }


            public SOEM Timeout(TimeSpan timeout)
            {
                NativeMethods.LinkSOEM.AUTDLinkSOEMTimeout(_soem, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
                return this;
            }

            public Link Build()
            {
                NativeMethods.LinkSOEM.AUTDLinkSOEMBuild(out var handle, _soem);
                NativeMethods.LinkSOEM.AUTDLinkSOEMDelete(_soem);
                return new Link(handle);
            }

            public static IEnumerable<EtherCATAdapter> EnumerateAdapters()
            {
                var size = NativeMethods.LinkSOEM.AUTDGetAdapterPointer(out var handle);
                for (var i = 0; i < size; i++)
                {
                    var sbDesc = new StringBuilder(128);
                    var sbName = new StringBuilder(128);
                    NativeMethods.LinkSOEM.AUTDGetAdapter(handle, i, sbDesc, sbName);
                    yield return new EtherCATAdapter(sbDesc.ToString(), sbName.ToString());
                }
                NativeMethods.LinkSOEM.AUTDFreeAdapterPointer(handle);
            }
        }

        public sealed class RemoteSOEM
        {
            private string _ip;
            private ushort _port;
            private TimeSpan _timeout = TimeSpan.FromMilliseconds(20);


            public RemoteSOEM()
            {
                _ip = "";
                _port = 50632;
            }

            public RemoteSOEM Ip(string ip)
            {
                _ip = ip;
                return this;
            }

            public RemoteSOEM Port(ushort port)
            {
                _port = port;
                return this;
            }

            public RemoteSOEM Timeout(TimeSpan timeout)
            {
                _timeout = timeout;
                return this;
            }

            public Link Build()
            {
                NativeMethods.LinkRemoteSOEM.AUTDLinkRemoteSOEM(out var handle, _ip, _port, (ulong)(_timeout.TotalMilliseconds * 1000 * 1000));
                return new Link(handle);
            }
        }

        public sealed class TwinCAT
        {
            public Link Build()
            {
                NativeMethods.LinkTwinCAT.AUTDLinkTwinCAT(out var handle);
                return new Link(handle);
            }
        }

        public sealed class RemoteTwinCAT
        {
            private readonly string _remoteAmsNetId;
            private string _remoteIp;
            private string _localAmsNetId;

            public RemoteTwinCAT(string remoteAmsNetId)
            {
                _remoteAmsNetId = remoteAmsNetId;
                _localAmsNetId = string.Empty;
                _remoteIp = string.Empty;
            }

            public RemoteTwinCAT RemoteIp(string remoteIp)
            {
                _remoteIp = remoteIp;
                return this;
            }

            public RemoteTwinCAT LocalAmsNetId(string localAmsNetId)
            {
                _localAmsNetId = localAmsNetId;
                return this;
            }

            public Link Build()
            {
                NativeMethods.LinkRemoteTwinCAT.AUTDLinkRemoteTwinCAT(out var handle, _remoteIp, _remoteAmsNetId, _localAmsNetId);
                return new Link(handle);
            }
        }

        public sealed class Simulator
        {
            private TimeSpan _timeout = TimeSpan.FromMilliseconds(20);

            public Simulator Timeout(TimeSpan timeout)
            {
                _timeout = timeout;
                return this;
            }

            public Link Build()
            {
                NativeMethods.LinkSimulator.AUTDLinkSimulator(out var handle, (ulong)(_timeout.TotalMilliseconds * 1000 * 1000));
                return new Link(handle);
            }
        }

        public readonly struct EtherCATAdapter : IEquatable<EtherCATAdapter>
        {
            public string Desc { get; }
            public string Name { get; }

            internal EtherCATAdapter(string desc, string name)
            {
                Desc = desc;
                Name = name;
            }

            public override string ToString() => $"{Desc}, {Name}";
            public bool Equals(EtherCATAdapter other) => Desc.Equals(other.Desc) && Name.Equals(other.Name);
            public static bool operator ==(EtherCATAdapter left, EtherCATAdapter right) => left.Equals(right);
            public static bool operator !=(EtherCATAdapter left, EtherCATAdapter right) => !left.Equals(right);
            public override bool Equals(object? obj) => obj is EtherCATAdapter adapter && Equals(adapter);
            public override int GetHashCode() => Desc.GetHashCode() ^ Name.GetHashCode();
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif
