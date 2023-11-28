import numpy as np
from pyautd3.modulation import Fourier, Sine

m = Fourier(Sine(100)) + Sine(150).with_phase(np.pi / 2.0)
