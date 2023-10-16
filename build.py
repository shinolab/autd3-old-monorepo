#!/usr/bin/env python3

'''
File: build.py
Project: autd3
Created Date: 16/10/2023
Author: Shun Suzuki
-----
Last Modified: 16/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''

import argparse
import glob
import re
import shutil
import subprocess
import os
import sys
import platform
from shutil import which


def err(msg: str):
    print('\033[91mERR \033[0m: ' + msg)


def warn(msg: str):
    print('\033[93mWARN\033[0m: ' + msg)


def info(msg: str):
    print('\033[92mINFO\033[0m: ' + msg)


is_windows = platform.system() == 'Windows'
is_macos = platform.system() == 'Darwin'
is_linux = platform.system() == 'Linux'
if not is_windows and not is_macos and not is_linux:
    err(f'platform "{platform.system()}" is not supported.')
    sys.exit(-1)

exe_ext = '.exe' if is_windows else ''


def is_cuda_available():
    return which('nvcc') is not None


def is_arrayfire_available():
    return 'AF_PATH' in os.environ


def setup_arm32_linker():
    os.makedirs('.cargo', exist_ok=True)
    with open('.cargo/config', 'w') as f:
        f.write('[target.armv7-unknown-linux-gnueabihf]\n')
        f.write('linker = "arm-linux-gnueabihf-gcc"\n')


def setup_aarch64_linker():
    os.makedirs('.cargo', exist_ok=True)
    with open('.cargo/config', 'w') as f:
        f.write('[target.aarch64-unknown-linux-gnu]\n')
        f.write('linker = "aarch64-linux-gnu-gcc"\n')


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

    os.chdir('src')

    commands = ['cargo', 'build']
    if args.release:
        commands.append('--release')
    if args.all:
        commands.append('--all')
        if not args.cuda:
            commands.append('--exclude=autd3-backend-cuda')
        if not args.af:
            commands.append('--exclude=autd3-backend-arrayfire')

    if is_linux and args.arch is not None:
        info('Skip build examples because cross compilation is not supported.')
        args.no_examples = True
        if args.arch == 'arm32':
            setup_arm32_linker()
            commands.append('--exclude=autd3-backend-cuda')
            commands.append('--exclude=autd3-backend-arrayfire')
            commands.append('--exclude=autd3-link-visualizer')
            commands.append('--target=armv7-unknown-linux-gnueabihf')
        elif args.arch == 'aarch64':
            setup_aarch64_linker()
            commands.append('--exclude=autd3-backend-cuda')
            commands.append('--exclude=autd3-backend-arrayfire')
            commands.append('--exclude=autd3-link-visualizer')
            commands.append('--target=aarch64-unknown-linux-gnu')
        else:
            err(f'arch "{args.arch}" is not supported.')
            sys.exit(-1)

    subprocess.run(commands).check_returncode()

    if not args.no_examples:
        info('Building examples...')
        os.chdir('examples')
        command = ['cargo', 'build', '--bins']
        if args.release:
            command.append('--release')
        features = 'async soem twincat'
        if args.all:
            features += " simulator remote_soem remote_twincat visualizer gpu python lightweight"
        command.append('--features')
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

    os.chdir('src')
    commands = ['cargo', 'clippy']
    if args.release:
        commands.append('--release')
    if args.all:
        commands.append('--all')
        if not args.cuda:
            commands.append('--exclude=autd3-backend-cuda')
        if not args.af:
            commands.append('--exclude=autd3-backend-arrayfire')
    commands.append('--')
    commands.append('-D')
    commands.append('warnings')
    subprocess.run(commands).check_returncode()
    os.chdir('..')

    os.chdir('capi')
    commands = ['cargo', 'clippy']
    if args.release:
        commands.append('--release')
    if args.all:
        commands.append('--all')
        if not args.cuda:
            commands.append('--exclude=autd3capi-backend-cuda')
    commands.append('--')
    commands.append('-D')
    commands.append('warnings')

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

    os.chdir('src')

    commands = ['cargo', 'test']
    if args.release:
        commands.append('--release')
    if args.all:
        commands.append('--all')
        if not args.cuda and not args.skip_cuda:
            commands.append('--exclude=autd3-backend-cuda')
        if not args.af:
            commands.append('--exclude=autd3-backend-arrayfire')
    commands.append('--features')
    commands.append('test-utilities')

    subprocess.run(commands).check_returncode()


def rust_run(args):
    examples = [
        'group_gain',
        'soem',
        'remote_soem',
        'twincat',
        'remote_twincat',
        'simulator',
        'visualizer',
        'freq_config',
        'async',
        'lightweight'
    ]

    if args.target not in examples:
        err(f'example "{args.target}" is not found.')
        info(f'Available examples: {examples}')
        return -1

    if args.target == 'async':
        args.features = 'async'
    if args.target == 'soem':
        args.features = 'soem'
    if args.target == 'remote_soem':
        args.features = 'remote_soem'
    if args.target == 'twincat':
        args.features = 'twincat'
    if args.target == 'remote_twincat':
        args.features = 'remote_twincat'
    if args.target == 'simulator':
        args.features = 'simulator'
    if args.target == 'visualizer':
        args.features = 'visualizer'
    if args.target == 'lightweight':
        args.features = 'lightweight'

    os.chdir('src/examples')

    commands = ['cargo', 'run']
    if args.release:
        commands.append('--release')
    commands.append('--bin')
    commands.append(args.target)
    if hasattr(args, 'features'):
        commands.append('--features')
        commands.append(args.features)

    subprocess.run(commands).check_returncode()


def build_capi(args, features=None):
    try:
        os.chdir('capi')

        if is_macos:
            args.cuda = False
        else:
            if not is_cuda_available():
                warn("CUDA is not installed. Skip building crates using CUDA.")
                args.cuda = False
            else:
                args.cuda = True

        commands = ['cargo', 'build']
        if args.release:
            commands.append('--release')
        commands.append('--all')
        if features is not None:
            commands.append('--features')
            commands.append(features)

        if not args.cuda:
            commands.append('--exclude=autd3capi-backend-cuda')

        if is_macos and args.universal:
            commands.append('--exclude=autd3capi-link-visualizer')
            commands_x86 = commands.copy()
            commands_x86.append('--target=x86_64-apple-darwin')
            subprocess.run(commands_x86).check_returncode()
            commands_aarch64 = commands.copy()
            commands_aarch64.append('--target=aarch64-apple-darwin')
            subprocess.run(commands_aarch64).check_returncode()
        else:
            if is_linux and args.arch is not None:
                if args.arch == 'arm32':
                    setup_arm32_linker()
                    commands.append('--exclude=autd3capi-backend-cuda')
                    commands.append('--exclude=autd3capi-link-visualizer')
                    commands.append('--target=armv7-unknown-linux-gnueabihf')
                elif args.arch == 'aarch64':
                    setup_aarch64_linker()
                    commands.append('--exclude=autd3capi-backend-cuda')
                    commands.append('--exclude=autd3capi-link-visualizer')
                    commands.append('--target=aarch64-unknown-linux-gnu')
                else:
                    err(f'arch "{args.arch}" is not supported.')
                    sys.exit(-1)
            subprocess.run(commands).check_returncode()
    except Exception as e:
        raise e
    finally:
        os.chdir('..')


def cpp_build(args):
    build_capi(args)

    os.makedirs('cpp/lib', exist_ok=True)
    os.makedirs('cpp/bin', exist_ok=True)
    if is_windows:
        target = 'capi/target/release' if args.release else 'capi/target/debug'
        for lib in glob.glob(f'{target}/*.dll.lib'):
            shutil.copy(lib, 'cpp/lib')
        for dll in glob.glob(f'{target}/*.dll'):
            shutil.copy(dll, 'cpp/bin')
        if not args.release:
            for pdb in glob.glob(f'{target}/*.pdb'):
                shutil.copy(pdb, 'cpp/lib')
    elif is_macos:
        if args.universal:
            target = 'capi/target/x86_64-apple-darwin/release' if args.release else 'capi/target/x86_64-apple-darwin/debug'
            for x64_lib in glob.glob(f'{target}/*.dylib'):
                base_name = os.path.basename(x64_lib)
                subprocess.run(['lipo',
                                '-create',
                                x64_lib,
                                f'./capi/target/aarch64-apple-darwin/release/{base_name}',
                                '-output',
                                f'./cpp/bin/{base_name}']).check_returncode()
        else:
            target = 'capi/target/release' if args.release else 'capi/target/debug'
            for lib in glob.glob(f'{target}/*.dylib'):
                shutil.copy(lib, 'cpp/bin')
    elif is_linux:
        target = 'capi/target/release' if args.release else 'capi/target/debug'
        if args.arch is not None:
            info('Skip build examples because cross compilation is not supported.')
            args.no_examples = True
            if args.arch == 'arm32':
                target = 'capi/target/armv7-unknown-linux-gnueabihf/release' if args.release else 'capi/target/armv7-unknown-linux-gnueabihf/debug'
            elif args.arch == 'aarch64':
                target = 'capi/target/aarch64-unknown-linux-gnu/release' if args.release else 'capi/target/aarch64-unknown-linux-gnu/debug'
            else:
                pass
        for lib in glob.glob(f'{target}/*.so'):
            shutil.copy(lib, 'cpp/bin')

    shutil.copyfile('capi/ThirdPartyNotice.txt', 'cpp/ThirdPartyNotice.txt')

    if not args.no_examples:
        info('Building examples...')
        os.chdir('cpp/examples')
        os.makedirs('build', exist_ok=True)
        os.chdir('build')
        command = ['cmake', '..', '-DAUTD_LOCAL_TEST=ON']
        if args.cmake_extra is not None:
            for cmd in args.cmake_extra.split(' '):
                command.append(cmd)
        subprocess.run(command).check_returncode()
        command = ['cmake', '--build', '.']
        if args.release:
            command.append('--config')
            command.append('Release')
        subprocess.run(command).check_returncode()


def cpp_test(args):
    args.release = True
    args.universal = None
    args.arch = None
    args.no_examples = True
    cpp_build(args)

    os.chdir('cpp/tests')
    os.makedirs('build', exist_ok=True)
    os.chdir('build')
    command = ['cmake', '..']
    if args.cuda and not args.skip_cuda:
        command.append('-DENABLE_BACKEND_CUDA=ON')
    if args.cmake_extra is not None:
        for cmd in args.cmake_extra.split(' '):
            command.append(cmd)
    subprocess.run(command).check_returncode()
    subprocess.run(['cmake', '--build', '.', '--config', 'Release']).check_returncode()

    target_dir = 'Release' if is_windows else '.'
    subprocess.run([f'{target_dir}/test_autd3{exe_ext}']).check_returncode()


def cpp_run(args):
    args.universal = None
    args.arch = None
    args.no_examples = False
    cpp_build(args)

    if is_windows:
        target_dir = 'Release' if args.release else 'Debug'
    else:
        target_dir = '.'

    subprocess.run([f'{target_dir}/{args.target}{exe_ext}']).check_returncode()


def cs_build(args):
    args.universal = True
    build_capi(args)

    if is_windows:
        target = 'capi/target/release' if args.release else 'capi/target/debug'
        for dll in glob.glob(f'{target}/*.dll'):
            shutil.copy(dll, 'dotnet/cs/src/native/windows/x64')
    elif is_macos:
        target = 'capi/target/x86_64-apple-darwin/release' if args.release else 'capi/target/x86_64-apple-darwin/debug'
        target_aarch64 = 'capi/target/aarch64-apple-darwin/release' if args.release else 'capi/target/aarch64-apple-darwin/debug'
        for x64_lib in glob.glob(f'{target}/*.dylib'):
            base_name = os.path.basename(x64_lib)
            subprocess.run(['lipo',
                            '-create',
                            x64_lib,
                            f'{target_aarch64}/{base_name}',
                            '-output',
                            f'./dotnet/cs/src/native/osx/universal/{base_name}']).check_returncode()
    elif is_linux:
        target = 'capi/target/release' if args.release else 'capi/target/debug'
        if args.arch is not None:
            info('Skip build examples because cross compilation is not supported.')
            args.no_examples = True
            if args.arch == 'arm32':
                target = 'capi/target/armv7-unknown-linux-gnueabihf/release' if args.release else 'capi/target/armv7-unknown-linux-gnueabihf/debug'
            elif args.arch == 'aarch64':
                target = 'capi/target/aarch64-unknown-linux-gnu/release' if args.release else 'capi/target/aarch64-unknown-linux-gnu/debug'
            else:
                pass
        for lib in glob.glob(f'{target}/*.so'):
            shutil.copy(lib, 'dotnet/cs/src/native/linux/x64')

    shutil.copyfile('LICENSE', 'dotnet/cs/src/LICENSE.txt')

    with open('capi/ThirdPartyNotice.txt', 'r') as notice:
        with open('dotnet/cs/src/LICENSE.txt', 'a') as f:
            f.write('\n=========================================================\n')
            f.write(notice.read())

    os.chdir('dotnet/cs/src')
    command = ['dotnet', 'build']
    if args.release:
        command.append('-c:Release')
    subprocess.run(command).check_returncode()

    _ = subprocess.run(['dotnet', 'nuget', 'remove', 'source', 'autd3sharp_local'], check=False, capture_output=True)
    bin_dir = 'Release' if args.release else 'Debug'
    subprocess.run(['dotnet', 'nuget', 'add', 'source', f'{os.getcwd()}/bin/{bin_dir}', '-n', 'autd3sharp_local'])

    if not args.no_examples:
        info('Building examples...')
        os.chdir('../example')
        command = ['dotnet', 'build']
        if args.release:
            command.append('-c:Release')
        subprocess.run(command).check_returncode()


def cs_test(args):
    args.universal = True
    args.release = True
    args.arch = None
    build_capi(args)

    if is_windows:
        target_dir = 'capi/target/release'
        for dll in glob.glob(f'{target_dir}/*.dll'):
            shutil.copy(dll, 'dotnet/cs/tests')
    elif is_macos:
        target_dir = 'capi/target/x86_64-apple-darwin/release'
        for x64_lib in glob.glob(f'{target_dir}/*.dylib'):
            base_name = os.path.basename(x64_lib)
            subprocess.run(['lipo',
                            '-create',
                            x64_lib,
                            f'./capi/target/aarch64-apple-darwin/release/{base_name}',
                            '-output',
                            f'./dotnet/cs/tests/{base_name}']).check_returncode()
    elif is_linux:
        target_dir = 'capi/target/release'
        for lib in glob.glob(f'{target_dir}/*.so'):
            shutil.copy(lib, 'dotnet/cs/tests')

    shutil.copyfile('LICENSE', 'dotnet/cs/src/LICENSE.txt')

    os.chdir('dotnet/cs/src')
    command = ['dotnet', 'build']
    command.append('-c:Release')
    subprocess.run(command).check_returncode()

    os.chdir('../tests')
    command = ['dotnet', 'test']
    subprocess.run(command).check_returncode()


def cs_run(args):
    args.arch = None
    args.no_examples = False
    cs_build(args)

    command = ['dotnet', 'run']
    command.append('--project')
    command.append(args.target)
    if args.release:
        command.append('-c:Release')
    subprocess.run(command).check_returncode()


def unity_build(args):
    ignore = shutil.ignore_patterns('NativeMethods')
    shutil.copytree('dotnet/cs/src', 'dotnet/unity/Assets/Scripts', dirs_exist_ok=True, ignore=ignore)
    try:
        os.remove('dotnet/unity/Assets/Scripts/AUTD3Sharp.csproj')
        os.remove('dotnet/unity/Assets/Scripts/AUTD3Sharp.nuspec')
        os.remove('dotnet/unity/Assets/Scripts/LICENSE.txt')
        os.remove('dotnet/unity/Assets/Scripts/.gitignore')
        shutil.rmtree('dotnet/unity/Assets/Scripts/obj')
        shutil.rmtree('dotnet/unity/Assets/Scripts/bin')
        shutil.rmtree('dotnet/unity/Assets/Scripts/native')
        shutil.rmtree('dotnet/unity/Assets/Scripts/Utils')
    except FileNotFoundError:
        pass

    if is_macos:
        shutil.copytree(
            'dotnet/unity/Assets/Scripts/NativeMethods',
            'dotnet/unity-mac/Assets/Scripts/NativeMethods',
            dirs_exist_ok=True)
    if is_linux:
        shutil.copytree(
            'dotnet/unity/Assets/Scripts/NativeMethods',
            'dotnet/unity-linux/Assets/Scripts/NativeMethods',
            dirs_exist_ok=True)

    args.universal = True
    args.arch = None
    build_capi(args, 'single_float use_meter')

    unity_dir = ''
    if is_windows:
        unity_dir = 'dotnet/unity'
    elif is_macos:
        unity_dir = 'dotnet/unity-mac'
    elif is_linux:
        unity_dir = 'dotnet/unity-linux'

    if is_windows:
        target = 'capi/target/release' if args.release else 'capi/target/debug'
        for dll in glob.glob(f'{target}/*.dll'):
            shutil.copy(dll, f'{unity_dir}/Assets/Plugins/x86_64')
    elif is_macos:
        target = 'capi/target/x86_64-apple-darwin/release' if args.release else 'capi/target/x86_64-apple-darwin/debug'
        target_aarch64 = 'capi/target/aarch64-apple-darwin/release' if args.release else 'capi/target/aarch64-apple-darwin/debug'
        for x64_lib in glob.glob(f'{target}/*.dylib'):
            base_name = os.path.basename(x64_lib)
            subprocess.run(['lipo',
                            '-create',
                            x64_lib,
                            f'{target_aarch64}/{base_name}',
                            '-output',
                            f'./{unity_dir}/Assets/Plugins/x86_64/{base_name}']).check_returncode()
    elif is_linux:
        target = 'capi/target/release' if args.release else 'capi/target/debug'
        for lib in glob.glob(f'{target}/*.so'):
            shutil.copy(lib, f'{unity_dir}/Assets/Plugins/x86_64')

    shutil.copy('LICENSE', f'{unity_dir}/Assets/LICENSE.md')
    with open('capi/ThirdPartyNotice.txt', 'r') as notice:
        with open(f'{unity_dir}/Assets/LICENSE.md', 'a') as f:
            f.write('\n=========================================================\n')
            f.write(notice.read())
    shutil.copy('CHANGELOG.md', f'{unity_dir}/Assets/CHANGELOG.md')

    if is_windows:
        os.chdir('server/simulator')
        commands = ['cargo', 'build']
        if args.release:
            commands.append('--release')
        commands.append('--features')
        commands.append('left_handed use_meter')
        os.chdir('../..')

        shutil.copy('server/src-tauri/target/release/simulator.exe', f'{unity_dir}/Assets/Editor/autd_simulator.exe')
        os.makedirs(f'{unity_dir}/Assets/Editor/assets', exist_ok=True)
        shutil.copy('server/simulator/assets/autd3.glb', f'{unity_dir}/Assets/Editor/assets/autd3.glb')

        with open('server/simulator/ThirdPartyNotice.txt', 'r') as notice:
            with open(f'{unity_dir}/Assets/LICENSE.md', 'a') as f:
                f.write('\n=========================================================\n')
                f.write('AUTD SIMULATOR ')
                f.write(notice.read())


def fs_build(args):
    no_examples = args.no_examples
    args.no_examples = True
    cs_build(args)

    if not no_examples:
        info('Building examples...')
        os.chdir('../../fs/example')
        command = ['dotnet', 'build']
        if args.release:
            command.append('-c:Release')
        subprocess.run(command).check_returncode()


def fs_run(args):
    args.arch = None
    args.no_examples = False
    fs_build(args)

    command = ['dotnet', 'run']
    command.append('--project')
    command.append(args.target)
    if args.release:
        command.append('-c:Release')
    subprocess.run(command).check_returncode()


def copy_py_bin(args):
    os.makedirs('python/pyautd3/bin', exist_ok=True)
    if is_windows:
        target = 'capi/target/release' if args.release else 'capi/target/debug'
        for dll in glob.glob(f'{target}/*.dll'):
            shutil.copy(dll, 'python/pyautd3/bin')
    elif is_macos:
        if args.universal:
            target = 'capi/target/x86_64-apple-darwin/release' if args.release else 'capi/target/x86_64-apple-darwin/debug'
            for x64_lib in glob.glob(f'{target}/*.dylib'):
                base_name = os.path.basename(x64_lib)
                subprocess.run(['lipo',
                                '-create',
                                x64_lib,
                                f'./capi/target/aarch64-apple-darwin/release/{base_name}',
                                '-output',
                                f'./python/pyautd3/bin/{base_name}']).check_returncode()
        else:
            target = 'capi/target/release' if args.release else 'capi/target/debug'
            for lib in glob.glob(f'{target}/*.dylib'):
                shutil.copy(lib, 'python/pyautd3/bin')
    elif is_linux:
        target = 'capi/target/release' if args.release else 'capi/target/debug'
        if args.arch is not None:
            info('Skip build examples because cross compilation is not supported.')
            args.no_examples = True
            if args.arch == 'arm32':
                target = 'capi/target/armv7-unknown-linux-gnueabihf/release' if args.release else 'capi/target/armv7-unknown-linux-gnueabihf/debug'
            elif args.arch == 'aarch64':
                target = 'capi/target/aarch64-unknown-linux-gnu/release' if args.release else 'capi/target/aarch64-unknown-linux-gnu/debug'
            else:
                pass
        for lib in glob.glob(f'{target}/*.so'):
            shutil.copy(lib, 'python/pyautd3/bin')

    shutil.copyfile('LICENSE', 'python/pyautd3/LICENSE.txt')
    shutil.copyfile('capi/ThirdPartyNotice.txt', 'python/pyautd3/ThirdPartyNotice.txt')


def build_wheel(args):
    if is_windows:
        with open('python/setup.cfg.template', 'r') as setup:
            content = setup.read()
            content = content.replace(r'${classifiers_os}', 'Operating System :: Microsoft :: Windows')
            content = content.replace(r'${plat_name}', 'win-amd64')
            with open('python/setup.cfg', 'w') as f:
                f.write(content)
        os.chdir('python')
        subprocess.run(['python', '-m', 'build', '-w']).check_returncode()
    elif is_macos:
        if args.universal:
            os.chdir('python')
            with open('setup.cfg.template', 'r') as setup:
                content = setup.read()
                content = content.replace(r'${classifiers_os}', 'Operating System :: MacOS :: MacOS X')
                content = content.replace(r'${plat_name}', 'macosx-10-13-x86_64')
                with open('setup.cfg', 'w') as f:
                    f.write(content)
            subprocess.run(['python3', '-m', 'build', '-w']).check_returncode()
            with open('setup.cfg.template', 'r') as setup:
                content = setup.read()
                content = content.replace(r'${classifiers_os}', 'Operating System :: MacOS :: MacOS X')
                content = content.replace(r'${plat_name}', 'macosx-11-0-arm64')
                with open('setup.cfg', 'w') as f:
                    f.write(content)
            subprocess.run(['python3', '-m', 'build', '-w']).check_returncode()
        else:
            with open('python/setup.cfg.template', 'r') as setup:
                content = setup.read()
                content = content.replace(r'${classifiers_os}', 'Operating System :: MacOS :: MacOS X')
                plat_name = ''
                if platform.machine() in ['ADM64', 'x86_64']:
                    plat_name = 'macosx-10-13-x86_64'
                else:
                    plat_name = 'macosx-11-0-arm64'
                content = content.replace(r'${plat_name}', plat_name)
                with open('python/setup.cfg', 'w') as f:
                    f.write(content)
            os.chdir('python')
            subprocess.run(['python3', '-m', 'build', '-w']).check_returncode()

    elif is_linux:
        with open('python/setup.cfg.template', 'r') as setup:
            content = setup.read()
            content = content.replace(r'${classifiers_os}', 'Operating System :: POSIX')
            plat_name = ''
            if args.arch is not None:
                if args.arch == 'arm32':
                    plat_name = 'linux_armv7l'
                elif args.arch == 'aarch64':
                    plat_name = 'linux_armv7l'
                else:
                    err(f'arch "{args.arch}" is not supported.')
                    sys.exit(-1)
            else:
                if platform.machine() in ['ADM64', 'x86_64']:
                    plat_name = 'manylinux1-x86_64'
                elif platform.machine() in ['armv7l']:
                    plat_name = 'linux_armv7l'
                elif platform.machine() in ['aarch64']:
                    plat_name = 'manylinux2014_aarch64'
                content = content.replace(r'${plat_name}', plat_name)
            with open('python/setup.cfg', 'w') as f:
                f.write(content)
        os.chdir('python')
        subprocess.run(['python3', '-m', 'build', '-w']).check_returncode()


def py_build(args):
    build_capi(args)
    copy_py_bin(args)
    build_wheel(args)

    if not args.no_install:
        with open('setup.cfg.template', 'r') as setup:
            content = setup.read()
            m = re.search('version = (.*)', content)
            version = m.group(1)
            command = []
            if is_windows:
                command.append('python')
            else:
                command.append('python3')
            command.append('-m')
            command.append('pip')
            command.append('install')
            plat_name = ''
            if is_windows:
                plat_name = 'win_amd64'
            elif is_macos:
                if platform.machine() in ['ADM64', 'x86_64']:
                    plat_name = 'macosx_10_13_x86_64'
                else:
                    plat_name = 'macosx_11_0_arm64'
            elif is_linux:
                if platform.machine() in ['ADM64', 'x86_64']:
                    plat_name = 'manylinux1_x86_64'
                elif platform.machine() in ['armv7l']:
                    plat_name = 'linux_armv7l'
                elif platform.machine() in ['aarch64']:
                    plat_name = 'manylinux2014_aarch64'
            else:
                err(f'platform "{platform.system()}/{platform.machine()}" is not supported.')
                sys.exit(-1)
            command.append(f'dist/pyautd3-{version}-py3-none-{plat_name}.whl')
            command.append('--force')
            subprocess.run(command).check_returncode()


def py_test(args):
    args.universal = None
    args.arch = None
    args.no_install = True
    build_capi(args)
    copy_py_bin(args)

    os.chdir('python')

    command = []
    if is_windows:
        command.append('python')
    else:
        command.append('python3')
    command.append('-m')
    command.append('mypy')
    command.append('pyautd3')
    subprocess.run(command).check_returncode()

    command = []
    if is_windows:
        command.append('python')
    else:
        command.append('python3')
    command.append('-m')
    command.append('pytest')
    if is_cuda_available() and not args.skip_cuda:
        command.append('--test_cuda')
    subprocess.run(command).check_returncode()


def command_help(args):
    print(parser.parse_args([args.command, '--help']))


if __name__ == '__main__':
    path = os.getcwd()

    try:
        os.chdir(os.path.dirname(os.path.abspath(__file__)))

        parser = argparse.ArgumentParser(description='autd3 library build script')
        subparsers = parser.add_subparsers()

        # build (rust)
        parser_build = subparsers.add_parser('build', help='see `build -h`')
        parser_build.add_argument('--all', action='store_true', help='build all crates')
        parser_build.add_argument('--release', action='store_true', help='release build')
        parser_build.add_argument('--arch', help='cross-compile for specific architecture (for Linux)')
        parser_build.add_argument('--no-examples', action='store_true', help='skip building examples')
        parser_build.set_defaults(handler=rust_build)

        # lint (rust)
        parser_lint = subparsers.add_parser('lint', help='see `lint -h`')
        parser_lint.add_argument('--all', action='store_true', help='lint all crates')
        parser_lint.add_argument('--release', action='store_true', help='release build')
        parser_lint.set_defaults(handler=rust_lint)

        # test (rust)
        parser_test = subparsers.add_parser('test', help='see `test -h`')
        parser_test.add_argument('--all', action='store_true', help='test all crates')
        parser_test.add_argument('--skip-cuda', action='store_true', help='force skip cuda test')
        parser_test.add_argument('--release', action='store_true', help='release build')
        parser_test.set_defaults(handler=rust_test)

        # run (rust)
        parser_run = subparsers.add_parser('run', help='see `run -h`')
        parser_run.add_argument('target', help='binary target')
        parser_run.add_argument('--release', action='store_true', help='release build')
        parser_run.set_defaults(handler=rust_run)

        # cpp
        parser_cpp = subparsers.add_parser('cpp', help='see `cpp -h`')
        subparsers_cpp = parser_cpp.add_subparsers()

        # cpp build
        parser_cpp_build = subparsers_cpp.add_parser('build', help='see `cpp build -h`')
        parser_cpp_build.add_argument('--release', action='store_true', help='release build')
        parser_cpp_build.add_argument('--universal', action='store_true', help='build universal binary (for macOS)')
        parser_cpp_build.add_argument('--arch', help='cross-compile for specific architecture (for Linux)')
        parser_cpp_build.add_argument('--no-examples', action='store_true', help='skip building examples')
        parser_cpp_build.add_argument('--cmake-extra', help='cmake extra args')
        parser_cpp_build.set_defaults(handler=cpp_build)

        # cpp test
        parser_cpp_test = subparsers_cpp.add_parser('test', help='see `cpp test -h`')
        parser_cpp_test.add_argument('--skip-cuda', action='store_true', help='force skip cuda test')
        parser_cpp_test.add_argument('--cmake-extra', help='cmake extra args')
        parser_cpp_test.set_defaults(handler=cpp_test)

        # cpp run
        parser_cpp_run = subparsers_cpp.add_parser('run', help='see `cpp run -h`')
        parser_cpp_run.add_argument('target', help='binary target')
        parser_cpp_run.add_argument('--release', action='store_true', help='release build')
        parser_cpp_run.set_defaults(handler=cpp_run)

        # cs
        parser_cs = subparsers.add_parser('cs', help='see `cs -h`')
        subparsers_cs = parser_cs.add_subparsers()

        # cs build
        parser_cs_build = subparsers_cs.add_parser('build', help='see `cs build -h`')
        parser_cs_build.add_argument('--release', action='store_true', help='release build')
        parser_cs_build.add_argument('--arch', help='cross-compile for specific architecture (for Linux)')
        parser_cs_build.add_argument('--no-examples', action='store_true', help='skip building examples')
        parser_cs_build.set_defaults(handler=cs_build)

        # cs test
        parser_cs_test = subparsers_cs.add_parser('test', help='see `cs test -h`')
        parser_cs_test.set_defaults(handler=cs_test)

        # cs run
        parser_cs_run = subparsers_cs.add_parser('run', help='see `cs run -h`')
        parser_cs_run.add_argument('target', help='binary target')
        parser_cs_run.add_argument('--release', action='store_true', help='release build')
        parser_cs_run.set_defaults(handler=cs_run)

        # unity
        parser_unity = subparsers.add_parser('unity', help='see `unity -h`')
        subparsers_unity = parser_unity.add_subparsers()

        # unity build
        parser_unity_build = subparsers_unity.add_parser('build', help='see `unity build -h`')
        parser_unity_build.add_argument('--release', action='store_true', help='release build')
        parser_unity_build.set_defaults(handler=unity_build)

        # fs
        parser_fs = subparsers.add_parser('fs', help='see `fs -h`')
        subparsers_fs = parser_fs.add_subparsers()

        # fs build
        parser_fs_build = subparsers_fs.add_parser('build', help='see `fs build -h`')
        parser_fs_build.add_argument('--release', action='store_true', help='release build')
        parser_fs_build.add_argument('--arch', help='cross-compile for specific architecture (for Linux)')
        parser_fs_build.add_argument('--no-examples', action='store_true', help='skip building examples')
        parser_fs_build.set_defaults(handler=fs_build)

        # fs run
        parser_fs_run = subparsers_fs.add_parser('run', help='see `fs run -h`')
        parser_fs_run.add_argument('target', help='binary target')
        parser_fs_run.add_argument('--release', action='store_true', help='release build')
        parser_fs_run.set_defaults(handler=fs_run)

        # python
        parser_py = subparsers.add_parser('python', help='see `python -h`')
        subparsers_py = parser_py.add_subparsers()

        # python build
        parser_py_build = subparsers_py.add_parser('build', help='see `python build -h`')
        parser_py_build.add_argument('--release', action='store_true', help='release build')
        parser_py_build.add_argument('--universal', action='store_true', help='build universal binary (for macOS)')
        parser_py_build.add_argument('--arch', help='cross-compile for specific architecture (for Linux)')
        parser_py_build.add_argument('--no-install', action='store_true', help='skip install python package')
        parser_py_build.set_defaults(handler=py_build)

        # python test
        parser_py_test = subparsers_py.add_parser('test', help='see `python test -h`')
        parser_py_test.add_argument('--release', action='store_true', help='release build')
        parser_py_test.add_argument('--skip-cuda', action='store_true', help='force skip cuda test')
        parser_py_test.set_defaults(handler=py_test)

        # help
        parser_help = subparsers.add_parser('help', help='see `help -h`')
        parser_help.add_argument('command', help='command name which help is shown')
        parser_help.set_defaults(handler=command_help)

        args = parser.parse_args()
        if hasattr(args, 'handler'):
            args.handler(args)
        else:
            parser.print_help()
    except Exception as e:
        err(str(e))
        sys.exit(-1)
    finally:
        os.chdir(path)
