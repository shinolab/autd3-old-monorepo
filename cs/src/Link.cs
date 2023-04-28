/*
 * File: Link.cs
 * Project: src
 * Created Date: 28/04/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/04/2023
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
                NativeMethods.Base.AUTDLinkDebug(out _builder);
            }

            public Debug LogFunc(OnLogOutputCallback output, OnLogFlushCallback flush)
            {
                NativeMethods.Base.AUTDLinkDebugLogFunc(_builder, Marshal.GetFunctionPointerForDelegate(output), Marshal.GetFunctionPointerForDelegate(flush));
                return this;
            }

            public Debug Level(DebugLevel level)
            {
                NativeMethods.Base.AUTDLinkDebugLogLevel(_builder, (int)level);
                return this;
            }

            public Debug Timeout(TimeSpan timeout)
            {
                NativeMethods.Base.AUTDLinkDebugTimeout(_builder, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
                return this;
            }


            public Link Build()
            {
                NativeMethods.Base.AUTDLinkDebugBuild(out var handle, _builder);
                return new Link(handle);
            }
        }

        public sealed class SOEM
        {
            [UnmanagedFunctionPointer(CallingConvention.Cdecl, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)] public delegate void OnLostCallbackDelegate(string str);

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

            public SOEM LogFunc(OnLogOutputCallback output, OnLogFlushCallback flush)
            {
                NativeMethods.LinkSOEM.AUTDLinkSOEMLogFunc(_soem, Marshal.GetFunctionPointerForDelegate(output), Marshal.GetFunctionPointerForDelegate(flush));
                return this;
            }

            public SOEM LogLevel(AUTD3Sharp.DebugLevel level)
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
            private IntPtr _builder = IntPtr.Zero;

            public RemoteSOEM(string ip, ushort port)
            {
                NativeMethods.LinkRemoteSOEM.AUTDLinkRemoteSOEM(out _builder, ip, port);
            }

            public RemoteSOEM LogFunc(OnLogOutputCallback output, OnLogFlushCallback flush)
            {
                NativeMethods.LinkRemoteSOEM.AUTDLinkRemoteSOEMLogFunc(_builder, Marshal.GetFunctionPointerForDelegate(output), Marshal.GetFunctionPointerForDelegate(flush));
                return this;
            }

            public RemoteSOEM Level(DebugLevel level)
            {
                NativeMethods.LinkRemoteSOEM.AUTDLinkRemoteSOEMLogLevel(_builder, (int)level);
                return this;
            }

            public RemoteSOEM Timeout(TimeSpan timeout)
            {
                NativeMethods.LinkRemoteSOEM.AUTDLinkRemoteSOEMTimeout(_builder, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
                return this;
            }

            public Link Build()
            {
                NativeMethods.LinkRemoteSOEM.AUTDLinkRemoteSOEMBuild(out var handle, _builder);
                return new Link(handle);
            }
        }

        public sealed class TwinCAT
        {
            IntPtr _builder = IntPtr.Zero;

            public TwinCAT()
            {
                NativeMethods.LinkTwinCAT.AUTDLinkTwinCAT(out _builder);
            }

            public TwinCAT LogFunc(OnLogOutputCallback output, OnLogFlushCallback flush)
            {
                NativeMethods.LinkTwinCAT.AUTDLinkTwinCATLogFunc(_builder, Marshal.GetFunctionPointerForDelegate(output), Marshal.GetFunctionPointerForDelegate(flush));
                return this;
            }

            public TwinCAT Level(DebugLevel level)
            {
                NativeMethods.LinkTwinCAT.AUTDLinkTwinCATLogLevel(_builder, (int)level);
                return this;
            }

            public TwinCAT Timeout(TimeSpan timeout)
            {
                NativeMethods.LinkTwinCAT.AUTDLinkTwinCATTimeout(_builder, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
                return this;
            }

            public Link Build()
            {
                NativeMethods.LinkTwinCAT.AUTDLinkTwinCATBuild(out var handle, _builder);
                return new Link(handle);
            }
        }

        public sealed class RemoteTwinCAT
        {
            IntPtr _builder = IntPtr.Zero;

            public RemoteTwinCAT(string serverAmsNetId)
            {
                NativeMethods.LinkRemoteTwinCAT.AUTDLinkRemoteTwinCAT(out _builder, serverAmsNetId);
            }

            public RemoteTwinCAT ServerIp(string serverIp)
            {
                NativeMethods.LinkRemoteTwinCAT.AUTDLinkRemoteTwinCATServerIpAddr(_builder, serverIp);
                return this;
            }

            public RemoteTwinCAT ClientAmsNetId(string clientAmsNetId)
            {
                NativeMethods.LinkRemoteTwinCAT.AUTDLinkRemoteTwinCATClientAmsNetId(_builder, clientAmsNetId);
                return this;
            }

            public RemoteTwinCAT LogFunc(OnLogOutputCallback output, OnLogFlushCallback flush)
            {
                NativeMethods.LinkRemoteTwinCAT.AUTDLinkRemoteTwinCATLogFunc(_builder, Marshal.GetFunctionPointerForDelegate(output), Marshal.GetFunctionPointerForDelegate(flush));
                return this;
            }

            public RemoteTwinCAT Level(DebugLevel level)
            {
                NativeMethods.LinkRemoteTwinCAT.AUTDLinkRemoteTwinCATLogLevel(_builder, (int)level);
                return this;
            }

            public RemoteTwinCAT Timeout(TimeSpan timeout)
            {
                NativeMethods.LinkRemoteTwinCAT.AUTDLinkRemoteTwinCATTimeout(_builder, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
                return this;
            }

            public Link Build()
            {
                NativeMethods.LinkRemoteTwinCAT.AUTDLinkRemoteTwinCATBuild(out var handle, _builder);
                return new Link(handle);
            }
        }

        public sealed class Simulator
        {
            IntPtr _builder = IntPtr.Zero;

            public Simulator()
            {
                NativeMethods.LinkSimulator.AUTDLinkSimulator(out _builder);
            }

            public Simulator LogFunc(OnLogOutputCallback output, OnLogFlushCallback flush)
            {
                NativeMethods.LinkSimulator.AUTDLinkSimulatorLogFunc(_builder, Marshal.GetFunctionPointerForDelegate(output), Marshal.GetFunctionPointerForDelegate(flush));
                return this;
            }

            public Simulator Level(DebugLevel level)
            {
                NativeMethods.LinkSimulator.AUTDLinkSimulatorLogLevel(_builder, (int)level);
                return this;
            }

            public Simulator Timeout(TimeSpan timeout)
            {
                NativeMethods.LinkSimulator.AUTDLinkSimulatorTimeout(_builder, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
                return this;
            }

            public Link Build()
            {
                NativeMethods.LinkSimulator.AUTDLinkSimulatorBuild(out var handle, _builder);
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
