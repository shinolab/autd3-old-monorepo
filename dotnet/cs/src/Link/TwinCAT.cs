/*
 * File: TwinCAT.cs
 * Project: Link
 * Created Date: 20/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 07/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

using System;
using System.Net;

namespace AUTD3Sharp.Link
{
    /// <summary>
    /// Link using TwinCAT3
    /// </summary>
    public sealed class TwinCAT
    {
        public sealed class TwinCATBuilder : Internal.ILinkBuilder
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

            LinkBuilderPtr Internal.ILinkBuilder.Ptr()
            {
                return NativeMethodsLinkTwinCAT.AUTDLinkTwinCATIntoBuilder(_ptr);
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
        public sealed class RemoteTwinCATBuilder : Internal.ILinkBuilder
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
                    fixed (byte* ap = serverAmsNetIdBytes)
                    {
                        var res = NativeMethodsLinkTwinCAT.AUTDLinkRemoteTwinCAT(ap);
                        if (res.result.Item1 == IntPtr.Zero)
                        {
                            var err = new byte[res.err_len];
                            fixed (byte* ep = err)
                                NativeMethodsDef.AUTDGetErr(res.err, ep);
                            throw new AUTDException(err);
                        }
                        _ptr = res.result;
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
                    fixed (byte* ap = serverIpBytes)
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
                    fixed (byte* ap = clientAmsNetIdBytes)
                        _ptr = NativeMethodsLinkTwinCAT.AUTDLinkRemoteTwinCATWithClientAmsNetId(_ptr, ap);
                }
                return this;
            }

            public RemoteTwinCATBuilder WithTimeout(TimeSpan timeout)
            {
                _ptr = NativeMethodsLinkTwinCAT.AUTDLinkRemoteTwinCATWithTimeout(_ptr, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
                return this;
            }

            LinkBuilderPtr Internal.ILinkBuilder.Ptr()
            {
                return NativeMethodsLinkTwinCAT.AUTDLinkRemoteTwinCATIntoBuilder(_ptr);
            }
        }

        public static RemoteTwinCATBuilder Builder(string serverAmsNetId)
        {
            return new RemoteTwinCATBuilder(serverAmsNetId);
        }
    }
}
