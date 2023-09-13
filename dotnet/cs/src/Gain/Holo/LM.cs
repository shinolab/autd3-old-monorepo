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
    /// Gain to produce multiple foci with Levenberg-Marquardt algorithm
    /// </summary>
    /// <remarks>
    /// <para>K.Levenberg, “A method for the solution of certain non-linear problems in least squares,” Quarterly of applied mathematics, vol.2, no.2, pp.164–168, 1944.</para> 
    /// <para> D.W.Marquardt, “An algorithm for least-squares estimation of non-linear parameters,” Journal of the society for Industrial and AppliedMathematics, vol.11, no.2, pp.431–441, 1963.</para> 
    /// <para>K.Madsen, H.Nielsen, and O.Tingleff, “Methods for non-linear least squares problems (2nd ed.),” 2004.</para> 
    /// </remarks>
    /// <typeparam name="TB">Backend</typeparam>
    public sealed class LM<TB> : Internal.Gain
        where TB : Backend
    {
        private readonly TB _backend;
        private readonly List<float_t> _foci;
        private readonly List<float_t> _amps;
        private float_t? _eps1;
        private float_t? _eps2;
        private float_t? _tau;
        private uint? _kMax;
        private float_t[]? _initial;

        private IAmplitudeConstraint? _constraint;

        public LM(TB backend)
        {
            _backend = backend;
            _foci = new List<float_t>();
            _amps = new List<float_t>();
        }

        /// <summary>
        /// Parameter. See the paper for details.
        /// </summary>
        /// <param name="focus"></param>
        /// <param name="amp"></param>
        /// <returns></returns>
        public LM<TB> AddFocus(Vector3 focus, float_t amp)
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
        public LM<TB> AddFociFromIter(IEnumerable<(Vector3, float_t)> iter)
        {
            return iter.Aggregate(this, (holo, point) => holo.AddFocus(point.Item1, point.Item2));
        }

        /// <summary>
        /// Parameter. See the paper for details.
        /// </summary>
        /// <param name="value"></param>
        /// <returns></returns>
        public LM<TB> WithEps1(float_t value)
        {
            _eps1 = value;
            return this;
        }

        /// <summary>
        /// Parameter. See the paper for details.
        /// </summary>
        /// <param name="value"></param>
        /// <returns></returns>
        public LM<TB> WithEps2(float_t value)
        {
            _eps2 = value;
            return this;
        }

        /// <summary>
        /// Parameter. See the paper for details.
        /// </summary>
        /// <param name="value"></param>
        /// <returns></returns>
        public LM<TB> WithTau(float_t value)
        {
            _tau = value;
            return this;
        }

        /// <summary>
        /// Parameter. See the paper for details.
        /// </summary>
        /// <param name="value"></param>
        /// <returns></returns>
        public LM<TB> WithKMax(uint value)
        {
            _kMax = value;
            return this;
        }

        /// <summary>
        /// Set amplitude constraint
        /// </summary>
        /// <param name="constraint"></param>
        /// <returns></returns>
        public LM<TB> WithConstraint(IAmplitudeConstraint constraint)
        {
            _constraint = constraint;
            return this;
        }

        /// <summary>
        /// Parameter. See the paper for details.
        /// </summary>
        /// <param name="value"></param>
        /// <returns></returns>
        public LM<TB> WithInitial(float_t[] value)
        {
            _initial = value;
            return this;
        }

        public override GainPtr GainPtr(Geometry geometry)
        {
            var ptr = _backend.Lm(_foci.ToArray(), _amps.ToArray(),
                (ulong)_amps.Count);
            if (_eps1.HasValue) ptr = _backend.LmWithEps1(ptr, _eps1.Value);
            if (_eps2.HasValue) ptr = _backend.LmWithEps2(ptr, _eps2.Value);
            if (_tau.HasValue) ptr = _backend.LmWithTau(ptr, _tau.Value);
            if (_kMax.HasValue) ptr = _backend.LmWithKMax(ptr, _kMax.Value);
            if (_initial != null)
                ptr = _backend.LmWithInitial(ptr, _initial, (ulong)_initial.Length);
            if (_constraint != null) ptr = _backend.LmWithConstraint(ptr, _constraint.Ptr());
            return ptr;
        }
    }
}

#if UNITY_2020_2_OR_NEWER
#nullable restore
#endif
