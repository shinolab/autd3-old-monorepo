// <auto-generated>
// This code is generated by csbindgen.
// DON'T CHANGE THIS DIRECTLY.
// </auto-generated>
#pragma warning disable CS8500
#pragma warning disable CS8981
using System;
using System.Runtime.InteropServices;


namespace AUTD3Sharp
{
    internal static unsafe partial class NativeMethodsModulationAudioFile
    {
        const string __DllName = "autd3capi_modulation_audio_file";



        [DllImport(__DllName, EntryPoint = "AUTDModulationWav", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ModulationPtr AUTDModulationWav(byte* path, byte* err);

        [DllImport(__DllName, EntryPoint = "AUTDModulationWavWithSamplingFrequencyDivision", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ModulationPtr AUTDModulationWavWithSamplingFrequencyDivision(ModulationPtr m, uint div);

        [DllImport(__DllName, EntryPoint = "AUTDModulationRawPCM", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ModulationPtr AUTDModulationRawPCM(byte* path, uint sample_rate, byte* err);

        [DllImport(__DllName, EntryPoint = "AUTDModulationRawPCMWithSamplingFrequencyDivision", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ModulationPtr AUTDModulationRawPCMWithSamplingFrequencyDivision(ModulationPtr m, uint div);


    }



}
    