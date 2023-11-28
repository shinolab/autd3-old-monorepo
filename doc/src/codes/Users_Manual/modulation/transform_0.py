from pyautd3 import EmitIntensity
from pyautd3.modulation import Sine

m = Sine(150).with_transform(lambda i, d: EmitIntensity(d.value // 2))
