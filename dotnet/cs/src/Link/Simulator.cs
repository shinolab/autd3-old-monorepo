/*
 * File: Simulator.cs
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

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

namespace AUTD3Sharp.Link
{
    /// <summary>
    /// Link for AUTD Simulator
    /// </summary>
    public sealed class Simulator : Internal.ILink<Simulator>
    {
        public sealed class SimulatorBuilder : Internal.ILinkBuilder
        {
            private LinkSimulatorBuilderPtr _ptr;

            internal SimulatorBuilder(ushort port)
            {
                _ptr = NativeMethodsLinkSimulator.AUTDLinkSimulator(port);
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
                var addrStr = addr.ToString();
                var addrBytes = System.Text.Encoding.UTF8.GetBytes(addrStr);
                unsafe
                {
                    fixed (byte* ep = err)
                    fixed (byte* ap = addrBytes)
                    {
                        _ptr = NativeMethodsLinkSimulator.AUTDLinkSimulatorWithAddr(_ptr, ap, ep);
                        if (_ptr.Item1 == IntPtr.Zero)
                            throw new AUTDException(err);
                    }
                }
                return this;
            }

            public SimulatorBuilder WithTimeout(TimeSpan timeout)
            {
                _ptr = NativeMethodsLinkSimulator.AUTDLinkSimulatorWithTimeout(_ptr, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
                return this;
            }

            LinkBuilderPtr Internal.ILinkBuilder.Ptr()
            {
                return NativeMethodsLinkSimulator.AUTDLinkSimulatorIntoBuilder(_ptr);
            }
        }

        public static SimulatorBuilder Builder(ushort port)
        {
            return new SimulatorBuilder(port);
        }

        private LinkPtr _ptr = new LinkPtr { Item1 = IntPtr.Zero };

        public void UpdateGeometry(Geometry geometry)
        {
            var err = new byte[256];
            unsafe
            {
                fixed (byte* ep = err)
                {
                    if (NativeMethodsLinkSimulator.AUTDLinkSimulatorUpdateGeometry(_ptr, geometry.Ptr, ep) == NativeMethodsDef.AUTD3_ERR)
                        throw new AUTDException(err);
                }
            }
        }

        Simulator Internal.ILink<Simulator>.Create(LinkPtr ptr, object? _)
        {
            return new Simulator
            {
                _ptr = ptr,
            };
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
