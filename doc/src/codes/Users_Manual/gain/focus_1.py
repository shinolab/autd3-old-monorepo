from pyautd3 import EmitIntensity
from pyautd3.gain import Focus

g = Focus([x, y, z]).with_intensity(EmitIntensity.maximum())
