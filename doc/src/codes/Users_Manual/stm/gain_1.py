from pyautd3 import SamplingConfiguration
from pyautd3.stm import GainSTM

stm = GainSTM.new_with_sampling_config(SamplingConfiguration.new_with_frequency(1))
