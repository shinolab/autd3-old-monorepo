# Modulationの自作

`Modulation`も独自のものを作成することができる.
ここでは, 周期中のある一瞬だけ出力する`Burst`を作ってみる[^fn_burst].

以下が, この`Burst`のサンプルである.
```cpp
class Burst final : public autd3::Modulation {
 public:
  std::vector<autd3::Amp> calc() override {
    std::vector buffer(_buf_size, autd3::Amp(0));
    buffer()[_buf_size - 1] = autd3::Amp(1);
  }

  explicit Burst(const size_t buf_size = 4000, const uint16_t freq_div = 40960) noexcept : Modulation(freq_div), _buf_size(buf_size) {}

 private:
  size_t _buf_size;
};
```

`Modulation`も`Gain`と同じく, `Controller::send`内部で`Modulation::calc`メソッドが呼ばれ, その返り値の変調データが使用される.
したがって, この`calc`の中で, 変調データを計算すれば良い.
`Modulation`サンプリング周波数$\SI{163.84}{MHz}/N$を決定する$N$は, `Modulation`のコンストラクタで設定する.
この例だと, デフォルトで$N=40960$なので, サンプリング周波数は$\SI{4}{kHz}$になる.
さらに, 例えば, `buf_size`を4000とすると, AMは$0$が$3999$回サンプリングされた後, $1$が$1$回サンプリングされる.
したがって, 周期$\SI{1}{s}$の中で, $\SI{0.25}{ms}=1/\SI{4}{kHz}$だけ出力されるようなAMがかかる.

[^fn_burst]: SDKにはない.
