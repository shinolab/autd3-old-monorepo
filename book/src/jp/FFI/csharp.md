# C\#

[autd3sharp](https://github.com/shinolab/autd3/tree/master/cs)は **.Net Standard 2.1** に対応したラッパーを提供している.

## Installation

[NuGet](https://www.nuget.org/packages/autd3sharp)で公開しているので, NuGetでインストールすること.

### Installation for Unity

Unityから使う場合は, [GitHub Release](https://github.com/shinolab/autd3/releases)にてunitypackageを公開しているので, そちらを使用すること.

本パッケージをインストールしたあと, `Project Settings > Player`から`Allow 'unsafe' code`にチェックをいれること. また警告を抑制するため, `Additional Compiler Arguments`に`-nullable:enable`を追加すること.

**なお, Unity版は座標系がz軸反転の左手系になり, 距離の単位がmになっているので注意すること.**

## Usage

基本的には, C++版と同じになるように設計している.

たとえば, [Getting Started](../Users_Manual/getting_started.md)と等価なコードは以下のようになる.

```csharp
{{#include ../../../samples/cs/Program.cs}}
```

より詳細なサンプルは[autd3sharpのexample](https://github.com/shinolab/autd3/tree/master/cs/example)を参照されたい.

## Trouble shooting

Q. linuxやmacから実行できない

A. 管理者権限で実行する

```
sudo dotnet run
```

---

Q. Ubuntu 20.04から実行できない

A. runtimeを指定する

```
sudo dotnet run -r ubuntu-x64
```

---

Q. .Net frameworkから使用できない

A. サポートしてない. ソースコードを丸々コピペすれば動くかもしれない.

---

その他, 質問があれば[GitHubのissue](https://github.com/shinolab/autd3/issues)に送られたい.
