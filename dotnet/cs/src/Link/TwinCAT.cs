/*
 * File: TwinCAT.cs
 * Project: Link
 * Created Date: 20/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

using AUTD3Sharp.Internal;
using AUTD3Sharp.NativeMethods;
using System;
using System.Net;

namespace AUTD3Sharp.Link
{
    /// <summary>
    /// Link using TwinCAT3
    /// </summary>
    public sealed class TwinCAT
    {
        public sealed class TwinCATBuilder : ILinkBuilder<TwinCAT>
        {
            private LinkTwinCATBuilderPtr _ptr;

            internal TwinCATBuilder()
            {
                _ptr = NativeMethodsLinkTwinCAT.AUTDLinkTwinCAT();
            }

            public TwinCATBuilder WithTimeout(TimeSpan timeout)
            {
                _ptr = NativeMethodsLinkTwinCAT.AUTDLinkTwinCATWithTimeout(_ptr, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
                return this;
            }

            LinkBuilderPtr ILinkBuilder<TwinCAT>.Ptr()
            {
                return NativeMethodsLinkTwinCAT.AUTDLinkTwinCATIntoBuilder(_ptr);
            }

            TwinCAT ILinkBuilder<TwinCAT>.ResolveLink(LinkPtr ptr)
            {
                return new TwinCAT();
            }
        }

        public static TwinCATBuilder Builder()
        {
            return new TwinCATBuilder();
        }
    }

    /// <summary>
    /// Link for remote TwinCAT3 server via <see href="https://github.com/Beckhoff/ADS">ADS</see> library
    /// </summary>
    public sealed class RemoteTwinCAT
    {
        public sealed class RemoteTwinCATBuilder : ILinkBuilder<RemoteTwinCAT>
        {
            private LinkRemoteTwinCATBuilderPtr _ptr;

            /// <summary>
            /// Constructor
            /// </summary>
            /// <param name="serverAmsNetId">Server AMS Net ID</param>
            /// <exception cref="AUTDException"></exception>
            public RemoteTwinCATBuilder(string serverAmsNetId)
            {
                var serverAmsNetIdBytes = System.Text.Encoding.UTF8.GetBytes(serverAmsNetId);
                unsafe
                {
                    fixed (byte* ap = &serverAmsNetIdBytes[0])
                    {
                        _ptr = NativeMethodsLinkTwinCAT.AUTDLinkRemoteTwinCAT(ap).Validate();
                    }
                }
            }

            /// <summary>
            /// Set server IP address
            /// </summary>
            /// <param name="serverIp"></param>
            /// <returns></returns>
            public RemoteTwinCATBuilder WithServerIp(IPAddress serverIp)
            {
                var serverIpBytes = serverIp.GetAddressBytes();
                unsafe
                {
                    fixed (byte* ap = &serverIpBytes[0])
                        _ptr = NativeMethodsLinkTwinCAT.AUTDLinkRemoteTwinCATWithServerIP(_ptr, ap);
                }

                return this;
            }

            /// <summary>
            /// Set client AMS Net ID
            /// </summary>
            /// <param name="clientAmsNetId"></param>
            /// <returns></returns>
            public RemoteTwinCATBuilder WithClientAmsNetId(string clientAmsNetId)
            {
                var clientAmsNetIdBytes = System.Text.Encoding.UTF8.GetBytes(clientAmsNetId);
                unsafe
                {
                    fixed (byte* ap = &clientAmsNetIdBytes[0])
                        _ptr = NativeMethodsLinkTwinCAT.AUTDLinkRemoteTwinCATWithClientAmsNetId(_ptr, ap);
                }
                return this;
            }

            public RemoteTwinCATBuilder WithTimeout(TimeSpan timeout)
            {
                _ptr = NativeMethodsLinkTwinCAT.AUTDLinkRemoteTwinCATWithTimeout(_ptr, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
                return this;
            }

            LinkBuilderPtr ILinkBuilder<RemoteTwinCAT>.Ptr()
            {
                return NativeMethodsLinkTwinCAT.AUTDLinkRemoteTwinCATIntoBuilder(_ptr);
            }

            RemoteTwinCAT ILinkBuilder<RemoteTwinCAT>.ResolveLink(LinkPtr ptr)
            {
                return new RemoteTwinCAT();
            }
        }

        public static RemoteTwinCATBuilder Builder(string serverAmsNetId)
        {
            return new RemoteTwinCATBuilder(serverAmsNetId);
        }
    }
}
