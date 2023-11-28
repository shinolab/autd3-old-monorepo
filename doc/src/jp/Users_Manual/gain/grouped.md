# Group

`Group`は振動子ごとに別々の`Gain`を使用するための`Gain`である.

`Group`では, デバイスと振動子に対してキーを割り当て, その各キーに`Gain`を紐付けて使用する.

```rust,edition2021
{{#include ../../../codes/Users_Manual/gain/group_0.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/gain/group_0.cpp}}
```
  - C++の場合, キーには`std::optional`を使用する必要がある.

```cs
{{#include ../../../codes/Users_Manual/gain/group_0.cs}}
```

```python
{{#include ../../../codes/Users_Manual/gain/group_0.py}}
```

上の場合は, ローカルインデックスが$0$から$100$の振動子は`Null`を, それ以外の振動子は`Focus`を出力する.

> NOTE:
> このサンプルでは, キーとして文字列を使用しているが, HashMapのキーとして使用できるものなら何でも良い.
