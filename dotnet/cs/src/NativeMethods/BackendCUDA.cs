// This file is autogenerated
using System;
using System.Runtime.InteropServices;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

namespace AUTD3Sharp
{
    namespace NativeMethods
    {
        internal static class BackendCUDA
        {
            private const string DLL = "autd3capi_backend_cuda";

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern BackendPtr AUTDCUDABackend(byte[] err);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern void AUTDCUDABackendDelete(BackendPtr backend);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloCUDASDP(BackendPtr backend, double[]? points, double[]? amps, ulong size);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloCUDASDPWithConstraint(GainPtr holo, ConstraintPtr constraint);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloCUDASDPWithAlpha(GainPtr holo, double alpha);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloCUDASDPWithLambda(GainPtr holo, double lambda);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloCUDASDPWithRepeat(GainPtr holo, uint repeat);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloCUDAEVP(BackendPtr backend, double[]? points, double[]? amps, ulong size);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloCUDAEVPWithConstraint(GainPtr holo, ConstraintPtr constraint);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloCUDAEVPWithGamma(GainPtr holo, double gamma);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloCUDAGS(BackendPtr backend, double[]? points, double[]? amps, ulong size);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloCUDAGSWithConstraint(GainPtr holo, ConstraintPtr constraint);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloCUDAGSWithRepeat(GainPtr holo, uint repeat);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloCUDAGSPAT(BackendPtr backend, double[]? points, double[]? amps, ulong size);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloCUDAGSPATWithConstraint(GainPtr holo, ConstraintPtr constraint);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloCUDAGSPATWithRepeat(GainPtr holo, uint repeat);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloCUDANaive(BackendPtr backend, double[]? points, double[]? amps, ulong size);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloCUDANaiveWithConstraint(GainPtr holo, ConstraintPtr constraint);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloCUDAGreedy(double[]? points, double[]? amps, ulong size);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloCUDAGreedyWithConstraint(GainPtr holo, ConstraintPtr constraint);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloCUDAGreedyWithPhaseDiv(GainPtr holo, uint div);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloCUDALM(BackendPtr backend, double[]? points, double[]? amps, ulong size);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloCUDALMWithConstraint(GainPtr holo, ConstraintPtr constraint);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloCUDALMWithEps1(GainPtr holo, double eps);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloCUDALMWithEps2(GainPtr holo, double eps);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloCUDALMWithTau(GainPtr holo, double tau);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloCUDALMWithKMax(GainPtr holo, uint kMax);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public unsafe static extern GainPtr AUTDGainHoloCUDALMWithInitial(GainPtr holo, double[]? initialPtr, ulong len);
        }
    }

}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif


