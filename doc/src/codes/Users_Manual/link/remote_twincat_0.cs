using AUTD3Sharp.Link;

RemoteTwinCAT.Builder("172.16.99.111.1.1")
        .WithServerIp(IPAddress.Parse("172.16.99.104"))
        .WithClientAmsNetId("172.16.99.62.1.1")