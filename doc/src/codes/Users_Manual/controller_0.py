from pyautd3 import ConfigureReadsFPGAInfo

autd.send(ConfigureReadsFPGAInfo(lambda _: True))

info = autd.fpga_info
