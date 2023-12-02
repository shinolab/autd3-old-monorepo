const auto tr = autd.geometry()[0][0];
const auto idx = tr.idx();
const autd3::Vector3 position = tr.position();
const auto rotation = tr.rotation();
const auto x_dir = tr.x_direction();
const auto y_dir = tr.y_direction();
const auto z_dir = tr.z_direction();

const auto sound_speed = autd.geometry()[0].sound_speed();
const auto wavelen = tr.wavelength(sound_speed);
const auto wavenum = tr.wavenumber(sound_speed);