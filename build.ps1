# File: build.ps1
# Project: autd3
# Created Date: 10/05/2022
# Author: Shun Suzuki
# -----
# Last Modified: 10/05/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Hapis Lab. All rights reserved.
# 

Param(
    [string]$BUILD_DIR = "./build",
    [ValidateSet(2019, 2022)]$VS_VERSION = 2022,
    [string]$ARCH = "x64"
)

Start-Transcript "build.log" | Out-Null
$ROOT_DIR = $PSScriptRoot
$PROJECT_DIR = Join-Path $ROOT_DIR $BUILD_DIR

function ColorEcho($color, $PREFIX, $message) {
    Write-Host $PREFIX -ForegroundColor $color -NoNewline
    Write-Host ":", $message
}

function FindInstallLocation([string]$displayName) {
    $reg = Get-ChildItem HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall | 
    ForEach-Object { Get-ItemProperty $_.PsPath } | 
    Where-Object DisplayName -match $displayName
    if ($reg) {
        return Join-Path $reg.InstallLocation \bin
    }
    else {
        return "NULL"
    }
}

if ($ARCH -eq "x86") {
    $ARCH = "Win32"
}

# Show VS info
$IDE_NAME = "Visual Studio "
if ($VS_VERSION -eq 2019) {
    $IDE_NAME = $IDE_NAME + "16 2019"
}
elseif ($VS_VERSION -eq 2022) {
    $IDE_NAME = $IDE_NAME + "17 2022"
}
else {
    ColorEcho "Red" "Error" "This script only support Visual Studio 2017, 2019, or 2022."
    exit
}
ColorEcho "Green" "INFO" "Use", $IDE_NAME, "with", $ARCH, "architecture."

# Creating project dir
ColorEcho "Green" "INFO" "Create project directory if not exists. [", $PROJECT_DIR, "]"
if (Test-Path $PROJECT_DIR) { 
    ColorEcho "Yellow" "WARNING" "Directory", $PROJECT_DIR, "is already exists."
    $ans = Read-Host "Overwrite? (Y/[N])"
    if (($ans -eq "y") -or ($ans -eq "Y")) {
        Remove-Item $PROJECT_DIR -Recurse -Force
        if (Test-Path $PROJECT_DIR) {         
            ColorEcho "Red" "Error" "Cannot remove directory", $PROJECT_DIR
            Stop-Transcript | Out-Null
            $host.UI.RawUI.ReadKey() | Out-Null
            exit
        }
        else {
            ColorEcho "Green" "INFO" "Remove directory", $PROJECT_DIR         
        }
    }
    else {
        ColorEcho "Green" "INFO" "Cancled..."
        Stop-Transcript | Out-Null
        $host.UI.RawUI.ReadKey() | Out-Null
        exit
    }
}
if (New-Item -Path $PROJECT_DIR -ItemType Directory) {
    ColorEcho "Green" "INFO" "Success to create project directory", $PROJECT_DIR
}

# Find CMake
if (-not (Get-Command cmake -ea SilentlyContinue)) {
    ColorEcho "Green" "INFO" "CMake not found in PATH. Looking for CMake..."
    $cmake_path = FindInstallLocation "CMake"
    if ($cmake_path -eq "NULL") {
        ColorEcho "Red" "Error" "CMake not found. Install CMake or set CMake binary folder to PATH."
        Stop-Transcript | Out-Null
        $host.UI.RawUI.ReadKey() | Out-Null
        exit
    }
    else {
        $env:Path = $env:Path + ";" + $cmake_path
    }
}
$cmake_version = 0
foreach ($line in cmake -version) {
    $ary = $line -split " "
    $cmake_version = $ary[2]
    break
}
ColorEcho "Green" "INFO" "CMake is found", $cmake_version

# Find Git & update submodule
if (-not (Get-Command git -ea SilentlyContinue)) {
    ColorEcho "Green" "INFO" "Git not found in PATH. Looking for Git..."
    $git_path = FindInstallLocation "Git"
    if ($git_path -eq "NULL") {
        ColorEcho "Yellow" "WARN" "Git not found. Git submodules are not updated."
    }
    else {
        $env:Path = $env:Path + ";" + $git_path
    }
}
if (Get-Command git -ea SilentlyContinue) {
    ColorEcho "Green" "INFO" "Git is found."
    ColorEcho "Green" "INFO" "Updating git submodules..."
    Invoke-Expression "git submodule update --init --recursive"
}

# CMake build
ColorEcho "Green" "INFO" "Creating VS solution..."
Push-Location $PROJECT_DIR
$command = "cmake .. -G " + '$IDE_NAME' + " -A " + $ARCH

$command += " -D BUILD_ALL=ON"

Invoke-Expression $command | Tee-Object -FilePath "build.log"
Pop-Location

ColorEcho "Green" "INFO" "Done."
Stop-Transcript | Out-Null
$host.UI.RawUI.ReadKey() | Out-Null
exit
