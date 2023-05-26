# Versioning

AUTD3 SDK follows [Semantic Versioning](https://semver.org/) from v8.2.0.

The version of AUTD3 SDK is denoted as vX.Y.Z, where
- X is the major version, and compatibility is not guaranteed between different major versions.
- Y is the minor version, which is raised when a feature is added with backward compatibility.
- Z denotes a patch version, which is raised when a bug is fixed with backward compatibility.

## Firmware version

On the other hand, the firmware version does not follow Semantic Versioning.
The firmware version is detoted as v2.x.y, where 
- x represents the major version of the firmware, and compatibility of the firmware between different major versions is not guaranteed.
- y represents the patch version, which is raised when a bug is fixed with backward compatibility.

You can check if the SDK uses the latest version of the supported firmware with the function `FirmwareInfo::is_supported`.
```cpp
  if (!std::all_of(firm_infos.begin(), firm_infos.end(), autd3::FirmwareInfo::is_supported))
    std::cerr << "WARN: You are using old firmware. Please consider updating to " << autd3::FirmwareInfo::latest_version() << std::endl;
```

The CPU and FPGA firmware have their own versions, and the normal operation is not guranteed in the case that these versions are different.
You can check if these versions match with the `FirmwareInfo::matches_version` function.
```cpp
  const auto firm_infos = autd.firmware_infos();
  if (!std::all_of(firm_infos.begin(), firm_infos.end(), autd3::FirmwareInfo::matches_version))
    std::cerr << "WARN: FPGA and CPU firmware version do not match" << std::endl;
```
