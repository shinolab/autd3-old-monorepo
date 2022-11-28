# Controller

ここでは, Controllerクラスに存在するその他の機能を紹介する.

## 音速の設定

SDK内部では, 波長を音速/周波数で計算しており, 種々の`Gain`では内部計算に波長を用いている.

音速は, 環境によって変わるので適切な値を設定する必要があり, 音速がずれていると, 波長がずれ, 正しく超音波を集束できない可能性がある.

音速は`Controller`クラスの`set_sound_speed`関数で設定する.
単位は$\SI{}{mm/s}$である.

```cpp
autd.set_sound_speed(340e3); // mm/s
```

なお, 音速は実際には振動子ごとに設定される.
上記は, 以下のエイリアスになっている.

```cpp
for (auto& tr : autd.geometry()) tr.sound_speed = sound_speed;
```

また, 室温から音速を設定するユーティリティ関数も用意されている.
引数の単位はセルシウス温度である.

```cpp
autd.set_sound_speed_from_temp(15); // 15℃
```

## Ack Check Timeout/応答確認タイムアウト時間

`ack_check_timeout`の値を0より大きくすると, デバイスへのデータ送信時に, 送信データがデバイスで処理されたかどうかを確認するようになる.

```cpp
autd.set_ack_check_timeout(std::chrono::milliseconds(20));
```

`ack_check_timeout`の値が0より大きい場合, 送信時に送信データがデバイスで処理されるか, `ack_check_timeout`時間が経過するまで待機する.
送信データがデバイスで処理されたのが確認できた場合に送信関数は`true`を返し, そうでない場合は`false`を返す.
なお, データが送信されたかどうかのチェックは後述の[Send intervals](#send-intervals)の時間間隔で行われる.

`ack_check_timeout`の値が0の場合, 送信関数はチェックを行わず, 必ず`true`を返す.

確実にデータを送信したい場合はこれを適当な値に設定しておくことをおすすめする.
なお, `ack_check_timeout`を設定すると[送信関数](#送信関数)の実行時間は増加する.

デフォルトは0にセットされている.

## Send intervals

`send_interval`の値は, 連続するフレームの送信間隔, 及び, [Ack Check Timeout](#ack-check-timeout)のデータチェック間隔になる.

```cpp
autd.set_send_interval(std::chrono::milliseconds(1));
```

デフォルトは$\SI{500}{\text{μ}s}$にセットされている.

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
autd << autd3::update_flag;
const auto fpga_info = autd.read_fpga_info();
```

`fpga_info`の返り値は`FPGAInfo`のデバイス分だけの`vector`である.

`FPGAInfo`には現在, デバイスの温度が一定以上であるかどうか (すなわち, Autoモードでファンが起動しているかどうか) を表すフラグがある.

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

送信関数とは, 実際にデバイスにデータを送信する関数の総称である.
これらの関数を呼び出すことで, `force fan`, `reads FPGA info`のフラグが更新される.

また, これらの関数は`ack_check_timeout`, 及び, `send_interval`の値によって挙動が変わる.

`ack_check_timeout`が0より大きい場合, これらの関数はデバイスがデータを処理するか指定のtimeout時間が経過するまで待機する.
特に, `Modulation`/`STM`を送信する際は1フレーム毎に確認が入るので, 処理時間が大きく増加する可能性がある.
また, `ack_check_timeout`時間経過後もデバイスがデータを処理したことを確認できなかった場合に`false`を返してくる.
`ack_check_timeout`が0の場合はデータが処理されたかどうかを確認しない, また, 返り値は常に`true`になる.

また, `send_interval`の値は, 連続するフレームを送信する際の間隔, 及び, 上記チェックの間隔に影響する.

送信関数の一覧は次のとおりである.

- `send`
- `<<`演算子
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
