'''
File: download_bin.py
Project: python
Created Date: 24/07/2023
Author: Shun Suzuki
-----
Last Modified: 21/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''

import glob
import requests
import shutil
import zipfile
import tarfile
import os
import sys
import platform
import argparse


def _get_version():
    with open("README.md", "r") as f:
        for line in f.readlines():
            if line.startswith("version: "):
                return line.replace("version: ", "").strip()
    raise LookupError("version info is not found in README.md")


def _set_package_version(version):
    init_py = ""
    with open("pyautd3/__init__.py", "r") as f:
        for line in f.readlines():
            if line.startswith("__version__"):
                line = '__version__ = "' + version.strip() + '"\n'
            init_py = init_py + line

    with open("pyautd3/__init__.py", "w") as f:
        f.write(init_py)


class Target:
    bin_ext: str = ""
    archive_ext: str = ""
    os_name: str = ""
    arch: str = ""

    def __init__(self, arch=None):
        is_32bit = sys.maxsize <= 2**32
        is_amd64 = platform.machine() in ("AMD64", "x86_64")

        if platform.system() == "Windows":
            if is_32bit:
                print("32bit is not supported")
                exit(1)
            if not is_amd64:
                print("Unsupported architecture:", platform.machine())
                exit(1)
            self.os_name = "win"
            self.bin_ext = "dll"
            self.archive_ext = "zip"
            self.arch = "x64"
        elif platform.system() == "Darwin":
            if is_32bit:
                print("32bit is not supported")
                exit(1)
            self.os_name = "macos"
            self.bin_ext = "dylib"
            self.archive_ext = "tar.gz"
            self.arch = "universal"
        elif platform.system() == "Linux":
            self.os_name = "linux"
            self.bin_ext = "so"
            self.archive_ext = "tar.gz"
            if arch is not None:
                self.arch = arch
            else:
                if is_32bit:
                    print("32bit is not supported")
                    exit(1)
                if not is_amd64:
                    print("Unsupported architecture:", platform.machine())
                    exit(1)
                self.arch = "x64"


if __name__ == "__main__":
    _set_package_version(_get_version())

    _VERSION_TRIPLE = ".".join(_get_version().split(".")[0:3])

    parser = argparse.ArgumentParser()
    parser.add_argument("--arch", required=False, metavar="NAME", type=str)

    args = parser.parse_args()

    asset_base_url = "https://github.com/shinolab/autd3/releases/download/"
    version = f"v{_VERSION_TRIPLE}"

    target = Target(args.arch)
    url = f"{asset_base_url}{version}/autd3-{version}-{target.os_name}-{target.arch}.{target.archive_ext}"

    tmp_archive_path = f"tmp.{target.archive_ext}"

    res = requests.get(url, stream=True)
    with open(tmp_archive_path, "wb") as fp:
        shutil.copyfileobj(res.raw, fp)

    if target.archive_ext == "zip":
        with zipfile.ZipFile(tmp_archive_path) as f:
            for info in f.infolist():
                if info.filename.startswith("bin") and info.filename.endswith(
                    target.bin_ext
                ):
                    f.extract(info, ".")
                if info.filename.endswith("ThirdPartyNotice.txt"):
                    f.extract(info, ".")
    elif target.archive_ext == "tar.gz":
        with tarfile.open(tmp_archive_path) as f:
            libraries = []
            for i in f.getmembers():
                if i.name.startswith("bin") and i.name.endswith(target.bin_ext):
                    libraries.append(i)
                if i.name.endswith("ThirdPartyNotice.txt"):
                    libraries.append(i)
            f.extractall(path=".", members=libraries)

    if os.path.exists("pyautd3/bin"):
        shutil.rmtree("pyautd3/bin")
    os.makedirs("pyautd3/bin")
    for f in glob.glob("./bin/*"):
        shutil.move(f, "pyautd3/bin")
    if os.path.exists("LICENSE.txt"):
        os.remove("LICENSE.txt")
    shutil.copy("../LICENSE", "LICENSE.txt")
    os.remove(tmp_archive_path)
    shutil.rmtree("./bin")
