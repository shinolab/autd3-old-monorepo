open_project autd3-fpga.xpr
update_compile_order -fileset sources_1

launch_runs synth_2 -jobs 8
wait_on_run synth_2

launch_runs impl_2 -jobs 8
wait_on_run impl_2

launch_runs impl_2 -to_step write_bitstream -jobs 8
wait_on_run impl_2

close_project
