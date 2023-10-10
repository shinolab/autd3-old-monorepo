/*
 * File: Simulator.cs
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
    /// Link for AUTD Simulator
    /// </summary>
    public class Simulator
    {
        public sealed class SimulatorBuilder : Internal.ILinkBuilder
        {
            private LinkSimulatorBuilderPtr _ptr;

            internal SimulatorBuilder(ushort port)
            {
                _ptr = NativeMethods.LinkSimulator.AUTDLinkSimulator(port);
            }

            /// <summary>
            /// Set server IP address
            /// </summary>
            /// <param name="addr"></param>
            /// <returns></returns>
            /// <exception cref="AUTDException"></exception>
            public SimulatorBuilder WithServerIp(IPAddress addr)
            {
                var err = new byte[256];
                _ptr = NativeMethods.LinkSimulator.AUTDLinkSimulatorWithAddr(_ptr, addr.ToString(), err);
                if (_ptr._0 == IntPtr.Zero)
                    throw new AUTDException(err);
                return this;
            }

            public SimulatorBuilder WithTimeout(TimeSpan timeout)
            {
                _ptr = NativeMethods.LinkSimulator.AUTDLinkSimulatorWithTimeout(_ptr, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
                return this;
            }

            public LinkBuilderPtr Ptr()
            {
                return NativeMethods.LinkSimulator.AUTDLinkSimulatorIntoBuilder(_ptr);
            }
        }

        public static SimulatorBuilder Builder(ushort port)
        {
            return new SimulatorBuilder(port);
        }
    }
}
