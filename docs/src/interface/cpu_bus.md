# CPU Bus

CPUボードとの通信は以下の信号線を用いる.

* \[16:0\] CPU_ADDR - Input, CPUボードから書き込まれるデータのアドレス. 下位$\SI{1}{bit}$は使用できないため, 実質$\SI{16}{bit}$
* \[15:0\] CPU_DATA - Input/Output, CPUボードから書き込まれるデータ. 
* CPU_CKIO - Input, CPUバスクロック
* CPU_CS1_N - Input, Enable
* CPU_WE0_N - Input, Write Enable信号
* CPU_RD_N - Input, 0でCPUからの読み込み
* CPU_RDWR - Input, 1でCPUからの読み込み, 0でCPUからの書き込み

XilinxのBRAM IPと直接接続できる (Native Port).
書き込みは, CPU_ADDRをaddrに, CPU_DATAをdin, CPU_CKIOをclk, ~CPU_CS1_Nをen, CPU_WE0_Nをweに接続する.
読み込みは, トライステートバッファを使用し, ~CPU_CS1_Nかつ~CPU_RD_NかつCPU_RDWRの場合にdoutをCPU_DATAに接続し, そうでないときはCPU_DATAをハイインピーダンスにする.
