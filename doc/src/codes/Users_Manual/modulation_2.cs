using AUTD3Sharp;

autd.Send(new ConfigureModDelay((dev, tr) => return dev.Idx == 0 && tr.Idx == 0 ? 1 : 0));