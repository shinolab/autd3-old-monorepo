# File: autd_firmware_writer.ps1
# Project: firmware
# Created Date: 14/02/2020
# Author: Shun Suzuki
# -----
# Last Modified: 11/09/2023
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2021 Shun Suzuki. All rights reserved.
# 

Param(
    [string]$version = "3.0.1",
    [string]$vivado_dir = "NULL"
)

function ColorEcho($color, $PREFIX, $message) {
    Write-Host $PREFIX -ForegroundColor $color -NoNewline
    Write-Host ":", $message
}

function TestCommand($command) {
    return -not -not (Get-Command $command -ea SilentlyContinue);
}

function GetInstallLocation($displayNamePattern) {
    $prog_reg = Get-ChildItem HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall | ForEach-Object { Get-ItemProperty $_.PsPath } | Where-Object DisplayName -match $displayNamePattern | Select-Object -first 1
    if ($prog_reg) {
        return $prog_reg.InstallLocation
    }
    else {
        return "NULL"
    }
}

function UpdateCPU([string]$cpuFirmwareFile) {
    $can_use_jlink = TestCommand jlink
    if (-not $can_use_jlink) {
        ColorEcho "Green" "INFO" "J-Link is not found in PATH. Looking for J-Link..."
        $jlink_path = GetInstallLocation 'J-Link'
        if ($jlink_path -eq "NULL") {
            ColorEcho "Red" "Error" "J-Link is not found. Install J-Link or add J-Link install folder to PATH."
            Stop-Transcript | Out-Null
            ColorEcho "Green" "INFO" "Press any key to exit..."
            $host.UI.RawUI.ReadKey() | Out-Null
            exit
        }
        else {
            $env:Path = $env:Path + ";" + $jlink_path
        }
    }
    ColorEcho "Green" "INFO" "Find J-Link"

    Copy-Item -Path $cpuFirmwareFile -Destination "tmp.bin" -Force
    $command = "jlink -device R7S910018_R4F -if JTAG -speed 4000 -jtagconf -1,-1 -autoconnect 1 -CommanderScript ./scripts/cpu_flash.jlink"
    $success = $TRUE
    Invoke-Expression $command | Out-String -Stream | ForEach-Object {
        [string]$line = $_
        Write-Host $line
        if ($line.Contains("Cannot connect")) {
            $success = $FALSE
        }
    }
    Remove-Item -Path "tmp.bin"
    if ($success) {
        ColorEcho "Green" "INFO" "Update CPU done."
    }
    return $success
}

function AddVivadoToPATH($vivado_dir, $edition) {
    if ($vivado_dir -eq "NULL") {
        ColorEcho "Green" "INFO" "Vivado is not found in PATH. Looking for Vivado..."
        $xilinx_path = GetInstallLocation 'Vivado|Vitis'
        if (($xilinx_path -eq "NULL")) {
            ColorEcho "Red" "Error" "Vivado is not found. Install Vivado."
            Stop-Transcript | Out-Null
            ColorEcho "Green" "INFO" "Press any key to exit..."
            $host.UI.RawUI.ReadKey() | Out-Null
            exit
        }
        
        $vivado_path = Join-Path $xilinx_path $edition
        if (-not (Test-Path $vivado_path)) {
            return
        }
        
        $vivados = Get-ChildItem $vivado_path
        if ($vivados.Length -eq 0) {
            return
        }

        $vivado_ver = $vivados | Select-Object -first 1
        ColorEcho "Green" "INFO" "Find", $edition, $vivado_ver.Name
        $vivado_dir = $vivado_ver.FullName
    }

    $vivado_bin = Join-Path $vivado_dir "bin"
    $vivado_lib = Join-Path $vivado_dir "lib" | Join-Path -ChildPath "win64.o" 
    $env:Path = $env:Path + ";" + $vivado_bin + ";" + $vivado_lib
}

function UpdateFPGA([string]$fpgaFirmwareFile, [string]$vivado_dir) {
    $can_use_vivado = TestCommand vivado
    $can_use_vivado_lab = TestCommand vivado_lab

    if ((-not $can_use_vivado) -and (-not $can_use_vivado_lab)) {
        AddVivadoToPATH $vivado_dir "Vivado"
        $can_use_vivado = TestCommand vivado
    }

    if ((-not $can_use_vivado) -and (-not $can_use_vivado_lab)) {
        AddVivadoToPATH $vivado_dir "Vivado_Lab"
        $can_use_vivado_lab = TestCommand vivado_lab
    }

    if ((-not $can_use_vivado) -and (-not $can_use_vivado_lab)) {
        ColorEcho "Red" "Error" "Vivado is not found. Install Vivado or add Vivado install folder to PATH."
        Stop-Transcript | Out-Null
        ColorEcho "Green" "INFO" "Press any key to exit..."
        $host.UI.RawUI.ReadKey() | Out-Null
        exit
    }

    $command = ""
    if ($can_use_vivado) {
        $command = "vivado -mode batch -nojournal -nolog -notrace -source ./scripts/fpga_configuration_script.tcl 2>&1"
    } elseif ($can_use_vivado_lab) {
        $command = "vivado_lab -mode batch -nojournal -nolog -notrace -source ./scripts/fpga_configuration_script.tcl 2>&1"
    }

    Copy-Item -Path $fpgaFirmwareFile -Destination "./scripts/tmp.mcs" -Force
    ColorEcho "Green" "INFO" "Invoking Vivado..."
    $success = $TRUE
    $result = Invoke-Expression $command | Out-String -Stream | ForEach-Object {
        [string]$line = $_
        Write-Host $line
        if ($line.TrimStart().StartsWith("ERROR")) {
            $success = $FALSE
        }
    }
    Remove-Item -Path "./scripts/tmp.mcs"
    if ($success) {
        ColorEcho "Green" "INFO" "Update FPGA done."
    }
    return $success
}

Start-Transcript "autd_firmware_writer.log" | Out-Null
Write-Host "AUTD3 Firmware Writer"
ColorEcho "Green" "INFO" "Make sure that you connected configuration cabels and AUTD's power is on."

if (-not (Test-Path "tmp")) {
    New-Item -ItemType directory -Path "tmp" | Out-Null
}
if (-not (Test-Path "tmp/v$version")) {
  ColorEcho "Green" "INFO" "Downloading firmware files..."
  Invoke-WebRequest "https://github.com/shinolab/autd3/releases/download/firmware%2Fv$version/firmware-v$version.zip" -OutFile "tmp.zip" | Out-Null
  Expand-Archive -Path "tmp.zip" -DestinationPath "tmp/v$version" -Force
  Remove-Item -Path "tmp.zip"
}

ColorEcho "Green" "INFO" "Found firmwares are..."
$firmwares = Get-ChildItem "tmp/v$version"
$fpga_firmware = ""
$cpu_firmware = ""
foreach ($firmware in $firmwares) {
    $ext = $firmware.Name.Split('.') | Select-Object -last 1
    if ($ext -eq "bin") {
        $cpu_firmware = $firmware.FullName
        ColorEcho "Blue" "CPU " $firmware.Name
    }
    if ($ext -eq "mcs") {
        $fpga_firmware = $firmware.FullName
        ColorEcho "Blue" "FPGA" $firmware.Name
    }
}

ColorEcho "Green" "INFO" "Select which firmware to be updated."
Write-Host "[0]: Both (Default)"
Write-Host "[1]: FPGA"
Write-Host "[2]: CPU"
do {
    try {
        $is_num = $true
        [int]$select = Read-host "Select"
    }
    catch { $is_num = $false }
}
until (($select -ge 0 -and $select -le 2) -and $is_num)

$update_cpu_result = $true
$update_fpga_result = $true

if ($select -eq 0) {
    $update_cpu_result = UpdateCPU $cpu_firmware
    $update_fpga_result = UpdateFPGA $fpga_firmware $vivado_dir
}
if ($select -eq 1) {
    $update_fpga_result = UpdateFPGA $fpga_firmware $vivado_dir
}
if ($select -eq 2) {
    $update_cpu_result = UpdateCPU $cpu_firmware
}

if ($update_cpu_result -and $update_fpga_result) {
    ColorEcho "Yellow" "INFO" "Please turn AUTD's power off and on again to load new firmware."
}
if (-not $update_cpu_result) {
    ColorEcho "Red" "ERROR" "Failed to update CPU. Make sure that AUTD is connected and power on."
}
if (-not $update_fpga_result) {
    ColorEcho "Red" "ERROR" "Failed to update FPGA. Make sure that AUTD is connected and power on."
}

Stop-Transcript | Out-Null
ColorEcho "Green" "INFO" "Press any key to exit..."
$host.UI.RawUI.ReadKey() | Out-Null
exit
