#!/usr/bin/env python3

"""
File: build.py
Project: autd3
Created Date: 16/10/2023
Author: Shun Suzuki
-----
Last Modified: 19/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""

import argparse
import contextlib
import glob
import re
import shutil
import subprocess
import os
import sys
import platform
from shutil import which


def err(msg: str):
    print("\033[91mERR \033[0m: " + msg)


def warn(msg: str):
    print("\033[93mWARN\033[0m: " + msg)


def info(msg: str):
    print("\033[92mINFO\033[0m: " + msg)


def rm_f(path):
    try:
        os.remove(path)
    except FileNotFoundError:
        pass


def onexc(func, path, exeption):
    import stat

    if not os.access(path, os.W_OK):
        os.chmod(path, stat.S_IWUSR)
        func(path)
    else:
        raise


def rmtree_f(path):
    try:
        shutil.rmtree(path, onexc=onexc)
    except FileNotFoundError:
        pass


def glob_norm(path, recursive):
    return list(map(lambda p: os.path.normpath(p), glob.glob(path, recursive=recursive)))


def rm_glob_f(path, exclude=None, recursive=True):
    if exclude is not None:
        for f in list(set(glob_norm(path, recursive=recursive)) - set(glob_norm(exclude, recursive=recursive))):
            rm_f(f)
    else:
        for f in glob.glob(path, recursive=recursive):
            rm_f(f)


def rmtree_glob_f(path):
    for f in glob.glob(path):
        rmtree_f(f)


@contextlib.contextmanager
def set_env(key, value):
    env = os.environ.copy()
    os.environ[key] = value
    try:
        yield
    finally:
        os.environ.clear()
        os.environ.update(env)


@contextlib.contextmanager
def working_dir(path):
    cwd = os.getcwd()
    os.chdir(path)
    try:
        yield
    finally:
        os.chdir(cwd)


is_windows = platform.system() == "Windows"
is_macos = platform.system() == "Darwin"
is_linux = platform.system() == "Linux"
if not is_windows and not is_macos and not is_linux:
    err(f'platform "{platform.system()}" is not supported.')
    sys.exit(-1)

exe_ext = ".exe" if is_windows else ""


def is_cuda_available():
    return which("nvcc") is not None


def is_arrayfire_available():
    return "AF_PATH" in os.environ


def setup_arm32_linker():
    os.makedirs(".cargo", exist_ok=True)
    with open(".cargo/config", "w") as f:
        f.write("[target.armv7-unknown-linux-gnueabihf]\n")
        f.write('linker = "arm-linux-gnueabihf-gcc"\n')


def setup_aarch64_linker():
    os.makedirs(".cargo", exist_ok=True)
    with open(".cargo/config", "w") as f:
        f.write("[target.aarch64-unknown-linux-gnu]\n")
        f.write('linker = "aarch64-linux-gnu-gcc"\n')


def fetch_submodule():
    with working_dir(os.path.dirname(os.path.abspath(__file__))):
        subprocess.run(["git", "submodule", "update", "--init", "--recursive"]).check_returncode()


def rust_build(args):
    if args.all:
        if is_macos:
            args.cuda = False
        else:
            if not is_cuda_available():
                warn("CUDA is not installed. Skip building crates using CUDA.")
                args.cuda = False
            else:
                args.cuda = True

        if is_arrayfire_available():
            args.af = True
        else:
            warn("ArrayFire is not installed. Skip building crates using ArrayFire.")
            args.af = False

    with working_dir("src"):
        commands = ["cargo", "build"]
        if args.release:
            commands.append("--release")
        if args.all:
            commands.append("--all")
            if not args.cuda:
                commands.append("--exclude=autd3-backend-cuda")
            if not args.af:
                commands.append("--exclude=autd3-backend-arrayfire")

        if is_linux and args.arch is not None:
            info("Skip build examples because cross compilation is not supported.")
            args.no_examples = True
            if args.arch == "arm32":
                setup_arm32_linker()
                commands.append("--exclude=autd3-backend-cuda")
                commands.append("--exclude=autd3-backend-arrayfire")
                commands.append("--exclude=autd3-link-visualizer")
                commands.append("--target=armv7-unknown-linux-gnueabihf")
            elif args.arch == "aarch64":
                setup_aarch64_linker()
                commands.append("--exclude=autd3-backend-cuda")
                commands.append("--exclude=autd3-backend-arrayfire")
                commands.append("--exclude=autd3-link-visualizer")
                commands.append("--target=aarch64-unknown-linux-gnu")
            else:
                err(f'arch "{args.arch}" is not supported.')
                sys.exit(-1)

        subprocess.run(commands).check_returncode()

    if not args.no_examples:
        info("Building examples...")
        with working_dir("src/examples"):
            command = ["cargo", "build", "--bins"]
            if args.release:
                command.append("--release")
            features = "async soem twincat"
            if args.all:
                features += " simulator remote_soem remote_twincat visualizer gpu python lightweight"
            command.append("--features")
            command.append(features)
            subprocess.run(command).check_returncode()


def rust_lint(args):
    if args.all:
        if is_macos:
            args.cuda = False
        else:
            if not is_cuda_available():
                warn("CUDA is not installed. Skip building crates using CUDA.")
                args.cuda = False
            else:
                args.cuda = True

        if is_arrayfire_available():
            args.af = True
        else:
            warn("ArrayFire is not installed. Skip building crates using ArrayFire.")
            args.af = False

    with working_dir("src"):
        commands = ["cargo", "clippy"]
        if args.release:
            commands.append("--release")
        if args.all:
            commands.append("--all")
            if not args.cuda:
                commands.append("--exclude=autd3-backend-cuda")
            if not args.af:
                commands.append("--exclude=autd3-backend-arrayfire")
        commands.append("--")
        commands.append("-D")
        commands.append("warnings")
        subprocess.run(commands).check_returncode()

    with working_dir("capi"):
        commands = ["cargo", "clippy"]
        if args.release:
            commands.append("--release")
        if args.all:
            commands.append("--all")
            if not args.cuda:
                commands.append("--exclude=autd3capi-backend-cuda")
        commands.append("--")
        commands.append("-D")
        commands.append("warnings")

        subprocess.run(commands).check_returncode()


def rust_test(args):
    if args.all:
        if is_macos:
            args.cuda = False
        else:
            if not is_cuda_available():
                warn("CUDA is not installed. Skip building crates using CUDA.")
                args.cuda = False
            else:
                args.cuda = True

        if is_arrayfire_available():
            args.af = True
        else:
            warn("ArrayFire is not installed. Skip building crates using ArrayFire.")
            args.af = False

    with working_dir("src"):
        commands = ["cargo", "test"]
        if args.release:
            commands.append("--release")
        if args.all:
            commands.append("--all")
            if not args.cuda or args.skip_cuda:
                commands.append("--exclude=autd3-backend-cuda")
            if not args.af:
                commands.append("--exclude=autd3-backend-arrayfire")
        commands.append("--features")
        commands.append("test-utilities")

        subprocess.run(commands).check_returncode()


def rust_run(args):
    examples = [
        "group_gain",
        "soem",
        "remote_soem",
        "twincat",
        "remote_twincat",
        "simulator",
        "visualizer",
        "freq_config",
        "async",
        "lightweight",
    ]

    if args.target not in examples:
        err(f'example "{args.target}" is not found.')
        info(f"Available examples: {examples}")
        return -1

    if args.target == "async":
        args.features = "async"
    if args.target == "soem":
        args.features = "soem"
    if args.target == "remote_soem":
        args.features = "remote_soem"
    if args.target == "twincat":
        args.features = "twincat"
    if args.target == "remote_twincat":
        args.features = "remote_twincat"
    if args.target == "simulator":
        args.features = "simulator"
    if args.target == "visualizer":
        args.features = "visualizer"
    if args.target == "lightweight":
        args.features = "lightweight"

    with working_dir("src/examples"):
        commands = ["cargo", "run"]
        if args.release:
            commands.append("--release")
        commands.append("--bin")
        commands.append(args.target)
        if hasattr(args, "features"):
            commands.append("--features")
            commands.append(args.features)

        subprocess.run(commands).check_returncode()


def rust_clear(_):
    with working_dir("src"):
        subprocess.run(["cargo", "clean"]).check_returncode()


def rust_coverage(args):
    if is_macos:
        args.cuda = False
    else:
        if not is_cuda_available():
            warn("CUDA is not installed. Skip building crates using CUDA.")
            args.cuda = False
        else:
            args.cuda = True

    if is_arrayfire_available():
        args.af = True
    else:
        warn("ArrayFire is not installed. Skip building crates using ArrayFire.")
        args.af = False

    with working_dir("src"):
        commands = [
            "cargo",
            "+nightly",
            "llvm-cov",
            "--features",
            "remote test-utilities python gpu",
            "--workspace",
            "--lcov",
            "--output-path",
            "lcov.info",
        ]
        if args.release:
            commands.append("--release")
        if not args.cuda or args.skip_cuda:
            commands.append("--exclude=autd3-backend-cuda")
        if not args.af:
            commands.append("--exclude=autd3-backend-arrayfire")

        subprocess.run(commands).check_returncode()


def capi_clear(_):
    with working_dir("capi"):
        subprocess.run(["cargo", "clean"]).check_returncode()


def build_capi(args, features=None):
    with working_dir("capi"):
        if is_macos:
            args.cuda = False
        else:
            if not is_cuda_available():
                warn("CUDA is not installed. Skip building crates using CUDA.")
                args.cuda = False
            else:
                args.cuda = True

        commands = ["cargo", "build"]
        if args.release:
            commands.append("--release")
        commands.append("--all")
        if features is not None:
            commands.append("--features")
            commands.append(features)

        if not args.cuda:
            commands.append("--exclude=autd3capi-backend-cuda")

        if is_macos and args.universal:
            commands.append("--exclude=autd3capi-link-visualizer")
            commands_x86 = commands.copy()
            commands_x86.append("--target=x86_64-apple-darwin")
            subprocess.run(commands_x86).check_returncode()
            commands_aarch64 = commands.copy()
            commands_aarch64.append("--target=aarch64-apple-darwin")
            subprocess.run(commands_aarch64).check_returncode()
        else:
            if is_linux and args.arch is not None:
                if args.arch == "arm32":
                    setup_arm32_linker()
                    commands.append("--exclude=autd3capi-backend-cuda")
                    commands.append("--exclude=autd3capi-link-visualizer")
                    commands.append("--target=armv7-unknown-linux-gnueabihf")
                elif args.arch == "aarch64":
                    setup_aarch64_linker()
                    commands.append("--exclude=autd3capi-backend-cuda")
                    commands.append("--exclude=autd3capi-link-visualizer")
                    commands.append("--target=aarch64-unknown-linux-gnu")
                else:
                    err(f'arch "{args.arch}" is not supported.')
                    sys.exit(-1)
            subprocess.run(commands).check_returncode()


def cpp_build(args):
    build_capi(args)

    os.makedirs("cpp/lib", exist_ok=True)
    os.makedirs("cpp/bin", exist_ok=True)
    if is_windows:
        target = "capi/target/release" if args.release else "capi/target/debug"
        for lib in glob.glob(f"{target}/*.dll.lib"):
            shutil.copy(lib, "cpp/lib")
        for dll in glob.glob(f"{target}/*.dll"):
            shutil.copy(dll, "cpp/bin")
        if not args.release:
            for pdb in glob.glob(f"{target}/*.pdb"):
                shutil.copy(pdb, "cpp/lib")
    elif is_macos:
        if args.universal:
            target = "capi/target/x86_64-apple-darwin/release" if args.release else "capi/target/x86_64-apple-darwin/debug"
            for x64_lib in glob.glob(f"{target}/*.dylib"):
                base_name = os.path.basename(x64_lib)
                subprocess.run(
                    [
                        "lipo",
                        "-create",
                        x64_lib,
                        f"./capi/target/aarch64-apple-darwin/release/{base_name}",
                        "-output",
                        f"./cpp/bin/{base_name}",
                    ]
                ).check_returncode()
        else:
            target = "capi/target/release" if args.release else "capi/target/debug"
            for lib in glob.glob(f"{target}/*.dylib"):
                shutil.copy(lib, "cpp/bin")
    elif is_linux:
        target = "capi/target/release" if args.release else "capi/target/debug"
        if args.arch is not None:
            info("Skip build examples because cross compilation is not supported.")
            args.no_examples = True
            if args.arch == "arm32":
                target = "capi/target/armv7-unknown-linux-gnueabihf/release" if args.release else "capi/target/armv7-unknown-linux-gnueabihf/debug"
            elif args.arch == "aarch64":
                target = "capi/target/aarch64-unknown-linux-gnu/release" if args.release else "capi/target/aarch64-unknown-linux-gnu/debug"
            else:
                pass
        for lib in glob.glob(f"{target}/*.so"):
            shutil.copy(lib, "cpp/bin")

    shutil.copyfile("LICENSE", "cpp/LICENSE")
    shutil.copyfile("README.md", "cpp/README.md")
    shutil.copyfile("capi/ThirdPartyNotice.txt", "cpp/ThirdPartyNotice.txt")

    if not args.no_examples:
        info("Building examples...")
        with working_dir("cpp/examples"):
            os.makedirs("build", exist_ok=True)
            with working_dir("build"):
                command = ["cmake", "..", "-DAUTD_LOCAL_TEST=ON"]
                if args.cmake_extra is not None:
                    for cmd in args.cmake_extra.split(" "):
                        command.append(cmd)
                subprocess.run(command).check_returncode()
                command = ["cmake", "--build", "."]
                if args.release:
                    command.append("--config")
                    command.append("Release")
                subprocess.run(command).check_returncode()


def cpp_test(args):
    args.release = True
    args.arch = None
    args.no_examples = True
    cpp_build(args)

    with working_dir("cpp/tests"):
        os.makedirs("build", exist_ok=True)
        with working_dir("build"):
            command = ["cmake", ".."]
            if args.cuda and not args.skip_cuda:
                command.append("-DENABLE_BACKEND_CUDA=ON")
            if args.cmake_extra is not None:
                for cmd in args.cmake_extra.split(" "):
                    command.append(cmd)
            subprocess.run(command).check_returncode()
            subprocess.run(["cmake", "--build", ".", "--config", "Release"]).check_returncode()

            target_dir = "Release" if is_windows else "."
            subprocess.run([f"{target_dir}/test_autd3{exe_ext}"]).check_returncode()


def cpp_run(args):
    args.universal = None
    args.arch = None
    args.no_examples = False
    args.cmake_extra = None
    cpp_build(args)

    if is_windows:
        target_dir = "Release" if args.release else "Debug"
    else:
        target_dir = "."

    subprocess.run([f"cpp/examples/build/{target_dir}/{args.target}{exe_ext}"]).check_returncode()


def cpp_clear(_):
    with working_dir("cpp"):
        rmtree_f("lib")
        rmtree_f("bin")
        rmtree_f("examples/build")
        rmtree_f("tests/build")


def cs_build(args):
    args.universal = True
    build_capi(args)

    if is_windows:
        target = "capi/target/release" if args.release else "capi/target/debug"
        for dll in glob.glob(f"{target}/*.dll"):
            shutil.copy(dll, "dotnet/cs/src/native/windows/x64")
    elif is_macos:
        target = "capi/target/x86_64-apple-darwin/release" if args.release else "capi/target/x86_64-apple-darwin/debug"
        target_aarch64 = "capi/target/aarch64-apple-darwin/release" if args.release else "capi/target/aarch64-apple-darwin/debug"
        for x64_lib in glob.glob(f"{target}/*.dylib"):
            base_name = os.path.basename(x64_lib)
            subprocess.run(
                [
                    "lipo",
                    "-create",
                    x64_lib,
                    f"{target_aarch64}/{base_name}",
                    "-output",
                    f"./dotnet/cs/src/native/osx/universal/{base_name}",
                ]
            ).check_returncode()
    elif is_linux:
        target = "capi/target/release" if args.release else "capi/target/debug"
        if args.arch is not None:
            info("Skip build examples because cross compilation is not supported.")
            args.no_examples = True
            if args.arch == "arm32":
                target = "capi/target/armv7-unknown-linux-gnueabihf/release" if args.release else "capi/target/armv7-unknown-linux-gnueabihf/debug"
            elif args.arch == "aarch64":
                target = "capi/target/aarch64-unknown-linux-gnu/release" if args.release else "capi/target/aarch64-unknown-linux-gnu/debug"
            else:
                pass
        for lib in glob.glob(f"{target}/*.so"):
            shutil.copy(lib, "dotnet/cs/src/native/linux/x64")

    shutil.copyfile("LICENSE", "dotnet/cs/src/LICENSE.txt")

    with open("capi/ThirdPartyNotice.txt", "r") as notice:
        with open("dotnet/cs/src/LICENSE.txt", "a") as f:
            f.write("\n=========================================================\n")
            f.write(notice.read())

    with working_dir("dotnet/cs/src"):
        command = ["dotnet", "build"]
        if args.release:
            command.append("-c:Release")
        subprocess.run(command).check_returncode()

        _ = subprocess.run(
            ["dotnet", "nuget", "remove", "source", "autd3sharp_local"],
            check=False,
            capture_output=True,
        )
        bin_dir = "Release" if args.release else "Debug"
        subprocess.run(
            [
                "dotnet",
                "nuget",
                "add",
                "source",
                f"{os.getcwd()}/bin/{bin_dir}",
                "-n",
                "autd3sharp_local",
            ]
        )

    if not args.no_examples:
        info("Building examples...")
        with working_dir("dotnet/cs/example"):
            command = ["dotnet", "build"]
            if args.release:
                command.append("-c:Release")
            subprocess.run(command).check_returncode()


def cs_test(args):
    args.universal = True
    args.release = True
    args.arch = None
    build_capi(args)

    if is_windows:
        target_dir = "capi/target/release"
        for dll in glob.glob(f"{target_dir}/*.dll"):
            shutil.copy(dll, "dotnet/cs/tests")
    elif is_macos:
        target_dir = "capi/target/x86_64-apple-darwin/release"
        for x64_lib in glob.glob(f"{target_dir}/*.dylib"):
            base_name = os.path.basename(x64_lib)
            subprocess.run(
                [
                    "lipo",
                    "-create",
                    x64_lib,
                    f"./capi/target/aarch64-apple-darwin/release/{base_name}",
                    "-output",
                    f"./dotnet/cs/tests/{base_name}",
                ]
            ).check_returncode()
    elif is_linux:
        target_dir = "capi/target/release"
        for lib in glob.glob(f"{target_dir}/*.so"):
            shutil.copy(lib, "dotnet/cs/tests")

    shutil.copyfile("LICENSE", "dotnet/cs/src/LICENSE.txt")

    with working_dir("dotnet/cs/src"):
        command = ["dotnet", "build"]
        command.append("-c:Release")
        subprocess.run(command).check_returncode()

    with working_dir("dotnet/cs/tests"):
        command = ["dotnet", "test"]
        subprocess.run(command).check_returncode()


def cs_run(args):
    args.arch = None
    args.no_examples = False
    cs_build(args)

    with working_dir("dotnet/cs/example"):
        command = ["dotnet", "run"]
        command.append("--project")
        command.append(args.target)
        if args.release:
            command.append("-c:Release")
        subprocess.run(command).check_returncode()


def cs_clear(_):
    with working_dir("dotnet/cs"):
        rmtree_f("src/bin")
        rmtree_f("src/obj")
        rm_f("src/LICENSE.txt")

        rmtree_f("tests/bin")
        rmtree_f("tests/obj")
        rm_glob_f("tests/*.dll")
        rm_glob_f("tests/*.dylib")
        rm_glob_f("tests/*.so")

        rmtree_glob_f("example/**/bin")
        rmtree_glob_f("example/**/obj")


def unity_build(args):
    ignore = shutil.ignore_patterns("NativeMethods")
    shutil.copytree(
        "dotnet/cs/src",
        "dotnet/unity/Assets/Scripts",
        dirs_exist_ok=True,
        ignore=ignore,
    )
    rm_f("dotnet/unity/Assets/Scripts/AUTD3Sharp.csproj")
    rm_f("dotnet/unity/Assets/Scripts/AUTD3Sharp.nuspec")
    rm_f("dotnet/unity/Assets/Scripts/LICENSE.txt")
    rm_f("dotnet/unity/Assets/Scripts/.gitignore")
    rmtree_f("dotnet/unity/Assets/Scripts/obj")
    rmtree_f("dotnet/unity/Assets/Scripts/bin")
    rmtree_f("dotnet/unity/Assets/Scripts/native")
    rmtree_f("dotnet/unity/Assets/Scripts/Utils")

    unity_dir = ""
    if is_windows:
        unity_dir = "dotnet/unity"
    elif is_macos:
        unity_dir = "dotnet/unity-mac"
    elif is_linux:
        unity_dir = "dotnet/unity-linux"

    if not is_windows:
        shutil.copytree(
            "dotnet/unity/Assets/Scripts",
            f"{unity_dir}/Assets/Scripts",
            dirs_exist_ok=True,
        )
        shutil.copytree(
            "dotnet/unity/Assets/Models",
            f"{unity_dir}/Assets/Models",
            dirs_exist_ok=True,
        )
        shutil.copytree(
            "dotnet/unity/Assets/Samples",
            f"{unity_dir}/Assets/Samples",
            dirs_exist_ok=True,
        )
        shutil.copytree(
            "dotnet/unity/Assets/Editor",
            f"{unity_dir}/Assets/Editor",
            dirs_exist_ok=True,
        )

    args.universal = True
    args.arch = None
    build_capi(args, "single_float use_meter")

    if is_windows:
        target = "capi/target/release" if args.release else "capi/target/debug"
        for dll in glob.glob(f"{target}/*.dll"):
            shutil.copy(dll, f"{unity_dir}/Assets/Plugins/x86_64")
    elif is_macos:
        target = "capi/target/x86_64-apple-darwin/release" if args.release else "capi/target/x86_64-apple-darwin/debug"
        target_aarch64 = "capi/target/aarch64-apple-darwin/release" if args.release else "capi/target/aarch64-apple-darwin/debug"
        for x64_lib in glob.glob(f"{target}/*.dylib"):
            base_name = os.path.basename(x64_lib)
            subprocess.run(
                [
                    "lipo",
                    "-create",
                    x64_lib,
                    f"{target_aarch64}/{base_name}",
                    "-output",
                    f"./{unity_dir}/Assets/Plugins/x86_64/{base_name}",
                ]
            ).check_returncode()
            shutil.copy(
                f"./{unity_dir}/Assets/Plugins/x86_64/{base_name}",
                f"./{unity_dir}/Assets/Plugins/aarch64/{base_name}",
            )
    elif is_linux:
        target = "capi/target/release" if args.release else "capi/target/debug"
        for lib in glob.glob(f"{target}/*.so"):
            shutil.copy(lib, f"{unity_dir}/Assets/Plugins/x86_64")

    shutil.copy("LICENSE", f"{unity_dir}/Assets/LICENSE.md")
    with open("capi/ThirdPartyNotice.txt", "r") as notice:
        with open(f"{unity_dir}/Assets/LICENSE.md", "a") as f:
            f.write("\n=========================================================\n")
            f.write(notice.read())
    shutil.copy("CHANGELOG.md", f"{unity_dir}/Assets/CHANGELOG.md")

    if is_windows:
        with working_dir("server/simulator"):
            commands = ["cargo", "build"]
            if args.release:
                commands.append("--release")
            commands.append("--features")
            commands.append("left_handed use_meter")
            subprocess.run(commands).check_returncode()

        simulator_src = "server/src-tauri/target/release/simulator.exe" if args.release else "server/src-tauri/target/debug/simulator.exe"
        shutil.copy(simulator_src, f"{unity_dir}/Assets/Editor/autd_simulator.exe")
        os.makedirs(f"{unity_dir}/Assets/Editor/assets", exist_ok=True)
        shutil.copy(
            "server/simulator/assets/autd3.glb",
            f"{unity_dir}/Assets/Editor/assets/autd3.glb",
        )

        with open("server/simulator/ThirdPartyNotice.txt", "r") as notice:
            with open(f"{unity_dir}/Assets/LICENSE.md", "a") as f:
                f.write("\n=========================================================\n")
                f.write("AUTD SIMULATOR ")
                f.write(notice.read())
    else:
        rm_f(f"{unity_dir}/Assets/Editor/assets.meta")
        rm_f(f"{unity_dir}/Assets/Editor/autd_simulator.exe.meta")
        rm_f(f"{unity_dir}/Assets/Editor/SimulatorRun.cs")
        rm_f(f"{unity_dir}/Assets/Editor/SimulatorRun.cs.meta")


def unity_clear(_):
    with working_dir("dotnet"):
        for unity_dir in ["unity", "unity-mac", "unity-linux"]:
            with working_dir(unity_dir):
                rmtree_f(".vs")
                rmtree_f("Library")
                rmtree_f("Logs")
                rmtree_f("obj")
                rmtree_f("Packages")
                rmtree_f("ProjectSettings")
                rmtree_f("UserSettings")
                rm_glob_f("Assets/Scripts/**/*.cs", exclude="Assets/Scripts/NativeMethods/*.cs")

        rm_glob_f("unity/Assets/Plugins/x86_64/*.dll")
        rm_glob_f("unity-mac/Assets/Plugins/aarch64/*.dylib")
        rm_glob_f("unity-mac/Assets/Plugins/x86_64/*.dylib")
        rm_glob_f("unity-linux/Assets/Plugins/x86_64/*.so")

        rm_f("unity/Assets/Editor/assets/autd3.glb")
        rm_f("unity/Assets/Editor/autd_simulator.exe")


def fs_build(args):
    no_examples = args.no_examples
    args.no_examples = True
    cs_build(args)

    if not no_examples:
        info("Building examples...")
        with working_dir("dotnet/fs/example"):
            command = ["dotnet", "build"]
            if args.release:
                command.append("-c:Release")
            subprocess.run(command).check_returncode()


def fs_run(args):
    args.arch = None
    args.no_examples = False
    fs_build(args)

    with working_dir("dotnet/fs/example"):
        command = ["dotnet", "run"]
        command.append("--project")
        command.append(args.target)
        if args.release:
            command.append("-c:Release")
        subprocess.run(command).check_returncode()


def fs_clear(_):
    with working_dir("dotnet/fs"):
        rmtree_glob_f("example/**/bin")
        rmtree_glob_f("example/**/obj")


def copy_py_bin(args):
    os.makedirs("python/pyautd3/bin", exist_ok=True)
    if is_windows:
        target = "capi/target/release" if args.release else "capi/target/debug"
        for dll in glob.glob(f"{target}/*.dll"):
            shutil.copy(dll, "python/pyautd3/bin")
    elif is_macos:
        if args.universal:
            target = "capi/target/x86_64-apple-darwin/release" if args.release else "capi/target/x86_64-apple-darwin/debug"
            for x64_lib in glob.glob(f"{target}/*.dylib"):
                base_name = os.path.basename(x64_lib)
                subprocess.run(
                    [
                        "lipo",
                        "-create",
                        x64_lib,
                        f"./capi/target/aarch64-apple-darwin/release/{base_name}",
                        "-output",
                        f"./python/pyautd3/bin/{base_name}",
                    ]
                ).check_returncode()
        else:
            target = "capi/target/release" if args.release else "capi/target/debug"
            for lib in glob.glob(f"{target}/*.dylib"):
                shutil.copy(lib, "python/pyautd3/bin")
    elif is_linux:
        target = "capi/target/release" if args.release else "capi/target/debug"
        if args.arch is not None:
            info("Skip build examples because cross compilation is not supported.")
            args.no_examples = True
            if args.arch == "arm32":
                target = "capi/target/armv7-unknown-linux-gnueabihf/release" if args.release else "capi/target/armv7-unknown-linux-gnueabihf/debug"
            elif args.arch == "aarch64":
                target = "capi/target/aarch64-unknown-linux-gnu/release" if args.release else "capi/target/aarch64-unknown-linux-gnu/debug"
            else:
                pass
        for lib in glob.glob(f"{target}/*.so"):
            shutil.copy(lib, "python/pyautd3/bin")

    shutil.copyfile("LICENSE", "python/pyautd3/LICENSE.txt")
    shutil.copyfile("capi/ThirdPartyNotice.txt", "python/pyautd3/ThirdPartyNotice.txt")


def build_wheel(args):
    with working_dir("python"):
        if is_windows:
            with open("setup.cfg.template", "r") as setup:
                content = setup.read()
                content = content.replace(r"${classifiers_os}", "Operating System :: Microsoft :: Windows")
                content = content.replace(r"${plat_name}", "win-amd64")
                with open("setup.cfg", "w") as f:
                    f.write(content)
            subprocess.run(["python", "-m", "build", "-w"]).check_returncode()
        elif is_macos:
            if args.universal:
                with open("setup.cfg.template", "r") as setup:
                    content = setup.read()
                    content = content.replace(r"${classifiers_os}", "Operating System :: MacOS :: MacOS X")
                    content = content.replace(r"${plat_name}", "macosx-10-13-x86_64")
                    with open("setup.cfg", "w") as f:
                        f.write(content)
                subprocess.run(["python3", "-m", "build", "-w"]).check_returncode()
                with open("setup.cfg.template", "r") as setup:
                    content = setup.read()
                    content = content.replace(r"${classifiers_os}", "Operating System :: MacOS :: MacOS X")
                    content = content.replace(r"${plat_name}", "macosx-11-0-arm64")
                    with open("setup.cfg", "w") as f:
                        f.write(content)
                subprocess.run(["python3", "-m", "build", "-w"]).check_returncode()
            else:
                with open("setup.cfg.template", "r") as setup:
                    content = setup.read()
                    content = content.replace(r"${classifiers_os}", "Operating System :: MacOS :: MacOS X")
                    plat_name = ""
                    if platform.machine() in ["ADM64", "x86_64"]:
                        plat_name = "macosx-10-13-x86_64"
                    else:
                        plat_name = "macosx-11-0-arm64"
                    content = content.replace(r"${plat_name}", plat_name)
                    with open("setup.cfg", "w") as f:
                        f.write(content)
                subprocess.run(["python3", "-m", "build", "-w"]).check_returncode()
        elif is_linux:
            with open("setup.cfg.template", "r") as setup:
                content = setup.read()
                content = content.replace(r"${classifiers_os}", "Operating System :: POSIX")
                plat_name = ""
                if args.arch is not None:
                    if args.arch == "arm32":
                        plat_name = "linux_armv7l"
                    elif args.arch == "aarch64":
                        plat_name = "manylinux2014_aarch64"
                    else:
                        err(f'arch "{args.arch}" is not supported.')
                        sys.exit(-1)
                else:
                    if platform.machine() in ["ADM64", "x86_64"]:
                        plat_name = "manylinux1-x86_64"
                    elif platform.machine() in ["armv7l"]:
                        plat_name = "linux_armv7l"
                    elif platform.machine() in ["aarch64"]:
                        plat_name = "manylinux2014_aarch64"
                content = content.replace(r"${plat_name}", plat_name)
                with open("setup.cfg", "w") as f:
                    f.write(content)
            subprocess.run(["python3", "-m", "build", "-w"]).check_returncode()


def py_build(args):
    build_capi(args)
    copy_py_bin(args)
    build_wheel(args)

    if not args.no_install:
        with working_dir("python"):
            version = ""
            with open("setup.cfg.template", "r") as setup:
                content = setup.read()
                m = re.search("version = (.*)", content)
                version = m.group(1)
            command = []
            if is_windows:
                command.append("python")
            else:
                command.append("python3")
            command.append("-m")
            command.append("pip")
            command.append("install")
            plat_name = ""
            if is_windows:
                plat_name = "win_amd64"
            elif is_macos:
                if platform.machine() in ["ADM64", "x86_64"]:
                    plat_name = "macosx_10_13_x86_64"
                else:
                    plat_name = "macosx_11_0_arm64"
            elif is_linux:
                if platform.machine() in ["ADM64", "x86_64"]:
                    plat_name = "manylinux1_x86_64"
                elif platform.machine() in ["armv7l"]:
                    plat_name = "linux_armv7l"
                elif platform.machine() in ["aarch64"]:
                    plat_name = "manylinux2014_aarch64"
            else:
                err(f'platform "{platform.system()}/{platform.machine()}" is not supported.')
                sys.exit(-1)
            command.append(f"dist/pyautd3-{version}-py3-none-{plat_name}.whl")
            command.append("--force")
            subprocess.run(command).check_returncode()


def py_test(args):
    args.universal = None
    args.arch = None
    args.no_install = True
    build_capi(args)
    copy_py_bin(args)

    with working_dir("python"):
        command = []
        if is_windows:
            command.append("python")
        else:
            command.append("python3")
        command.append("-m")
        command.append("mypy")
        command.append("pyautd3")
        subprocess.run(command).check_returncode()

        command = []
        if is_windows:
            command.append("python")
        else:
            command.append("python3")
        command.append("-m")
        command.append("pytest")
        if is_cuda_available() and not args.skip_cuda:
            command.append("--test_cuda")
        subprocess.run(command).check_returncode()


def py_clear(_):
    with working_dir("python"):
        rm_f("setup.cfg")
        rmtree_f("dist")
        rmtree_f("build")
        rmtree_f("pyautd3.egg-info")
        rmtree_f("pyautd3/bin")
        rmtree_f(".mypy_cache")
        rmtree_f(".pytest_cache")
        rmtree_f("pyautd3/__pycache__")
        rmtree_f("tests/__pycache__")
        rm_f("pyautd3/LICENSE.txt")
        rm_f("pyautd3/ThirdPartyNotice.txt")


def server_build(args):
    with working_dir("server"):
        if is_windows:
            subprocess.run(["npm", "install"], shell=True).check_returncode()
        else:
            subprocess.run(["npm", "install"]).check_returncode()

        if is_macos:
            command_x86 = ["cargo", "build", "--release", "--target=x86_64-apple-darwin"]
            command_aarch64 = ["cargo", "build", "--release", "--target=aarch64-apple-darwin"]

            with working_dir("simulator"):
                subprocess.run(command_x86).check_returncode()
                subprocess.run(command_aarch64).check_returncode()

            with working_dir("SOEMAUTDServer"):
                subprocess.run(command_x86).check_returncode()
                subprocess.run(command_aarch64).check_returncode()

            with working_dir("LightweightTwinCATAUTDServer"):
                subprocess.run(command_x86).check_returncode()
                subprocess.run(command_aarch64).check_returncode()

            if not args.external_only:
                subprocess.run(
                    [
                        "npm",
                        "run",
                        "tauri",
                        "build",
                        "--",
                        "--target",
                        "universal-apple-darwin",
                    ]
                ).check_returncode()
        else:
            command = ["cargo", "build", "--release"]

            with working_dir("simulator"):
                subprocess.run(command).check_returncode()

            with working_dir("SOEMAUTDServer"):
                subprocess.run(command).check_returncode()

            with working_dir("LightweightTwinCATAUTDServer"):
                subprocess.run(command).check_returncode()

            if not args.external_only:
                if is_windows:
                    subprocess.run(["npm", "run", "tauri", "build"], shell=True).check_returncode()
                else:
                    subprocess.run(["npm", "run", "tauri", "build"]).check_returncode()


def server_clear(_):
    with working_dir("server"):
        if is_windows:
            subprocess.run(["npm", "cache", "clean", "--force"], shell=True).check_returncode()
        else:
            subprocess.run(["npm", "cache", "clean", "--force"]).check_returncode()
        rmtree_f("node_modules")
        rmtree_f("dist")

        with working_dir("src-tauri"):
            rmtree_f("assets")
            rm_f("NOTICE")
            rm_glob_f("LICENSE*")
            rm_glob_f("LightweightTwinCATAUTDServer*")
            rm_glob_f("simulator*")
            rm_glob_f("SOEMAUTDServer*")
            subprocess.run(["cargo", "clean"]).check_returncode()


def doc_build(args):
    with working_dir("doc"):
        command = ["mdbook", "build", "--dest-dir", f"book/{args.target}"]
        if args.open:
            command.append("--open")
        with set_env("MDBOOK_BOOK__src", f"src/{args.target}"):
            subprocess.run(command).check_returncode()


def doc_test(args):
    rust_clear(args)

    with working_dir("src"):
        command = ["cargo", "build", "--all", "--features", "remote", "--exclude", "autd3-backend-arrayfire", "--exclude", "autd3-backend-cuda"]
        subprocess.run(command).check_returncode()

    with working_dir("doc"):
        for t in args.target.split(","):
            command = ["mdbook", "test", "--dest-dir", f"book/{t}", "-L", "./../src/target/debug/deps"]
            with set_env("MDBOOK_BOOK__src", f"src/{t}"):
                subprocess.run(command).check_returncode()


def command_help(args):
    print(parser.parse_args([args.command, "--help"]))


if __name__ == "__main__":
    fetch_submodule()

    with working_dir(os.path.dirname(os.path.abspath(__file__))):
        parser = argparse.ArgumentParser(description="autd3 library build script")
        subparsers = parser.add_subparsers()

        # build (rust)
        parser_build = subparsers.add_parser("build", help="see `build -h`")
        parser_build.add_argument("--all", action="store_true", help="build all crates")
        parser_build.add_argument("--release", action="store_true", help="release build")
        parser_build.add_argument("--arch", help="cross-compile for specific architecture (for Linux)")
        parser_build.add_argument("--no-examples", action="store_true", help="skip building examples")
        parser_build.set_defaults(handler=rust_build)

        # lint (rust)
        parser_lint = subparsers.add_parser("lint", help="see `lint -h`")
        parser_lint.add_argument("--all", action="store_true", help="lint all crates")
        parser_lint.add_argument("--release", action="store_true", help="release build")
        parser_lint.set_defaults(handler=rust_lint)

        # test (rust)
        parser_test = subparsers.add_parser("test", help="see `test -h`")
        parser_test.add_argument("--all", action="store_true", help="test all crates")
        parser_test.add_argument("--skip-cuda", action="store_true", help="force skip cuda test")
        parser_test.add_argument("--release", action="store_true", help="release build")
        parser_test.set_defaults(handler=rust_test)

        # run (rust)
        parser_run = subparsers.add_parser("run", help="see `run -h`")
        parser_run.add_argument("target", help="binary target")
        parser_run.add_argument("--release", action="store_true", help="release build")
        parser_run.set_defaults(handler=rust_run)

        # clear (rust)
        parser_clear = subparsers.add_parser("clear", help="see `clear -h`")
        parser_clear.set_defaults(handler=rust_clear)

        # coverage (rust)
        parser_cov = subparsers.add_parser("cov", help="see `cov -h`")
        parser_cov.add_argument("--release", action="store_true", help="release build")
        parser_cov.add_argument("--skip-cuda", action="store_true", help="force skip cuda test")
        parser_cov.set_defaults(handler=rust_coverage)

        # capi
        parser_capi = subparsers.add_parser("capi", help="see `capi -h`")
        subparsers_capi = parser_capi.add_subparsers()

        # capi clear
        parser_capi_clear = subparsers_capi.add_parser("clear", help="see `capi clear -h`")
        parser_capi_clear.set_defaults(handler=capi_clear)

        # cpp
        parser_cpp = subparsers.add_parser("cpp", help="see `cpp -h`")
        subparsers_cpp = parser_cpp.add_subparsers()

        # cpp build
        parser_cpp_build = subparsers_cpp.add_parser("build", help="see `cpp build -h`")
        parser_cpp_build.add_argument("--release", action="store_true", help="release build")
        parser_cpp_build.add_argument(
            "--universal",
            action="store_true",
            help="build universal binary (for macOS)",
        )
        parser_cpp_build.add_argument("--arch", help="cross-compile for specific architecture (for Linux)")
        parser_cpp_build.add_argument("--no-examples", action="store_true", help="skip building examples")
        parser_cpp_build.add_argument("--cmake-extra", help="cmake extra args")
        parser_cpp_build.set_defaults(handler=cpp_build)

        # cpp test
        parser_cpp_test = subparsers_cpp.add_parser("test", help="see `cpp test -h`")
        parser_cpp_test.add_argument("--skip-cuda", action="store_true", help="force skip cuda test")
        parser_cpp_test.add_argument(
            "--universal",
            action="store_true",
            help="build universal binary (for macOS)",
        )
        parser_cpp_test.add_argument("--cmake-extra", help="cmake extra args")
        parser_cpp_test.set_defaults(handler=cpp_test)

        # cpp run
        parser_cpp_run = subparsers_cpp.add_parser("run", help="see `cpp run -h`")
        parser_cpp_run.add_argument("target", help="binary target")
        parser_cpp_run.add_argument("--release", action="store_true", help="release build")
        parser_cpp_run.set_defaults(handler=cpp_run)

        # cpp clear
        parser_cpp_clear = subparsers_cpp.add_parser("clear", help="see `cpp clear -h`")
        parser_cpp_clear.set_defaults(handler=cpp_clear)

        # cs
        parser_cs = subparsers.add_parser("cs", help="see `cs -h`")
        subparsers_cs = parser_cs.add_subparsers()

        # cs build
        parser_cs_build = subparsers_cs.add_parser("build", help="see `cs build -h`")
        parser_cs_build.add_argument("--release", action="store_true", help="release build")
        parser_cs_build.add_argument("--arch", help="cross-compile for specific architecture (for Linux)")
        parser_cs_build.add_argument("--no-examples", action="store_true", help="skip building examples")
        parser_cs_build.set_defaults(handler=cs_build)

        # cs test
        parser_cs_test = subparsers_cs.add_parser("test", help="see `cs test -h`")
        parser_cs_test.set_defaults(handler=cs_test)

        # cs run
        parser_cs_run = subparsers_cs.add_parser("run", help="see `cs run -h`")
        parser_cs_run.add_argument("target", help="binary target")
        parser_cs_run.add_argument("--release", action="store_true", help="release build")
        parser_cs_run.set_defaults(handler=cs_run)

        # cs clear
        parser_cs_clear = subparsers_cs.add_parser("clear", help="see `cs clear -h`")
        parser_cs_clear.set_defaults(handler=cs_clear)

        # unity
        parser_unity = subparsers.add_parser("unity", help="see `unity -h`")
        subparsers_unity = parser_unity.add_subparsers()

        # unity build
        parser_unity_build = subparsers_unity.add_parser("build", help="see `unity build -h`")
        parser_unity_build.add_argument("--release", action="store_true", help="release build")
        parser_unity_build.set_defaults(handler=unity_build)

        # unity clear
        parser_unity_clear = subparsers_unity.add_parser("clear", help="see `unity clear -h`")
        parser_unity_clear.set_defaults(handler=unity_clear)

        # fs
        parser_fs = subparsers.add_parser("fs", help="see `fs -h`")
        subparsers_fs = parser_fs.add_subparsers()

        # fs build
        parser_fs_build = subparsers_fs.add_parser("build", help="see `fs build -h`")
        parser_fs_build.add_argument("--release", action="store_true", help="release build")
        parser_fs_build.add_argument("--arch", help="cross-compile for specific architecture (for Linux)")
        parser_fs_build.add_argument("--no-examples", action="store_true", help="skip building examples")
        parser_fs_build.set_defaults(handler=fs_build)

        # fs run
        parser_fs_run = subparsers_fs.add_parser("run", help="see `fs run -h`")
        parser_fs_run.add_argument("target", help="binary target")
        parser_fs_run.add_argument("--release", action="store_true", help="release build")
        parser_fs_run.set_defaults(handler=fs_run)

        # fs clear
        parser_fs_clear = subparsers_fs.add_parser("clear", help="see `fs clear -h`")
        parser_fs_clear.set_defaults(handler=fs_clear)

        # python
        parser_py = subparsers.add_parser("python", help="see `python -h`")
        subparsers_py = parser_py.add_subparsers()

        # python build
        parser_py_build = subparsers_py.add_parser("build", help="see `python build -h`")
        parser_py_build.add_argument("--release", action="store_true", help="release build")
        parser_py_build.add_argument(
            "--universal",
            action="store_true",
            help="build universal binary (for macOS)",
        )
        parser_py_build.add_argument("--arch", help="cross-compile for specific architecture (for Linux)")
        parser_py_build.add_argument("--no-install", action="store_true", help="skip install python package")
        parser_py_build.set_defaults(handler=py_build)

        # python test
        parser_py_test = subparsers_py.add_parser("test", help="see `python test -h`")
        parser_py_test.add_argument("--release", action="store_true", help="release build")
        parser_py_test.add_argument("--skip-cuda", action="store_true", help="force skip cuda test")
        parser_py_test.set_defaults(handler=py_test)

        # python clear
        parser_py_clear = subparsers_py.add_parser("clear", help="see `python clear -h`")
        parser_py_clear.set_defaults(handler=py_clear)

        # server
        parser_server = subparsers.add_parser("server", help="see `server -h`")
        subparsers_server = parser_server.add_subparsers()

        # server build
        parser_server_build = subparsers_server.add_parser("build", help="see `server build -h`")
        parser_server_build.add_argument(
            "--external-only",
            action="store_true",
            help="build external dependencies only",
        )
        parser_server_build.set_defaults(handler=server_build)

        # server clear
        parser_server_clear = subparsers_server.add_parser("clear", help="see `server clear -h`")
        parser_server_clear.set_defaults(handler=server_clear)

        # doc
        parser_doc = subparsers.add_parser("doc", help="see `doc -h`")
        subparsers_doc = parser_doc.add_subparsers()

        # doc build
        parser_doc_build = subparsers_doc.add_parser("build", help="see `doc build -h`")
        parser_doc_build.add_argument("target", help="build target [jp|en]")
        parser_doc_build.add_argument("--open", help="open browser after build", action="store_true")
        parser_doc_build.set_defaults(handler=doc_build)

        # doc test
        parser_doc_test = subparsers_doc.add_parser("test", help="see `doc test -h`")
        parser_doc_test.add_argument("target", help="test target [jp|en]")
        parser_doc_test.set_defaults(handler=doc_test)

        # help
        parser_help = subparsers.add_parser("help", help="see `help -h`")
        parser_help.add_argument("command", help="command name which help is shown")
        parser_help.set_defaults(handler=command_help)

        args = parser.parse_args()
        if hasattr(args, "handler"):
            args.handler(args)
        else:
            parser.print_help()
