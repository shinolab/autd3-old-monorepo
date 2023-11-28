from pyautd3.gain.holo import GSPAT, NalgebraBackend, pascal

backend = NalgebraBackend()
g = (
    GSPAT(backend)
    .add_focus([x1, y1, z1], 5e3 * pascal)
    .add_focus([x2, y2, z2], 5e3 * pascal)
)
