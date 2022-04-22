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
    plt.rcParams['text.latex.preamble'] = r'\usepackage{sfmath}'
    plt.style.use('navy')


V = 15
T = 25
DPI = 300
ext = '.png'


def signal():
    P = T / 2
    D = T / 2 * 0.7
    t = np.linspace(0, 30, 10000)
    y = np.array([V / 2 if (T - P - D / 2) < x <= T - P + D / 2 else -V / 2 for x in t])
    y2 = np.ones(len(y)) * (V * (D / T - 1 / 2))
    N = 1
    for n in range(1, 1 + N):
        y2 += 2 * V / (np.pi * n) * np.sin(np.pi * n * D / T) * np.cos(-2 * np.pi * n * (t + P) / T)

    fig = plt.figure(figsize=(6, 4), dpi=DPI)
    ax = fig.add_subplot(111)
    ax.set_xlim((0, 30))
    ax.set_ylim((-10, 10))

    ax.set_xticks([0, T - P - D / 2, P, T - P + D / 2, T])
    ax.set_xticklabels(['$0$', '$T-P-\\frac{D}{2}$', '$T-P$', '$T-P+\\frac{D}{2}$', '$T$'])

    ax.set_yticks([-V / 2, 0, V / 2])
    ax.set_yticklabels(['$-\\frac{V}{2}$', '$0$', '$\\frac{V}{2}$'])

    ax.spines['top'].set_visible(False)
    ax.spines['right'].set_visible(False)

    ax.set_xlabel('Time')
    ax.set_ylabel('Voltage')

    ax.plot(t, y)
    ax.plot(t, y2)

    plt.tight_layout()
    plt.savefig('signal' + ext)


def pwm():
    S = -3.5
    D = 12.5
    x = np.linspace(S - 5.5, 30, 10000)
    y = np.array([V if S < x <= S + D or S + T < x <= S + D + T else 0 for x in x])

    fig = plt.figure(figsize=(6, 4), dpi=DPI)
    ax = fig.add_subplot(111)
    ax.set_xlim((S - 5.5, 30))
    ax.set_ylim((0, 20))

    ax.set_xticks([S, 0, S + D / 2, S + D, S + T, T])
    ax.set_xticklabels(['$T-P-\\frac{D}{2}$', '$0$', '$T-P$', '$T-P+\\frac{D}{2}$', '$2T-P-\\frac{D}{2}$', '$T$'])

    ax.set_yticks([0, V])
    ax.set_yticklabels(['', ''])

    ax.spines['top'].set_visible(False)
    ax.spines['right'].set_visible(False)
    ax.spines['left'].set_position(('data', 0))

    ax.set_xlabel('Time')
    ax.set_ylabel('')

    ax.plot(x, y)

    plt.tight_layout()
    plt.savefig('left' + ext)

    ####
    D = 12.5
    S = T - D + 3.5
    x = np.linspace(0, 30 + 5.5, 10000)
    y = np.array([V if S - T < x <= S + D - T or S < x <= S + D else 0 for x in x])

    fig = plt.figure(figsize=(6, 4), dpi=DPI)
    ax = fig.add_subplot(111)
    ax.set_xlim((0, 30 + 5.5))
    ax.set_ylim((0, 20))

    ax.set_xticks([0, S + D - T, S, S + D / 2, T, S + D])
    ax.set_xticklabels(['$0$', '$P+\\frac{D}{2}$', '$T-P-\\frac{D}{2}$', '$T-P$', '$T$', '$T-P+\\frac{D}{2}$'])

    ax.set_yticks([0, V])
    ax.set_yticklabels(['', ''])

    ax.spines['top'].set_visible(False)
    ax.spines['right'].set_visible(False)
    ax.spines['left'].set_position(('data', 0))

    ax.set_xlabel('Time')
    ax.set_ylabel('')

    ax.plot(x, y)

    plt.tight_layout()
    plt.savefig('right' + ext)


if __name__ == '__main__':
    setup_pyplot()

    signal()
    pwm()
