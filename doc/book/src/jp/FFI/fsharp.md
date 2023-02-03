# F\#

[autd3sharp](https://github.com/shinolab/autd3/tree/master/cs)は **.Net Standard 2.1** に対応したラッパーを提供している.

## Installation

[NuGet](https://www.nuget.org/packages/autd3sharp)で公開しているので, NuGetでインストールすること.

## Usage

基本的には, C++版と同じになるように設計している.

たとえば, [チュートリアル](../Users_Manual/getting_started.md)と等価なコードは以下のようになる.

```fsharp
{{#include ../../../samples/fs/Program.fs}}
```

より詳細なサンプルは[autd3sharpのexample](https://github.com/shinolab/autd3/tree/master/fs/example)を参照されたい.

## Troubleshooting

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
