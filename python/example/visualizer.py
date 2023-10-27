"""
File: visualizer.py
Project: example
Created Date: 13/10/2023
Author: Shun Suzuki
-----
Last Modified: 13/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""

import numpy as np

from pyautd3 import AUTD3, Controller
from pyautd3.gain import Focus
from pyautd3.link.visualizer import PlotRange, PyPlotConfig, PythonBackend, Visualizer
from pyautd3.modulation import Square

if __name__ == "__main__":
    autd = (
        Controller.builder()
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))
        .open_with(Visualizer.builder().with_backend(PythonBackend()))
    )

    center = autd.geometry.center + np.array([0, 0, 150])

    g = Focus(center)
    m = Square(150)

    autd.send((m, g))

    autd.link.plot_phase(PyPlotConfig(fname="phase.png"), autd.geometry)

    autd.link.plot_field(
        PyPlotConfig(fname="x.png"),
        PlotRange(
            x_start=center[0] - 50,
            x_end=center[0] + 50,
            y_start=center[1],
            y_end=center[1],
            z_start=center[2],
            z_end=center[2],
            resolution=1,
        ),
        autd.geometry,
    )

    autd.link.plot_field(
        PyPlotConfig(fname="xy.png"),
        PlotRange(
            x_start=center[0] - 20,
            x_end=center[0] + 20,
            y_start=center[1] - 30,
            y_end=center[1] + 30,
            z_start=center[2],
            z_end=center[2],
            resolution=1,
        ),
        autd.geometry,
    )

    autd.link.plot_field(
        PyPlotConfig(fname="yz.png"),
        PlotRange(
            x_start=center[0],
            x_end=center[0],
            y_start=center[1] - 30,
            y_end=center[1] + 30,
            z_start=0,
            z_end=center[2] + 50,
            resolution=2,
        ),
        autd.geometry,
    )

    autd.link.plot_field(
        PyPlotConfig(fname="zx.png"),
        PlotRange(
            x_start=center[0] - 30,
            x_end=center[0] + 30,
            y_start=center[1],
            y_end=center[1],
            z_start=0,
            z_end=center[2] + 50,
            resolution=2,
        ),
        autd.geometry,
    )

    autd.link.plot_modulation(PyPlotConfig(fname="mod.png"))

    # Calculate acoustic pressure without plotting
    points = [center]
    p = autd.link.calc_field(points, autd.geometry)
    print(f"Acoustic pressure at ({center[0]}, {center[1]}, {center[2]}) = {p[0]}")

    autd.close()
