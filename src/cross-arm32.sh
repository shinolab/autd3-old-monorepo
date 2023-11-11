#!/usr/bin/bash

apt-get update
apt-get install -y build-essential
apt-get install -y python3.10 python3.10-distutils
apt-get install -y protobuf-compiler
apt-get install -y pkg-config libfreetype6-dev libfontconfig1-dev
apt-get install -y gcc-arm-linux-gnueabihf g++-arm-linux-gnueabihf
