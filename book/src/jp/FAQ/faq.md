# FAQ

## macOS, linuxで動かせない.

- `link::SOEM`を使用する場合, root権限が必要.

## `link::SOEM`使用時にうまく動かない

- この問題は
   * オンボードのethernetインターフェースを使用している

  かつ, 以下のいずれかの状況で発生することが確認されている

   * RealSense, Azure Kinect, Webカメラ等を使用する
      * 基本的にカメラをアクティブにした時点で発生
   * 動画や音声を再生する
      * または, インターネットブラウザでそのためのサイト (Youtube等) を開く
   * Unityを使用する
      * 起動するだけで発生
   * Blenderでアニメーションを再生する
      * その他の操作 (モデリング等) は問題ない

- この問題の回比較としては, 以下のいずれかを試されたい
  1. USB to Ethernetアダプターを使用する
     - なお, オンボードではなくとも, PCIe接続のethernetアダプターでも同様の問題が発生する
  1. `link::TwinCAT`, または, `link::RemoteTwinCAT`を使用する
  1. `cycle_ticks`の値を増やす. `cycle_ticks`を20程度にすると動く場合がある
     - ただし, この場合, 送信レイテンシが大きくなる

- 上記以外の状況でも発生した, 或いは, 上記状況でも発生しなかった等の報告があれば, [GitHubのIssue](https://github.com/shinolab/autd3/issues/20)に積極的に報告していただけると幸いである.

## その他

- 質問やバグ報告は[GithubのIssue](https://github.com/shinolab/autd3/issues)へお気軽にどうぞ
