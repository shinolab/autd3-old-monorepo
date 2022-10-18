# Silencer

AUTD3 has a `Silencer` that suppresses a rapid fluctuation of the transducer drive signal and makes it quiet.

## Theory

For details, please refer to the paper by Suzuki et al.[^suzuki2020]

The following is a summary, 

* Amplitude-modulated ultrasound produces audible sound
* Phase changes cause amplitude fluctuations when the transducers are driven.
    * Thus, audible noise is produced.
* Amplitude fluctuation can be suppressed by changing the phase shift step by step.
    * Therefore, the noise can be reduced.
* The finer the interpolation is, the more noise can be reduced.

## Silencer settings

To configure `Silencer`, send `SilencerConfig`.

```cpp
  autd3::SilencerConfig config;
  autd.send(config);
```

`SilencerConfig` has two settings: `step` and `cycle`.
See below for details, but as a rule of thumb, the smaller the `step` and the larger the `cycle`, the quieter it will be.

## Phase change by Silencer

`Silencer` linearly interpolates phase change: it is almost equivalent to passing the time series data of phase $P$ through a (simple) moving average filter.
However, the difference is that the periodicity of the phase data is taken into account.

For example, consider the case where the ultrasound period $T$ is $T=12$.
That is, $P=0$ corresponds to $0\,\mathrm{rad}$ and $P=12$ corresponds to $2\pi\,\mathrm{rad}$. 
Suppose that the phase changes from $P=2$ to $P=6$ at time $t_s$.
In this case, the phase change by `Silencer` is shown in the following figure.

<figure>
  <img src="https://raw.githubusercontent.com/shinolab/autd3/master/book/src/fig/Users_Manual/silent/phase.svg"/>
<figcaption>Change of phase $P$</figcaption>
</figure>

On the other hand, suppose that the phase changes from $P=2$ to $P=10$ at time $t_s$.
The phase change by `Silencer` at this time is shown in the following figure.
This is because $P=-2$ is closer to $P=2$ than $P=10$.

<figure>
  <img src="https://raw.githubusercontent.com/shinolab/autd3/master/book/src/fig/Users_Manual/silent/phase2.svg"/>
<figcaption>Change of phase $P$ (when phase change is larger than $\pi$)</figcaption>
</figure>

In other words, the `Silencer` calculated next phase from current phase $P$ target value $P_r$ using the following formula:
$$
    P \leftarrow \begin{cases}
        P + \mathrm{sign}(P_r - P) \min (|P_r - P|, \Delta) & \text{if } |P_r - P| \le T/2\\
        P - \mathrm{sign}(P_r - P) \min (|P_r - P|, \Delta) & \text{(otherwise)}\\
    \end{cases},
$$
where $\Delta$ is the amount of update per step (`step` in `SilencerConfig`).
The update frequency can be set by `cycle` of `SilencerConfig`, which is $\SI{163.84}{MHz}/cycle$.

The smaller $\Delta$ is, and the slower the update cycle is, the smoother the phase change becomes, and the noise is suppressed.

<figure>
    <img src="https://raw.githubusercontent.com/shinolab/autd3/master/book/src/fig/Users_Manual/silent/duty.svg"/>
<figcaption>Difference of change by $\Delta$</figcaption>
</figure>

Due to this implementation, there are cases where the behavior is different from that of the moving average filter.
One is the case that the amount of phase change is larger than $\pi$, and the other is the case that the phase changes once more during the process.
An example of a later case is shown below.
Although the moving average filter is correct in terms of fidelity to the original time series, it is difficult to consider the case where the phase change is larger than $\pi$ and to make $\Delta$ variable (that is, to make the filter length variable), so the current implementation is used.

<figure>
  <img src="https://raw.githubusercontent.com/shinolab/autd3/master/book/src/fig/Users_Manual/silent/mean.svg"/>
<figcaption>Comparison with moving average filter</figcaption>
</figure>

## Changes in duty ratio

Since amplitude variation causes noise, filtering the duty ratio $D$ equivalently can suppress noise caused by AM.

Since the duty ratio $D$ is not periodic, unlike the phase, `Silencer` update duty ration by the following equation for the current $D$ and the target $D_r$,
$$
    D \leftarrow D + \mathrm{sign}(D_r - D) \min (|D_r - D|, \Delta).
$$

[^suzuki2020]: Suzuki, Shun, et al. "Reducing amplitude fluctuation by gradual phase shift in midair ultrasound haptics." IEEE transactions on haptics 13.1 (2020): 87-93.
