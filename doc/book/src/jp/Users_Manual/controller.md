# Controller

ここでは, Controllerクラスに存在するその他の機能を紹介する.

## Check Trials

`check_trials`の値を1以上にすると, デバイスへのデータ送信時に, 送信データがきちんとデバイスで処理されたかどうかを確認するようになる.

```cpp
autd.check_trials = 50;
```

`check_trials`の値が1以上の場合, `send`関数内で最大で`check_trials`回のチェックを行い,
失敗した場合は`false`を返す.

なお, `check_trials`の値が0の場合, `send`関数はチェックを行わず, 必ず`true`を返す.

信頼性の低いlinkを使用する際はOnにしておくことをおすすめする. なお,
Onにすると[送信関数](#送信関数)の実行時間は増加する.

デフォルトは0にセットされている.

## Send intervals

`send_interval`の値は, 連続するフレームの送信間隔, 及び,
[Check Trials](#check-trials)のデータチェック間隔に影響を与える.
これらの間隔は$\SI{500}{\text{μ}s}\times \text{send\_interval}$となる.

```cpp
autd.send_intervals = 1;
```

デフォルトは1にセットされている.

## Force fan

AUTD3のファン制御はAuto, Off, Onの3つのモードが有る. Autoモードでは温度監視ICがICの温度を監視し,
一定温度以上になった際に自動的にファンを起動する. Offモードではファンは常時オフであり, Onモードでは常時オンになる.

モードの切替は, ファン横のジャンパスイッチで行う. 少しわかりにくいが, 下図のようにファン側をショートするとAuto, 真ん中でOff, 右側でOnとなる.

<figure>
  <img src="../fig/Users_Manual/fan.jpg"/>
  <figcaption>AUTD Fan jumper switch</figcaption>
</figure>

Autoモードの場合は温度が高くなると自動的にファンが起動する. `force_fan`フラグはこのAutoモードでファンを強制的に起動するためのフラグである.
実際にフラグが更新されるのは[送信関数](#送信関数)のどれかを呼び出し後になる.

```cpp
autd.force_fan = true;
```

## Read FPGA info

`reads_fpga_info`フラグをONにすると, デバイスがFPGAの状態を返すようになる.
実際にフラグが更新されるのは[送信関数](#送信関数)のどれかを呼び出し後になる.

FPGAの状態は`fpga_info`関数で取得できる.

```cpp
autd.reads_fpga_info = true;
autd.update_flag();
const auto fpga_info = autd.read_fpga_info();
```

`fpga_info`の返り値は`FPGAInfo`のデバイス分だけの`vector`である.

## stop

`autd3::stop`で出力を止めることができる.

```cpp
autd << autd3::stop;
```

## clear

デバイス内のフラグや`Gain`/`Modulation`データ等をクリアする.


```cpp
autd << autd3::clear;
```

## Firmware information

`firmware_infos`関数でFirmwareのバージョン情報を取得できる.

```cpp
for (auto&& firm_info : autd.firmware_infos()) std::cout << firm_info << std::endl;
```

## 送信関数

送信関数とは, 実際にデバイスにデータを送信する関数の総称である. これらの関数を呼び出すことで, `force fan`,
`reads FPGA info`のフラグが更新される.

また, これらの関数は`check_trials`, 及び, `send_interval`の値によって挙動が変わる.

`check_trials`が1以上の場合, これらの関数はデバイスが実際にデータを処理するまで待機する. 特に,
`Modulation`/`STM`を送信する際は1フレーム毎に確認が入るので, 処理時間が大きく増加する可能性がある. また,
`check_trials`回のチェックを行ってもデバイスがデータを処理したことを確認できなかった場合に`false`を返してくる.
`check_trials`が0の場合はデータが処理されたかどうかを確認しない, また, 返り値は常に`true`になる.

また, `send_interval`の値は, 連続するフレームを送信する際の間隔, 及び, 上記チェックの間隔に影響する. 具体的には,
これらの間隔は$\SI{500}{\text{μ}s}\times \text{send\_interval}$となる.

送信関数の一覧は次のとおりである.

- `send`
- `<<` operator
- `update_flag` (非推奨)
- `clear` (非推奨)
- `stop` (非推奨)

## 非同期送信

`send_async`関数, または, stream演算子を使用する場合は, `autd3::async`を予め送っておくことで, データ送信をnon-blockingにすることができる.

```cpp
autd3::modulation::Sine m(...);
autd3::gain::Focus g(...);

autd.send_async(std::move(m), std::move(g));
// or
autd << autd3::async << std::move(m), std::move(g);
```

これらの関数は右辺値のみ受け取ることに注意する.

また, 同期送信と非同期送信を混ぜた場合の動作は保証されないので注意されたい.
