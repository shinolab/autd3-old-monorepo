/*
 * File: FIR.cs
 * Project: Modulation
 * Created Date: 12/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 07/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */


#if UNITY_2018_3_OR_NEWER
#define USE_SINGLE
using Math = UnityEngine.Mathf;
#endif

#if USE_SINGLE
using float_t = System.Single;
#else
using float_t = System.Double;
#endif

namespace AUTD3Sharp.Modulation
{
    public sealed class Lpf : Internal.Modulation
    {
        private readonly Internal.Modulation _m;
        private readonly uint _nTaps;
        private readonly float_t _cutoff;

        public Lpf(Internal.Modulation m, uint nTaps, float_t cutoff)
        {
            _m = m;
            _nTaps = nTaps;
            _cutoff = cutoff;
        }

        internal override ModulationPtr ModulationPtr()
        {
            return NativeMethodsBase.AUTDModulationWithLowPass(_m.ModulationPtr(), _nTaps, _cutoff);
        }
    }


    public sealed class Hpf : Internal.Modulation
    {
        private readonly Internal.Modulation _m;
        private readonly uint _nTaps;
        private readonly float_t _cutoff;

        public Hpf(Internal.Modulation m, uint nTaps, float_t cutoff)
        {
            _m = m;
            _nTaps = nTaps;
            _cutoff = cutoff;
        }

        internal override ModulationPtr ModulationPtr()
        {
            return NativeMethodsBase.AUTDModulationWithHighPass(_m.ModulationPtr(), _nTaps, _cutoff);
        }
    }

    public sealed class Bpf : Internal.Modulation
    {
        private readonly Internal.Modulation _m;
        private readonly uint _nTaps;
        private readonly float_t _fLow;
        private readonly float_t _fHigh;

        public Bpf(Internal.Modulation m, uint nTaps, float_t fLow, float_t fHigh)
        {
            _m = m;
            _nTaps = nTaps;
            _fLow = fLow;
            _fHigh = fHigh;
        }

        internal override ModulationPtr ModulationPtr()
        {
            return NativeMethodsBase.AUTDModulationWithBandPass(_m.ModulationPtr(), _nTaps, _fLow, _fHigh);
        }
    }

    public sealed class Bsf : Internal.Modulation
    {
        private readonly Internal.Modulation _m;
        private readonly uint _nTaps;
        private readonly float_t _fLow;
        private readonly float_t _fHigh;

        public Bsf(Internal.Modulation m, uint nTaps, float_t fLow, float_t fHigh)
        {
            _m = m;
            _nTaps = nTaps;
            _fLow = fLow;
            _fHigh = fHigh;
        }

        internal override ModulationPtr ModulationPtr()
        {
            return NativeMethodsBase.AUTDModulationWithBandStop(_m.ModulationPtr(), _nTaps, _fLow, _fHigh);
        }
    }

    public static class FirModulationExtensions
    {
        public static Lpf WithLowPass(this Internal.Modulation s, uint nTaps, float_t cutoff)
        {
            return new Lpf(s, nTaps, cutoff);
        }

        public static Hpf WithHighPass(this Internal.Modulation s, uint nTaps, float_t cutoff)
        {
            return new Hpf(s, nTaps, cutoff);
        }

        public static Bpf WithBandPass(this Internal.Modulation s, uint nTaps, float_t fLow, float_t fHigh)
        {
            return new Bpf(s, nTaps, fLow, fHigh);
        }

        public static Bsf WithBandStop(this Internal.Modulation s, uint nTaps, float_t fLow, float_t fHigh)
        {
            return new Bsf(s, nTaps, fLow, fHigh);
        }
    }
}
