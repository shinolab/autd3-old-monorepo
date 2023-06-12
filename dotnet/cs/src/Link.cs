/*
 * File: Link.cs
 * Project: src
 * Created Date: 28/04/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
* Copyright (c) 2021-2023 Shun Suzuki. All rights reserved.
 * 
 */

using System;
using System.Collections.Generic;
using System.Net;
using System.Runtime.InteropServices;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

namespace AUTD3Sharp
{
    namespace Link
    {
        [ComVisible(false)]
        public class Link
        {
            internal LinkPtr Ptr;

            internal Link(LinkPtr ptr)
            {
                Ptr = ptr;
            }

            internal Link() : this(new LinkPtr())
            {
            }
        }

        [UnmanagedFunctionPointer(CallingConvention.Cdecl, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)] public delegate void OnLogOutputCallback(string str);

        [UnmanagedFunctionPointer(CallingConvention.Cdecl)] public delegate void OnLogFlushCallback();

        public sealed class Debug : Link
        {
            public Debug() : base(NativeMethods.Base.AUTDLinkDebug())
            {
            }

            public Debug WithLogFunc(OnLogOutputCallback output, OnLogFlushCallback flush)
            {
                Ptr = NativeMethods.Base.AUTDLinkDebugWithLogFunc(Ptr, Marshal.GetFunctionPointerForDelegate(output), Marshal.GetFunctionPointerForDelegate(flush));
                return this;
            }

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

        public sealed class SOEM : Link
        {
            [UnmanagedFunctionPointer(CallingConvention.Cdecl, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)] public delegate void OnLostCallbackDelegate(string str);

            public SOEM() : base(NativeMethods.LinkSOEM.AUTDLinkSOEM())
            {

            }

            public SOEM WithIfname(string ifname)
            {
                Ptr = NativeMethods.LinkSOEM.AUTDLinkSOEMIfname(Ptr, ifname);
                return this;
            }

            public SOEM WithBufSize(uint size)
            {
                Ptr = NativeMethods.LinkSOEM.AUTDLinkSOEMBufSize(Ptr, size);
                return this;
            }

            public SOEM WithSendCycle(ushort sendCycle)
            {
                Ptr = NativeMethods.LinkSOEM.AUTDLinkSOEMSendCycle(Ptr, sendCycle);
                return this;
            }

            public SOEM WithSync0Cycle(ushort sync0Cycle)
            {
                Ptr = NativeMethods.LinkSOEM.AUTDLinkSOEMSync0Cycle(Ptr, sync0Cycle);
                return this;
            }

            public SOEM WithSyncMode(SyncMode syncMode)
            {
                Ptr = NativeMethods.LinkSOEM.AUTDLinkSOEMSyncMode(Ptr, syncMode);
                return this;
            }

            public SOEM WithTimerStrategy(TimerStrategy timerStrategy)
            {
                Ptr = NativeMethods.LinkSOEM.AUTDLinkSOEMTimerStrategy(Ptr, timerStrategy);
                return this;
            }

            public SOEM WithOnLost(OnLostCallbackDelegate onLost)
            {
                Ptr = NativeMethods.LinkSOEM.AUTDLinkSOEMOnLost(Ptr, Marshal.GetFunctionPointerForDelegate(onLost));
                return this;
            }

            public SOEM WithStateCheckInterval(TimeSpan interval)
            {
                Ptr = NativeMethods.LinkSOEM.AUTDLinkSOEMStateCheckInterval(Ptr, (uint)interval.TotalMilliseconds);
                return this;
            }

            public SOEM WithLogFunc(OnLogOutputCallback output, OnLogFlushCallback flush)
            {
                Ptr = NativeMethods.LinkSOEM.AUTDLinkSOEMLogFunc(Ptr, Marshal.GetFunctionPointerForDelegate(output), Marshal.GetFunctionPointerForDelegate(flush));
                return this;
            }

            public SOEM WithLogLevel(Level level)
            {
                Ptr = NativeMethods.LinkSOEM.AUTDLinkSOEMLogLevel(Ptr, level);
                return this;
            }

            public SOEM WithTimeout(TimeSpan timeout)
            {
                Ptr = NativeMethods.LinkSOEM.AUTDLinkSOEMTimeout(Ptr, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
                return this;
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

        public sealed class RemoteSOEM : Link
        {
            public RemoteSOEM(IPEndPoint ip)
            {
                var err = new byte[256];
                Ptr = NativeMethods.LinkSOEM.AUTDLinkRemoteSOEM(ip.ToString(), err);
                if (Ptr._0 == IntPtr.Zero)
                    throw new AUTDException(err);
            }

            public RemoteSOEM WithTimeout(TimeSpan timeout)
            {
                Ptr = NativeMethods.LinkSOEM.AUTDLinkRemoteSOEMTimeout(Ptr, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
                return this;
            }

        }

        public sealed class TwinCAT : Link
        {
            public TwinCAT()
            {
                var err = new byte[256];
                Ptr = NativeMethods.LinkTwinCAT.AUTDLinkTwinCAT(err);
                if (Ptr._0 == IntPtr.Zero)
                    throw new AUTDException(err);
            }

            public TwinCAT WithTimeout(TimeSpan timeout)
            {
                Ptr = NativeMethods.LinkTwinCAT.AUTDLinkTwinCATTimeout(Ptr, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
                return this;
            }
        }

        public sealed class RemoteTwinCAT : Link
        {
            public RemoteTwinCAT(string serverAmsNetId)
            {
                var err = new byte[256];
                Ptr = NativeMethods.LinkTwinCAT.AUTDLinkRemoteTwinCAT(serverAmsNetId, err);
                if (Ptr._0 == IntPtr.Zero)
                    throw new AUTDException(err);
            }

            public RemoteTwinCAT WithServerIp(IPAddress serverIp)
            {
                Ptr = NativeMethods.LinkTwinCAT.AUTDLinkRemoteTwinCATServerIP(Ptr, serverIp.ToString());
                return this;
            }

            public RemoteTwinCAT WithClientAmsNetId(string clientAmsNetId)
            {
                Ptr = NativeMethods.LinkTwinCAT.AUTDLinkRemoteTwinCATClientAmsNetId(Ptr, clientAmsNetId);
                return this;
            }

            public RemoteTwinCAT WithTimeout(TimeSpan timeout)
            {
                Ptr = NativeMethods.LinkTwinCAT.AUTDLinkRemoteTwinCATTimeout(Ptr, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
                return this;
            }
        }

        public sealed class Simulator : Link
        {
            public Simulator(ushort port)
            {
                Ptr = NativeMethods.LinkSimulator.AUTDLinkSimulator(port);
            }

            public Simulator WithServerIp(IPAddress addr)
            {
                var err = new byte[256];
                Ptr = NativeMethods.LinkSimulator.AUTDLinkSimulatorAddr(Ptr, addr.ToString(), err);
                if (Ptr._0 == IntPtr.Zero)
                    throw new AUTDException(err);
                return this;
            }

            public Simulator WithTimeout(TimeSpan timeout)
            {
                Ptr = NativeMethods.LinkSimulator.AUTDLinkSimulatorTimeout(Ptr, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
                return this;
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
