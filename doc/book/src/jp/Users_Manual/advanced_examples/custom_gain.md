# Gainの自作

`Gain`クラスを継承することで独自の`Gain`を作成することができる.
ここでは, `Focus`と同じように単一焦点を生成する`FocalPoint`を実際に定義してみることにする.

```cpp
#include "autd3.hpp"

class FocalPoint final : public autd3::Gain {
 public:
  explicit FocalPoint(autd3::Vector3 point) : _point(std::move(point)) {}

  void calc(const autd3::Geometry& geometry) override {
    std::for_each(geometry.begin(), geometry.end(), [&](const auto& transducer) {
        const auto dist = (_point - transducer.position()).norm();
        const auto phase = transducer.align_phase_at(dist);
        this->_drives[transducer.id()].amp = 1.0;
        this->_drives[transducer.id()].phase = phase;
      });
  } 

 private:
  autd::Vector3 _point;
};
```

`Controller::send`関数は`Gain`型を継承したクラスを引数に取る.
そのため, `Gain`型を継承をしておく.

`Controller::send`関数内部では`Geometry`を引数にした`Gain::calc`メソッドが呼ばれる.
そのため, この`calc`メソッド内で位相/振幅の計算を行えば良い.
Geometryにはイテレータが定義されており, `Transducer`のイテレータが返され, ここから振動子の位置を取得できる.
ある点$\bp$で多数の振動子からの放出された超音波の音圧が最大になるためには, $\bp$での位相が揃えば良い.
これは, `Transducer`クラスに用意されている`align_phase_at`関数で計算できる.
