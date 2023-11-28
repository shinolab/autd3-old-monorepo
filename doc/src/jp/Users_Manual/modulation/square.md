# Square

矩形波状の`Modulation`.

コンストラクタには周波数$f$を整数で指定する.

```rust,edition2021
{{#include ../../../codes/Users_Manual/modulation/square_0.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/modulation/square_0.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/modulation/square_0.cs}}
```

```python
{{#include ../../../codes/Users_Manual/modulation/square_0.py}}
```

## 振幅の指定

Low/Highレベルの振幅はそれぞれ, `with_low`, `with_high`で指定できる.

```rust,edition2021
{{#include ../../../codes/Users_Manual/modulation/square_1.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/modulation/square_1.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/modulation/square_1.cs}}
```

```python
{{#include ../../../codes/Users_Manual/modulation/square_1.py}}
```

## Duty比の指定

`with_duty`で矩形波のDuty比を指定できる.

```rust,edition2021
{{#include ../../../codes/Users_Manual/modulation/square_2.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/modulation/square_2.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/modulation/square_2.cs}}
```

```python
{{#include ../../../codes/Users_Manual/modulation/square_2.py}}
```