/*
 * File: Link.cs
 * Project: src
 * Created Date: 28/04/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/08/2023
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

        /// <summary>
        /// Link for debugging
        /// </summary>
        public sealed class Debug : Link
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

        /// <summary>
        /// Link using <see cref="https://github.com/OpenEtherCATsociety/SOEM">SOEM</see>
        /// </summary>
        public sealed class SOEM : Link
        {
            [UnmanagedFunctionPointer(CallingConvention.Cdecl, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)] public delegate void OnLostCallbackDelegate(string str);

            public SOEM() : base(NativeMethods.LinkSOEM.AUTDLinkSOEM())
            {

            }

            /// <summary>
            /// Set network interface name
            /// </summary>
            /// <param name="ifname">Interface name. If empty, this link will automatically find the network interface that is connected to AUTD3 devices.</param>
            /// <returns></returns>
            public SOEM WithIfname(string ifname)
            {
                Ptr = NativeMethods.LinkSOEM.AUTDLinkSOEMIfname(Ptr, ifname);
                return this;
            }

            /// <summary>
            /// Set buffer size
            /// </summary>
            /// <param name="size"></param>
            /// <returns></returns>
            public SOEM WithBufSize(uint size)
            {
                Ptr = NativeMethods.LinkSOEM.AUTDLinkSOEMBufSize(Ptr, size);
                return this;
            }

            /// <summary>
            /// Set send cycle (the unit is 500us)
            /// </summary>
            /// <param name="sendCycle"></param>
            /// <returns></returns>
            public SOEM WithSendCycle(ushort sendCycle)
            {
                Ptr = NativeMethods.LinkSOEM.AUTDLinkSOEMSendCycle(Ptr, sendCycle);
                return this;
            }

            /// <summary>
            /// Set sync0 cycle (the unit is 500us)
            /// </summary>
            /// <param name="sync0Cycle"></param>
            /// <returns></returns>
            public SOEM WithSync0Cycle(ushort sync0Cycle)
            {
                Ptr = NativeMethods.LinkSOEM.AUTDLinkSOEMSync0Cycle(Ptr, sync0Cycle);
                return this;
            }

            /// <summary>
            /// Set sync mode
            /// </summary>
            /// <remarks>See <see cref="https://infosys.beckhoff.com/content/1033/ethercatsystem/2469122443.html">Beckhoff's site</see> for more details.</remarks>
            /// <param name="syncMode"></param>
            /// <returns></returns>
            public SOEM WithSyncMode(SyncMode syncMode)
            {
                Ptr = NativeMethods.LinkSOEM.AUTDLinkSOEMSyncMode(Ptr, syncMode);
                return this;
            }

            /// <summary>
            /// Set timer strategy
            /// </summary>
            /// <param name="timerStrategy"></param>
            /// <returns></returns>
            public SOEM WithTimerStrategy(TimerStrategy timerStrategy)
            {
                Ptr = NativeMethods.LinkSOEM.AUTDLinkSOEMTimerStrategy(Ptr, timerStrategy);
                return this;
            }

            /// <summary>
            /// Set callback function when the link is lost
            /// </summary>
            /// <param name="onLost"></param>
            /// <returns></returns>
            public SOEM WithOnLost(OnLostCallbackDelegate onLost)
            {
                Ptr = NativeMethods.LinkSOEM.AUTDLinkSOEMOnLost(Ptr, Marshal.GetFunctionPointerForDelegate(onLost));
                return this;
            }

            /// <summary>
            /// Set state check interval
            /// </summary>
            /// <param name="interval"></param>
            /// <returns></returns>
            public SOEM WithStateCheckInterval(TimeSpan interval)
            {
                Ptr = NativeMethods.LinkSOEM.AUTDLinkSOEMStateCheckInterval(Ptr, (uint)interval.TotalMilliseconds);
                return this;
            }

            /// <summary>
            /// Set log function
            /// </summary>
            /// <param name="output"></param>
            /// <param name="flush"></param>
            /// <returns></returns>
            public SOEM WithLogFunc(OnLogOutputCallback output, OnLogFlushCallback flush)
            {
                Ptr = NativeMethods.LinkSOEM.AUTDLinkSOEMLogFunc(Ptr, Marshal.GetFunctionPointerForDelegate(output), Marshal.GetFunctionPointerForDelegate(flush));
                return this;
            }

            /// <summary>
            /// Set log level
            /// </summary>
            /// <param name="level"></param>
            /// <returns></returns>
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
                var handle = NativeMethods.LinkSOEM.AUTDGetAdapterPointer();
                var len = NativeMethods.LinkSOEM.AUTDGetAdapterSize(handle);
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

        /// <summary>
        /// Link to connect to remote SOEMServer
        /// </summary>
        public sealed class RemoteSOEM : Link
        {
            /// <summary>
            /// Constructor
            /// </summary>
            /// <param name="ip">IP address and port of SOEMServer (e.g., "127.0.0.1:8080")</param>
            /// <exception cref="AUTDException"></exception>
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

        /// <summary>
        /// Link using TwinCAT3
        /// </summary>
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

        /// <summary>
        /// Link for remote TwinCAT3 server via <see cref="https://github.com/Beckhoff/ADS">ADS</see> library
        /// </summary>
        public sealed class RemoteTwinCAT : Link
        {
            /// <summary>
            /// Constructor
            /// </summary>
            /// <param name="serverAmsNetId">Server AMS Net ID</param>
            /// <exception cref="AUTDException"></exception>
            public RemoteTwinCAT(string serverAmsNetId)
            {
                var err = new byte[256];
                Ptr = NativeMethods.LinkTwinCAT.AUTDLinkRemoteTwinCAT(serverAmsNetId, err);
                if (Ptr._0 == IntPtr.Zero)
                    throw new AUTDException(err);
            }

            /// <summary>
            /// Set server IP address
            /// </summary>
            /// <param name="serverIp"></param>
            /// <returns></returns>
            public RemoteTwinCAT WithServerIp(IPAddress serverIp)
            {
                Ptr = NativeMethods.LinkTwinCAT.AUTDLinkRemoteTwinCATServerIP(Ptr, serverIp.ToString());
                return this;
            }

            /// <summary>
            /// Set client AMS Net ID
            /// </summary>
            /// <param name="clientAmsNetId"></param>
            /// <returns></returns>
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

        /// <summary>
        /// Link for AUTD Simulator
        /// </summary>
        public sealed class Simulator : Link
        {
            public Simulator(ushort port)
            {
                Ptr = NativeMethods.LinkSimulator.AUTDLinkSimulator(port);
            }

            /// <summary>
            /// Set server IP address
            /// </summary>
            /// <param name="addr"></param>
            /// <returns></returns>
            /// <exception cref="AUTDException"></exception>
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
