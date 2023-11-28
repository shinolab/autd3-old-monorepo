# Transform

`Transform` is a feature to apply some post-processing to `Modulation`.

```rust,edition2021
{{#include ../../../codes/Users_Manual/modulation/transform_0.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/modulation/transform_0.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/modulation/transform_0.cs}}
```

```python
{{#include ../../../codes/Users_Manual/modulation/transform_0.py}}
```

`with_transform` takes `Fn(usize, EmitIntensity) -> EmitIntensity` as an argument, where the first argument is the index and the second argument is the original modulation data.
