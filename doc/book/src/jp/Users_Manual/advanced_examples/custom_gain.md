# Gainの自作

`Gain`クラスを継承することで独自の`Gain`を作成することができる.
ここでは, `Focus`と同じように単一焦点を生成する`FocalPoint`を実際に定義してみることにする.

```cpp
#include "autd3.hpp"

class FocalPoint final : public autd3::Gain {
 public:
  explicit FocalPoint(autd3::Vector3 point) : _point(std::move(point)) {}

  std::vector<autd3::driver::Drive> calc(const autd3::Geometry& geometry) override {
    std::vector<autd3::driver::Drive> drives;
    drives.reserve(geometry.num_transducers());
    std::transform(geometry.begin(), geometry.end(), std::back_inserter(drives), [&](const auto& transducer) { {
        const auto dist = (_point - transducer.position()).norm();
        const auto phase = transducer.align_phase_at(dist);
        return driver::Drive{autd3::Phase(phase), autd3::Amp(1.0)};
      });
    return drives;
  } 

 private:
  autd::Vector3 _point;
};
```

`Controller::send`関数は`Gain`型を継承したクラスを引数に取る.
そのため, `Gain`型を継承をしておく.

`Controller::send`関数内部では`Geometry`を引数にした`Gain::calc`メソッドが呼ばれ, その返り値の振幅/位相データが使用される.
そのため, この`calc`メソッド内で位相/振幅の計算を行えば良い.
Geometryにはイテレータが定義されており, `Transducer`のイテレータが返され, ここから振動子の位置を取得できる.
ある点$\bp$で多数の振動子からの放出された超音波の音圧が最大になるためには, $\bp$での位相が揃えば良い.
これは, `Transducer`クラスに用意されている`align_phase_at`関数で計算できる.
