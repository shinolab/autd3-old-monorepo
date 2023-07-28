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
        internal static class LinkSimulator
        {
            private const string DLL = "autd3capi_link_simulator";

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern LinkPtr AUTDLinkSimulator(ushort port);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern LinkPtr AUTDLinkSimulatorAddr(LinkPtr simulator, string addr, byte[] err);

            [DllImport(DLL, CallingConvention = CallingConvention.Cdecl)] public static extern LinkPtr AUTDLinkSimulatorTimeout(LinkPtr simulator, ulong timeoutNs);
        }
    }

}

#if UNITY_2020_2_OR_NEWER
#nullable disable
#endif


