/*
 * File: Audit.cs
 * Project: Link
 * Created Date: 22/09/2023
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

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

namespace AUTD3Sharp.Link
{
    public sealed class Audit
    {
        public sealed class AuditBuilder : Internal.ILinkBuilder<Audit>
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

        public ulong LastTimeoutNs()
        {
            return NativeMethodsBase.AUTDLinkAuditLastTimeoutNs(_ptr);
        }

        public void Up()
        {
            NativeMethodsBase.AUTDLinkAuditUp(_ptr);
        }

        public void BreakDown()
        {
            NativeMethodsBase.AUTDLinkAuditBreakDown(_ptr);
        }

        public void Update(int idx)
        {
            NativeMethodsBase.AUTDLinkAuditCpuUpdate(_ptr, (uint)idx);
        }

        public int FpgaFlags(int idx)
        {
            return NativeMethodsBase.AUTDLinkAuditCpuFpgaFlags(_ptr, (uint)idx);
        }

        public int SilencerStep(int idx)
        {
            return NativeMethodsBase.AUTDLinkAuditFpgaSilencerStep(_ptr, (uint)idx);
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
                fixed (byte* p = buf)
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
                fixed (ushort* p = buf)
                    NativeMethodsBase.AUTDLinkAuditFpgaModDelays(_ptr, (uint)idx, p);
            }
            return buf;
        }

        public short[] DutyFilters(int idx)
        {
            var n = (int)NativeMethodsBase.AUTDLinkAuditCpuNumTransducers(_ptr, (uint)idx);
            var buf = new short[n];
            unsafe
            {
                fixed (short* p = buf)
                    NativeMethodsBase.AUTDLinkAuditFpgaDutyFilters(_ptr, (uint)idx, p);
            }
            return buf;
        }

        public short[] PhaseFilters(int idx)
        {
            var n = (int)NativeMethodsBase.AUTDLinkAuditCpuNumTransducers(_ptr, (uint)idx);
            var buf = new short[n];
            unsafe
            {
                fixed (short* p = buf)
                    NativeMethodsBase.AUTDLinkAuditFpgaPhaseFilters(_ptr, (uint)idx, p);
            }
            return buf;
        }

        public (ushort[], ushort[]) DutiesAndPhases(int idx, int stmIdx)
        {
            var n = (int)NativeMethodsBase.AUTDLinkAuditCpuNumTransducers(_ptr, (uint)idx);
            var duties = new ushort[n];
            var phases = new ushort[n];
            unsafe
            {
                fixed (ushort* pd = duties)
                fixed (ushort* pp = phases)
                {
                    NativeMethodsBase.AUTDLinkAuditFpgaDutiesAndPhases(_ptr, (uint)idx, (uint)stmIdx, pd, pp);
                }
            }
            return (duties, phases);
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
