# STM operator

STM operator回路のダイアグラムは以下の通りである.

<figure>
<img alt="STM operator" src="../figs/stm_operator/diagram.svg">
<figcaption>STM Operator Diagram</figcaption>
</figure>

| In/Out | 名前             | バス幅         | 説明                             |
| :----- | :--------------- | :------------- | :------------------------------- |
|   In   | CLK              | $1$            | クロック ($40.96\,\mathrm{MHz}$) |
|   In   | SYS_TIME         | $64$           | 同期時刻                         |
|   In   | CYCLE            | $16$           | STM周期$T_s-1$                   |
|   In   | FREQ_DIV         | $32$           | STM周波数分周比$N_s$             |
|   In   | CPU_BUS          | -              | CPUバス                          |
|   In   | ULTRASOUND_CYCLE | $13\times 249$ | 超音波周期$T$                    |
|   In   | SOUND_SPEED      | $32$           | 音速. 単位は$\SI{1/1024}{m/s}$   |
|   In   | GAIN_MODE        | $1$            | Gain mode選択信号                |
|   Out  | DUTY             | $13\times 249$ | Duty比$D$                        |
|   Out  | PHASE            | $13\times 249$ | 位相$P$                          |
|   Out  | IDX              | $16$           | 変調データインデックス (Debug用) |
|   Out  | START            | $1$            | 計算開始フラグ (Debug用)         |
|   Out  | DONE             | $1$            | 計算終了フラグ (Debug用)         |

### Memory

MemoryはCPUから書き込まれた変調データを格納する.
BRAMのサイズは$\SI{128}{bit}\times 65536$である.

### Sampler

SamplerはSTM周期$T_s$, STM周波数分周比$N_s$, 及び, SYS_TIMEから現在の変調データのインデックス$i$を計算する.

変調データのインデックス$i$は
$$
i = \left\lfloor \frac{\text{SYS\_TIME}}{N_s} \right\rfloor \bmod T_s
$$
として計算される.
したがって, STMデータのサンプリング周波数$f_\text{STM}$は
$$
f_\text{STM} = \frac{163.84\,\mathrm{MHz}}{N_s}
$$
となり, $i$はこの周波数で$0$から$T_s-1$まで周期的にカウントアップされる.

> Note: Samplerに渡されるCYCLEは$T_s-1$であることに注意する.
> すなわち, $T_s=\text{CYCLE}+1$である.

### Gain

GainモジュールはSTM BRAMに書き込まれたデータを生のDuty比/位相データとみなして出力する.

### Focus

FocusモジュールはSTM BRAMに書き込まれたデータを焦点の位置データとみなして, 焦点位置から適切な位相を計算して出力する.

### MUX

GAIN_MODEが$1$のときはGainモジュールの出力を, $0$のときはFocusモジュールの出力を選択して出力する.
