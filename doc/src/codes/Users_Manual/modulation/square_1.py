from pyautd3 import EmitIntensity
from pyautd3.modulation import Square

m = Square(150).with_low(EmitIntensity.minimum()).with_high(EmitIntensity.maximum())
