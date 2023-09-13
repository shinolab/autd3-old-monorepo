#if UNITY_2018_3_OR_NEWER
#define USE_SINGLE
#endif

using System.Collections.Generic;
using System.Linq;

#if UNITY_2020_2_OR_NEWER
#nullable enable
#endif

#if UNITY_2018_3_OR_NEWER
using UnityEngine;
using Vector3 = UnityEngine.Vector3;
#else
using Vector3 = AUTD3Sharp.Utils.Vector3d;
#endif

#if USE_SINGLE
using float_t = System.Single;
#else
using float_t = System.Double;
#endif

namespace AUTD3Sharp.Gain.Holo
{
    /// <summary>
    /// Gain to produce multiple foci with GS-PAT algorithm
    /// </summary>
    /// <remarks>Diego Martinez Plasencia et al. "GS-pat: high-speed multi-point sound-fields for phased arrays of transducers," ACMTrans-actions on Graphics (TOG), 39(4):138–1, 2020.</remarks>
    /// <typeparam name="TB">Backend</typeparam>
    public sealed class GSPAT<TB> : Internal.Gain
        where TB : Backend
    {
        private readonly TB _backend;
        private readonly List<float_t> _foci;
        private readonly List<float_t> _amps;
        private uint? _repeat;
        private IAmplitudeConstraint? _constraint;

        public GSPAT(TB backend)
        {
            _backend = backend;
            _foci = new List<float_t>();
            _amps = new List<float_t>();
        }

        public GSPAT<TB> AddFocus(Vector3 focus, float_t amp)
        {
            _foci.Add(focus.x);
            _foci.Add(focus.y);
            _foci.Add(focus.z);
            _amps.Add(amp);
            return this;
        }

        /// <summary>
        /// Add foci
        /// </summary>
        /// <param name="iter">Enumerable of foci and amps</param>
        public GSPAT<TB> AddFociFromIter(IEnumerable<(Vector3, float_t)> iter)
        {
            return iter.Aggregate(this, (holo, point) => holo.AddFocus(point.Item1, point.Item2));
        }

        /// <summary>
        /// Parameter. See the paper for details.
        /// </summary>
        /// <param name="value"></param>
        /// <returns></returns>
        public GSPAT<TB> WithRepeat(uint value)
        {
            _repeat = value;
            return this;
        }

        /// <summary>
        /// Set amplitude constraint
        /// </summary>
        /// <param name="constraint"></param>
        /// <returns></returns>
        public GSPAT<TB> WithConstraint(IAmplitudeConstraint constraint)
        {
            _constraint = constraint;
            return this;
        }

        public override GainPtr GainPtr(Geometry geometry)
        {
            var ptr = _backend.Gspat(_foci.ToArray(), _amps.ToArray(),
                (ulong)_amps.Count);
            if (_repeat.HasValue) ptr = _backend.GspatWithRepeat(ptr, _repeat.Value);
            if (_constraint != null) ptr = _backend.GspatWithConstraint(ptr, _constraint.Ptr());
            return ptr;
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
