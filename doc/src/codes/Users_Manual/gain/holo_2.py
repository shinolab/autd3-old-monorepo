from pyautd3.gain.holo import GSPAT, NalgebraBackend

backend = NalgebraBackend()
g = GSPAT(backend).with_repeat(100)
