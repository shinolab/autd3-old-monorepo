from pyautd3 import EmitIntensity
from pyautd3.modulation import Sine

m = (
    Sine(150)
    .with_intensity(EmitIntensity.minimum())
    .with_offset(EmitIntensity.minimum() / 2)
)
