# Migration Guide from v1.x

Here is a guide for migration from version 1.x.

## Removed features

### Output enable, Output balance flag

These flags have been removed.

### pause, resume

These functions have been removed.

If you want to stop output, call `stop` or send `modulation::Static(0.0)`.
If you want to resume output, send the desired data again.

### Duty Offset specification

This function is removed.

## Changed API

### Silent mode

Silent mode flag is removed.

Instead, you can adjust the silent mode more finely by sending the `SilentConfig`.

The default `SilentConfig` is roughly equivalent to the old `silent_mode = true`, and `SilentConfig::none()` is equivalent to `silent_mode = false`.

See [Silencer](silencer.md) for details.

### Synchronize

The `Controller::synchronize` function must be called once at the beginning.
