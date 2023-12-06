autd.send(autd3::ConfigureReadsFPGAInfo([](const auto&) { return true; }));

const auto info = autd.fpga_info();