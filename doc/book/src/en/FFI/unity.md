# Unity

**Unity version has a left-handed coordinate system with z-axis inversion, and the unit of distance is in meters.**

> Note: Using Unity and SOEM link might be cause an error. Please see [FAQ](https://shinolab.github.io/autd3/book/en/FAQ/faq#frequent-send-failures-when-using-linksoem) for more details. 

## Installation

Please install from Unity Package Manager.

### from npmjs

Append the following to `Packages/manifest.json`

```json
{
  "scopedRegistries": [
    {
      "name": "shinolab",
      "url": "https://registry.npmjs.com",
      "scopes": [ "com.shinolab" ]
    }
  ],
  "dependencies": {
    "com.shinolab.autd3": "8.1.1",
    ...
```

### GitHub

- Open Window→Package Manager, then click "+" and select "Add Package from git URL", add `https://github.com/shinolab/autd3.git#upm/latest`
    - If you want to use the old version, add `https://github.com/shinolab/autd3.git#upm/vX.Y.Z` instead

## Samples

- Import "Samples/Simple" from Unity Package Manager

- Please also see [autd3sharp's example](https://github.com/shinolab/autd3/tree/master/cs/example).

## Editor Extensions

- AUTD/Enumerate Adapters: list EtherCAT adapters
- AUTD/Simulator: Run AUTD-Simulator for Unity

## Troubleshooting

Q. I can't run the program from Linux or mac.

A. Currently, not supported.

---

If you have any other questions, please send them to [GitHub issue](https://github.com/shinolab/autd3/issues).
