# Group

`Group` is a `Gain` to use different `Gain` for each transducer.

```rust,edition2021
{{#include ../../../codes/Users_Manual/gain/group_0.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/gain/group_0.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/gain/group_0.cs}}
```

```python
{{#include ../../../codes/Users_Manual/gain/group_0.py}}
```

In the above case, transducers whose local indices are less or equal than 100 produce `Null`, and the others produce `Focus`.

> NOTE:
> In this sample, we use string as a key, but you can use any type that can be used as a key of HashMap.
