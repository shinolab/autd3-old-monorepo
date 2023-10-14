# Spatio-Temporal Modulation

SDK provides a function to switch `Gain` periodically (Spatio-Temporal Modulation, STM).
The SDK provides `FocusSTM` that supports only a single focus and `GainSTM` that support arbitrary `Gain`.
`FocusSTM` and `GainSTM` use the timer on the AUTD3 hardware, so the time accuracy is high, but there are many restrictions.

- [FocusSTM](./stm/focus.md)
- [GainSTM](./stm/gain.md)

## FocusSTM/GainSTM common API

### frequency

Get the frequency of STM.

### sampling_frequency

Get the sampling frequency of STM.

### sampling_frequency_division

Get the sampling frequency division of STM.

### start_idx/finish_idx

`FocusSTM`/`GainSTM` are usually not specified which focus/`Gain` to start from.
To specify this, use `with_start_idx` as follows.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let autd = Controller::builder().open_with(autd3::link::Nop::builder()).unwrap();
let stm = FocusSTM::new(1.0).with_start_idx(Some(0));
# Ok(())
# }
```

```cpp
auto stm = autd3::FocusSTM(1).with_start_idx(0);
```

```cs
var stm = new FocusSTM(1).WithStartIdx(0);
```

```python
stm = FocusSTM(1).with_start_idx(0)
```

This allows you to start from the focus/`Gain` specified by the index `start_idx`.

Also, you can specify which focus/`Gain` to end with `finish_idx`.

Note that the focus/`Gain` specified by `finish_idx` is not output at the end.
The STM output a focus/`Gain` one before `finish_idx`, and then the STM ends.

`start_idx` and `finish_idx` are only valid for transitions from the normal `Gain` to `FocusSTM`/`GainSTM` and from `FocusSTM`/`GainSTM` to the normal `Gain`.

To disable these settings, do the following.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() {
let stm = FocusSTM::new(1.0).with_start_idx(None);
# }
```

```cpp
auto stm = autd3::FocusSTM(1).with_start_idx(std::nullopt);
```

```cs
var stm = new FocusSTM(1).WithStartIdx(null);
```

```python
stm = FocusSTM(1).with_start_idx(None)
```
