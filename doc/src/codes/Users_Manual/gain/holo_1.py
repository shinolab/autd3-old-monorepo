from pyautd3 import EmitIntensity
from pyautd3.gain.holo import GSPAT, EmissionConstraint, NalgebraBackend

backend = NalgebraBackend()
g = GSPAT(backend).with_constraint(EmissionConstraint.uniform(EmitIntensity.maximum()))
