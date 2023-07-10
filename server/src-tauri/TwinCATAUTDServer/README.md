# TwinCATAUTDServer

TwinCAT AUTD Server for `link::TwinCAT` and `link::RemoteTwinCAT`.

# Usage

```
  TwinCATAUTDServer.exe [options]
```

## Options
* -c, --client <client>    Client IP address [default: ""]
* -s, --sync0 <sync0>      Sync0 cycle time in units of 500us [default: 2]
* -t, --task <task>        Send task cycle time in units of 500us [default: 2]
* -b, --base <base>        CPU base time in units of 500us [default: 1]
* -m, --mode <DC|FreeRun>  Sync mode [default: DC]
* -k, --keep               Keep TwinCAT XAE Shell window open [default: False]
* --version                Show version information
* -?, -h, --help           Show help and usage information
