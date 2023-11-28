from pyautd3 import EmitIntensity
from pyautd3.gain import Plane

g = Plane([nx, ny, nz]).with_intensity(EmitIntensity.maximum())
