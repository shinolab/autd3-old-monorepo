/*
 * File: TwinCAT.cs
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
                _ptr = NativeMethods.LinkTwinCAT.AUTDLinkTwinCAT();
            }

            public TwinCATBuilder WithTimeout(TimeSpan timeout)
            {
                _ptr = NativeMethods.LinkTwinCAT.AUTDLinkTwinCATWithTimeout(_ptr, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
                return this;
            }

            public LinkBuilderPtr Ptr()
            {
                return NativeMethods.LinkTwinCAT.AUTDLinkTwinCATIntoBuilder(_ptr);
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
                var err = new byte[256];
                _ptr = NativeMethods.LinkTwinCAT.AUTDLinkRemoteTwinCAT(serverAmsNetId, err);
                if (_ptr._0 == IntPtr.Zero)
                    throw new AUTDException(err);
            }

            /// <summary>
            /// Set server IP address
            /// </summary>
            /// <param name="serverIp"></param>
            /// <returns></returns>
            public RemoteTwinCATBuilder WithServerIp(IPAddress serverIp)
            {
                _ptr = NativeMethods.LinkTwinCAT.AUTDLinkRemoteTwinCATWithServerIP(_ptr, serverIp.ToString());
                return this;
            }

            /// <summary>
            /// Set client AMS Net ID
            /// </summary>
            /// <param name="clientAmsNetId"></param>
            /// <returns></returns>
            public RemoteTwinCATBuilder WithClientAmsNetId(string clientAmsNetId)
            {
                _ptr = NativeMethods.LinkTwinCAT.AUTDLinkRemoteTwinCATWithClientAmsNetId(_ptr, clientAmsNetId);
                return this;
            }

            public RemoteTwinCATBuilder WithTimeout(TimeSpan timeout)
            {
                _ptr = NativeMethods.LinkTwinCAT.AUTDLinkRemoteTwinCATWithTimeout(_ptr, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
                return this;
            }

            public LinkBuilderPtr Ptr()
            {
                return NativeMethods.LinkTwinCAT.AUTDLinkRemoteTwinCATIntoBuilder(_ptr);
            }
        }

        public static RemoteTwinCATBuilder Builder(string serverAmsNetId)
        {
            return new RemoteTwinCATBuilder(serverAmsNetId);
        }
    }
}
