# Deviceの自作

SDKはAUTD3以外のデバイスもサポートしている.

> NOTE: 実際にはAUTD3以外のデバイスは存在しないため, 以下は現在`Simulator`においてのみサポートされている.


Deviceを自作する場合は, `autd3::Device`を継承し, `get_transducers`関数をオーバライドして, その関数から, デバイスに存在する振動子の`vector`を返せば良い.

以下に, 同心円状のアレイデバイスのサンプルを載せる.

```cpp
class ConcentricArray final : autd3::core::Device {
public:
	ConcentricArray() = default;

	[[nodiscard]] std::vector<autd3::core::Transducer> get_transducers(const size_t start_id) const override
	{
		std::vector<autd3::core::Transducer> transducers;
		size_t id = start_id;
		transducers.emplace_back(autd3::core::Transducer(id++, autd3::Vector3::Zero(), autd3::Quaternion::Identity()));
		for (size_t layer = 1; layer <= 5; layer++)
		{
			for (size_t i = 0; i < 6 * layer; i++) {
				const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(6 * layer);
				const autd3::Vector3 pos = static_cast<double>(layer) * 10.0 * autd3::Vector3(std::cos(theta), std::sin(theta), 0);
				transducers.emplace_back(autd3::core::Transducer(id++, pos, autd3::Quaternion::Identity()));
			}
		}
		return transducers;
	}
};
```

このデバイスを`add_device`すると, 以下のようになる.
ここでは, 焦点を生成している.

<figure>
  <img src="../../fig/Users_Manual/custom_device.jpg"/>
  <figcaption>ConcentricArray</figcaption>
</figure>


## 制約

- デバイスあたりの振動子の上限は256個となっている. これ以上の振動子を使用する場合は, 複数のデバイスに分割されたい.
