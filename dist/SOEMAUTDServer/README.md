# SOEMAUTDServer

SOEM AUTD Server for `link::RemoteSOEM`.

# Usage

```
  SOEMAUTDServer.exe [options]
```

## Options
* -h, --help                    shows help message and exits
* -v, --version                 prints version information and exits
* -i, --ifname <ifname>         Interface name
* -c, --client <addr>           Client IP address [default: ""]
* -p, --port <port>             Client port [default: 50632]
* -s, --sync0 <n>               Sync0 cycle time in units of 500us [default: 2]
* -t, --send <n>                Send cycle time in units of 500us [default: 2]
* -f, --freerun                 Set free run mode
* -l, --disable_high_precision  Disable high precision mode
* -e, --state_check_interval    State check interval in ms [default: 500]
* -d, --debug                   Set debug mode
