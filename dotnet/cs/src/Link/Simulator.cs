/*
 * File: Simulator.cs
 * Project: Link
 * Created Date: 20/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 14/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */


using AUTD3Sharp.Internal;
using System;
using System.Net;
using System.Threading.Tasks;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

namespace AUTD3Sharp.Link
{
    /// <summary>
    /// Link for AUTD Simulator
    /// </summary>
    public sealed class Simulator
    {
        public sealed class SimulatorBuilder : Internal.ILinkBuilder<Simulator>
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
                var addrStr = addr.ToString();
                var addrBytes = System.Text.Encoding.UTF8.GetBytes(addrStr);
                unsafe
                {
                    fixed (byte* ap = addrBytes)
                    {
                        var res = NativeMethodsLinkSimulator.AUTDLinkSimulatorWithAddr(_ptr, ap);
                        if (res.result.Item1 != IntPtr.Zero) return this;
                        var err = new byte[res.err_len];
                        fixed (byte* ep = err)
                            NativeMethodsDef.AUTDGetErr(res.err, ep);
                        throw new AUTDException(err);
                    }
                }
            }

            public SimulatorBuilder WithTimeout(TimeSpan timeout)
            {
                _ptr = NativeMethodsLinkSimulator.AUTDLinkSimulatorWithTimeout(_ptr, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
                return this;
            }

            LinkBuilderPtr ILinkBuilder<Simulator>.Ptr()
            {
                return NativeMethodsLinkSimulator.AUTDLinkSimulatorIntoBuilder(_ptr);
            }

            Simulator ILinkBuilder<Simulator>.ResolveLink(LinkPtr ptr)
            {
                return new Simulator
                {
                    _ptr = ptr,
                };
            }
        }

        public static SimulatorBuilder Builder(ushort port)
        {
            return new SimulatorBuilder(port);
        }

        private LinkPtr _ptr = new LinkPtr { Item1 = IntPtr.Zero };

        public async Task UpdateGeometryAsync(Geometry geometry)
        {
            var res = await Task.Run(() => NativeMethodsLinkSimulator.AUTDLinkSimulatorUpdateGeometry(_ptr, geometry.Ptr));
            if (res.result != NativeMethodsDef.AUTD3_ERR) return;
            var err = new byte[res.err_len];
            unsafe
            {
                fixed (byte* p = err)
                    NativeMethodsDef.AUTDGetErr(res.err, p);
                throw new AUTDException(err);
            }
        }

        public void UpdateGeometry(Geometry geometry)
        {
            var res = NativeMethodsLinkSimulator.AUTDLinkSimulatorUpdateGeometry(_ptr, geometry.Ptr);
            if (res.result != NativeMethodsDef.AUTD3_ERR) return;
            var err = new byte[res.err_len];
            unsafe
            {
                fixed (byte* p = err)
                    NativeMethodsDef.AUTDGetErr(res.err, p);
                throw new AUTDException(err);
            }
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
