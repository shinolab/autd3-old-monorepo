// This file was automatically generated from header file
using System;
using System.Runtime.InteropServices;
            
namespace AUTD3Sharp.NativeMethods
{
    internal static class LinkRemoteTwinCAT
    {
        const string DLL = "autd3capi-link-remote-twincat";

        [DllImport(DLL, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true, CallingConvention = CallingConvention.Cdecl)] public static extern void AUTDLinkRemoteTwinCAT(out IntPtr @out, string serverIpAddr, string serverAmsNetId, string clientAmsNetId);
    }
}
