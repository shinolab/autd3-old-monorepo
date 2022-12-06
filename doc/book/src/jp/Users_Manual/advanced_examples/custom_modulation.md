# Modulationの自作

`Modulation`も独自のものを作成することができる.
ここでは, 周期中のある一瞬だけ出力する`Burst`を作ってみる[^fn_burst].

以下が, この`Burst`のサンプルである.
```cpp
class Burst final : public autd3::Modulation {
 public:
  void calc() override {
    this->_buffer.resize(_buf_size, 0);
    this->_buffer[_buf_size - 1] = 0xFF;
  }

  explicit Burst(const size_t buf_size = 4000, const uint16_t freq_div = 40960) noexcept : _buf_size(buf_size) 
  {
    _freq_div = freq_div;
  }

 private:
  size_t _buf_size;
};
```

`Modulation`も`Gain`と同じく, `Controller::send`内部で`Modulation::calc`メソッドが呼ばれる.
この`calc`の中で, `buffer`の中身を書き換えれば良い.
`Modulation`サンプリング周波数$\SI{163.84}{MHz}/N$を決定する$N$は`_freq_div`に設定する.
この例だと, デフォルトで$N=40960$なので, サンプリング周波数は$\SI{4}{kHz}$になる.
さらに, 例えば, `buf_size`を4000とすると, AMは$0$が$3999$回サンプリングされた後, $255$が$1$回サンプリングされる.
したがって, 周期$\SI{1}{s}$の中で, $\SI{0.25}{ms}=1/\SI{4}{kHz}$だけ出力されるようなAMがかかる.

[^fn_burst]: SDKにはない.
