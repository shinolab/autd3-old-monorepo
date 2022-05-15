#!/bin/sh
cd $1
mkdir build
cd build 
cmake .. -DBUILD_ALL=ON -DBUILD_DOC=ON
make doc
