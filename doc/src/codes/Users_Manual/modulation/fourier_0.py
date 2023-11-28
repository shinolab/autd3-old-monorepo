from pyautd3.modulation import Fourier, Sine

m = Fourier(Sine(100)).add_component(Sine(150)).add_components_from_iter([Sine(200)])
