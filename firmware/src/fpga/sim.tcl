set project_directory   [file dirname [info script]]
set project_name        "autd3-fpga"

open_project [file join $project_directory $project_name]

update_compile_order -fileset sources_1
update_compile_order -fileset sim_1

proc build_ip {path} {
    set name [lrange [split [lrange [split $path "/"] end end] "."] 0 0]
    generate_target all [get_files $path]
    catch { config_ip_cache -export [get_ips -all $name] }
    export_ip_user_files -of_objects [get_files $path] -no_script -sync -force -quiet
    set run [get_files -of_objects [get_fileset sources_1] $path]
    create_ip_run $run -quiet
    set jobname ${name}_synth_1
    launch_runs $jobname -jobs 8 -quiet
    wait_on_run $jobname
}

set file_list [glob -nocomplain -join $project_directory autd3-fpga.srcs/sources_1/ip/* *.xci]
foreach file_path $file_list {
    set is_ooc false
    set fid [open $file_path r]
    while {[gets $fid line] >= 0} {
        if {[string first "OUT_OF_CONTEXT" $line] != -1} {
            set is_ooc true
        }
    }
    close $fid
    if $is_ooc {
        build_ip $file_path
    }
}

set_property -name {xsim.simulate.runtime} -value {all} -objects [get_filesets sim_1]
set_property top $argv [get_filesets sim_1]
launch_simulation

close_project
