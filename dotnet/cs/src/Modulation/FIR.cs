/*
 * File: FIR.cs
 * Project: Modulation
 * Created Date: 12/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/10/2023
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
    using Base = NativeMethods.Base;

    public sealed class LPF : Internal.Modulation
    {
        private readonly Internal.Modulation _m;
        private readonly uint _nTaps;
        private readonly float_t _cutoff;

        public LPF(Internal.Modulation m, uint nTaps, float_t cutoff)
        {
            _m = m;
            _nTaps = nTaps;
            _cutoff = cutoff;
        }

        public override ModulationPtr ModulationPtr()
        {
            return Base.AUTDModulationWithLowPass(_m.ModulationPtr(), _nTaps, _cutoff);
        }
    }


    public sealed class HPF : Internal.Modulation
    {
        private readonly Internal.Modulation _m;
        private readonly uint _nTaps;
        private readonly float_t _cutoff;

        public HPF(Internal.Modulation m, uint nTaps, float_t cutoff)
        {
            _m = m;
            _nTaps = nTaps;
            _cutoff = cutoff;
        }

        public override ModulationPtr ModulationPtr()
        {
            return Base.AUTDModulationWithHighPass(_m.ModulationPtr(), _nTaps, _cutoff);
        }
    }

    public sealed class BPF : Internal.Modulation
    {
        private readonly Internal.Modulation _m;
        private readonly uint _nTaps;
        private readonly float_t _fLow;
        private readonly float_t _fHigh;

        public BPF(Internal.Modulation m, uint nTaps, float_t fLow, float_t fHigh)
        {
            _m = m;
            _nTaps = nTaps;
            _fLow = fLow;
            _fHigh = fHigh;
        }

        public override ModulationPtr ModulationPtr()
        {
            return Base.AUTDModulationWithBandPass(_m.ModulationPtr(), _nTaps, _fLow, _fHigh);
        }
    }

    public sealed class BSF : Internal.Modulation
    {
        private readonly Internal.Modulation _m;
        private readonly uint _nTaps;
        private readonly float_t _fLow;
        private readonly float_t _fHigh;

        public BSF(Internal.Modulation m, uint nTaps, float_t fLow, float_t fHigh)
        {
            _m = m;
            _nTaps = nTaps;
            _fLow = fLow;
            _fHigh = fHigh;
        }

        public override ModulationPtr ModulationPtr()
        {
            return Base.AUTDModulationWithBandStop(_m.ModulationPtr(), _nTaps, _fLow, _fHigh);
        }
    }

    public static class FIRModulationExtensions
    {
        public static LPF WithLowPass(this Internal.Modulation s, uint nTaps, float_t cutoff)
        {
            return new LPF(s, nTaps, cutoff);
        }

        public static HPF WithHighPass(this Internal.Modulation s, uint nTaps, float_t cutoff)
        {
            return new HPF(s, nTaps, cutoff);
        }

        public static BPF WithBandPass(this Internal.Modulation s, uint nTaps, float_t fLow, float_t fHigh)
        {
            return new BPF(s, nTaps, fLow, fHigh);
        }

        public static BSF WithBandStop(this Internal.Modulation s, uint nTaps, float_t fLow, float_t fHigh)
        {
            return new BSF(s, nTaps, fLow, fHigh);
        }
    }
}
