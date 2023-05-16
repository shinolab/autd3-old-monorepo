# File: sim.ps1
# Project: autd3-fpga
# Created Date: 17/03/2022
# Author: Shun Suzuki
# -----
# Last Modified: 16/05/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
# 

Param(
    [string]$target,
    [string]$vivado_dir = "NULL"
)

function ColorEcho($color, $PREFIX, $message) {
    Write-Host $PREFIX -ForegroundColor $color -NoNewline
    Write-Host ":", $message
}

function FindVivado() {
    $xilinx_reg = Get-ChildItem HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall | ForEach-Object { Get-ItemProperty $_.PsPath } | Where-Object DisplayName -match 'Vivado|Vitis' | Select-Object -first 1
    if ($xilinx_reg) {
        return $xilinx_reg.InstallLocation
    }
    else {
        return "NULL"
    }
}

function RunSimulation($sim_target) {
    ColorEcho "Green" "INFO" "Simulation target is $sim_target"
    ColorEcho "Green" "INFO" "Invoking Vivado..."
    if (-not (Get-Command vivado -ea SilentlyContinue)) {
        if ($vivado_dir -eq "NULL") {
            ColorEcho "Green" "INFO" "Vivado is not found in PATH. Looking for Vivado..."
            $xilinx_path = FindVivado
            if (($xilinx_path -eq "NULL")) {
                ColorEcho "Red" "Error" "Vivado is not found. Install Vivado."
                Stop-Transcript | Out-Null
                $host.UI.RawUI.ReadKey() | Out-Null
                exit
            }
            
            $vivado_path = Join-Path $xilinx_path "Vivado"
            if (-not (Test-Path $vivado_path)) {
                ColorEcho "Red" "Error" "Vivado is not found. Install Vivado."
                Stop-Transcript | Out-Null
                $host.UI.RawUI.ReadKey() | Out-Null
                exit
            }
            
            $vivados = Get-ChildItem $vivado_path
            if ($vivados.Length -eq 0) {
                ColorEcho "Red" "Error" "Vivado is not found. Install Vivado."
                Stop-Transcript | Out-Null
                $host.UI.RawUI.ReadKey() | Out-Null
                exit
            }

            $vivado_ver = $vivados | Select-Object -first 1
            ColorEcho "Green" "INFO" "Vivado", $vivado_ver.Name, "found"
            $vivado_dir = $vivado_ver.FullName
        }

        $vivado_bin = Join-Path $vivado_dir "bin"
        $vivado_lib = Join-Path $vivado_dir "lib" | Join-Path -ChildPath "win64.o" 
        $env:Path = $env:Path + ";" + $vivado_bin + ";" + $vivado_lib
    }

    $command = "vivado -mode batch -source sim.tcl -tclargs $sim_target"
    Invoke-Expression $command
}

if ($target -eq "all") {
    $all_simulations = @(
        'sim_modulator'
        'sim_operator_normal'
        'sim_operator_stm_focus'
        'sim_operator_stm_gain'
        'sim_pwm'
        'sim_silencer'
    );
    foreach ($sim_target in $all_simulations){
        RunSimulation($sim_target);
    }
} else {
    RunSimulation($target);
}
