/*
 * File: CUDAAttribute.cs
 * Project: Holo
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3Sharp.Gain.Holo;

namespace tests.Gain.Holo
{
    public sealed class IgnoreIfCUDAIsNotFoundFactAttribute : FactAttribute
    {
        public IgnoreIfCUDAIsNotFoundFactAttribute()
        {
            try
            {
                _ = new CUDABackend();
            }
            catch (Exception e)
            {
                Skip = e.Message;
            }
        }
    }
}
