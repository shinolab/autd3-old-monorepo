# Silencer

Silencer回路のダイアグラムは以下の通りである.

<figure>
<img alt="Silencer" src="../figs/silent/diagram.svg">
<figcaption>Silencer Diagram</figcaption>
</figure>

| In/Out | 名前         | バス幅         | 説明                                |
| :----- | :----------- | :------------- | :---------------------------------- |
|   In   | CLK          | $1$            | クロック ($40.96\,\mathrm{MHz}$)    |
|   In   | SYS_TIME     | $64$           | 同期時刻                            |
|   In   | CYCLE_S      | $16$           | 更新周期$T_s$                       |
|   In   | CYCLE        | $13\times 249$ | 周期$T$                             |
|   In   | STEP         | $13$           | 更新幅$\Delta$                      |
|   In   | DUTY         | $13\times 249$ | Duty比$D$                           |
|   In   | PHASE        | $13\times 249$ | 位相$P$                             |
|   Out  | DUTY_S       | $13\times 249$ | 静音化済みDuty比$D_s$               |
|   Out  | PHASE_S      | $13\times 249$ | 静音化済み位相$P_s$                 |
|   Out  | DONE         | $1$            | フィルタ処理完了フラグ (デバッグ用) |

### Timer

Timerは, SYS_TIMEと更新周期$T_s$を参照して, $T_s$毎に更新フラグUPDATEをアサートする.

### Silent Filter

Silent Filterでは, 前述の静音化フィルタ処理が行われる.

Silent Filterの計算はUPDATEフラグがアサートされたタイミングで開始される.
また, 計算は$40.96\,\mathrm{MHz}$のクロックで, 振動子に対してパイプライン的に行われる.
計算のレイテンシは12クロックなので, $T_s$を$4\times (249+12)=1044$未満にすることはできない.

計算が終了するとDONEフラグをアサートする.

### Buffer

Silent Filterの計算は並行に行われるため, DUTY_S, PHASE_Sの値も並行に更新される.
Bufferの役割は, これらの更新タイミングを揃える事である.

Bufferによる更新は, DONEフラグがアサートされたタイミングで行われる.
