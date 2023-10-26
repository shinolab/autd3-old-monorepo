#!/usr/bin/bash

apt-get update
apt-get install -y build-essential
apt-get install -y python3.10 python3.10-distutils
apt-get install -y protobuf-compiler
apt-get install -y pkg-config libfreetype6-dev libfontconfig1-dev
apt-get install -y gcc-aarch64-linux-gnu g++-aarch64-linux-gnu

apt-get install -y wget
wget -qO- https://packages.lunarg.com/lunarg-signing-key-pub.asc | tee /etc/apt/trusted.gpg.d/lunarg.asc
wget -qO /etc/apt/sources.list.d/lunarg-vulkan-jammy.list http://packages.lunarg.com/vulkan/lunarg-vulkan-jammy.list
apt-get update
apt-get install -y shaderc
