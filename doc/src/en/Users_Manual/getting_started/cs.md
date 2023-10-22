# C# tutorial

First, open a terminal and prepare a directory for the sample.
The, install AUTD3Sharp library.

```shell
dotnet new console --name autd3-sample
cd autd3-sample
dotnet add package AUTD3Sharp
```

Next, make `Program.cs` file.
This is the source code for generating a focus with $\SI{150}{Hz}$ AM modulation. 

```csharp,filename=Program.cs
{{#include ../../../../samples/cs/Program.cs}}
```

Then, run the program.

```shell
dotnet run -c:Release
```

## For Linux, macOS users

You may need to run with administrator privileges when using SOEM on Linux or macOS.

```shell
sudo dotnet run -c:Release
```
