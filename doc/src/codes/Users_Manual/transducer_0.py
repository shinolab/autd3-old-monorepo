tr = autd.geometry[0][0]
idx = tr.idx
position = tr.position
rotation = tr.rotation
x_dir = tr.x_direction
y_dir = tr.y_direction
z_dir = tr.z_direction

sound_speed = autd.geometry[0].sound_speed
wavelen = tr.wavelength(sound_speed)
wavenum = tr.wavenumber(sound_speed)
