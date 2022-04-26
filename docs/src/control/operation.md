# Operation

## 初期化

初期化操作では, CPU内部データ, 及び, FPGA内のデータをリセットする.
リセットされた後は, 振動子は何も出力しない.

初期化操作を行うにはMSG_IDを0x00にする. 

この操作は他の操作とは同時に行えない.

## 超音波周期の設定/同期

超音波周期の設定, 及び, 同期を行う場合は, MSG_IDを0x05から0xFFのいずれか, 且つ, 以前のフレームとは別の値にする.
また, CPU_CTL_REGのDO_SYNC bitをセットし, HEAD_DATAの上位$\SI{2}{byte}$にSYNC0の周期を書き込む.
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

### Gain STM (STM_GAIN_MODE = 1)
