'''
File: pwm.py
Project: pwm
Created Date: 16/03/2022
Author: Shun Suzuki
-----
Last Modified: 18/03/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Hapis Lab. All rights reserved.

'''


from dataclasses import dataclass
import numpy as np
import matplotlib.pyplot as plt


def setup_pyplot():
    plt.rcParams['text.usetex'] = True
    plt.rcParams['axes.grid'] = False
    plt.rcParams['xtick.direction'] = 'in'
    plt.rcParams['ytick.direction'] = 'in'
    plt.rcParams['xtick.major.width'] = 1.0
    plt.rcParams['ytick.major.width'] = 1.0
    plt.rcParams['font.size'] = 12
    plt.rcParams['font.family'] = 'sans-serif'
    plt.rcParams['font.sans-serif'] = 'Arial'
    plt.rcParams["mathtext.fontset"] = 'stixsans'
    plt.rcParams['ps.useafm'] = True
    plt.rcParams['pdf.use14corefonts'] = True
    plt.rcParams['text.latex.preamble'] = r'\usepackage{sfmath}\usepackage{amsmath}'
    plt.style.use('navy')


s = 2
t = 10
cycle = 12
DPI = 300
ext = '.svg'


def impl_duty(raw, start, delta):
    f = np.zeros(len(raw), dtype=raw.dtype)
    current = start
    f[0] = current
    for i, r in enumerate(raw[:-1]):
        current += np.sign(r - current) * min(abs(r - current), delta)
        f[i + 1] = current
    return f


def impl_phase(raw, start, delta, cycle):
    f = np.ones(len(raw), dtype=raw.dtype) * start
    current = start
    for i, r in enumerate(raw[:-1]):
        if abs(r - current) < cycle / 2:
            current += np.sign(r - current) * min(abs(r - current), delta)
        else:
            current -= np.sign(r - current) * min(abs(r - current), delta)
        current = (current + cycle) % cycle
        f[i + 1] = current
    return f


def mean_filter(raw, start, N):
    f = np.ones(len(raw), dtype=raw.dtype) * start
    for i in range(len(raw) - 1):
        s = i + 1 - N if 0 <= i + 1 - N else 0
        e = i + 1
        f[i + 1] = np.mean(raw[s:e])

    return f


def duty():
    deltas = [1, 2, 3, 8]

    T = 20
    Ts = 6
    x = np.linspace(0, T - 1, T, dtype=int)

    fig = plt.figure(figsize=(6, 4), dpi=DPI)
    ax = fig.add_subplot(111)
    ax.set_xlim((0, T - 1))
    ax.set_ylim((0, cycle))

    yr = np.concatenate([np.ones(Ts, dtype=int) * s, np.ones(T - Ts, dtype=int) * t])
    ax.plot(x, yr, marker='.', label='Raw')

    for delta in deltas:
        y = impl_duty(yr, s, delta)
        ax.plot(x, y, marker='.', label=fr'$\Delta ={delta}$' if delta < (t - s) else rf'$\Delta \ge {t-s}$')

    ax.set_xticks([0, Ts])
    ax.set_xticklabels(['0', '$t_s$'])

    ax.set_yticks([s, t])
    ax.set_yticklabels([f'{s}', f'{t}'])

    # delete right up frame
    ax.spines['top'].set_visible(False)
    ax.spines['right'].set_visible(False)
    ax.get_xaxis().tick_bottom()
    ax.get_yaxis().tick_left()

    ax.legend(frameon=False)

    plt.tight_layout()
    plt.savefig('duty' + ext)


def phase():
    deltas = [1]

    T = 20
    Ts = 5
    x = np.linspace(0, T - 1, T, dtype=int)

    fig = plt.figure(figsize=(6, 4), dpi=DPI)
    ax = fig.add_subplot(111)
    ax.set_xlim((0, T - 1))
    ax.set_ylim((0, cycle))

    t1 = 6

    yr = np.concatenate([np.ones(Ts, dtype=int) * s, np.ones(T - Ts, dtype=int) * t1])
    ax.plot(x, yr, marker='.', label='Raw')

    for delta in deltas:
        y = impl_phase(yr, s, delta, cycle)
        ax.plot(x, y, marker='.', label='Filtered')

    ax.set_xticks([0, Ts])
    ax.set_xticklabels(['0', '$t_s$'])

    ax.set_yticks([0, s, t1, cycle])
    ax.set_yticklabels(['0', f'{s}', f'{t1}', f'{cycle}'])
    ax.set_ylabel('$P$')

    # delete right up frame
    ax.spines['top'].set_visible(False)
    ax.spines['right'].set_visible(False)
    ax.get_xaxis().tick_bottom()
    ax.get_yaxis().tick_left()

    ax.legend(frameon=False)

    plt.tight_layout()
    plt.savefig('phase' + ext)

    # over
    fig = plt.figure(figsize=(6, 4), dpi=DPI)
    ax = fig.add_subplot(111)
    ax.set_xlim((0, T - 1))
    ax.set_ylim((0, cycle))

    yr = np.concatenate([np.ones(Ts, dtype=int) * s, np.ones(T - Ts, dtype=int) * t])
    ax.plot(x, yr, marker='.', label='Raw')

    for delta in deltas:
        y = impl_phase(yr, s, delta, cycle)
        ax.plot(x, y, marker='.', label='Filtered')

    ax.set_xticks([0, Ts])
    ax.set_xticklabels(['0', '$t_s$'])

    ax.set_yticks([0, s, t, cycle])
    ax.set_yticklabels(['0', f'{s}', f'{t}', f'{cycle}'])
    ax.set_ylabel('$P$')

    # delete right up frame
    ax.spines['top'].set_visible(False)
    ax.spines['right'].set_visible(False)
    ax.get_xaxis().tick_bottom()
    ax.get_yaxis().tick_left()

    ax.legend(frameon=False)

    plt.tight_layout()
    plt.savefig('phase2' + ext)


def mean():
    deltas = [2]

    T = 20
    Ts = 6
    Ts2 = 9
    x = np.linspace(0, T - 1, T, dtype=int)

    fig = plt.figure(figsize=(6, 4), dpi=DPI)
    ax = fig.add_subplot(111)
    ax.set_xlim((0, T - 1))
    ax.set_ylim((0, 20))

    t = 15
    s = 5
    t2 = 11

    yr = np.concatenate([np.ones(Ts, dtype=int) * s, np.ones(Ts2 - Ts, dtype=int) * t, np.ones(T - Ts2, dtype=int) * t2])
    ax.plot(x, yr, marker='.', label='Raw')

    for delta in deltas:
        y = impl_duty(yr, s, delta)
        ym = mean_filter(yr, s, 5)
        ax.plot(x, y, marker='.', label='Filtered')
        ax.plot(x, ym, marker='.', label='Moving Average')

    ax.set_xticks([0, Ts])
    ax.set_xticklabels(['0', '$t_s$'])

    ax.set_yticks([s, t2, t])
    ax.set_yticklabels([f'{s}', f'{t2}', f'{t}'])

    # delete right up frame
    ax.spines['top'].set_visible(False)
    ax.spines['right'].set_visible(False)
    ax.get_xaxis().tick_bottom()
    ax.get_yaxis().tick_left()

    ax.legend(frameon=False)

    plt.tight_layout()
    plt.savefig('mean' + ext)

    # over
    deltas = [1]

    fig = plt.figure(figsize=(6, 4), dpi=DPI)
    ax = fig.add_subplot(111)
    ax.set_xlim((0, T - 1))
    ax.set_ylim((0, cycle))

    s = 2
    t = 10

    yr = np.concatenate([np.ones(Ts, dtype=int) * s, np.ones(T - Ts, dtype=int) * t])
    ax.plot(x, yr, marker='.', label='Raw')

    for delta in deltas:
        y = impl_phase(yr, s, delta, cycle)
        ax.plot(x, y, marker='.', label='Filtered')
        ym = mean_filter(yr, s, 4)
        ax.plot(x, ym, marker='.', label='Moving Average')

    ax.set_xticks([0, Ts])
    ax.set_xticklabels(['0', '$t_s$'])

    ax.set_yticks([0, s, t, cycle])
    ax.set_yticklabels(['0', f'{s}', f'{t}', f'{cycle}'])
    ax.set_ylabel('$P$')

    # delete right up frame
    ax.spines['top'].set_visible(False)
    ax.spines['right'].set_visible(False)
    ax.get_xaxis().tick_bottom()
    ax.get_yaxis().tick_left()

    ax.legend(frameon=False)

    plt.tight_layout()
    plt.savefig('mean2' + ext)


if __name__ == '__main__':
    setup_pyplot()

    duty()
    phase()
    mean()
