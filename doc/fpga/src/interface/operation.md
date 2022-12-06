# Operation

## 初期化/同期

AUTD3の振動子の周波数は$\SI{163.84}{MHz}/\text{CYCLE}$となる.
ここで, CYCLEは下位$\SI{13}{bit}$が使用される. 
CYCLEの書き込みは, 下記の同期の前に行う必要がある.

AUTD3デバイス同士を同期させるには, 次回SYNC0の前までに, 次回SYNC0が発火するEtherCATシステム時間 ($\SI{64}{bit}$, $\SI{1}{ns}$単位) をEC_SYNC_TIMEに書き込み, CTL_REGの第8ビットをセットする.

## Modulator

Modulatorを使用する場合は, MOD_CYCLE ($\SI{16}{bit}$) とMOD_FREQ_DIV ($\SI{32}{bit}$) を書き込む.
そして, 変調データ列をModulator BRAMに書き込む.

サンプリングされる変調データのインデックス$i$は$\SI{163.84}{MHz}$でカウントアップされるSYS_TIMEとMOD_CYCLE, MOD_FREQ_DIVから
$$
i = \left\lfloor\frac{\text{SYS\_TIME}}{\text{MOD\_FREQ\_DIV}}\right\rfloor\ \bmod\ (\text{MOD\_CYCLE} + 1)
$$
として計算される.
そのため, サンプリング周波数は$\SI{163.84}{MHz}/\text{MOD\_FREQ\_DIV}$となり, インデックスは$0$から$\text{MOD\_CYCLE}$まで進む.

また, MOD_FREQ_DIVは4の倍数であることが望ましく, そうでない場合, サンプリング間隔が不均一になる.
さらに, 計算レイテンシの都合上, MOD_FREQ_DIVは$1160$以上である必要がある.

Modulatorを無効にすることはできないので, 変調をかけたくない場合はMOD_CYCLEを$0$に, MOD_FREQ_DIVを$1160$以上の適当な数字に設定し, mod\[0\]に0xFFを書き込む.

## Silencer

Silencerを使用する場合は1ステップあたりの更新量SILENT_STEP ($\SI{16}{bit}$) とSILENT_CYCLEを書き込む.

Silencerの更新レートは$\SI{163.84}{MHz}/\text{SILENT\_CYCLE}$となる.
また, SILENT_CYCLEは4の倍数であることが望ましく, そうでない場合, 更新間隔が不均一になる.
さらに, 計算レイテンシの都合上, SILENT_CYCLEは$1044$以上である必要がある.

Silencerを無効にすることはできないので, 静音化を無効にしたい場合はSILENT_STEPを超音波周期$CYCLE$のいずれよりも大きな値に設定する.

## Normal operation

通常モードで使用する場合は, CTL_REGの5番目のビットをクリアし, Normal BRAMに位相とDuty比データを書き込む.

## STM operation

STMを使用する場合は, STM_CYCLE ($\SI{16}{bit}$) とSTM_FREQ_DIV ($\SI{32}{bit}$) を書き込む.

Focus STMの場合は焦点データをSTM BRAMに書き込み, CTL_REGの5番目のビットをセットし, 6番目のビットをクリアする.
Focus STMを使用する場合は, 追加でSOUND_SPEED ($\SI{32}{bit}$) も書き込む. SOUND_SPEEDの単位は$\SI{1/1024}{m/s}$である.
焦点データは(x,y,z)の位置データを単位$\SI{0.025}{mm}$の$\SI{18}{bit}$符号あり固定小数点数で指定する.
また, duty_shiftを指定して, Duty比を調整できる. Duty比$D$は超音波周期$CYCLE$に対して
$$
    D = \left\lfloor \frac{T}{2} \right\rfloor \gg \text{duty\_shift}
$$
となる.

Gain STMの場合は, Duty比と位相データをSTM BRAMに書き込み, CTL_REGの5番目と6番目のビットをセットする.

どちらのモードでも, サンプリングされるSTMデータのインデックス$i$は$\SI{163.84}{MHz}$でカウントアップされるSYS_TIMEとSTM_CYCLE, STM_FREQ_DIVから
$$
i = \left\lfloor\frac{\text{SYS\_TIME}}{\text{STM\_FREQ\_DIV}}\right\rfloor\ \bmod\ (\text{STM\_CYCLE} + 1)
$$
として計算される.
そのため, サンプリング周波数は$\SI{163.84}{MHz}/\text{STM\_FREQ\_DIV}$となり, インデックスは$0$から$\text{STM\_CYCLE}$まで進む.
なお, Gain STMの場合のSTM_CYCLEの上限は$1023$であることに注意する.
また, STM_FREQ_DIVは4の倍数であることが望ましく, そうでない場合, サンプリング間隔が不均一になる.
さらに, 計算レイテンシの都合上, STM_FREQ_DIVは$1612$以上である必要がある.
