/*
 * File: Audit.cs
 * Project: Link
 * Created Date: 22/09/2023
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

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

namespace AUTD3Sharp.Link
{
    public sealed class Audit
    {
        public sealed class AuditBuilder : ILinkBuilder<Audit>
        {
            private LinkAuditBuilderPtr _ptr;

            internal AuditBuilder()
            {
                _ptr = NativeMethodsBase.AUTDLinkAudit();
            }

            public AuditBuilder WithTimeout(TimeSpan timeout)
            {
                _ptr = NativeMethodsBase.AUTDLinkAuditWithTimeout(_ptr, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
                return this;
            }

            LinkBuilderPtr ILinkBuilder<Audit>.Ptr()
            {
                return NativeMethodsBase.AUTDLinkAuditIntoBuilder(_ptr);
            }

            Audit ILinkBuilder<Audit>.ResolveLink(LinkPtr ptr)
            {
                return new Audit
                {
                    _ptr = ptr
                };
            }
        }

        private LinkPtr _ptr = new LinkPtr { Item1 = IntPtr.Zero };

        public static AuditBuilder Builder()
        {
            return new AuditBuilder();
        }

        public void Down()
        {
            NativeMethodsBase.AUTDLinkAuditDown(_ptr);
        }

        public bool IsOpen()
        {
            return NativeMethodsBase.AUTDLinkAuditIsOpen(_ptr);
        }

        public bool IsForceFan(int idx)
        {
            return NativeMethodsBase.AUTDLinkAuditFpgaIsForceFan(_ptr, (uint)idx);
        }

        public ulong LastTimeoutNs()
        {
            return NativeMethodsBase.AUTDLinkAuditLastTimeoutNs(_ptr);
        }

        public void BreakDown()
        {
            NativeMethodsBase.AUTDLinkAuditBreakDown(_ptr);
        }

        public void Update(int idx)
        {
            NativeMethodsBase.AUTDLinkAuditCpuUpdate(_ptr, (uint)idx);
        }

        public ushort SilencerStepIntensity(int idx)
        {
            return NativeMethodsBase.AUTDLinkAuditFpgaSilencerStepIntensity(_ptr, (uint)idx);
        }

        public ushort SilencerStepPhase(int idx)
        {
            return NativeMethodsBase.AUTDLinkAuditFpgaSilencerStepPhase(_ptr, (uint)idx);
        }

        public byte DebugOutputIdx(int idx)
        {
            return NativeMethodsBase.AUTDLinkAuditFpgaDebugOutputIdx(_ptr, (uint)idx);
        }

        public void AssertThermalSensor(int idx)
        {
            NativeMethodsBase.AUTDLinkAuditFpgaAssertThermalSensor(_ptr, (uint)idx);
        }

        public void DeassertThermalSensor(int idx)
        {
            NativeMethodsBase.AUTDLinkAuditFpgaDeassertThermalSensor(_ptr, (uint)idx);
        }

        public byte[] Modulation(int idx)
        {
            var n = (int)NativeMethodsBase.AUTDLinkAuditFpgaModulationCycle(_ptr, (uint)idx);
            var buf = new byte[n];
            unsafe
            {
                fixed (byte* p = &buf[0])
                    NativeMethodsBase.AUTDLinkAuditFpgaModulation(_ptr, (uint)idx, p);
            }
            return buf;
        }

        public uint ModulationFrequencyDivision(int idx)
        {
            return NativeMethodsBase.AUTDLinkAuditFpgaModulationFrequencyDivision(_ptr, (uint)idx);
        }

        public ushort[] ModDelays(int idx)
        {
            var n = (int)NativeMethodsBase.AUTDLinkAuditCpuNumTransducers(_ptr, (uint)idx);
            var buf = new ushort[n];
            unsafe
            {
                fixed (ushort* p = &buf[0])
                    NativeMethodsBase.AUTDLinkAuditFpgaModDelays(_ptr, (uint)idx, p);
            }
            return buf;
        }

        public (byte[], byte[]) IntensitiesAndPhases(int idx, int stmIdx)
        {
            var n = (int)NativeMethodsBase.AUTDLinkAuditCpuNumTransducers(_ptr, (uint)idx);
            var intensities = new byte[n];
            var phases = new byte[n];
            unsafe
            {
                fixed (byte* pd = &intensities[0])
                fixed (byte* pp = &phases[0])
                {
                    NativeMethodsBase.AUTDLinkAuditFpgaIntensitiesAndPhases(_ptr, (uint)idx, (uint)stmIdx, pd, pp);
                }
            }
            return (intensities, phases);
        }

        public uint StmCycle(int idx)
        {
            return NativeMethodsBase.AUTDLinkAuditFpgaStmCycle(_ptr, (uint)idx);
        }

        public bool IsStmGainMode(int idx)
        {
            return NativeMethodsBase.AUTDLinkAuditFpgaIsStmGainMode(_ptr, (uint)idx);
        }

        public uint StmFrequencyDivision(int idx)
        {
            return NativeMethodsBase.AUTDLinkAuditFpgaStmFrequencyDivision(_ptr, (uint)idx);
        }

        public int StmStartIdx(int idx)
        {
            return NativeMethodsBase.AUTDLinkAuditFpgaStmStartIdx(_ptr, (uint)idx);
        }

        public int StmFinishIdx(int idx)
        {
            return NativeMethodsBase.AUTDLinkAuditFpgaStmFinishIdx(_ptr, (uint)idx);
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
