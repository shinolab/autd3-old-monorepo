# Setup/Compile

## Install Vivado

本文章で利用しているのはWindows版のVivado ML Standard 2022.1.1である.

Vivadoは[Xilinx社のサイト](https://japan.xilinx.com/products/design-tools/vivado.html)から無料でダウンロードできる (要登録).

基本的にはダウンロードしたインストーラを起動し, 指示に従えばよい.
コンパイルするためには, Vitisは不要であり, 最低限Artix-7 Seriesのサポートがあれば十分である.

## Build project

Vivadoのインストール後, autd3-fpgaのリポジトリをダウンロードし, `build.ps1`スクリプトをPowerShellで実行すると, `autd3-fpga.xpr`ファイルが生成される.
