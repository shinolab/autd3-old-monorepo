/*
 * File: TwinCAT.cs
 * Project: Link
 * Created Date: 20/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/09/2023
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
    public sealed class TwinCAT : Internal.Link
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
            Ptr = NativeMethods.LinkTwinCAT.AUTDLinkTwinCATWithTimeout(Ptr, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
            return this;
        }
    }

    /// <summary>
    /// Link for remote TwinCAT3 server via <see href="https://github.com/Beckhoff/ADS">ADS</see> library
    /// </summary>
    public sealed class RemoteTwinCAT : Internal.Link
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
            Ptr = NativeMethods.LinkTwinCAT.AUTDLinkRemoteTwinCATWithServerIP(Ptr, serverIp.ToString());
            return this;
        }

        /// <summary>
        /// Set client AMS Net ID
        /// </summary>
        /// <param name="clientAmsNetId"></param>
        /// <returns></returns>
        public RemoteTwinCAT WithClientAmsNetId(string clientAmsNetId)
        {
            Ptr = NativeMethods.LinkTwinCAT.AUTDLinkRemoteTwinCATWithClientAmsNetId(Ptr, clientAmsNetId);
            return this;
        }

        public RemoteTwinCAT WithTimeout(TimeSpan timeout)
        {
            Ptr = NativeMethods.LinkTwinCAT.AUTDLinkRemoteTwinCATWithTimeout(Ptr, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
            return this;
        }
    }
}
