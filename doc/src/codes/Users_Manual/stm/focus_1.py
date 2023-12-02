from pyautd3 import SamplingConfiguration
from pyautd3.stm import FocusSTM

stm = FocusSTM.from_sampling_config(SamplingConfiguration.from_frequency(1))
