# Simulator

Simulator link is a link used when using [AUTD simulator](../../Simulator/simulator.md).

Before using this link, you need to start AUTD simulator.

[[_TOC_]]

## Simulator link API

### Contructor

Simulator link's constructor takes a port number of AUTD simulator.

```rust,should_panic,edition2021
{{#include ../../../codes/Users_Manual/link/simulator_0.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/link/simulator_0.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/link/simulator_0.cs}}
```

```python
{{#include ../../../codes/Users_Manual/link/simulator_0.py}}
```

### AUTD simulator server IP address

You can specify the IP address of the server running AUTD simulator with `with_server_ip`.

```rust,should_panic,edition2021
{{#include ../../../codes/Users_Manual/link/simulator_1.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/link/simulator_1.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/link/simulator_1.cs}}
```

```python
{{#include ../../../codes/Users_Manual/link/simulator_1.py}}
```

The default server IP address is localhost.


### Update `Geometry`

If you update `Geometry`, the geometry in the simulator side will not be updated automatically.
To update `Geometry`, use `update_geometry` function.

```rust,should_panic,edition2021
{{#include ../../../codes/Users_Manual/link/simulator_2.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/link/simulator_2.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/link/simulator_2.cs}}
```

```python
{{#include ../../../codes/Users_Manual/link/simulator_2.py}}
```
