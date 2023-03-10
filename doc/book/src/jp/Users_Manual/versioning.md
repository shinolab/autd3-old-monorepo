# バージョニング

AUTD3はv8.2.0から[セマンティック バージョニング](https://semver.org/lang/ja/)に従っている.

AUTD3 SDKのバージョンはvX.Y.Zと表記される.

- Xはメジャーバージョンを表し, これが異なるSDK間では互換性は保証されない.
- Yはマイナーバージョンを表し, 後方互換性を保つような機能追加があった場合に上げられる.
- Zはパッチバージョンを表し, 後方互換性を保つようなバグ修正があった場合に上げられる.

## ファームウェアのバージョン

一方, ファームウェアのバージョンはセマンティック バージョニングには従っていない.
ファームウェアのバージョンはv2.x.yとなっている.
ここで, ファームウェアのxがメジャーバージョンを表しており, 異なるx間のファームウェアの互換性は保証されない.
また, yはパッチバージョンを表しており, これは互換性を保つバグ修正があった場合に上げられる.

SDKがサポートするファームウェアの最新版を使用しているかどうかは, `FirmwareInfo::is_supported`関数にて確認できる.
```cpp
  if (!std::all_of(firm_infos.begin(), firm_infos.end(), autd3::FirmwareInfo::is_supported))
    std::cerr << "WARN: You are using old firmware. Please consider updating to " << autd3::FirmwareInfo::latest_version() << std::endl;
```

また, CPUとFPGAそれぞれにバージョンがあり, これらのバージョンが異なる場合の動作は保証していない.
これらのバージョンが一致しているかどうかは, `FirmwareInfo::matches_version`関数にて確認できる.
```cpp
  const auto firm_infos = autd.firmware_infos();
  if (!std::all_of(firm_infos.begin(), firm_infos.end(), autd3::FirmwareInfo::matches_version))
    std::cerr << "WARN: FPGA and CPU firmware version do not match" << std::endl;
```
