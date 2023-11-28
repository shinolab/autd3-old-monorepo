from pyautd3 import UpdateFlags

autd.geometry[0].reads_fpga_info = True
autd.send(UpdateFlags())

info = autd.fpga_info
