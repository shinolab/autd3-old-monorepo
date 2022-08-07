# AUTDServer

TwinCAT AUTD Server for `link::TwinCAT` and `link::RemoteTwinCAT`.

# Usage

```
  AUTDServer [options]
```

## Options
* -c, --client <client>    Client IP address []
* -s, --sync0 <sync0>      Sync0 cycle time in units of ns [default: 500000]
* -t, --task <task>        Send task cycle time in units of 0.1us [default: 5000]
* -b, --base <base>        CPU base time in units of 0.1us [default: 5000]
* -m, --mode <DC|FreeRun>  Sync mode [default: DC]
* -k, --keep               Keep TwinCAT config window open [default: False]
* --version                Show version information
* -?, -h, --help           Show help and usage information

Please see https://github.com/shinolab/AUTDServer for more details.
