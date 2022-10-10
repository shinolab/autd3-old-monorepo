/*
 * File: IGain.cs
 * Project: Gain
 * Created Date: 24/08/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/08/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

namespace AUTD3_GUI_Controller.Models.Gain;

public interface IGain
{
    public AUTD3Sharp.Gain.Gain ToGain();
}
