# FocusSTM

- The maximum number of sampling points is $65536$.
- The sampling frequency is $\clkf/N$.

THe following is an example of using `FocusSTM` to focus on a point $\SI{150}{mm}$ directly above the center of the array with a radius of $\SI{30}{mm}$ centered on the center of the array.

```rust,edition2021
{{#include ../../../codes/Users_Manual/stm/focus_0.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/stm/focus_0.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/stm/focus_0.cs}}
```

```python
{{#include ../../../codes/Users_Manual/stm/focus_0.py}}
```

`FocusSTM`'s constructor takes the STM frequency as an argument.
Note that the specified frequency and the actual frequency may differ due to constraints on the number of sampling points and the sampling period.
For example, the above example runs 200 points at $\SI{1}{Hz}$, so the sampling frequency should be $\SI{200}{Hz}=\clkf/102400$.
However, if `point_num=199`, the sampling frequency must be $\SI{199}{Hz}$, but there is no integer $N$ that satisfies $\SI{199}{Hz}=\clkf/N$.
Therefore, the closest $N$ is selected.
As a result, the specified frequency and the actual frequency are shifted.
`frequency` can be used to check the actual frequency.

## Specify the sampling configuration

You can specify the sampling frequency by `from_sampling_config` instead of frequency.

```rust,edition2021
{{#include ../../../codes/Users_Manual/stm/focus_1.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/stm/focus_1.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/stm/focus_1.cs}}
```

```python
{{#include ../../../codes/Users_Manual/stm/focus_1.py}}
```
