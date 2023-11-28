autd.Geometry[0].ReadsFPGAInfo = true;
autd.Send(new UpdateFlags());

var info = autd.FPGAInfo;