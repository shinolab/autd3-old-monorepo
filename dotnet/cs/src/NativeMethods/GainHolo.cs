// This file was automatically generated from header file
using System;
using System.Runtime.InteropServices;

namespace AUTD3Sharp
{
    namespace NativeMethods
    {
        internal static class GainHolo
        {
            private const string DLL = "autd3capi-gain-holo";

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDDefaultBackend();

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDGainHoloSDP(IntPtr backend);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloSDPAlpha(IntPtr holo, double alpha);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloSDPLambda(IntPtr holo, double lambda);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloSDPRepeat(IntPtr holo, uint repeat);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDGainHoloEVP(IntPtr backend);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloEVPGamma(IntPtr holo, double gamma);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDGainHoloGS(IntPtr backend);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloGSRepeat(IntPtr holo, uint repeat);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDGainHoloGSPAT(IntPtr backend);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloGSPATRepeat(IntPtr holo, uint repeat);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDGainHoloNaive(IntPtr backend);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDGainHoloGreedy();

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloGreedyPhaseDiv(IntPtr holo, uint div);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern IntPtr AUTDGainHoloLM(IntPtr backend);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloLMEps1(IntPtr holo, double eps1);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloLMEps2(IntPtr holo, double eps2);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloLMTau(IntPtr holo, double tau);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloLMKMax(IntPtr holo, uint kMax);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloLMInitial(IntPtr holo, double[]? ptr, ulong len);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloAdd(IntPtr holo, double x, double y, double z, double amp);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloSetDotCareConstraint(IntPtr holo);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloSetNormalizeConstraint(IntPtr holo);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloSetUniformConstraint(IntPtr holo, double value);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDGainHoloSetClampConstraint(IntPtr holo, double min, double max);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDDeleteGainHolo(IntPtr holo);
    }
    }
}
