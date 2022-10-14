/*
 * File: IModulation.cs
 * Project: Modulation
 * Created Date: 24/08/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/08/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

namespace AUTD3_GUI_Controller.Models.Modulation;

public interface IModulation
{
    public AUTD3Sharp.Modulation.Modulation ToModulation();
}
