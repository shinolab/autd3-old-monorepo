"""
File: download_bin.py
Project: python
Created Date: 24/07/2023
Author: Shun Suzuki
-----
Last Modified: 24/07/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""

import glob
import requests
import shutil
import zipfile
import tarfile
import os
import sys
import platform


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


_set_package_version(_get_version())

_VERSION_TRIPLE = ".".join(_get_version().split(".")[0:3])


class Target:
    bin_ext = ""
    archive_ext = ""
    os_name = ""
    arch = "x64"

    def __init__(self, os_name, arch, bin_ext, archive_ext):
        self.os_name = os_name
        self.arch = arch
        self.bin_ext = bin_ext
        self.archive_ext = archive_ext


asset_base_url = "https://github.com/shinolab/autd3/releases/download/"
version = f"v{_VERSION_TRIPLE}"

targets = []
pf = platform.system()
if pf == "Windows":
    if sys.maxsize <= 2**32:
        print("32bit is not supported")
        exit(1)
    if platform.machine() in ("i386", "AMD64", "x86_64"):
        targets.append(Target("win", "x64", "dll", "zip"))
    else:
        print("Unsupported architecture:", platform.machine())
        exit(1)
elif pf == "Darwin":
    if sys.maxsize <= 2**32:
        print("32bit is not supported")
        exit(1)
    targets.append(Target("macos", "universal", "dylib", "tar.gz"))
elif pf == "Linux":
    if sys.maxsize <= 2**32:
        print("32bit is not supported")
        exit(1)
    if platform.machine() in ("i386", "AMD64", "x86_64"):
        targets.append(Target("linux", "x64", "so", "tar.gz"))
    else:
        print("Unsupported architecture:", platform.machine())
        exit(1)

for target in targets:
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
