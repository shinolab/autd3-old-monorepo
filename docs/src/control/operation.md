# Operation

## 初期化

初期化操作では, CPU内部データ, 及び, FPGA内のデータをリセットする.
リセットされた後は, 振動子は何も出力しない.

初期化操作を行うにはMSG_IDを0x00にする. 

この操作は他の操作とは同時に行えない.

## 超音波周期の設定/同期

超音波周期の設定, 及び, 同期を行う場合は, MSG_IDを0x05から0xFFのいずれか, 且つ, 以前のフレームとは別の値にする.
また, CPU_CTL_REGのDO_SYNC bitをセットする.
さらに, Bodyデータに超音波周期を書き込んでおく.

この操作は他の操作とは同時に行えない.

## Modulatorの設定

Modulatorの設定はHeaderのみで行う.

HEAD_DATAの長さはFPGA内のBRAMサイズより少ないので, いくつかのフレームに分けて送信する.

Modulatorを設定する場合は, MSG_IDを0x05から0xFFのいずれか, 且つ, 以前のフレームとは別の値にする.
さらに, CPU_CTL_REGのDO_SYNC bit, CONFIG_SILENCER bitをクリアしておく必要がある.

まず, 最初の変調データを書き込むには, CPU_CTL_REGのMOD_BEGIN bitをセットし, HEAD_DATAの上位$\SI{1}{byte}$にHEAD_DATAに含まれる変調データのサイズを書き込み, 続く$\SI{1}{byte}$に変調サンプリング周波数分周比 (FREQ_DIV) を書き込み, さらに続く$\SI{120}{byte}$に可能な限り変調データを書き込む.
変調データがすべて含まれている場合はCPU_CTL_REGのMOD_END bitをセットし, 終了する.
そうでない場合は, MSG_IDを別の値に設定し, HEAD_DATAの上位$\SI{1}{byte}$にHEAD_DATAに含まれる変調データのサイズを書き込み, 続く$\SI{124}{byte}$に可能な限り変調データを書き込む, というのを繰り返す.
変調データをすべて送信した場合はCPU_CTL_REGのMOD_END bitをセットする.

MOD_BEGINがセットされているフレームが送信されてから, MOD_ENDがセットされているフレームが送信されるまでの間に同期, 及び, Silencerの設定を行うことは禁止される.

この操作は, 「Normal動作時のDuty比/位相の設定」と「STM動作時のDuty比/位相の設定」の操作と同時に行うことができるが, それ以外の操作とは同時に行えない.

## Silencerの設定

Silencerの設定はHeaderのみで行う.

Silencerを設定する場合は, MSG_IDを0x05から0xFFのいずれか, 且つ, 以前のフレームとは別の値にする.
また, CPU_CTL_REGのDO_SYNC bitをクリアし, CONFIG_SILENCER bitをセットしておく必要がある.
さらにHEAD_DATAの上位$\SI{2}{byte}$に1ステップあたりの更新量 (STEP) を書き込み, 続く$\SI{4}{byte}$に更新周波数分周比を書き込む.

この操作は, 「Normal動作時のDuty比/位相の設定」と「STM動作時のDuty比/位相の設定」の操作と同時に行うことができるが, それ以外の操作とは同時に行えない.

## Normal動作時のDuty比/位相の設定

Normal動作時のDuty比と位相データはBodyに書き込む.
この際, FPGA_CTL_REGのLEGACY_MODE bitとCPU_CTL_REGのIS_DUTY bitに応じて書き込むデータは異なる.
また, MSG_IDは0x05から0xFFのいずれか, 且つ, 以前のフレームとは別の値にしておく.
さらに, FPGA_CTL_REGのOP_MODE bitはクリアしておく.

### LEGACY_MODE = 0の場合

非LEGACYモードの場合は, BodyデータにはCPU_CTL_REGのIS_DUTY bitが0の場合は位相データを, 1の場合はDuty比データを順番に入れておく.

- IS_DUTY = 0

| Index       | DATA (1byte)         |
|-------------|----------------------|
| 0           | phase\[0\]\[7:0\]    |
| 1           | phase\[0\]\[12:8\]   |
| ︙          | ︙                   |
| 496         | phase\[248\]\[12:8\] |
| 497         | phase\[\248]\[12:8\] |


- IS_DUTY = 1

| Index       | DATA (1byte)        |
|-------------|---------------------|
| 0           | duty\[0\]\[7:0\]    |
| 1           | duty\[0\]\[12:8\]   |
| ︙          | ︙                  |
| 496         | duty\[248\]\[12:8\] |
| 497         | duty\[\248]\[12:8\] |


### LEGACY_MODE = 1の場合

LEGACYモードの場合は, Bodyデータには$\SI{1}{byte}$の位相/Duty比データを順番に入れておく.

| Index       | DATA (1byte)  |
|-------------|---------------|
| 0           | phase\[0\]    |
| 1           | duty\[0\]     |
| ︙          | ︙            |
| 496         | phase\[248\]  |
| 497         | duty\[248\]   |

## STM動作時のDuty比/位相の設定

STM動作時のデータはBodyに書き込む.
この際, FPGA_CTL_REGのSTM_GAIN_MODE, LEGACY_MODE bitとCPU_CTL_REGのIS_DUTY bitに応じて書き込むデータは異なる.
また, MSG_IDは0x05から0xFFのいずれか, 且つ, 以前のフレームとは別の値にしておく.
さらに, FPGA_CTL_REGのOP_MODE bitはセットしておく.

### Point STM (STM_GAIN_MODE = 0)

Point STMの場合も, Modulatorと同じく, いくつかのフレームに分けてデータを送信する.
最初のフレームではCPU_CTL_REGのSTM_BEGIN bitをセットする.
また, Bodyの先頭$\SI{2}{byte}$に点列数を, 続く$\SI{4}{byte}$にサンプリング周波数分周比を, さらに続く$\SI{4}{byte}$に音速を書き込み, 残りの$\SI{488}{byte}$に点列データを書き込む.
点列データがすべて含まれている場合はCPU_CTL_REGのSTM_END bitをセットし, 終了する.
そうでない場合は, MSG_IDを別の値に設定し, Bodyの上位$\SI{2}{byte}$に点列データのサイズを書き込み, 続く$\SI{496}{byte}$に点列データを書き込む, というのを繰り返す.
変調データをすべて送信した場合はCPU_CTL_REGのSTM_END bitをセットする.

### Gain STM (STM_GAIN_MODE = 1)

Gain STMの場合は, 最初のフレームのCPU_CTL_REGのSTM_BEGIN bitをセットし, Bodyの先頭$\SI{4}{byte}$にサンプリング周波数分周比を書き込む. 残りは使用しない.
その後, 1パターンずつ, Normal動作と同様のデータをBodyに書き込み送信する.
最終フレームではCPU_CTL_REGのSTM_END bitをセットする.

## Version情報の取得

Version情報を取得するには, MSG_IDを特定の値にしたフレームを送信すれば良い.

- 0x01: CPU version number
- 0x03: FPGA version number
- 0x04: FPGA function bits

Version情報が取得された後, Ackの上位$\SI{8}{bit}$に上記MSG_IDが, 下位$\SI{8}{bit}$にVersion情報が書き込まれたフレームが返送される.

### Version number

| Version number | Version        | 
|----------------|----------------| 
| 0x00 (0)       | v0.3 or former | 
| 0x01 (1)       | v0.4           | 
| 0x02 (2)       | v0.5           | 
| 0x03 (3)       | v0.6           | 
| 0x04 (4)       | v0.7           | 
| 0x05 (5)       | v0.8           | 
| 0x06 (6)       | v0.9           | 
| 0x0A (10)      | v1.0           | 
| 0x0B (11)      | v1.1           | 
| 0x0C (12)      | v1.2           | 
| 0x0D (13)      | v1.3           | 
| 0x10 (16)      | v1.6           | 
| 0x11 (17)      | v1.7           | 
| 0x12 (18)      | v1.8           | 
| 0x13 (19)      | v1.9           | 
| 0x14 (20)      | v1.10          | 
| 0x15 (21)      | v1.11          | 
| 0x80 (128)     | v2.0           |
| 0x81 (129)     | v2.1           |
| 0x82 (130)     | v2.2           |

### Function bit

Function bitは各機能が有効になっているかを表す.

| Bit | Function  | 
|-----|-----------| 
| 0   | STM       | 
| 1   | Modulator | 
| 2   | Silencer  | 
| 3   | Mod delay | 
| 4   | -         | 
| 5   | -         | 
| 6   | -         | 
| 7   | -         | 
