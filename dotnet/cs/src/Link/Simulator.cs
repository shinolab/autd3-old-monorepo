/*
 * File: Simulator.cs
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
    /// Link for AUTD Simulator
    /// </summary>
    public sealed class Simulator : Internal.Link
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
            Ptr = NativeMethods.LinkSimulator.AUTDLinkSimulatorWithAddr(Ptr, addr.ToString(), err);
            if (Ptr._0 == IntPtr.Zero)
                throw new AUTDException(err);
            return this;
        }

        public Simulator WithTimeout(TimeSpan timeout)
        {
            Ptr = NativeMethods.LinkSimulator.AUTDLinkSimulatorWithTimeout(Ptr, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
            return this;
        }
    }
}
