import numpy as np
from pyautd3 import Drive, EmitIntensity, Geometry, Phase
from pyautd3.gain import Gain


class Focus(Gain):
    def __init__(self, point):
        self.point = np.array(point)

    def calc(self, geometry: Geometry) -> dict[int, np.ndarray]:
        return Gain._transform(
            geometry,
            lambda dev, tr: Drive(
                Phase.from_rad(
                    np.linalg.norm(tr.position - self.point)
                    * tr.wavenumber(dev.sound_speed)
                ),
                EmitIntensity.maximum(),
            ),
        )
