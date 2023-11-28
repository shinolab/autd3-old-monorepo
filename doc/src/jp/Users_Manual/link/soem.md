# SOEM

[SOEM](https://github.com/OpenEtherCATsociety/SOEM)は有志が開発しているオープンソースのEherCAT Masterライブラリである.
TwinCATとは異なり通常のWindows上で動作するためリアルタイム性は保証されない.
そのため, 基本的にTwinCATを使用することを推奨する.
SOEMを使用するのはやむを得ない理由があるか, 開発時のみに限定するべきである.
一方, SOEMはクロスプラットフォームで動作し, インストールも単純という利点がある.

Windowsの場合は, [npcap](https://nmap.org/npcap/)を**WinPcap API compatible mode**でインストールしておくこと.
Linux/macOSの場合は, 特に準備は必要ない.

> NOTE: `SOEM`を使用する場合, `Controller`をopenしてから10-20秒ほどはEtherCATスレーブ同士の同期が完了していない可能性があるので注意されたい. (この時間は個体差や同期信号/送信サイクルによって変化する.)
> この期間, デバイス間の超音波の同期は保証されない.

[[_TOC_]]

## SOEMリンクのAPI

SOEMリンクで指定できるオプションは以下の通りである.

```rust,should_panic,edition2021
{{#include ../../../codes/Users_Manual/link/soem_0.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/link/soem_0.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/link/soem_0.cs}}
```

```python
{{#include ../../../codes/Users_Manual/link/soem_0.py}}
```

- `ifname`: ネットワークインタフェース名. デフォルトでは空白であり, 空白の場合はAUTD3デバイスが接続されているネットワークインタフェースを自動的に選択する.
- `buf_size`: 送信キューバッファサイズ. 通常は変更する必要はない.
- `on_err`: 何らかのエラーが発生したときのコールバック. コールバック関数はエラーメッセージを引数に取る.
- `state_check_interval`: エラーが出ているかどうかを確認する間隔. デフォルトは$\SI{100}{ms}$.
- `on_lost`: 回復不能なエラー (例えば, ケーブルが抜けるなど) が発生したときのコールバック[^fn_soem_err]. コールバック関数はエラーメッセージを引数に取る.
- `sync0_cycle`: 同期信号の周期. デフォルトは2 (単位は$\SI{500}{us}$).
- `send_cycle`: 送信サイクル. デフォルトは2 (単位は$\SI{500}{us}$).
    - `SOEM`も大量のデバイスを接続すると挙動が不安定になる場合がある[^fn_soem]. このときは, `sync0_cycle`と`send_cycle`の値を増やす. これら値はエラーが出ない中で, 可能な限り小さな値が望ましい. デフォルトは2であり, どの程度の値にすべきかは接続している台数に依存する. 例えば, 9台の場合は3, 4程度の値にしておけば動作するはずである.
- `timer_strategy`: タイマーの戦略. デフォルトは`Sleep`である.
    - `Sleep`       : 標準ライブラリのsleepを用いる
    - `BusyWait`    : ビジーウェイトを用いる. 高解像度だが, CPU負荷が高い.
    - `NativeTimer` : OSのタイマー機能を用いる
        - WindowsではTimerQueueTimer, linuxではPOSIXタイマー, macOSではGrand Central Dispatch Timer
- `sync_mode`: 同期モード. 詳細は[Beckhoffの説明](https://infosys.beckhoff.com/english.php?content=../content/1033/ethercatsystem/2469122443.html&id=)を参照されたい.

# RemoteSOEM

このLinkは`SOEM`を動かすサーバーPCとユーザプログラムを動かすクライアントPCを分離するためのものである.

`RemoteSOEM`を使用する場合はPCを2台用意する必要がある.
この時, 片方のPCは`SOEM` linkが使えるである必要がある.
このPCをここでは"サーバ"と呼ぶ.
一方, 開発側のPC, 即ちSDKを使用する側は特に制約はなく, サーバと同じLANに繋がっていれば良い, こちらをここでは"クライアント"と呼ぶ.

まず, サーバとAUTDデバイスを接続する.
また, サーバとクライアントを別のLANで繋ぐ[^fn_remote_soem].
そして, サーバとクライアント間のLANのIPを確認しておく.
ここでは例えば, サーバ側が`172.16.99.104`, クライアント側が`172.16.99.62`だったとする.

## AUTD Server

`RemoteSOEM`を使用する場合, サーバに`AUTD Server`をインストールする必要がある.
[GitHub Releases](https://github.com/shinolab/autd3/releases)にてインストーラを配布しているので, これをダウンロードし, 指示に従ってインストールする.

`AUTD Server`を実行すると, 以下のような画面になるので, `SOEM`タブを開く.

<figure>
  <img src="../../fig/Users_Manual/autdserver_remotesoem.jpg"/>
</figure>

ポートに適当なポート番号を指定し, `Run`ボタンを押す.

AUTD3デバイスが見つかり, クライアントとの接続待ちである旨のメッセージが表示されれば成功である.

なお, `AUTD Server`では`SOEM`と同等のオプションを指定できる.

## RemoteSOEMリンクのAPI

`RemoteSOEM`のコンストラクタでは, <サーバのIP:ポート>を指定する.

```rust,should_panic,edition2021
{{#include ../../../codes/Users_Manual/link/remote_soem_0.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/link/remote_soem_0.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/link/remote_soem_0.cs}}
```

```python
{{#include ../../../codes/Users_Manual/link/remote_soem_0.py}}
```

## ファイアウォール

TCP関係のエラーが出る場合は, ファイアウォールでブロックされている可能性がある.
その場合は, ファイアウォールの設定でTCP/UDPの指定したポートの接続を許可する.

[^fn_soem_err]: なお, 回復不能なので直ちに終了するくらいしかできることはない.

[^fn_soem]: TwinCATよりは緩く, 普通に動くこともある.

[^fn_remote_soem]: 無線LANでも可
