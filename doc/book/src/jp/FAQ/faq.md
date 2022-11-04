# FAQ

## "No AUTD3 devices found"と表示される

- macOS, linuxで`link::SOEM`を使用する場合, root権限が必要

   ```
   sudo ./examples/example_soem
   ```

   - linuxの場合, `setcap`コマンドで以下の権限を設定することで回避することもできる
   
      ```
      sudo setcap cap_net_raw,cap_net_admin=eip ./examples/example_soem
      ./examples/example_soem
      ```

- (Windows) 最新のnpcapを使用する

- WSL等の仮想マシンは対応していない
   - VirtualBoxなどで動く場合があるが, 挙動は不安定になる

## "One ore more slaves are not responding"と表示される

- Driverを更新する
   - WindowsでRealtekを利用している場合, [公式サイト](https://www.realtek.com/ja/component/zoo/category/network-interface-controllers-10-100-1000m-gigabit-ethernet-pci-express-software)から`Win10 Auto Installation Program (NDIS)`と書かれた方のDriverをインストールすること (Windows 11でも).

- (Windows) 最新のnpcapを使用する

- (Windows) high precisionモードにする
   ```cpp
     auto link = autd3::link::SOEM()
                  ︙
                  .high_precision(true)
                  ︙
                  .build();
   ```

- `send_cycle`と`sync0_cycle`の値を増やす
   ```cpp
     auto link = autd3::link::SOEM()
                  ︙
                  .sync0_cycle(2)
                  .send_cycle(2)
                  ︙
                  .build();
   ```

## `link::SOEM`使用時に送信が頻繁に失敗する

- この問題は
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
  1. `link::TwinCAT`, `link::RemoteTwinCAT`, または, `link::RemoteSOEM`を使用する
  1. USB to Ethernetアダプターを使用する
     - 少なくとも「ASIX AX88179」のチップを採用しているもので正常に動作することが確認されている
     - なお, オンボードではなくとも, PCIe接続のethernetアダプターでも同様の問題が発生する
  1. FreeRunモードにする
  1. `send_cycle`, 及び, `sync0_cycle`の値を増やす
     - ただし, この場合, 送信レイテンシが大きくなる
  1. Linuxやmacを使用する.
     - ただし, 仮想マシンはNG

- 上記以外の状況でも発生した, 或いは, 上記状況でも発生しなかった等の報告があれば, [GitHubのIssue](https://github.com/shinolab/autd3/issues/20)に積極的に報告していただけると幸いである.

## リンクが頻繁に途切れる

- 超音波の出力時にこれが頻発する場合は, 電力が足りているかを確認すること
   - デバイス一台で最大50W消費する

## `link::RemoteTwinCAT`使用時にエラーが出る

- ファイアウォールでブロックされている可能性があるため, ファイアウォールを切るか, TCP/UDPの48898番ポートを開ける.
- クライアントPCのサーバー以外とのLANをすべて切断する.

## その他

- 質問やバグ報告は[GithubのIssue](https://github.com/shinolab/autd3/issues)へお気軽にどうぞ
