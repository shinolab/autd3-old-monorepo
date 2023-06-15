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

## For Linux, macOS users

You may need to run with administrator privileges when using SOEM on Linux or macOS.

```shell
sudo python main.py
```
