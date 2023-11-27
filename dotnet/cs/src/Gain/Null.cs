/*
 * File: Null.cs
 * Project: Gain
 * Created Date: 13/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 07/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

namespace AUTD3Sharp.Gain
{
    /// <summary>
    /// Gain to output nothing
    /// </summary>
    public sealed class Null : Internal.Gain
    {
        internal override GainPtr GainPtr(Geometry geometry) => NativeMethodsBase.AUTDGainNull();
    }
}
