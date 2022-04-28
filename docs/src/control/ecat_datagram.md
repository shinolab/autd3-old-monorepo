# EtherCAT Datagram

EtherCATからのデータは主に全デバイス共通のHeader部と各デバイス固有のBody部に分かれている.

## Header

Headerは$\SI{128}{byte}$のデータ量を持つ.

| Index       | DATA (1byte)          |
|-------------|-----------------------|
| 0           | MEG_ID                |
| 1           | FPGA_CTL_REG          |
| 2           | CPU_CTL_REG           |
| 3 - 127     | HEAD_DATA             |

3-127番目のデータは各操作ごとに異なる内容となる.

MSG_IDはEtherCATのデータを区別する役割がある.
EtherCATはその仕様上, 同じフレームが何度もデバイスに送られることがある.
そのため, MSG_IDを参照して, すでに処理したかどうかを判定している.

FPGA_CTL_REGはFPGAに書き込まれる制御レジスタであり, 以下の意味を持つ.

* FPGA_CTL_REG
    * 0: LEGACY_MODE
    * 4: FORCE_FAN
    * 5: OP_MODE (0: Normal, 1: STM)
    * 6: SEQ_MODE (0: Point STM, 1: Gain STM)
    * 8: SYNC_SET

CPU_CTL_REGはCPUの制御レジスタであり, 以下の意味を持つ.

* CPU_CTL_REG
    * 0: MOD_BEGIN
    * 1: MOD_END
    * 2: STM_BEGIN
    * 3: STM_END
    * 4: IS_DUTY
    * 5: CONFIG_SILENCER
    * 6: READS_FPGA_INFO
    * 7: DO_SYNC

## Body

Bodyは$\SI{498}{byte}$のデータ量を持ち, 内容は操作毎に異なる.
