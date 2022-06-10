# Link

LinkはDeviceとのインターフェースである.
以下の中から一つを選択する必要がある.

## TwinCAT

TwinCATはPCでEherCATを使用する際の唯一の公式の方法である.
TwinCATはWindowsのみをサポートする非常に特殊なソフトウェアであり, Windowsを半ば強引にリアルタイム化する.

また, 特定のネットワークコントローラが必要になるため,
[対応するネットワークコントローラの一覧](https://infosys.beckhoff.com/english.php?content=../content/1033/tc3_overview/9309844363.html&id=)を確認すること.

> Note: 或いは, TwinCATのインストール後に, `C:/TwinCAT/3.1/Driver/System/TcI8254x.inf`に対応するデバイスのVendor IDとDevice IDが書かれているので, デバイスマネージャー→イーサネットアダプタ→プロパティ→詳細→ハードウェアIDと照らし合わせることでも確認できる.

### How to install TwinCAT

前提として, TwinCATはHyper-VやVirtual Machine Platformと共存できない.
そのため, これらのfeatureを無効にする必要がある.
これには, 例えば, PowerShellを管理者権限で起動し,
```
Disable-WindowsOptionalFeature -Online -FeatureName Microsoft-Hyper-V-Hypervisor
Disable-WindowsOptionalFeature -online -featurename VirtualMachinePlatform
```
と打ち込めば良い.

まず, TwinCAT XAEを[公式サイト](https://www.beckhoff.com/en-en/)からダウンロードする.
ダウンロードには登録 (無料) が必要になる.

ダウンロードしたインストーラを起動し, 指示に従う.
**この時, TwinCAT XAE Shell installにチェックを入れ, Visual Studio Integrationのチェックを外すこと.**

インストール後に再起動し, `C:/TwinCAT/3.1/System/win8settick.bat`を管理者権限で実行し, 再び再起動する.

最後に, SDK内の`AUTDServer/AUTD.xml`を`C:/TwinCAT/3.1/Config/Io/EtherCAT`にコピーする.

### AUTDServer

TwinCATのLinkを使うには, まず, `AUTDServer/AUTDServer.exe`を実行する.
AUTDServer実行後に, IPアドレスの入力を求められるがここは空欄のままEnterすればよい.
すると, しばらくしてTwinCAT XAE Shellが起動する.
最後に, Shellを閉じるかを聞かれるが, 以下の設定がまだなら`No`を入力し, 設定を続ける.
以下の設定が済んでいるなら, 閉じてしまってかまわない.

> Note: もし閉じてしまった場合は, `%Temp%/TwinCATAUTDServer/TwinCATAUTDServer.sln`をTcXaeShell Applicationとして開けば良い. `%Temp%`は環境変数で, 普通は`C:/Users/(user name)/AppData/Local/Temp`である.

なお, AUTDServerはPCの電源を切る, スリープモードに入る等でLinkが途切れるので, その都度実行し直すこと.

#### Install Driver

初回はEherCAT用のドライバのインストールが必要になる.
TwinCAT XAE Shell上部メニューからTwinCAT→Show Realtime Ethernet Compatible Devicesを開きCompatible devicesの中の対応デバイスを選択肢Installをクリックする.
なお, Compatible devicesに何も表示されていない場合はそのPCのイーサネットデバイスはTwinCATに対応していない.

#### License

また, 初回はライセンス関係のエラーが出るので, XAE ShellでSolution Explorer→SYSTEM→Licenseを開き, "7 Days Trial License ..."をクリックし, 画面に表示される文字を入力する.
なお. ライセンスは7日間限定のトライアルライセンスだが, 切れたら再び同じ作業を行うことで再発行できる.
ライセンスを発行し終わったら, TwinCAT XAE Shellを閉じて, 再び"AUTDServer.exe"を実行する.

### Trouble shooting

大量のDeviceを使用しようとすると, 下の図のようなエラーが発生することがある.
このときは, `settings.ini`内の`CycleTicks`との値を増やし, AUTDServerを再び実行する.
`CycleTicks`の値はエラーが出ない中で, 可能な限り小さな値が望ましい.
デフォルトは1であり, どの程度の値にすべきかは接続している台数に依存する.
例えば, 9台の場合は2, 3程度の値にしておけば動作するはずである.

<figure>
  <img src="https://raw.githubusercontent.com/shinolab/autd3/master/book/src/fig/Users_Manual/tcerror.jpg"/>
  <figcaption>TwinCAT error when using 9 devices</figcaption>
</figure>

## RemoteTwinCAT

前述の通り, AUTD3とTwinCATを使う場合はWindows OSと特定のネットワークアダプタが必要になる.
しかし, Windows以外のPCで開発したい需要も多い (後述のSOEMもマルチプラットフォームで動作する).
その場合は, RemoteTwinCAT linkを用いて遠隔からTwinCATを操作することができる.

RemoteTwinCATを使用する場合はPCを2台用意する必要がある.
この時, 片方のPCは上記のTwinCAT linkが使えるである必要がある.
このPCをここでは"サーバ"と呼ぶ.
一方, 開発側のPC, 即ちSDKを使用する側は特に制約はなく, サーバと同じLANに繋がっていれば良い, こちらをここでは"クライアント"と呼ぶ.

まず, サーバとAUTDデバイスを接続する.
この時使うLANのアダプタはTwinCAT linkと同じく, TwinCAT対応のアダプタである必要がある.
また, サーバとクライアントを別のLANで繋ぐ.
こちらのLANアダプタはTwinCAT対応である必要はない[^fn_remote_twin].
そして, サーバとクライアント間のLANのIPを確認しておく.
ここでは例えば, サーバ側が"169.254.205.219", クライアント側が"169.254.175.45"だったとする.
次に, サーバでAUTDServerを起動する.
起動後にIPの入力を求められるが, ここでクライアント側のIP (この例だと`169.254.175.45`) を入力する.
また, 最後に"No"を入力し, TwinCATAUTDServerを開いたままにしておく.
以下の図のように, System→Routesを開き, Current RouteタブのAmsNetId及び, NetId ManagementタブのLocal NetIdを確認する.

<figure>
  <img src="https://raw.githubusercontent.com/shinolab/autd3/master/book/src/fig/Users_Manual/Current_Route.jpg"/>
  <img src="https://raw.githubusercontent.com/shinolab/autd3/master/book/src/fig/Users_Manual/NetId_Management.jpg"/>
  <figcaption>AmsNetId/Local NetId</figcaption>
</figure>

ここでは, それぞれ"169.254.175.45.1.1", "172.16.99.194.1.1"だったとする.
この時, クライアント側は`autd3/link/remote_twincat.hpp`ヘッダーをincludeして,
```cpp
#include "autd3/link/remote_twincat.hpp"

...
  const string remote_ipv4_addr = "169.254.205.219";
  const string remote_ams_net_id = "172.16.99.194.1.1";
  const string local_ams_net_id = "169.254.175.45.1.1";
  auto link = link::RemoteTwinCAT(remote_ipv4_addr, remote_ams_net_id).local_ams_net_id(local_ams_net_id).build();
```
のようにすれば良い.

なお, TCP関係のエラーが出る場合は, ファイアウォールでADSプロトコルがブロックされている可能性がある.
その場合は, ファイアウォールの設定でTCP/UDPの48898番ポートの接続を許可する.

## SOEM

[SOEM](https://github.com/OpenEtherCATsociety/SOEM)は有志が開発しているOpen-sourceのEherCAT Masterライブラリである.
TwinCATとは異なり通常のWindows上で動作するためリアルタイム性は保証されない.
そのため, 基本的にTwinCATを使用することを推奨する.
SOEMを使用するのはやむを得ない理由があるか, 開発時のみに限定するべきである.
一方, SOEMはクロスプラットフォームで動作し, インストールも単純という利点がある.

Windowsの場合は, [npcap](https://nmap.org/npcap/), または, [WinPcap](https://www.winpcap.org/)を予めインストールしておくこと.
npcapはWinPcapの後継であり, こちらの利用を推奨する.
**なお, npcapをインストールする場合は"WinPcap API compatible mode"でインストールすること.**
Linux/macの場合は, 特に準備は必要ない.

SOEMのLinkを使用する際は`autd3/link/soem.hpp`ヘッダーをインクルードする.
```cpp
#include "autd3/link/soem.hpp"

...
  auto link = link::SOEM(ifname, autd.geometry().num_devices()).build();
```
`SOEM()`の第1引数はインターフェース名で, 第2引数はデバイスの数である.
インターフェース名はAUTD3デバイスに接続しているehernetインターフェース名である.
これの一覧は, `SOEM::enumerate_adapters`関数によって取得できる.
```cpp
  const auto adapters = link::SOEM::enumerate_adapters();
  for (auto&& [desc, name] : adapters) cout << desc << ", " << name << endl;
```

なお, SOEMも大量のDeviceを使用すると挙動が不安定になる時がある[^fn_soem].
このときは, `cycle_ticks`関数を使用し, その値を増やす.
```cpp
  auto link = link::SOEM(ifname, autd.geometry().num_devices())
                .cycle_ticks(4)
                .build();
```
`cycle_ticks`の値はエラーが出ない中で, 可能な限り小さな値が望ましい.
デフォルトは2であり, どの程度の値にすべきかは接続している台数に依存する.
例えば, 9台の場合は4程度の値にしておけば動作するはずである.

また, SOEM Linkは回復不能なエラー (例えば, ケーブルが抜けるなど) が発生したときのコールバックを設定することができる[^fn_soem_err].
callbackはエラーメッセージを引数に取る.
```cpp
  auto link = link::SOEM(ifname, autd.geometry().num_devices())
                .cycle_ticks(4)
                .on_lost([](const string& msg) {
                  cerr << "Link is lost\n";
                  cerr << msg;
                  quick_exit(-1);
                })
                .build();
```

さらに, Windowsの場合はHigh Precisionモードの設定ができる.
```cpp
  auto link = link::SOEM(ifname, autd.geometry().num_devices())
                .cycle_ticks(4)
                .on_lost([](const string& msg) {
                  cerr << "Link is lost\n";
                  cerr << msg;
                  quick_exit(-1);
                })
                .high_precision(true)
                .build();
```
High Precisionモードを`true`にすると, より高精度なタイマが使用できるが, 代わりにCPUの負荷が高くなる.

## Emulator

Emulator linkは[autd-emulator](https://github.com/shinolab/autd-emulator) を使用する際に使うLinkである.

使用の前に, AUTD Emulatorを実行しておく必要がある.

EmulatorのLinkを使用する際は`autd3/link/emulator.hpp`ヘッダーをインクルードする.
```cpp
#include "autd3/link/emulator.hpp"

...

  auto link = autd::link::Emulator(autd.geometry()).port(50632).build();
```
`Emulator()`の引数はGeometryである.
ポート番号はautd-emulatorの設定と同じにしておく.

[^fn_remote_twin]: 無線LANでも可

[^fn_soem]: TwinCATよりは緩く, 普通に動くこともある.

[^fn_soem_err]: なお, 回復不能なので直ちに終了するくらいしかできることはない.
