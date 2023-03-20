/*
 * File: TimerStrategy.cs
 * Project: src
 * Created Date: 20/03/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/03/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

namespace AUTD3Sharp
{
    public enum TimerStrategy : byte { Sleep = 0, BusyWait = 1, NativeTimer = 2 }
}
