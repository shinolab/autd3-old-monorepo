# Package

version       = "9.0.1"
author        = "shun suzuki"
description   = "A new awesome nimble package"
license       = "MIT"
srcDir        = "src"
bin           = @["soem", "simulator"]

# Dependencies

requires "nim >= 1.6.6"
requires "https://github.com/shinolab/autd3.git?subdir=nim == 9.0.1"
