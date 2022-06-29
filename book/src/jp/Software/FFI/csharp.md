# C\#

[autd3sharp](https://github.com/shinolab/autd3sharp)は **.Net Standard 2.1** に対応したラッパーを提供している.

## Installation

[NuGet](https://www.nuget.org/packages/autd3sharp)で公開しているので, NuGetでインストールすること.

### Installation for Unity

Unityから使う場合は, [GitHub Release](https://github.com/shinolab/autd3sharp/releases)にてunitypackageを公開しているので, そちらを使用すること.

本パッケージをインストールしたあと, `Project Settings > Player`から`Allow 'unsafe' code`にチェックをいれること. また警告を抑制するため, `Additional Compiler Arguments`に`-nullable:enable`を追加すること.

**なお, Unity版は座標系がz軸反転の左手系になり, 距離の単位がmになっているので注意すること.**

## Usage

基本的には, C++版と同じになるように設計している.

たとえば, [Getting Started](../Users_Manual/getting_started.md)と等価なコードは以下のようになる.

```csharp
using AUTD3Sharp;
using AUTD3Sharp.Utils;

namespace example
{
    internal class Program
    {
        public static string GetIfname()
        {
            var adapters = SOEM.EnumerateAdapters();
            var etherCATAdapters = adapters as EtherCATAdapter[] ?? adapters.ToArray();
            foreach (var (adapter, index) in etherCATAdapters.Select((adapter, index) => (adapter, index)))
                Console.WriteLine($"[{index}]: {adapter}");

            Console.Write("Choose number: ");
            int i;
            while (!int.TryParse(Console.ReadLine(), out i)) { }
            return etherCATAdapters.ElementAt(i).Name;
        }

        public static void Main()
        {
            var autd = new Controller();
            autd.AddDevice(Vector3d.Zero, Vector3d.Zero);

            var ifname = GetIfname();
            var link = new SOEM(ifname, autd.NumDevices, 2);
            if (!autd.Open(link))
            {
                Console.WriteLine(Controller.LastError);
                return;
            }

            autd.CheckTrials = 50;

            autd.Clear();

            autd.Synchronize();

            var firmList = autd.FirmwareInfoList().ToArray();
            foreach (var (firm, index) in firmList.Select((firm, i) => (firm, i)))
                Console.WriteLine($"AUTD {index}: {firm}");

            var config = new SilencerConfig();
            autd.Send(config);

            const double x = Controller.TransSpacing * ((Controller.NumTransInX - 1) / 2.0);
            const double y = Controller.TransSpacing * ((Controller.NumTransInY - 1) / 2.0);
            const double z = 150.0;
            var g = new Focus(new Vector3d(x, y, z));
            var m = new Sine(150);
            autd.Send(m, g);

            Console.ReadKey(true);

            autd.Close();
        }
    }
}
```

より詳細なサンプルは[autd3sharpのexample](https://github.com/shinolab/autd3sharp/tree/master/example)を参照されたい.

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

A. サポートしてないです. ソースコードを丸々コピペすれば動くかもしれません.

---

その他, 質問があれば[GitHubのissue](https://github.com/shinolab/autd3sharp/issues)にてお願いします.
