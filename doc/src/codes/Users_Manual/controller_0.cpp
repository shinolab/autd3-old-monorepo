autd.geometry()[0].reads_fpga_info(true);
autd.send(autd3::UpdateFlags());

const auto info = autd.fpga_info();