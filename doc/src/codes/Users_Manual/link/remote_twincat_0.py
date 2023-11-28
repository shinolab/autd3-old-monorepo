from pyautd3.link.twincat import RemoteTwinCAT

RemoteTwinCAT.builder("172.16.99.111.1.1")\
    .with_server_ip("172.16.99.104")\
    .with_client_ams_net_id("172.16.99.62.1.1")