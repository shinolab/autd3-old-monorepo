# Synchronizer

Synchronizer回路のダイアグラムは以下の通りである.

<figure>
<img alt="Synchronizer" src="../figs/synchronizer/diagram.svg">
<figcaption>Synchronizer Diagram</figcaption>
</figure>

| In/Out | 名前                  | バス幅         | 説明                                  |
| :----- | :-----------          | :------------- | :--------------------------------     |
|   In   | CLK                   | $1$            | クロック ($163.84\,\mathrm{MHz}$)     |
|   In   | ECAT_SYNC_TIME        | $64$           | 同期開始時刻. 単位は$\SI{1}{ns}$      |
|   In   | SET                   | $1$            | 同期開始フラグ                        |
|   In   | ECAT_SYNC             | $1$            | 同期信号                              |
|   Out  | SYS_TIME              | $64$           | 同期時刻                              |

SynchronizerはすべてのFPGAで同期した時刻SYS_TIMEを生成するモジュールである.

ECAT_SYNCが立ち上がる前に, ECAT_SYNC_TIMEにECAT_SYNCが立ち上がる時刻 (EtherCAT時間) を書き込み, SETを立ち上げることで初期化する.

以降, SYS_TIMEはCLKによりインクリメントされていく.
ただし, CLKを生成する水晶振動子には個体差があるため, ECAT_SYNC信号が立ち上がるたびに補正を実行する.
補正は, SYS_TIMEが基準より進んでいる場合 (即ち, 水晶振動子の周波数が基準より高い場合) はインクリメントを止め, SYS_TIMEが遅れている場合 (即ち, 水晶振動子の周波数が基準より低い場合) は余分にインクリメントすることにより実行する.
そのため, SYS_TIMEは逆戻りすることはない. 
