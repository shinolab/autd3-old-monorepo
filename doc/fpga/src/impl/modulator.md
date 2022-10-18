# Modulator

Modulator回路のダイアグラムは以下の通りである.

<figure>
<img alt="Modulator" src="../figs/modulation/diagram.svg">
<figcaption>Modulator Diagram</figcaption>
</figure>

| In/Out | 名前         | バス幅         | 説明                             |
| :----- | :----------- | :------------- | :------------------------------- |
|   In   | CLK          | $1$            | クロック ($40.96\,\mathrm{MHz}$) |
|   In   | SYS_TIME     | $64$           | 同期時刻                         |
|   In   | CYCLE_M      | $16$           | AM周期$T_m-1$                    |
|   In   | FREQ_DIV_M   | $32$           | AM周波数分周比$N_m$              |
|   In   | DELAY_M      | $16\times 249$ | AMディレイ                       |
|   In   | CPU_BUS      | -              | CPUバス                          |
|   In   | DUTY         | $13\times 249$ | Duty比$D$                        |
|   In   | PHASE        | $13\times 249$ | 位相$P$                          |
|   Out  | DUTY_M       | $13\times 249$ | AM変調済み済みDuty比$D_m$        |
|   Out  | PHASE        | $13\times 249$ | 位相$P$                          |
|   Out  | IDX          | $16$           | 変調データインデックス (Debug用) |
|   Out  | START        | $1$            | 計算開始フラグ (Debug用)         |
|   Out  | DONE         | $1$            | 計算終了フラグ (Debug用)         |

### Memory

MemoryはCPUから書き込まれた変調データを格納する.
BRAMのサイズは$8\,\mathrm{bit}\times 65536$である.

### Sampler

SamplerはAM周期$T_m$, AM周波数分周比$N_m$, 及び, SYS_TIMEから現在の変調データのインデックス$i$を計算し, Memoryから変調データ$M$を取り出し, Multiplierに渡す.

変調データのインデックス$i$は
$$
i = \left\lfloor \frac{\text{SYS\_TIME}}{N_m} - \text{DELAY\_M} \right\rfloor \bmod T_m
$$
として計算される.
したがって, AMデータのサンプリング周波数$f_\text{AM}$は
$$
f_\text{AM} = \frac{163.84\,\mathrm{MHz}}{N_m}
$$
となり, $i$はこの周波数で$0$から$T_m-1$まで周期的にカウントアップされる.

> NOTE: Samplerに渡されるCYCLE_Mは$T_m-1$であることに注意する.
> すなわち, $T_m=\text{CYCLE\_M}+1$である.

また, 変調データをサンプルしたタイミングでSTARTフラグをアサートする.

### Multiplier

MultiplierはSTARTフラグがアサートされたタイミングでDuty比$D$をキャプチャし, 変調データ$M$を各々かけ合わせる.
変調後のDuty比$D_m$は
$$
    D_m = \frac{D\times M}{255}
$$
となる.

また, 計算が完了したタイミングでDONEフラグをアサートする.

### Buffer

Multiplierの計算は並行に行われるため, DUTY_Mの値は並行に更新される.
また, AM変調の計算のせいでDuty比のデータが位相データに対して遅れる.
Bufferの役割は, これらの更新タイミングを揃える事である.

Bufferは, STARTフラグがアサートされたタイミング位相データ$P$をキャプチャし, DONEフラグがアサートされたタイミングでキャプチャした位相データ$P$, 及び, Multiplierから渡されたDuty比$D_m$を出力する.
