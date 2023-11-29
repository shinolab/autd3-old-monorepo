/*
 * File: Fourier.cs
 * Project: Modulation
 * Created Date: 13/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

using System.Collections.Generic;
using System.Linq;
using AUTD3Sharp.NativeMethods;

namespace AUTD3Sharp.Modulation
{
    /// <summary>
    /// Multi-frequency sine wave modulation
    /// </summary>
    public sealed class Fourier : Internal.Modulation
    {
        private readonly List<Sine> _components;

        /// <summary>
        /// Constructor
        /// </summary>
        public Fourier(Sine sine)
        {
            _components = new List<Sine> { sine };
        }

        public Fourier AddComponent(Sine sine)
        {
            _components.Add(sine);
            return this;
        }

        public Fourier AddComponentsFromIter(IEnumerable<Sine> iter)
        {
            return iter.Aggregate(this, (fourier, sine) => fourier.AddComponent(sine));
        }

        public static Fourier operator +(Fourier a, Sine b)
            => a.AddComponent(b);

        internal override ModulationPtr ModulationPtr()
        {
            return _components.Skip(1).Aggregate(NativeMethodsBase.AUTDModulationFourier(_components[0].ModulationPtr()), (current, sine) => NativeMethodsBase.AUTDModulationFourierAddComponent(current, sine.ModulationPtr()));
        }
    }
}
