# Custom Modulation Tutorial

You can create your own `Modulation` as well as `Gain`.
Here, we try to create a `Burst` that outputs only for a certain moment in a cycle.

The following is a sample of `Burst`.

```rust
use autd3::{
    core::{
        error::AUTDInternalError,
        modulation::Modulation,
    },
    prelude::*,
    traits::Modulation,
};

#[derive(Modulation, Clone, Copy)]
pub struct Burst {
    freq_div: u32,
}

impl Burst {
    pub fn new() -> Self {
        Self { freq_div: 5120 }
    }
}

impl Modulation for Burst {
    fn calc(&mut self) -> Result<Vec<float>, AUTDInternalError> {
        Ok((0..4000)
            .map(|i| if i == 3999 { 1.0 } else { 0.0 })
            .collect())
    }
}

# fn main() { 
# }
#
```

```cpp
class BurstModulation final : public autd3::Modulation {
public:
    std::vector<double> calc() const override {
        std::vector<double> buffer(_buf_size, 0);
        buffer[_buf_size - 1] = 1.0;
        return buffer;
    }

    explicit BurstModulation(const size_t buf_size = 4000, const uint32_t sampling_freq_div = 5120) noexcept
        : autd3::Modulation(sampling_freq_div), _buf_size(buf_size) {}

    explicit BurstModulation(const size_t buf_size = 4000, const double sampling_freq = 4e3) noexcept
        : autd3::Modulation(sampling_freq), _buf_size(buf_size) {}

private:
    size_t _buf_size;
};
```

```cs
public class Burst : Modulation
{
    private readonly int _length;

    public Burst(int length, uint sampleFreqDiv = 5120) : base(sampleFreqDiv)
    {
        _length = length;
    }
    public Burst(int length, double sampleFreq = 4e3) : base(sampleFreq)
    {
        _length = length;
    }

    public override double[] Calc()
    {
        var buf = new double[_length];
        buf[0] = 1;
        return buf;
    }
}
```

```python
from pyautd3.modulation import Modulation

class Burst(Modulation):
    _length: int

    def __init__(self, length: int, freq_div = 5120):
        super().__init__(freq_div)
        self._length = length

    def calc(self):
        buf = np.zeros(self._length)
        buf[0] = 1.0
        return buf
```
