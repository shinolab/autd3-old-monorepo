# Unity

Unity版は座標系がz軸反転の左手系になり, 距離の単位がmになっているので注意すること.
また, 浮動小数点数型に`double`ではなく, `float`を使用している.

## Installation

v2.6からは, Unity Package Manager経由でインストールする.

[npmjs](#npmjs), または, [GitHub](#github)からインストールできるが, 基本的にnpmjsからのインストールを推奨する.

### npmjs

1. Edit→Project Settingsから「Package Manager」を開く
1. Scoped Registryにて以下を追加し, 保存する
  - Name    : shinolab
  - URL     : https://registry.npmjs.com
  - Scope(s): com.shinolab
1. Window→「Package Manager」を開く
1. 左上のPackagesドロップダウンメニューから, 「My Registries」を選択する 
1. 「autd3-unity」を選択し, インストールする

或いは, プロジェクト直下のPackagesフォルダ内の`manifest.json`に以下のように直接追記しても良い.

```json
{
  "scopedRegistries": [
    {
      "name": "shinolab",
      "url": "https://registry.npmjs.com",
      "scopes": [ "com.shinolab" ]
    }
  ],
  "dependencies": {
    "com.shinolab.autd3": "9.0.0",
    ...
```

### GitHub

- Window→Package Managerを開き, 左上の+ボタンから「Add Package from git URL」を選択し, `https://github.com/shinolab/autd3.git#upm/latest`を追加する.
    - 最新版以外を追加する場合は, `https://github.com/shinolab/autd3.git#upm/vX.Y.Z`で指定する.

## Sample

- Unity Package ManagerからSamples/Simpleをインポートする

- また, [autd3sharpのexample](https://github.com/shinolab/autd3/tree/master/cs/example)も合わせて参照されたい.

## Editor拡張

- インストール後にメニューバーにAUTDタブが追加される
    - AUTD/Enumerate Adapters: EtherCATアダプターの一覧表示
    - AUTD/Simulator: RunボタンでUnity用のシミュレータを起動

## Troubleshooting

Q. linuxやmacから実行できない

A. 現在, サポートしていない.

---

その他, 質問があれば[GitHubのissue](https://github.com/shinolab/autd3/issues)に送られたい.
