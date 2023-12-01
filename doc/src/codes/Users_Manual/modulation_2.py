from pyautd3 import ConfigureModDelay

autd.send(ConfigureModDelay(lambda dev, tr: 1 if dev.idx == 0 and tr.idx == 0 else 0))
