# C++ tutorial

## Install dependencies

In this tutorial, we use [CMake](https://cmake.org/) to build the program.

## Build first program

First, open a terminal and prepare a directory for the sample.

```shell
mkdir autd3-sample
cd autd3-sample
```

Then, make `CMakeLists.txt` and `main.cpp` files.

```shell,filename=
└─autd3-sample
        CMakeLists.txt
        main.cpp
```

Next, edit `CMakeLists.txt` as follows,

```ignore,filename=CMakeLists.txt
{{#include ../../../../samples/cpp/CMakeLists.txt}}
```

And, edit `main.cpp` as follows.
This is the source code for generating a focus with $\SI{150}{Hz}$ AM modulation. 

```cpp,filename=main.cpp
{{#include ../../../../samples/cpp/main.cpp}}
```

Then, build with CMake.

```shell
mkdir build
cd build
cmake .. -DCMAKE_BUILD_TYPE=Release
cmake --build . --config Release
```

Finally, run the program.

```shell,filename=Windows
.\Release\main.exe
```

```shell,filename=Linux/macOS
sudo ./main
```
