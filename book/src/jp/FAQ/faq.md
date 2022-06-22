# FAQ

## macOS, linuxで動かせない.

- `link::SOEM`を使用する場合, root権限が必要です.

## RealSense, Azure Kinect, Webカメラ使用時に動かない

- `link::SOEM`をオンボードのethernetインターフェースと共に使用する場合に上手く動かない場合があります. その場合は,
  以下の回避策のいずれかを試してみてください.
  1. USB to Ethernetアダプターを使用する.
     - なお, オンボードではないPCIeのethernetアダプター使用時にも同様の問題が発生します.
  1. `link::TwinCAT`, または, `link::RemoteTwinCAT`を使用する.
  1. `cycle_ticks`の値を増やす. `cycle_ticks`を20程度にすると動く場合があります.
     - ただし, この場合, 送信レイテンシが大きくなります.

## その他

- 質問やバグ報告は[GithubのIssue](https://github.com/shinolab/autd3/issues)へお気軽にどうぞ.
