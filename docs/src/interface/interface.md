# Interface

FPGAと外部との接続は以下のようになっている.

* \[16:0\] CPU_ADDR - Input, CPUボードから書き込まれるデータのアドレス. 下位$\SI{1}{bit}$は使用できないため, 実質$\SI{16}{bit}$
* \[15:0\] CPU_DATA - Input/Output, CPUボードから書き込まれるデータ. 
* \[252:1\] XDCR_OUT - Output, PWMの出力. ドライバを介して振動子に接続されている.
* CPU_CKIO - Input, CPUバスクロック
* CPU_CS1_N - Input, Enable
* RESET_N - Input, リセット信号
* CPU_WE0_N - Input, Write Enable信号
* CPU_WE1_N - Input, Write Enable信号 (未使用)
* CPU_RD_N - Input, 0でCPUからの読み込み
* CPU_RDWR - Input, 1でCPUからの読み込み, 0でCPUからの書き込み
* MRCC_25P6M - Input, $\SI{25.6}{MHz}$のクロック
* CAT_SYNC0 - Input, EtherCAT sync0同期信号
* FORCE_FAN - Output, ファン強制起動
* THERMO - Input, 温度監視ICからの入力
* \[3:0\] GPIO_IN - GPIOピンの入力
* \[3:0\] GPIO_OUT - GPIOピンの出力
