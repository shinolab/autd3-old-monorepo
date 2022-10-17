# C\#

[autd3sharp](https://github.com/shinolab/autd3/tree/master/cs) provides a wrapper for **.Net Standard 2.1**.

## Installation

The library is available at [NuGet](https://www.nuget.org/packages/autd3sharp).

### Installation for Unity

If you want to use it from Unity, use the unitypackage available at [GitHub Release](https://github.com/shinolab/autd3/releases).

After installing this package, go to `Project Settings > Player` and check `Allow 'unsafe' code`. 
Also, add `-nullable:enable` in `Additional Compiler Arguments` to suppress warnings.

**Note that the Unity version has a left-handed coordinate system with z-axis inversion, and the unit of distance is in meters.**

## Usage

The C\# version is designed to be basically the same as the C++ version.

For example, the following code is equivalent to [Getting Started](../Users_Manual/getting_started.md).

```csharp
{{#include ../../../samples/cs/Program.cs}}
```

See [autd3sharp's example](https://github.com/shinolab/autd3/tree/master/cs/example) for more detailed examples.

## Troubleshooting

Q. I can't run the program from linux or mac.

A. Run as administrator.

```
sudo dotnet run
```

---

Q. Cannot run from Ubuntu 20.04

A. Specify runtime

```
sudo dotnet run -r ubuntu-x64
```

---

Q. Cannot use from .Net framework

A. Not supported. If you copy and paste the whole source code, it might work.

---

If you have any other questions, please send them to [GitHub issue](https://github.com/shinolab/autd3/issues).
