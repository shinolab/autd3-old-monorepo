set project_directory   [file dirname [info script]]
set project_name        "autd3-fpga"

cd $project_directory
create_project -force $project_name $project_directory

set_property PART xc7a200tfbg676-2 [current_project]

set_property "default_lib"        "xil_defaultlib" [current_project]
set_property "simulator_language" "Verilog"        [current_project]
set_property "target_language"    "Verilog"        [current_project]


if {[string equal [get_filesets -quiet sources_1] ""]} {
    create_fileset -srcset sources_1
}

if {[string equal [get_filesets -quiet constrs_1] ""]} {
    create_fileset -constrset constrs_1
}

if {[string equal [get_filesets -quiet sim_1] ""]} {
    create_fileset -simset sim_1
}


set synth_1_flow     "Vivado Synthesis 2022"
set synth_1_strategy "Flow_PerfOptimized_high"
if {[string equal [get_runs -quiet synth_1] ""]} {
    create_run -name synth_1 -flow $synth_1_flow -strategy $synth_1_strategy -constrset constrs_1
} else {
    set_property flow     $synth_1_flow     [get_runs synth_1]
    set_property strategy $synth_1_strategy [get_runs synth_1]
}
set synth_2_flow     "Vivado Synthesis 2022"
set synth_2_strategy "Vivado Synthesis Defaults"
if {[string equal [get_runs -quiet synth_2] ""]} {
    create_run -name synth_2 -flow $synth_2_flow -strategy $synth_2_strategy -constrset constrs_1
} else {
    set_property flow     $synth_2_flow     [get_runs synth_2]
    set_property strategy $synth_2_strategy [get_runs synth_2]
}
set synth_3_flow     "Vivado Synthesis 2022"
set synth_3_strategy "Flow_AreaOptimized_high"
if {[string equal [get_runs -quiet synth_3] ""]} {
    create_run -name synth_3 -flow $synth_3_flow -strategy $synth_3_strategy -constrset constrs_1
} else {
    set_property flow     $synth_3_flow     [get_runs synth_3]
    set_property strategy $synth_3_strategy [get_runs synth_3]
}
current_run -synthesis [get_runs synth_1]

set impl_1_flow      "Vivado Implementation 2022"
set impl_1_strategy  "Performance_ExplorePostRoutePhysOpt"
if {[string equal [get_runs -quiet impl_1] ""]} {
    create_run -name impl_1 -flow $impl_1_flow -strategy $impl_1_strategy -constrset constrs_1 -parent_run synth_1
} else {
    set_property flow     $impl_1_flow      [get_runs impl_1]
    set_property strategy $impl_1_strategy  [get_runs impl_1]
}
set impl_2_flow      "Vivado Implementation 2022"
set impl_2_strategy  "Vivado Implementation Defaults"
if {[string equal [get_runs -quiet impl_2] ""]} {
    create_run -name impl_2 -flow $impl_2_flow -strategy $impl_2_strategy -constrset constrs_1 -parent_run synth_2
} else {
    set_property flow     $impl_2_flow      [get_runs impl_2]
    set_property strategy $impl_2_strategy  [get_runs impl_2]
}
set impl_3_flow      "Vivado Implementation 2022"
set impl_3_strategy  "Area_ExploreWithRemap"
if {[string equal [get_runs -quiet impl_3] ""]} {
    create_run -name impl_3 -flow $impl_3_flow -strategy $impl_3_strategy -constrset constrs_1 -parent_run synth_3
} else {
    set_property flow     $impl_3_flow      [get_runs impl_3]
    set_property strategy $impl_3_strategy  [get_runs impl_3]
}
current_run -implementation [get_runs impl_1]


add_files -fileset constrs_1 -norecurse [file join $project_directory "rtl/constrs_1/new/top.xdc"]
add_files -fileset constrs_1 -norecurse [file join $project_directory "rtl/constrs_1/new/timing.xdc"]
set_property used_in_synthesis false [get_files rtl/constrs_1/new/timing.xdc]

proc add_verilog_file {fileset_name library_name file_name} {
    set file    [file normalize $file_name]
    set fileset [get_filesets   $fileset_name] 
    add_files -norecurse -fileset $fileset $file
    set file_obj [get_files -of_objects $fileset $file]
    set_property "file_type" "SystemVerilog" $file_obj
    set_property "library" $library_name $file_obj
}
set file_list [glob -nocomplain -join rtl/sources_1/new/* *.sv]
foreach src_file_path $file_list {
  add_verilog_file sources_1 xil_defaultlib $src_file_path
}
set file_list [glob -nocomplain -join rtl/sources_1/new/*/** *.sv]
foreach src_file_path $file_list {
  add_verilog_file sources_1 xil_defaultlib $src_file_path
}
set file_list [glob -nocomplain rtl/sources_1/new/*.sv]
foreach src_file_path $file_list {
  add_verilog_file sources_1 xil_defaultlib $src_file_path
}

proc add_header_file {fileset_name library_name file_name} {
    set file    [file normalize $file_name]
    set fileset [get_filesets   $fileset_name] 
    add_files -norecurse -fileset $fileset $file
    set file_obj [get_files -of_objects $fileset $file]
    set_property "file_type" "Verilog Header" $file_obj
    set_property "library" $library_name $file_obj
}
set file_list [glob -nocomplain rtl/sources_1/new/headers/*.vh]
foreach header_file_path $file_list {
  add_header_file sources_1 xil_defaultlib $header_file_path
}

set file_list [glob -nocomplain -join rtl/sources_1/ip/* *.xci]
foreach xci_file_path $file_list {
  import_ip $xci_file_path
}

proc add_sim_file {fileset_name library_name file_name} {
    set file    [file normalize $file_name]
    set fileset [get_filesets   $fileset_name] 
    add_files -norecurse -fileset $fileset $file
    set file_obj [get_files -of_objects $fileset $file]
    set_property "file_type" "SystemVerilog" $file_obj
    set_property "library" $library_name $file_obj
}
set file_list [glob -nocomplain rtl/sim_1/new/*.sv]
foreach sim_file_path $file_list {
  add_sim_file sim_1 xil_defaultlib $sim_file_path
}
set file_list [glob -nocomplain -join rtl/sim_1/new/* *.sv]
foreach sim_file_path $file_list {
  add_sim_file sim_1 xil_defaultlib $sim_file_path
}

set_msg_config -id {Synth 8-7080} -new_severity {ADVISORY}
set_msg_config -id {Synth 8-7129} -new_severity {ADVISORY}
set_msg_config -id {Synth 8-5640} -new_severity {ADVISORY}
# set_msg_config -id {Synth 8-5858} -new_severity {ADVISORY}

set_property top top [get_filesets sources_1]
set_property top sim_pwm [get_filesets sim_1]
set_property top_lib xil_defaultlib [get_filesets sim_1]

set_property -name {xsim.compile.tcl.pre} -value [file join $project_directory "rtl/sim_1/new/rand.tcl"] -objects [get_filesets sim_1]
