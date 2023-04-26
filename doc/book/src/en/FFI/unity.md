# Unity

**Unity version has a left-handed coordinate system with z-axis inversion, and the unit of distance is in meters.**

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
    "com.shinolab.autd3": "8.5.0",
    ...
```

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
