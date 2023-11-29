/*
 * File: Modulation.cs
 * Project: Internal
 * Created Date: 08/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

using System.Runtime.InteropServices;
using AUTD3Sharp.NativeMethods;

namespace AUTD3Sharp.Internal
{
    [ComVisible(false)]
    public abstract class Modulation : IDatagram
    {
        public SamplingConfiguration SamplingConfiguration => new SamplingConfiguration(NativeMethodsBase.AUTDModulationSamplingConfig(ModulationPtr()));

        DatagramPtr IDatagram.Ptr(Geometry geometry) => NativeMethodsBase.AUTDModulationIntoDatagram(ModulationPtr());

        internal abstract ModulationPtr ModulationPtr();

        public int Length => NativeMethodsBase.AUTDModulationSize(ModulationPtr()).Validate();
    }

    public abstract class ModulationWithSamplingConfig<T> : Modulation
        where T : ModulationWithSamplingConfig<T>
    {
        protected SamplingConfiguration? Config;

        /// <summary>
        /// Set sampling frequency division
        /// </summary>
        /// <param name="config">Sampling configuration.</param>
        /// <returns></returns>
        public T WithSamplingConfig(SamplingConfiguration config)
        {
            Config = config;
            return (T)this;
        }
    }
}
