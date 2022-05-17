# Modulation

`Modulation`はAM変調を制御するための仕組みである.
`Modulation`は, バッファに貯められた$\SI{8}{bit}$データから, 一定のサンプリングレートでデータを順番にサンプリングし, Duty比に掛け合わすことで実現されている.
現在, `Modulation`には以下の制約がある.

* バッファサイズは最大で65536
* サンプリングレートは$\SI{163.84}{MHz}/N$で, $N$は32-bit符号なし整数であり, $2320$以上の値である必要がある.
* Modulationは全デバイスで共通
* Modulationは自動でループする. 1ループだけ, 等の制御は不可能.

SDKにはデフォルトでいくつかの種類のAMを生成するための`Modulation`がデフォルトで用意されている.

## Static

変調なし.

```cpp
  modulation::Static m;
```

なお, 第1引数は0-1の規格化された振幅を引数に取れ, 超音波の出力を一律で変更するために使うことができる.

## Sine

音圧をSin波状に変形するための`Modulation`.
```cpp
  autd::modulation::Sine m(f, amplitude, offset); 
```

第1引数は周波数$f$, 第2引数は$amplitude$ (デフォルトで1), 第3引数は$offset$ (デフォルトで0.5)になっており, 音圧の波形が
$$
    \frac{amplitude}{2} \times \sin(ft) + offset
$$
となるようなAMをかける.
ただし, 上記で$\[0,1\]$を超えるような値は$\[0,1\]$に収まるように変換される.
また, サンプリング周波数はデフォルトで$\SI{4}{kHz}$ ($N=40960$) になっている.


## SineSquared

放射圧, すなわち, 音圧の二乗をSin波状に変形するための`Modulation`.
引数等は`Sine`と同じ.

## SineLegacy

古いversionにあった`Sine Modulation`と互換.
周波数として, `double`の値を取れるが, 厳密に指定周波数になるのではなく, 出力可能な周波数の内, 最も近い周波数が選ばれる.
また, 音圧ではなくDuty比がSin波状になる.

## Square

矩形波状の`Modulation`.

```cpp
  modulation::Square m(f, low, high); 
```
第1引数は周波数$f$, 第2引数はlow (デフォルトで0), 第3引数はhigh (デフォルトで1)になっており, 音圧の波形はlowとhighが周波数$f$で繰り返される.
また, 第4引数にduty比を指定できる.
duty比は$t_\text{high}/T = t_\text{high}f$で定義される, ここで, $t_\text{high}$は1周期$T=1/f$の内, highを出力する時間である.


## Wav

`Wav`はWavファイルをもとに構成される`Modulation`である.

```
  const filesystem::path path = filesystem::path(string("sin150.wav"));
  modulation::Wav m(path.string());
```

`Wav`を使用するには`BUILD_MODULATION_AUDIO_FILE` optionをONにしてコンパイルする必要がある.

## RawPCM

`RawPCM`は符号なし8-bitのバイナリデータファイルをもとに構成される`Modulation`である.

```
  const filesystem::path path = filesystem::path(string("sin150.wav"));
  modulation::RawPCM m(path.string(), 4e3);
```

`RawPCM`を使用するには`BUILD_MODULATION_AUDIO_FILE` optionをONにしてコンパイルする必要がある.

## Create Custom Modulation Tutorial

`Modulation`も独自の`Modulation`を作成することができる.
ここでは, 周期中のある一瞬だけ出力する`Burst`を作ってみる[^fn_burst].

以下が, この`Burst`のサンプルである.
```cpp
class Burst final : public Modulation {
 public:
  void calc() override {
    this->_buffer.resize(_buf_size, 0);
    this->_buffer[_buf_size - 1] = 0xFF;
  }

  explicit Burst(const size_t buf_size = 4000, const uint16_t freq_div = 40960) noexcept : _buf_size(buf_size) 
  {
    _props.freq_div = freq_div;
  }

 private:
  size_t _buf_size;
};
```

`Modulation`も`Gain`と同じく, `Controller::send`内部で`Modulation::calc`メソッドが呼ばれる.
この`calc`の中で, `buffer`の中身を書き換えれば良い.
`Modulation`サンプリング周波数$\SI{163.84}{MHz}/N$を決定する$N$は`_props.freq_div`に設定する.
この例だと, デフォルトで$N=40960$なので, サンプリング周波数は$\SI{4}{kHz}$になる.
さらに, 例えば, `buf_size`を4000とすると, AMは$0$が$3999$回サンプリングされた後, $255$が一回サンプリングされる.
したがって, 周期$\SI{1}{s}$の中で, $\SI{0.25}{ms}=1/\SI{4}{kHz}$だけ出力されるようなAMがかかる.

## Modulation common functions

### Sampling frequency division ratio

`sampling_freq_div_ratio`でサンプリング周波数の分周比$N$の確認, 設定ができる.
サンプリング周波数の基本周波数は$\SI{40}{kHz}$である.
`sampling_freq_div_ratio`は1以上65536以下の整数が指定できる.

```cpp
    m.sampling_frequency_division() = 20480; // 163.84MHz/20480 = 8kHz
```

### Sampling frequency

`sampling_frequency`でサンプリング周波数を取得できる.

[^fn_burst]: SDKにはない.
