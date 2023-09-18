# Python tutorial

First, install `pyautd3` library.

```shell
pip install pyautd3
```

Next, make `main.py` file as follows.
This is the source code for generating a focus with $\SI{150}{Hz}$ AM modulation. 

```python,filename=main.py
{{#include ../../../../samples/python/main.py}}
```

Then, run the program.

```shell
python main.py
```

## For linux users

You may need to run with administrator privileges when using SOEM on Linux.

```shell
sudo setcap cap_net_raw,cap_net_admin=eip <your python path>
python main.py
```

## For macOS users

You may need to run with administrator privileges when using SOEM on macOS.

```shell
sudo chmod +r /dev/bpf*
python main.py
```
