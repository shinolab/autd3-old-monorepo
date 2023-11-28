from pyautd3 import EmitIntensity
from pyautd3.gain import Bessel

g = Bessel([x, y, z], [nx, ny, nz], theta).with_intensity(EmitIntensity.maximum())
