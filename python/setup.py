'''
File: setup.py
Project: pyautd
Created Date: 10/09/2020
Author: Shun Suzuki
-----
Last Modified: 09/10/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2021 Shun Suzuki. All rights reserved.
'''

import glob
import setuptools
import requests
import shutil
import zipfile
import tarfile
import os


def _get_version():
    with open('README.md', 'r') as f:
        for line in f.readlines():
            if line.startswith('version: '):
                return line.replace('version: ', '').strip()
    raise LookupError('version info is not found in README.md')


def _set_package_version(version):
    init_py = ''
    with open('pyautd3/__init__.py', 'r') as f:
        for line in f.readlines():
            if line.startswith('__version__'):
                line = '__version__ = \'' + version.strip() + '\'\n'
            init_py = init_py + line

    with open('pyautd3/__init__.py', 'w') as f:
        f.write(init_py)


with open('README.md', 'r') as fh:
    long_description = fh.read()

_set_package_version(_get_version())

_VERSION_TRIPLE = '.'.join(_get_version().split('.')[0:3])


class Target:
    bin_ext = ''
    archive_ext = ''
    os_name = ''
    arch = 'x64'

    def __init__(self, os_name, arch, bin_ext, archive_ext):
        self.os_name = os_name
        self.arch = arch
        self.bin_ext = bin_ext
        self.archive_ext = archive_ext


def download_bin():
    asset_base_url = 'https://github.com/shinolab/autd3/releases/download/'
    version = f'v{_VERSION_TRIPLE}'

    targets = [
        Target('win', 'x64', 'dll', 'zip'),
        Target('macos', 'universal', 'dylib', 'tar.gz'),
        Target('linux', 'x64', 'so', 'tar.gz'),
        Target('linux', 'arm64', 'so', 'tar.gz'),
        Target('linux', 'arm32', 'so', 'tar.gz'),
    ]

    for target in targets:
        url = f'{asset_base_url}{version}/autd3-{version}-{target.os_name}-{target.arch}.{target.archive_ext}'

        tmp_archive_path = f'tmp.{target.archive_ext}'

        res = requests.get(url, stream=True)
        with open(tmp_archive_path, 'wb') as fp:
            shutil.copyfileobj(res.raw, fp)

        if target.archive_ext == 'zip':
            with zipfile.ZipFile(tmp_archive_path) as f:
                for info in f.infolist():
                    if info.filename.startswith('bin') and info.filename.endswith(target.bin_ext):
                        f.extract(info, '.')
        elif target.archive_ext == 'tar.gz':
            with tarfile.open(tmp_archive_path) as f:
                libraries = []
                for i in f.getmembers():
                    if i.name.startswith('bin') and i.name.endswith(target.bin_ext):
                        libraries.append(i)
                f.extractall(path='.', members=libraries)

        os.makedirs(f'pyautd3/bin/{target.os_name}_{target.arch}', exist_ok=True)
        for f in glob.glob('./bin/*'):
            shutil.move(f, f'pyautd3/bin/{target.os_name}_{target.arch}')
        os.remove(tmp_archive_path)


download_bin()

data_files = []
for f in glob.glob('pyautd3/bin/**/*'):
    data_files.append(f)

setuptools.setup(
    name='pyautd3',
    version=_get_version(),
    author='Shun Suzuki',
    author_email='suzuki@hapis.k.u-tokyo.ac.jp',
    description='AUTD3 library wrapper for python',
    long_description=long_description,
    long_description_content_type='text/markdown',
    url='https://github.com/shinolab/pyautd',
    classifiers=[
        'Programming Language :: Python :: 3',
        'Operating System :: Microsoft :: Windows',
        'Operating System :: POSIX',
        'Operating System :: MacOS'
    ],
    license='See https://github.com/shinolab/pyautd',
    platforms=["Windows", "Linux", "Mac OS-X"],
    include_package_data=True,
    package_dir={'pyautd3': 'pyautd3'},
    packages=['pyautd3', 'pyautd3.native_methods'],
    python_requires='>=3.7',
    data_files=[(
        'lib/site-packages', data_files
    )],
)
