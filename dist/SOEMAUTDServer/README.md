# SOEMAUTDServer

SOEM AUTD Server for `link::RemoteSOEM`.

# Usage

```
  SOEMAUTDServer.exe [options]
```

## Options
* -h, --help                                            shows help message and exits
* -v, --version                                         prints version information and exits
* -i, --ifname <ifname>                                 Interface name
* -b, --buf_size <size>                                 Buffer size (unlimited if 0) [default: 0]
* -c, --client <addr>                                   Client IP address [default: ""]
* -p, --port <port>                                     Client port [default: 50632]
* -s, --sync0 <n>                                       Sync0 cycle time in units of 500us [default: 2]
* -t, --send <n>                                        Send cycle time in units of 500us [default: 2]
* -e, --state_check_interval                            State check interval in ms [default: 500]
* -m, --sync_mode <"dc", "freerun">                     Sync mode [default: "freerun"]
* -w, --timer_strategy <"sleep", "busywait", "timer">   Timer strategy [default: "sleep"]
* -d, --debug                   Set debug mode
