from pyautd3 import SamplingConfiguration
from pyautd3.modulation import Sine

m = Sine(150).with_sampling_config(SamplingConfiguration.from_frequency(4e3))
