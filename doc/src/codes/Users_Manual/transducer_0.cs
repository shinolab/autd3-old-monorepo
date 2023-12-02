var tr = autd.Geometry[0][0];
var trIdx = tr.Idx;
var position = tr.Position;
var rotation = tr.Rotation;
var xDir = tr.XDirection;
var yDir = tr.YDirection;
var zDir = tr.ZDirection;

var soundSpeed = autd.Geometry[0].SoundSpeed;
var wavelen = tr.Wavelength(soundSpeed);
var wavenum = tr.Wavenumber(soundSpeed);