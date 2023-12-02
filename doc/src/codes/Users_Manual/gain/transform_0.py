import numpy as np
from pyautd3 import Drive, EmitIntensity, Phase
from pyautd3.gain import Uniform

g = Uniform(EmitIntensity.maximum()).with_transform(
    lambda dev, tr, d: Drive(
        Phase.from_rad(d.phase.radian + np.pi), EmitIntensity(d.intensity.value // 2)
    )
)
