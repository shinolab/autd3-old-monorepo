from pyautd3 import EmitIntensity, Phase
from pyautd3.gain import Uniform

g = Uniform(EmitIntensity.maximum()).with_phase(Phase(0))
