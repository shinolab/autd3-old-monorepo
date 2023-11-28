import numpy as np
from pyautd3 import Drive, EmitIntensity
from pyautd3.gain import Uniform

g = Uniform(EmitIntensity.maximum()).with_transform(
    lambda dev, tr, d: Drive(d.phase + np.pi, EmitIntensity(d.intensity.value // 2))
)
