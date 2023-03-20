/*
 * File: Link.cs
 * Project: src
 * Created Date: 28/04/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/03/2023
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

        public sealed class Bundle
        {
            private readonly List<Link> _links;

            public Bundle(Link link)
            {
                _links = new List<Link> { link };
            }

            public Bundle Link(Link link)
            {
                _links.Add(link);
                return this;
            }

            public Link Build()
            {
                var n = _links.Count;
                var links = new IntPtr[n];
                for (var i = 0; i < n; i++)
                    links[i] = _links[i].LinkPtr;
                NativeMethods.LinkBundle.AUTDLinkBundle(out var handle, links, n);
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
                NativeMethods.LinkDebug.AUTDLinkDebug(out var handle, (int)_level, _output, _flush);
                return new Link(handle);
            }
        }

        public sealed class SOEM
        {
            [UnmanagedFunctionPointer(CallingConvention.Cdecl, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)] public delegate void OnLostCallbackDelegate(string str);

            [UnmanagedFunctionPointer(CallingConvention.Cdecl, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)] public delegate void OnLogOutputCallback(string str);

            [UnmanagedFunctionPointer(CallingConvention.Cdecl)] public delegate void OnLogFlushCallback();


            IntPtr _output = IntPtr.Zero;
            IntPtr _flush = IntPtr.Zero;
            AUTD3Sharp.DebugLevel _level = AUTD3Sharp.DebugLevel.Info;

            private string _ifname;
            private ulong _bufSize;
            private ushort _sendCycle;
            private ushort _sync0Cycle;
            private AUTD3Sharp.SyncMode _syncMode;
            private AUTD3Sharp.TimerStrategy _timerStrategy;
            private IntPtr _onLost;
            private ulong _checkInterval;

            public SOEM()
            {
                _ifname = "";
                _bufSize = 0;
                _sendCycle = 2;
                _sync0Cycle = 2;
                _syncMode = AUTD3Sharp.SyncMode.FreeRun;
                _timerStrategy = AUTD3Sharp.TimerStrategy.Sleep;
                _onLost = IntPtr.Zero;
                _checkInterval = 500;
            }

            public SOEM Ifname(string ifname)
            {
                _ifname = ifname;
                return this;
            }

            public SOEM BufSize(ulong size)
            {
                _bufSize = size;
                return this;
            }

            public SOEM SendCycle(ushort sendCycle)
            {
                _sendCycle = sendCycle;
                return this;
            }

            public SOEM Sync0Cycle(ushort sync0Cycle)
            {
                _sync0Cycle = sync0Cycle;
                return this;
            }

            [Obsolete("This methods is deprecated. Use SyncMode(SyncMode) instead.")]
            public SOEM FreeRun(bool freerun)
            {
                _syncMode = freerun ? AUTD3Sharp.SyncMode.FreeRun : AUTD3Sharp.SyncMode.DC;
                return this;
            }

            public SOEM SyncMode(SyncMode syncMode)
            {
                _syncMode = syncMode;
                return this;
            }

            [Obsolete("This methods is deprecated. Use TimerStrategy(TimerStrategy) instead.")]
            public SOEM HighPrecision(bool highPrecision)
            {
                _timerStrategy = highPrecision ? AUTD3Sharp.TimerStrategy.BusyWait : AUTD3Sharp.TimerStrategy.Sleep;
                return this;
            }

            public SOEM TimerStrategy(TimerStrategy timerStrategy)
            {
                _timerStrategy = timerStrategy;
                return this;
            }

            public SOEM OnLost(OnLostCallbackDelegate onLost)
            {
                _onLost = Marshal.GetFunctionPointerForDelegate(onLost);
                return this;
            }

            public SOEM CheckInterval(ulong interval)
            {
                _checkInterval = interval;
                return this;
            }

            public SOEM DebugLogFunc(OnLogOutputCallback output, OnLogFlushCallback flush)
            {
                _output = Marshal.GetFunctionPointerForDelegate(output);
                _flush = Marshal.GetFunctionPointerForDelegate(flush);
                return this;
            }

            public SOEM DebugLevel(AUTD3Sharp.DebugLevel level)
            {
                _level = level;
                return this;
            }

            public Link Build()
            {
                NativeMethods.LinkSOEM.AUTDLinkSOEM(out var handle, _ifname, _bufSize, _sync0Cycle, _sendCycle, _syncMode == AUTD3Sharp.SyncMode.FreeRun, _onLost, (byte)_timerStrategy, _checkInterval, (int)_level, _output, _flush);
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

            public Link Build()
            {
                NativeMethods.LinkRemoteSOEM.AUTDLinkRemoteSOEM(out var handle, _ip, _port);
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
            public Link Build()
            {
                NativeMethods.LinkSimulator.AUTDLinkSimulator(out var handle);
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
