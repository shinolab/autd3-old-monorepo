/*
 * File: Link.cs
 * Project: src
 * Created Date: 28/04/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
* Copyright (c) 2021-2023 Shun Suzuki. All rights reserved.
 * 
 */

using System;
using System.Collections.Generic;
using System.Net;
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

            public Debug LogFunc(Level level, OnLogOutputCallback output, OnLogFlushCallback flush)
            {
                _builder = NativeMethods.Base.AUTDLinkDebugLogFunc(_builder, level, Marshal.GetFunctionPointerForDelegate(output), Marshal.GetFunctionPointerForDelegate(flush));
                return this;
            }

            public Debug LogLevel(Level level)
            {
                _builder = NativeMethods.Base.AUTDLinkDebugLogLevel(_builder, level);
                return this;
            }

            public Debug Timeout(TimeSpan timeout)
            {
                _builder = NativeMethods.Base.AUTDLinkDebugTimeout(_builder, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
                return this;
            }


            public Link Build()
            {
                var handle = NativeMethods.Base.AUTDLinkDebugBuild(_builder);
                return new Link(handle);
            }
        }

        public sealed class SOEM
        {
            [UnmanagedFunctionPointer(CallingConvention.Cdecl, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)] public delegate void OnLostCallbackDelegate(string str);

            IntPtr _builder;

            public SOEM()
            {
                _builder = NativeMethods.LinkSOEM.AUTDLinkSOEM();
            }

            public SOEM Ifname(string ifname)
            {
                _builder = NativeMethods.LinkSOEM.AUTDLinkSOEMIfname(_builder, ifname);
                return this;
            }

            public SOEM BufSize(uint size)
            {
                _builder = NativeMethods.LinkSOEM.AUTDLinkSOEMBufSize(_builder, size);
                return this;
            }

            public SOEM SendCycle(ushort sendCycle)
            {
                _builder = NativeMethods.LinkSOEM.AUTDLinkSOEMSendCycle(_builder, sendCycle);
                return this;
            }

            public SOEM Sync0Cycle(ushort sync0Cycle)
            {
                _builder = NativeMethods.LinkSOEM.AUTDLinkSOEMSync0Cycle(_builder, sync0Cycle);
                return this;
            }

            public SOEM SyncMode(SyncMode syncMode)
            {
                _builder = NativeMethods.LinkSOEM.AUTDLinkSOEMSyncMode(_builder, syncMode);
                return this;
            }

            public SOEM TimerStrategy(TimerStrategy timerStrategy)
            {
                _builder = NativeMethods.LinkSOEM.AUTDLinkSOEMTimerStrategy(_builder, timerStrategy);
                return this;
            }

            public SOEM OnLost(OnLostCallbackDelegate onLost)
            {
                _builder = NativeMethods.LinkSOEM.AUTDLinkSOEMOnLost(_builder, Marshal.GetFunctionPointerForDelegate(onLost));
                return this;
            }

            public SOEM StateCheckInterval(TimeSpan interval)
            {
                _builder = NativeMethods.LinkSOEM.AUTDLinkSOEMStateCheckInterval(_builder, (uint)interval.TotalMilliseconds);
                return this;
            }

            public SOEM LogFunc(Level level, OnLogOutputCallback output, OnLogFlushCallback flush)
            {
                _builder = NativeMethods.LinkSOEM.AUTDLinkSOEMLogFunc(_builder, level, Marshal.GetFunctionPointerForDelegate(output), Marshal.GetFunctionPointerForDelegate(flush));
                return this;
            }

            public SOEM LogLevel(Level level)
            {
                _builder = NativeMethods.LinkSOEM.AUTDLinkSOEMLogLevel(_builder, level);
                return this;
            }

            public SOEM Timeout(TimeSpan timeout)
            {
                _builder = NativeMethods.LinkSOEM.AUTDLinkSOEMTimeout(_builder, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
                return this;
            }

            public Link Build()
            {
                var handle = NativeMethods.LinkSOEM.AUTDLinkSOEMBuild(_builder);
                return new Link(handle);
            }

            public static IEnumerable<EtherCATAdapter> EnumerateAdapters()
            {
                var handle = NativeMethods.LinkSOEM.AUTDGetAdapterPointer(out var len);
                for (uint i = 0; i < len; i++)
                {
                    var sbDesc = new byte[128];
                    var sbName = new byte[128];
                    NativeMethods.LinkSOEM.AUTDGetAdapter(handle, i, sbDesc, sbName);
                    yield return new EtherCATAdapter(System.Text.Encoding.UTF8.GetString(sbDesc), System.Text.Encoding.UTF8.GetString(sbName));
                }
                NativeMethods.LinkSOEM.AUTDFreeAdapterPointer(handle);
            }
        }

        public sealed class RemoteSOEM
        {
            private IntPtr _builder = IntPtr.Zero;

            public RemoteSOEM(IPAddress ip, ushort port)
            {
                _builder = NativeMethods.LinkSOEM.AUTDLinkRemoteSOEM(ip.ToString(), port);
            }

            public RemoteSOEM Timeout(TimeSpan timeout)
            {
                _builder = NativeMethods.LinkSOEM.AUTDLinkRemoteSOEMTimeout(_builder, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
                return this;
            }

            public Link Build()
            {
                var handle = NativeMethods.LinkSOEM.AUTDLinkRemoteSOEMBuild(_builder);
                return new Link(handle);
            }
        }

        public sealed class TwinCAT
        {
            IntPtr _builder = IntPtr.Zero;

            public TwinCAT()
            {
                _builder = NativeMethods.LinkTwinCAT.AUTDLinkTwinCAT();
            }

            public TwinCAT Timeout(TimeSpan timeout)
            {
                _builder = NativeMethods.LinkTwinCAT.AUTDLinkTwinCATTimeout(_builder, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
                return this;
            }

            public Link Build()
            {
                var err = new byte[256];
                var handle = NativeMethods.LinkTwinCAT.AUTDLinkTwinCATBuild(_builder, err);
                if (handle == IntPtr.Zero)
                {
                    throw new AUTDException(err);
                }
                return new Link(handle);
            }
        }

        public sealed class RemoteTwinCAT
        {
            IntPtr _builder = IntPtr.Zero;

            public RemoteTwinCAT(string serverAmsNetId)
            {
                _builder = NativeMethods.LinkTwinCAT.AUTDLinkRemoteTwinCAT(serverAmsNetId);
            }

            public RemoteTwinCAT ServerIp(IPAddress serverIp)
            {
                _builder = NativeMethods.LinkTwinCAT.AUTDLinkRemoteTwinCATServerIP(_builder, serverIp.ToString());
                return this;
            }

            public RemoteTwinCAT ClientAmsNetId(string clientAmsNetId)
            {
                _builder = NativeMethods.LinkTwinCAT.AUTDLinkRemoteTwinCATClientAmsNetId(_builder, clientAmsNetId);
                return this;
            }

            public RemoteTwinCAT Timeout(TimeSpan timeout)
            {
                _builder = NativeMethods.LinkTwinCAT.AUTDLinkRemoteTwinCATTimeout(_builder, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
                return this;
            }

            public Link Build()
            {
                var err = new byte[256];
                var handle = NativeMethods.LinkTwinCAT.AUTDLinkRemoteTwinCATBuild(_builder, err);
                if (handle == IntPtr.Zero)
                {
                    throw new AUTDException(err);
                }
                return new Link(handle);
            }
        }

        public sealed class Simulator
        {
            IntPtr _builder = IntPtr.Zero;

            public Simulator(ushort port)
            {
                _builder = NativeMethods.LinkSimulator.AUTDLinkSimulator(port);
            }

            public Simulator Addr(IPAddress addr)
            {
                _builder = NativeMethods.LinkSimulator.AUTDLinkSimulatorAddr(_builder, addr.ToString());
                return this;
            }

            public Simulator Timeout(TimeSpan timeout)
            {
                _builder = NativeMethods.LinkSimulator.AUTDLinkSimulatorTimeout(_builder, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
                return this;
            }

            public Link Build()
            {
                var handle = NativeMethods.LinkSimulator.AUTDLinkSimulatorBuild(_builder);
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
