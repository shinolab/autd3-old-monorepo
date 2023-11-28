# Transform

You can use `Transform` to apply some post-processing to `Gain`.

```rust,edition2021
{{#include ../../../codes/Users_Manual/gain/transform_0.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/gain/transform_0.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/gain/transform_0.cs}}
```

```python
{{#include ../../../codes/Users_Manual/gain/transform_0.py}}
```

The argument of `with_transform` is `Fn(&Device, &Transducer, &Drive) -> Drive`.
