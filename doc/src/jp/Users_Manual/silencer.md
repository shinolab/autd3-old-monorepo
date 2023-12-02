# Silencer

AUTD3には出力を静音化するためのSilencerが用意されている.
Silencerは, 振動子の駆動信号の急激な変動を抑制し, 静音化する.

## 理論

詳細は鈴木らの論文[^suzuki2020]を参照されたい.

大まかに概要を述べると, 

* 振幅変調された超音波は可聴音を生じる
* 超音波振動子を駆動する際に, 位相変化が振幅変動を引き起こす
    * したがって, 可聴音の騒音が生じる
* 位相変化を線形に補間し, 段階的に変化させることで振幅変動を抑えられる
    * したがって, 騒音を低減できる
* 補間を細かくやると, その分だけ騒音を小さくできる

となる.

## Silencerの設定

Silencerの設定には`Silencer`を送信する.

`Silencer`には`step`を設定できる.
詳細は以下を参照されたいが, 大まかには`step`を小さくするほどより静かになる.

Silencerはデフォルトで適当な値に設定されている.
Silencerを無効化する場合は, `disable`を送信する.

```rust,edition2021
{{#include ../../codes/Users_Manual/silencer_0.rs}}
```

```cpp
{{#include ../../codes/Users_Manual/silencer_0.cpp}}
```

```cs
{{#include ../../codes/Users_Manual/silencer_0.cs}}
```

```python
{{#include ../../codes/Users_Manual/silencer_0.py}}
```


## Silencerによる位相の変化

Silencerは位相$P$の変化を線形補間し, 段階的にすることで静音化を行う.
即ち, 位相$P$の時系列データを(単純)移動平均フィルタに通しているのにほとんど等しい.
ただし, 位相データが周期的であるという事を考慮している点で異なる.

例えば, 超音波の周期$T$が$T=12$の場合を考える. 即ち, $P=0$が$0\,\mathrm{rad}$, $P=12$が$2\pi\,\mathrm{rad}$に対応する. 
ここで, 時刻$t_s$で, 位相が$P=2$から$P=6$に変化したとする.
この時, Silencerによる位相変化は以下の図のようになる.

<figure>
  <img src="../fig/Users_Manual/silent/phase.svg"/>
<figcaption>位相$P$の変化</figcaption>
</figure>

一方, 時刻$t_s$で, 位相が$P=2$から$P=10$に変化したとする.
この時のSilencerによる位相変化は以下の図のようになる.
これは, $P=10$よりも, $P=-2$のほうが近いためである.

<figure>
  <img src="../fig/Users_Manual/silent/phase2.svg"/>
<figcaption>位相$P$の変化 (位相変化量が$\pi$より大きい場合)</figcaption>
</figure>

つまり, Silencerは現在の$P$と目標値$P_r$に対して
$$
    P \leftarrow \begin{cases}
        P + \mathrm{sign}(P_r - P) \min (|P_r - P|, \Delta) & \text{if } |P_r - P| \le T/2\\
        P - \mathrm{sign}(P_r - P) \min (|P_r - P|, \Delta) & \text{(otherwise)}\\
    \end{cases},
$$
として位相$P$を更新する.
ここで, $\Delta$は1ステップ当たりの更新量 (`Silencer`の`step`) を表す.
なお, 更新周波数は$\ufreq$となっている.

$\Delta$が小さいほど, 位相変化はなだらかになり騒音が抑制される.

<figure>
  <img src="../fig/Users_Manual/silent/duty.svg"/>
<figcaption>$\Delta$による変化の違い</figcaption>
</figure>

この実装の都合上, 移動平均フィルタとは異なる挙動を示す場合がある.
一つは, 上に示した位相変化量が$\pi$より大きい場合であり, もう一つが, 途中でもう一度位相が変化する場合である.
この時の位相変化の例を以下に示す.
元時系列に対する忠実度という観点では移動平均フィルタが正しいが, 位相変化量が$\pi$より大きい場合を考慮したり, $\Delta$を可変にする (即ち, フィルタ長を可変にする) のが大変なため現在のような実装となっている.

<figure>
  <img src="../fig/Users_Manual/silent/mean.svg"/>
<figcaption>移動平均フィルタとの比較</figcaption>
</figure>

## SilencerによるDuty比の変化

振幅変動が騒音を引き起こすので, Duty比$D$も同等のフィルタをかけることでAMによる騒音を抑制できる.

Duty比$D$は位相とは異なり周期的ではないので, 現在の$D$と目標値$D_r$に対して
$$
    D \leftarrow D + \mathrm{sign}(D_r - D) \min (|D_r - D|, \Delta),
$$
のように更新する.

[^suzuki2020]: Suzuki, Shun, et al. "Reducing amplitude fluctuation by gradual phase shift in midair ultrasound haptics." IEEE transactions on haptics 13.1 (2020): 87-93.
