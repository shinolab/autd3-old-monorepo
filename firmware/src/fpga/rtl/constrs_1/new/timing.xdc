set_input_delay -clock cpu_bsc_75M -min 0.200 [get_ports CPU_WE0_N]
set_input_delay -clock cpu_bsc_75M -max 0.500 [get_ports CPU_WE0_N]
set_input_delay -clock cpu_bsc_75M -min 0.200 [get_ports CPU_CS1_N]
set_input_delay -clock cpu_bsc_75M -max 0.500 [get_ports CPU_CS1_N]
set_input_delay -clock cpu_bsc_75M -min 0.200 [get_ports CPU_DATA*]
set_input_delay -clock cpu_bsc_75M -max 0.500 [get_ports CPU_DATA*]
set_input_delay -clock cpu_bsc_75M -min 0.200 [get_ports CPU_ADDR*]
set_input_delay -clock cpu_bsc_75M -max 0.500 [get_ports CPU_ADDR*]
set_output_delay -clock cpu_bsc_75M -min 0.200 [get_ports CPU_DATA*]
set_output_delay -clock cpu_bsc_75M -max 0.500 [get_ports CPU_DATA*]

set_input_delay -clock [get_clocks -of_objects [get_pins ultrasound_cnt_clk_gen/MMCME2_ADV_inst/CLKOUT0]] -min 0.500 [get_ports CAT_SYNC0]
set_input_delay -clock [get_clocks -of_objects [get_pins ultrasound_cnt_clk_gen/MMCME2_ADV_inst/CLKOUT0]] -max 1.500 [get_ports CAT_SYNC0]
set_input_delay -clock [get_clocks -of_objects [get_pins ultrasound_cnt_clk_gen/MMCME2_ADV_inst/CLKOUT0]] -min 0.500 [get_ports RESET_N]
set_input_delay -clock [get_clocks -of_objects [get_pins ultrasound_cnt_clk_gen/MMCME2_ADV_inst/CLKOUT0]] -max 1.500 [get_ports RESET_N]
set_input_delay -clock [get_clocks -of_objects [get_pins ultrasound_cnt_clk_gen/MMCME2_ADV_inst/CLKOUT0]] -min 0.500 [get_ports THERMO]
set_input_delay -clock [get_clocks -of_objects [get_pins ultrasound_cnt_clk_gen/MMCME2_ADV_inst/CLKOUT0]] -max 1.500 [get_ports THERMO]

set_property CFGBVS VCCO [current_design]
set_property CONFIG_VOLTAGE 3.3 [current_design]

set_property BITSTREAM.GENERAL.COMPRESS TRUE [current_design]
set_property BITSTREAM.CONFIG.CONFIGRATE 33 [current_design]
set_property CONFIG_MODE SPIx4 [current_design]

create_debug_core u_ila_0 ila
set_property ALL_PROBE_SAME_MU true [get_debug_cores u_ila_0]
set_property ALL_PROBE_SAME_MU_CNT 2 [get_debug_cores u_ila_0]
set_property C_ADV_TRIGGER false [get_debug_cores u_ila_0]
set_property C_DATA_DEPTH 1024 [get_debug_cores u_ila_0]
set_property C_EN_STRG_QUAL true [get_debug_cores u_ila_0]
set_property C_INPUT_PIPE_STAGES 0 [get_debug_cores u_ila_0]
set_property C_TRIGIN_EN false [get_debug_cores u_ila_0]
set_property C_TRIGOUT_EN false [get_debug_cores u_ila_0]
set_property port_width 1 [get_debug_ports u_ila_0/clk]
connect_debug_port u_ila_0/clk [get_nets [list ultrasound_cnt_clk_gen/clk_out]]
set_property PROBE_TYPE DATA_AND_TRIGGER [get_debug_ports u_ila_0/probe0]
set_property port_width 16 [get_debug_ports u_ila_0/probe0]
connect_debug_port u_ila_0/probe0 [get_nets [list {main/intensity_m[0]} {main/intensity_m[1]} {main/intensity_m[2]} {main/intensity_m[3]} {main/intensity_m[4]} {main/intensity_m[5]} {main/intensity_m[6]} {main/intensity_m[7]} {main/intensity_m[8]} {main/intensity_m[9]} {main/intensity_m[10]} {main/intensity_m[11]} {main/intensity_m[12]} {main/intensity_m[13]} {main/intensity_m[14]} {main/intensity_m[15]}]]
create_debug_port u_ila_0 probe
set_property PROBE_TYPE DATA_AND_TRIGGER [get_debug_ports u_ila_0/probe1]
set_property port_width 8 [get_debug_ports u_ila_0/probe1]
connect_debug_port u_ila_0/probe1 [get_nets [list {main/intensity_stm[0]} {main/intensity_stm[1]} {main/intensity_stm[2]} {main/intensity_stm[3]} {main/intensity_stm[4]} {main/intensity_stm[5]} {main/intensity_stm[6]} {main/intensity_stm[7]}]]
create_debug_port u_ila_0 probe
set_property PROBE_TYPE DATA_AND_TRIGGER [get_debug_ports u_ila_0/probe2]
set_property port_width 8 [get_debug_ports u_ila_0/probe2]
connect_debug_port u_ila_0/probe2 [get_nets [list {main/phase_e[0]} {main/phase_e[1]} {main/phase_e[2]} {main/phase_e[3]} {main/phase_e[4]} {main/phase_e[5]} {main/phase_e[6]} {main/phase_e[7]}]]
create_debug_port u_ila_0 probe
set_property PROBE_TYPE DATA_AND_TRIGGER [get_debug_ports u_ila_0/probe3]
set_property port_width 8 [get_debug_ports u_ila_0/probe3]
connect_debug_port u_ila_0/probe3 [get_nets [list {main/phase_normal[0]} {main/phase_normal[1]} {main/phase_normal[2]} {main/phase_normal[3]} {main/phase_normal[4]} {main/phase_normal[5]} {main/phase_normal[6]} {main/phase_normal[7]}]]
create_debug_port u_ila_0 probe
set_property PROBE_TYPE DATA_AND_TRIGGER [get_debug_ports u_ila_0/probe4]
set_property port_width 8 [get_debug_ports u_ila_0/probe4]
connect_debug_port u_ila_0/probe4 [get_nets [list {main/phase_stm[0]} {main/phase_stm[1]} {main/phase_stm[2]} {main/phase_stm[3]} {main/phase_stm[4]} {main/phase_stm[5]} {main/phase_stm[6]} {main/phase_stm[7]}]]
create_debug_port u_ila_0 probe
set_property PROBE_TYPE DATA_AND_TRIGGER [get_debug_ports u_ila_0/probe5]
set_property port_width 8 [get_debug_ports u_ila_0/probe5]
connect_debug_port u_ila_0/probe5 [get_nets [list {main/intensity[0]} {main/intensity[1]} {main/intensity[2]} {main/intensity[3]} {main/intensity[4]} {main/intensity[5]} {main/intensity[6]} {main/intensity[7]}]]
create_debug_port u_ila_0 probe
set_property PROBE_TYPE DATA_AND_TRIGGER [get_debug_ports u_ila_0/probe6]
set_property port_width 16 [get_debug_ports u_ila_0/probe6]
connect_debug_port u_ila_0/probe6 [get_nets [list {main/step_intensity_s[0]} {main/step_intensity_s[1]} {main/step_intensity_s[2]} {main/step_intensity_s[3]} {main/step_intensity_s[4]} {main/step_intensity_s[5]} {main/step_intensity_s[6]} {main/step_intensity_s[7]} {main/step_intensity_s[8]} {main/step_intensity_s[9]} {main/step_intensity_s[10]} {main/step_intensity_s[11]} {main/step_intensity_s[12]} {main/step_intensity_s[13]} {main/step_intensity_s[14]} {main/step_intensity_s[15]}]]
create_debug_port u_ila_0 probe
set_property PROBE_TYPE DATA_AND_TRIGGER [get_debug_ports u_ila_0/probe7]
set_property port_width 16 [get_debug_ports u_ila_0/probe7]
connect_debug_port u_ila_0/probe7 [get_nets [list {main/step_phase_s[0]} {main/step_phase_s[1]} {main/step_phase_s[2]} {main/step_phase_s[3]} {main/step_phase_s[4]} {main/step_phase_s[5]} {main/step_phase_s[6]} {main/step_phase_s[7]} {main/step_phase_s[8]} {main/step_phase_s[9]} {main/step_phase_s[10]} {main/step_phase_s[11]} {main/step_phase_s[12]} {main/step_phase_s[13]} {main/step_phase_s[14]} {main/step_phase_s[15]}]]
create_debug_port u_ila_0 probe
set_property PROBE_TYPE DATA_AND_TRIGGER [get_debug_ports u_ila_0/probe8]
set_property port_width 9 [get_debug_ports u_ila_0/probe8]
connect_debug_port u_ila_0/probe8 [get_nets [list {main/pulse_width_e[0]} {main/pulse_width_e[1]} {main/pulse_width_e[2]} {main/pulse_width_e[3]} {main/pulse_width_e[4]} {main/pulse_width_e[5]} {main/pulse_width_e[6]} {main/pulse_width_e[7]} {main/pulse_width_e[8]}]]
create_debug_port u_ila_0 probe
set_property PROBE_TYPE DATA_AND_TRIGGER [get_debug_ports u_ila_0/probe9]
set_property port_width 16 [get_debug_ports u_ila_0/probe9]
connect_debug_port u_ila_0/probe9 [get_nets [list {main/stm_idx[0]} {main/stm_idx[1]} {main/stm_idx[2]} {main/stm_idx[3]} {main/stm_idx[4]} {main/stm_idx[5]} {main/stm_idx[6]} {main/stm_idx[7]} {main/stm_idx[8]} {main/stm_idx[9]} {main/stm_idx[10]} {main/stm_idx[11]} {main/stm_idx[12]} {main/stm_idx[13]} {main/stm_idx[14]} {main/stm_idx[15]}]]
create_debug_port u_ila_0 probe
set_property PROBE_TYPE DATA_AND_TRIGGER [get_debug_ports u_ila_0/probe10]
set_property port_width 16 [get_debug_ports u_ila_0/probe10]
connect_debug_port u_ila_0/probe10 [get_nets [list {main/intensity_s[0]} {main/intensity_s[1]} {main/intensity_s[2]} {main/intensity_s[3]} {main/intensity_s[4]} {main/intensity_s[5]} {main/intensity_s[6]} {main/intensity_s[7]} {main/intensity_s[8]} {main/intensity_s[9]} {main/intensity_s[10]} {main/intensity_s[11]} {main/intensity_s[12]} {main/intensity_s[13]} {main/intensity_s[14]} {main/intensity_s[15]}]]
create_debug_port u_ila_0 probe
set_property PROBE_TYPE DATA_AND_TRIGGER [get_debug_ports u_ila_0/probe11]
set_property port_width 9 [get_debug_ports u_ila_0/probe11]
connect_debug_port u_ila_0/probe11 [get_nets [list {main/time_cnt[0]} {main/time_cnt[1]} {main/time_cnt[2]} {main/time_cnt[3]} {main/time_cnt[4]} {main/time_cnt[5]} {main/time_cnt[6]} {main/time_cnt[7]} {main/time_cnt[8]}]]
create_debug_port u_ila_0 probe
set_property PROBE_TYPE DATA_AND_TRIGGER [get_debug_ports u_ila_0/probe12]
set_property port_width 8 [get_debug_ports u_ila_0/probe12]
connect_debug_port u_ila_0/probe12 [get_nets [list {main/intensity_normal[0]} {main/intensity_normal[1]} {main/intensity_normal[2]} {main/intensity_normal[3]} {main/intensity_normal[4]} {main/intensity_normal[5]} {main/intensity_normal[6]} {main/intensity_normal[7]}]]
create_debug_port u_ila_0 probe
set_property PROBE_TYPE DATA_AND_TRIGGER [get_debug_ports u_ila_0/probe13]
set_property port_width 8 [get_debug_ports u_ila_0/probe13]
connect_debug_port u_ila_0/probe13 [get_nets [list {main/phase[0]} {main/phase[1]} {main/phase[2]} {main/phase[3]} {main/phase[4]} {main/phase[5]} {main/phase[6]} {main/phase[7]}]]
create_debug_port u_ila_0 probe
set_property PROBE_TYPE DATA_AND_TRIGGER [get_debug_ports u_ila_0/probe14]
set_property port_width 8 [get_debug_ports u_ila_0/probe14]
connect_debug_port u_ila_0/probe14 [get_nets [list {main/phase_s[0]} {main/phase_s[1]} {main/phase_s[2]} {main/phase_s[3]} {main/phase_s[4]} {main/phase_s[5]} {main/phase_s[6]} {main/phase_s[7]}]]
create_debug_port u_ila_0 probe
set_property PROBE_TYPE DATA_AND_TRIGGER [get_debug_ports u_ila_0/probe15]
set_property port_width 8 [get_debug_ports u_ila_0/probe15]
connect_debug_port u_ila_0/probe15 [get_nets [list {main/phase_m[0]} {main/phase_m[1]} {main/phase_m[2]} {main/phase_m[3]} {main/phase_m[4]} {main/phase_m[5]} {main/phase_m[6]} {main/phase_m[7]}]]
create_debug_port u_ila_0 probe
set_property PROBE_TYPE DATA_AND_TRIGGER [get_debug_ports u_ila_0/probe16]
set_property port_width 1 [get_debug_ports u_ila_0/probe16]
connect_debug_port u_ila_0/probe16 [get_nets [list main/dout_valid]]
create_debug_port u_ila_0 probe
set_property PROBE_TYPE DATA_AND_TRIGGER [get_debug_ports u_ila_0/probe17]
set_property port_width 1 [get_debug_ports u_ila_0/probe17]
connect_debug_port u_ila_0/probe17 [get_nets [list main/dout_valid_e]]
create_debug_port u_ila_0 probe
set_property PROBE_TYPE DATA_AND_TRIGGER [get_debug_ports u_ila_0/probe18]
set_property port_width 1 [get_debug_ports u_ila_0/probe18]
connect_debug_port u_ila_0/probe18 [get_nets [list main/dout_valid_m]]
create_debug_port u_ila_0 probe
set_property PROBE_TYPE DATA_AND_TRIGGER [get_debug_ports u_ila_0/probe19]
set_property port_width 1 [get_debug_ports u_ila_0/probe19]
connect_debug_port u_ila_0/probe19 [get_nets [list main/dout_valid_normal]]
create_debug_port u_ila_0 probe
set_property PROBE_TYPE DATA_AND_TRIGGER [get_debug_ports u_ila_0/probe20]
set_property port_width 1 [get_debug_ports u_ila_0/probe20]
connect_debug_port u_ila_0/probe20 [get_nets [list main/dout_valid_s]]
create_debug_port u_ila_0 probe
set_property PROBE_TYPE DATA_AND_TRIGGER [get_debug_ports u_ila_0/probe21]
set_property port_width 1 [get_debug_ports u_ila_0/probe21]
connect_debug_port u_ila_0/probe21 [get_nets [list main/dout_valid_stm]]
create_debug_port u_ila_0 probe
set_property PROBE_TYPE DATA_AND_TRIGGER [get_debug_ports u_ila_0/probe22]
set_property port_width 1 [get_debug_ports u_ila_0/probe22]
connect_debug_port u_ila_0/probe22 [get_nets [list main/op_mode]]
create_debug_port u_ila_0 probe
set_property PROBE_TYPE DATA_AND_TRIGGER [get_debug_ports u_ila_0/probe23]
set_property port_width 1 [get_debug_ports u_ila_0/probe23]
connect_debug_port u_ila_0/probe23 [get_nets [list main/stm_gain_mode]]
create_debug_port u_ila_0 probe
set_property PROBE_TYPE DATA_AND_TRIGGER [get_debug_ports u_ila_0/probe24]
set_property port_width 1 [get_debug_ports u_ila_0/probe24]
connect_debug_port u_ila_0/probe24 [get_nets [list main/update]]
set_property C_CLK_INPUT_FREQ_HZ 300000000 [get_debug_cores dbg_hub]
set_property C_ENABLE_CLK_DIVIDER false [get_debug_cores dbg_hub]
set_property C_USER_SCAN_CHAIN 1 [get_debug_cores dbg_hub]
connect_debug_port dbg_hub/clk [get_nets clk]
