/*
 * File: Audit.cs
 * Project: Link
 * Created Date: 22/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */


using System;
using System.Runtime.InteropServices;
using AUTD3Sharp.Internal;
using AUTD3Sharp.NativeMethods;

namespace AUTD3Sharp.Link
{
    public sealed class Audit : Internal.Link
    {
        public Audit() : base(NativeMethods.Base.AUTDLinkAudit())
        {
        }

        public Audit WithTimeout(TimeSpan timeout)
        {
            Ptr = NativeMethods.Base.AUTDLinkAuditWithTimeout(Ptr, (ulong)(timeout.TotalMilliseconds * 1000 * 1000));
            return this;
        }

        public static void Down(Controller cnt)
        {
            NativeMethods.Base.AUTDLinkAuditDown(Base.AUTDAuditLinkGet(cnt.Ptr));
        }

        public static bool IsOpen(Controller cnt)
        {
            return NativeMethods.Base.AUTDLinkAuditIsOpen(Base.AUTDAuditLinkGet(cnt.Ptr));
        }

        public static ulong LastTimeoutNs(Controller cnt)
        {
            return NativeMethods.Base.AUTDLinkAuditLastTimeoutNs(Base.AUTDAuditLinkGet(cnt.Ptr));
        }

        public static void Up(Controller cnt)
        {
            NativeMethods.Base.AUTDLinkAuditUp(Base.AUTDAuditLinkGet(cnt.Ptr));
        }

        public static void BreakDown(Controller cnt)
        {
            NativeMethods.Base.AUTDLinkAuditBreakDown(Base.AUTDAuditLinkGet(cnt.Ptr));
        }

        public static void Update(Controller cnt, int idx)
        {
            NativeMethods.Base.AUTDLinkAuditCpuUpdate(Base.AUTDAuditLinkGet(cnt.Ptr), (uint)idx);
        }

        public static int FpgaFlags(Controller cnt, int idx)
        {
            return NativeMethods.Base.AUTDLinkAuditCpuFpgaFlags(Base.AUTDAuditLinkGet(cnt.Ptr), (uint)idx);
        }

        public static bool IsLegacy(Controller cnt, int idx)
        {
            return NativeMethods.Base.AUTDLinkAuditFpgaIsLegacyMode(Base.AUTDAuditLinkGet(cnt.Ptr), (uint)idx);
        }

        public static int SilencerStep(Controller cnt, int idx)
        {
            return NativeMethods.Base.AUTDLinkAuditFpgaSilencerStep(Base.AUTDAuditLinkGet(cnt.Ptr), (uint)idx);
        }

        public static void AssertThermalSensor(Controller cnt, int idx)
        {
            NativeMethods.Base.AUTDLinkAuditFpgaAssertThermalSensor(Base.AUTDAuditLinkGet(cnt.Ptr), (uint)idx);
        }

        public static void DeassertThermalSensor(Controller cnt, int idx)
        {
            NativeMethods.Base.AUTDLinkAuditFpgaDeassertThermalSensor(Base.AUTDAuditLinkGet(cnt.Ptr), (uint)idx);
        }

        public static byte[] Modulation(Controller cnt, int idx)
        {
            var n = (int)NativeMethods.Base.AUTDLinkAuditFpgaModulationCycle(Base.AUTDAuditLinkGet(cnt.Ptr), (uint)idx);
            var buf = new byte[n];
            NativeMethods.Base.AUTDLinkAuditFpgaModulation(Base.AUTDAuditLinkGet(cnt.Ptr), (uint)idx, buf);
            return buf;
        }

        public static uint ModulationFrequencyDivision(Controller cnt, int idx)
        {
            return NativeMethods.Base.AUTDLinkAuditFpgaModulationFrequencyDivision(Base.AUTDAuditLinkGet(cnt.Ptr), (uint)idx);
        }

        public static ushort[] Cycles(Controller cnt, int idx)
        {
            var n = (int)NativeMethods.Base.AUTDLinkAuditCpuNumTransducers(Base.AUTDAuditLinkGet(cnt.Ptr), (uint)idx);
            var buf = new ushort[n];
            NativeMethods.Base.AUTDLinkAuditFpgaCycles(Base.AUTDAuditLinkGet(cnt.Ptr), (uint)idx, buf);
            return buf;
        }

        public static ushort[] ModDelays(Controller cnt, int idx)
        {
            var n = (int)NativeMethods.Base.AUTDLinkAuditCpuNumTransducers(Base.AUTDAuditLinkGet(cnt.Ptr), (uint)idx);
            var buf = new ushort[n];
            NativeMethods.Base.AUTDLinkAuditFpgaModDelays(Base.AUTDAuditLinkGet(cnt.Ptr), (uint)idx, buf);
            return buf;
        }

        public static short[] DutyFilters(Controller cnt, int idx)
        {
            var n = (int)NativeMethods.Base.AUTDLinkAuditCpuNumTransducers(Base.AUTDAuditLinkGet(cnt.Ptr), (uint)idx);
            var buf = new short[n];
            NativeMethods.Base.AUTDLinkAuditFpgaDutyFilters(Base.AUTDAuditLinkGet(cnt.Ptr), (uint)idx, buf);
            return buf;
        }

        public static short[] PhaseFilters(Controller cnt, int idx)
        {
            var n = (int)NativeMethods.Base.AUTDLinkAuditCpuNumTransducers(Base.AUTDAuditLinkGet(cnt.Ptr), (uint)idx);
            var buf = new short[n];
            NativeMethods.Base.AUTDLinkAuditFpgaPhaseFilters(Base.AUTDAuditLinkGet(cnt.Ptr), (uint)idx, buf);
            return buf;
        }

        public static (ushort[], ushort[]) DutiesAndPhases(Controller cnt, int idx, int stmIdx)
        {
            var n = (int)NativeMethods.Base.AUTDLinkAuditCpuNumTransducers(Base.AUTDAuditLinkGet(cnt.Ptr), (uint)idx);
            var duties = new ushort[n];
            var phases = new ushort[n];
            NativeMethods.Base.AUTDLinkAuditFpgaDutiesAndPhases(Base.AUTDAuditLinkGet(cnt.Ptr), (uint)idx, (uint)stmIdx, duties, phases);
            return (duties, phases);
        }

        public static uint StmCycle(Controller cnt, int idx)
        {
            return NativeMethods.Base.AUTDLinkAuditFpgaStmCycle(Base.AUTDAuditLinkGet(cnt.Ptr), (uint)idx);
        }

        public static bool IsStmGainMode(Controller cnt, int idx)
        {
            return NativeMethods.Base.AUTDLinkAuditFpgaIsStmGainMode(Base.AUTDAuditLinkGet(cnt.Ptr), (uint)idx);
        }

        public static uint StmFrequencyDivision(Controller cnt, int idx)
        {
            return NativeMethods.Base.AUTDLinkAuditFpgaStmFrequencyDivision(Base.AUTDAuditLinkGet(cnt.Ptr), (uint)idx);
        }

        public static int StmStartIdx(Controller cnt, int idx)
        {
            return NativeMethods.Base.AUTDLinkAuditFpgaStmStartIdx(Base.AUTDAuditLinkGet(cnt.Ptr), (uint)idx);
        }

        public static int StmFinishIdx(Controller cnt, int idx)
        {
            return NativeMethods.Base.AUTDLinkAuditFpgaStmFinishIdx(Base.AUTDAuditLinkGet(cnt.Ptr), (uint)idx);
        }
    }
}
