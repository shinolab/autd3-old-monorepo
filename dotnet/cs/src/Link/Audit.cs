/*
 * File: Audit.cs
 * Project: Link
 * Created Date: 22/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */


using System;

namespace AUTD3Sharp.Link
{
    public sealed class Audit : Internal.ILink<Audit>
    {
        public sealed class AuditBuilder : Internal.ILinkBuilder
        {
            private LinkAuditBuilderPtr _ptr;

            internal AuditBuilder()
            {
                _ptr = NativeMethods.Base.AUTDLinkAudit();
            }

            public AuditBuilder WithTimeout(TimeSpan timeout)
            {
                _ptr = NativeMethods.Base.AUTDLinkAuditWithTimeout(_ptr, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
                return this;
            }

            public LinkBuilderPtr Ptr()
            {
                return NativeMethods.Base.AUTDLinkAuditIntoBuilder(_ptr);
            }
        }

        private LinkPtr _ptr = new LinkPtr { _0 = IntPtr.Zero };

        public static AuditBuilder Builder()
        {
            return new AuditBuilder();
        }

        public void Down()
        {
            NativeMethods.Base.AUTDLinkAuditDown(_ptr);
        }

        public bool IsOpen()
        {
            return NativeMethods.Base.AUTDLinkAuditIsOpen(_ptr);
        }

        public ulong LastTimeoutNs()
        {
            return NativeMethods.Base.AUTDLinkAuditLastTimeoutNs(_ptr);
        }

        public void Up()
        {
            NativeMethods.Base.AUTDLinkAuditUp(_ptr);
        }

        public void BreakDown()
        {
            NativeMethods.Base.AUTDLinkAuditBreakDown(_ptr);
        }

        public void Update(int idx)
        {
            NativeMethods.Base.AUTDLinkAuditCpuUpdate(_ptr, (uint)idx);
        }

        public int FpgaFlags(int idx)
        {
            return NativeMethods.Base.AUTDLinkAuditCpuFpgaFlags(_ptr, (uint)idx);
        }

        public bool IsLegacy(int idx)
        {
            return NativeMethods.Base.AUTDLinkAuditFpgaIsLegacyMode(_ptr, (uint)idx);
        }

        public int SilencerStep(int idx)
        {
            return NativeMethods.Base.AUTDLinkAuditFpgaSilencerStep(_ptr, (uint)idx);
        }

        public void AssertThermalSensor(int idx)
        {
            NativeMethods.Base.AUTDLinkAuditFpgaAssertThermalSensor(_ptr, (uint)idx);
        }

        public void DeassertThermalSensor(int idx)
        {
            NativeMethods.Base.AUTDLinkAuditFpgaDeassertThermalSensor(_ptr, (uint)idx);
        }

        public byte[] Modulation(int idx)
        {
            var n = (int)NativeMethods.Base.AUTDLinkAuditFpgaModulationCycle(_ptr, (uint)idx);
            var buf = new byte[n];
            NativeMethods.Base.AUTDLinkAuditFpgaModulation(_ptr, (uint)idx, buf);
            return buf;
        }

        public uint ModulationFrequencyDivision(int idx)
        {
            return NativeMethods.Base.AUTDLinkAuditFpgaModulationFrequencyDivision(_ptr, (uint)idx);
        }

        public ushort[] Cycles(int idx)
        {
            var n = (int)NativeMethods.Base.AUTDLinkAuditCpuNumTransducers(_ptr, (uint)idx);
            var buf = new ushort[n];
            NativeMethods.Base.AUTDLinkAuditFpgaCycles(_ptr, (uint)idx, buf);
            return buf;
        }

        public ushort[] ModDelays(int idx)
        {
            var n = (int)NativeMethods.Base.AUTDLinkAuditCpuNumTransducers(_ptr, (uint)idx);
            var buf = new ushort[n];
            NativeMethods.Base.AUTDLinkAuditFpgaModDelays(_ptr, (uint)idx, buf);
            return buf;
        }

        public short[] DutyFilters(int idx)
        {
            var n = (int)NativeMethods.Base.AUTDLinkAuditCpuNumTransducers(_ptr, (uint)idx);
            var buf = new short[n];
            NativeMethods.Base.AUTDLinkAuditFpgaDutyFilters(_ptr, (uint)idx, buf);
            return buf;
        }

        public short[] PhaseFilters(int idx)
        {
            var n = (int)NativeMethods.Base.AUTDLinkAuditCpuNumTransducers(_ptr, (uint)idx);
            var buf = new short[n];
            NativeMethods.Base.AUTDLinkAuditFpgaPhaseFilters(_ptr, (uint)idx, buf);
            return buf;
        }

        public (ushort[], ushort[]) DutiesAndPhases(int idx, int stmIdx)
        {
            var n = (int)NativeMethods.Base.AUTDLinkAuditCpuNumTransducers(_ptr, (uint)idx);
            var duties = new ushort[n];
            var phases = new ushort[n];
            NativeMethods.Base.AUTDLinkAuditFpgaDutiesAndPhases(_ptr, (uint)idx, (uint)stmIdx, duties, phases);
            return (duties, phases);
        }

        public uint StmCycle(int idx)
        {
            return NativeMethods.Base.AUTDLinkAuditFpgaStmCycle(_ptr, (uint)idx);
        }

        public bool IsStmGainMode(int idx)
        {
            return NativeMethods.Base.AUTDLinkAuditFpgaIsStmGainMode(_ptr, (uint)idx);
        }

        public uint StmFrequencyDivision(int idx)
        {
            return NativeMethods.Base.AUTDLinkAuditFpgaStmFrequencyDivision(_ptr, (uint)idx);
        }

        public int StmStartIdx(int idx)
        {
            return NativeMethods.Base.AUTDLinkAuditFpgaStmStartIdx(_ptr, (uint)idx);
        }

        public int StmFinishIdx(int idx)
        {
            return NativeMethods.Base.AUTDLinkAuditFpgaStmFinishIdx(_ptr, (uint)idx);
        }

        public Audit Create(LinkPtr ptr)
        {
            return new Audit
            {
                _ptr = ptr
            };
        }
    }
}
