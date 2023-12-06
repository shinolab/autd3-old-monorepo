dev = autd.geometry[0]
idx = dev.idx
dev.enable = False
dev.sound_speed = 340e3
dev.set_sound_speed_from_temp(15.0)
attenuation = dev.attenuation = 0
t = np.array([1.0, 0.0, 0.0])
r = np.array([1.0, 0.0, 0.0, 0.0])
dev.translate(t)
dev.rotate(r)
dev.affine(t, r)
