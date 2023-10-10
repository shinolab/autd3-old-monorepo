/*
 * File: SOEM.cs
 * Project: Link
 * Created Date: 20/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */


using System;
using System.Collections.Generic;
using System.Net;
using System.Runtime.InteropServices;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

namespace AUTD3Sharp.Link
{
    /// <summary>
    /// Link using <see href="https://github.com/OpenEtherCATsociety/SOEM">SOEM</see>
    /// </summary>
    public sealed class SOEM
    {
        [UnmanagedFunctionPointer(CallingConvention.Cdecl, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)] public delegate void OnLostCallbackDelegate(string str);

        public sealed class SOEMBuilder : Internal.ILinkBuilder
        {
            private LinkSOEMBuilderPtr _ptr;

            internal SOEMBuilder()
            {
                _ptr = NativeMethods.LinkSOEM.AUTDLinkSOEM();
            }

            /// <summary>
            /// Set network interface name
            /// </summary>
            /// <param name="ifname">Interface name. If empty, this link will automatically find the network interface that is connected to AUTD3 devices.</param>
            /// <returns></returns>
            public SOEMBuilder WithIfname(string ifname)
            {
                _ptr = NativeMethods.LinkSOEM.AUTDLinkSOEMWithIfname(_ptr, ifname);
                return this;
            }

            /// <summary>
            /// Set buffer size
            /// </summary>
            /// <param name="size"></param>
            /// <returns></returns>
            public SOEMBuilder WithBufSize(uint size)
            {
                _ptr = NativeMethods.LinkSOEM.AUTDLinkSOEMWithBufSize(_ptr, size);
                return this;
            }

            /// <summary>
            /// Set send cycle (the unit is 500us)
            /// </summary>
            /// <param name="sendCycle"></param>
            /// <returns></returns>
            public SOEMBuilder WithSendCycle(ushort sendCycle)
            {
                _ptr = NativeMethods.LinkSOEM.AUTDLinkSOEMWithSendCycle(_ptr, sendCycle);
                return this;
            }

            /// <summary>
            /// Set sync0 cycle (the unit is 500us)
            /// </summary>
            /// <param name="sync0Cycle"></param>
            /// <returns></returns>
            public SOEMBuilder WithSync0Cycle(ushort sync0Cycle)
            {
                _ptr = NativeMethods.LinkSOEM.AUTDLinkSOEMWithSync0Cycle(_ptr, sync0Cycle);
                return this;
            }

            /// <summary>
            /// Set sync mode
            /// </summary>
            /// <remarks>See <see href="https://infosys.beckhoff.com/content/1033/ethercatsystem/2469122443.html">Beckhoff's site</see> for more details.</remarks>
            /// <param name="syncMode"></param>
            /// <returns></returns>
            public SOEMBuilder WithSyncMode(SyncMode syncMode)
            {
                _ptr = NativeMethods.LinkSOEM.AUTDLinkSOEMWithSyncMode(_ptr, syncMode);
                return this;
            }

            /// <summary>
            /// Set timer strategy
            /// </summary>
            /// <param name="timerStrategy"></param>
            /// <returns></returns>
            public SOEMBuilder WithTimerStrategy(TimerStrategy timerStrategy)
            {
                _ptr = NativeMethods.LinkSOEM.AUTDLinkSOEMWithTimerStrategy(_ptr, timerStrategy);
                return this;
            }

            /// <summary>
            /// Set callback function when the link is lost
            /// </summary>
            /// <param name="onLost"></param>
            /// <returns></returns>
            public SOEMBuilder WithOnLost(OnLostCallbackDelegate onLost)
            {
                _ptr = NativeMethods.LinkSOEM.AUTDLinkSOEMWithOnLost(_ptr, Marshal.GetFunctionPointerForDelegate(onLost));
                return this;
            }

            /// <summary>
            /// Set state check interval
            /// </summary>
            /// <param name="interval"></param>
            /// <returns></returns>
            public SOEMBuilder WithStateCheckInterval(TimeSpan interval)
            {
                _ptr = NativeMethods.LinkSOEM.AUTDLinkSOEMWithStateCheckInterval(_ptr, (uint)interval.TotalMilliseconds);
                return this;
            }

            public SOEMBuilder WithTimeout(TimeSpan timeout)
            {
                _ptr = NativeMethods.LinkSOEM.AUTDLinkSOEMWithTimeout(_ptr, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
                return this;
            }

            public LinkBuilderPtr Ptr()
            {
                return NativeMethods.LinkSOEM.AUTDLinkSOEMIntoBuilder(_ptr);
            }
        }

        public static SOEMBuilder Builder()
        {
            return new SOEMBuilder();
        }

        public static IEnumerable<EtherCATAdapter> EnumerateAdapters()
        {
            var handle = NativeMethods.LinkSOEM.AUTDAdapterPointer();
            var len = NativeMethods.LinkSOEM.AUTDAdapterGetSize(handle);
            for (uint i = 0; i < len; i++)
            {
                var sbDesc = new byte[128];
                var sbName = new byte[128];
                NativeMethods.LinkSOEM.AUTDAdapterGetAdapter(handle, i, sbDesc, sbName);
                yield return new EtherCATAdapter(System.Text.Encoding.UTF8.GetString(sbDesc), System.Text.Encoding.UTF8.GetString(sbName));
            }
            NativeMethods.LinkSOEM.AUTDAdapterPointerDelete(handle);
        }
    }

    /// <summary>
    /// Link to connect to remote SOEMServer
    /// </summary>
    public sealed class RemoteSOEM
    {
        public sealed class RemoteSOEMBuilder : Internal.ILinkBuilder
        {
            private LinkRemoteSOEMBuilderPtr _ptr;

            /// <summary>
            /// Constructor
            /// </summary>
            /// <param name="ip">IP address and port of SOEMServer (e.g., "127.0.0.1:8080")</param>
            /// <exception cref="AUTDException"></exception>
            internal RemoteSOEMBuilder(IPEndPoint ip)
            {
                var err = new byte[256];
                _ptr = NativeMethods.LinkSOEM.AUTDLinkRemoteSOEM(ip.ToString(), err);
                if (_ptr._0 == IntPtr.Zero)
                    throw new AUTDException(err);
            }

            public RemoteSOEMBuilder WithTimeout(TimeSpan timeout)
            {
                _ptr = NativeMethods.LinkSOEM.AUTDLinkRemoteSOEMWithTimeout(_ptr, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
                return this;
            }

            public LinkBuilderPtr Ptr()
            {
                return NativeMethods.LinkSOEM.AUTDLinkRemoteSOEMIntoBuilder(_ptr);
            }
        }

        public static RemoteSOEMBuilder Builder(IPEndPoint ip)
        {
            return new RemoteSOEMBuilder(ip);
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

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
