# PWM output

XDCR_OUTはドライバを介して振動子に接続されている.
ただし, 252本の信号線の内, XDCR_OUT\[20\], XDCR_OUT\[21\], XDCR_OUT\[35\]は振動子と接続されていない.
また, ドライバによって, FPGAのlow level出力 ($\SI{0}{V}$) は$\SI{-12}{V}$に, FPGAのhigh level出力 ($\SI{3.3}{V}$) は$\SI{12}{V}$に変換される.
