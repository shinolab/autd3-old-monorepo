# FAQ

[[_TOC_]]

## "No AUTD3 devices found"と表示される

- macOS, linuxで`link::SOEM`を使用する場合, root権限が必要

   - linuxの場合, `setcap`コマンドで以下の権限を設定することで回避することもできる
   
      ```shell
      sudo setcap cap_net_raw,cap_net_admin=eip <your executable file>
      ```

   - macOSの場合, `/dev/bpf*`ファイルに読み取り権限を追加することで回避することもできる
   
      ```shell
      sudo chmod +r /dev/bpf*
      ```

- (Windows) 最新のnpcapを使用する

- WSL等の仮想マシンは対応していない
   - VirtualBoxなどで動く場合があるが, 挙動は不安定になる

## "One ore more slaves are not responding"と表示される

- Driverを更新する
   - WindowsでRealtekを利用している場合, [公式サイト](https://www.realtek.com/ja/component/zoo/category/network-interface-controllers-10-100-1000m-gigabit-ethernet-pci-express-software)から`Win10 Auto Installation Program (NDIS)`と書かれた方のDriverをインストールすること (Windows 11でも).

- (Windows) 最新のnpcapを使用する

- `send_cycle`と`sync0_cycle`の値を増やす

## `link::SOEM`使用時に送信が頻繁に失敗する

- この問題は
   * `sync_mode`を`DC`にしている

   かつ,

   * オンボードのethernetインターフェースを使用している

  かつ, 以下のいずれかの状況で発生することが確認されている

   * RealSense, Azure Kinect, Webカメラ等を使用する
      * 基本的にカメラをアクティブにした時点で発生
   * 動画や音声を再生する
      * または, インターネットブラウザで動画サイト (Youtube等) を開く
   * Unityを使用する
      * 起動するだけで発生
   * Blenderでアニメーションを再生する
      * その他の操作 (モデリング等) は問題ない

- この問題の回避策としては, 以下のいずれかを試されたい
  1. `timer_strategy`を`NativeTimer`にする
  1. `sync_mode`を`FreeRun`にする
  1. Linuxやmacを使用する.
     - ただし, 仮想マシンはNG
  1. `TwinCAT`, `RemoteTwinCAT`, または, `RemoteSOEM`リンクを使用する
  1. USB to Ethernetアダプターを使用する
     - 少なくとも「ASIX AX88179」のチップを採用しているもので正常に動作することが確認されている
     - なお, オンボードではなくとも, PCIe接続のethernetアダプターでも同様の問題が発生する

- 上記以外の状況でも発生した, 或いは, 上記状況でも発生しなかった等の報告があれば, [GitHubのIssue](https://github.com/shinolab/autd3/issues/20)に積極的に報告していただけると幸いである.

## リンクが頻繁に途切れる

- 超音波の出力時にこれが頻発する場合は, 電力が足りているかを確認すること
   - デバイス一台で最大50W消費する

## `RemoteTwinCAT`リンク使用時にエラーが出る

- ファイアウォールでブロックされている可能性があるため, ファイアウォールを切るか, TCP/UDPの48898番ポートを開ける.
- クライアントPCのサーバー以外とのLANをすべて切断する.

## 振動子の位相/振幅データにアクセスするには?

1. [`Transform`](../gain/transform.md)を利用する.
1. 自分で所望の`Gain`を作成する. [Gainの自作](../advanced_examples/custom_gain.md)を参照.

## AM変調データにアクセスするには?

1. [`Transform`](../modulation/transform.md)を利用する.
1. 自分で所望の`Modulation`を作成する. [Modulationの自作](../advanced_examples/custom_modulation.md)を参照.

## その他

- 質問やバグ報告は[GithubのIssue](https://github.com/shinolab/autd3/issues)へお気軽にどうぞ
