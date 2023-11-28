from pyautd3 import SamplingConfiguration
from pyautd3.stm import FocusSTM

stm = FocusSTM.new_with_sampling_config(SamplingConfiguration.new_with_frequency(1))
