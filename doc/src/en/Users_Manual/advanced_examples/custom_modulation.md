# Custom Modulation Tutorial

You can create your own `Modulation` as well as `Gain`.
Here, we try to create a `Burst` that outputs only for a certain moment in a cycle.

The following is a sample of `Burst`.
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

Like `Gain`, `Modulation::calc` method is called inside `Controller::send`.
In this `calc`, you have to calculate and return modulation data.

The argument of `Modualation` constructor is the sampling frequency divisionv $N$, which determines the `Modulation` sampling frequency $\SI{163.84}{MHz}/N$.
In this example, the sampling frequency is $\SI{4}{kHz}$ ($N=40960$) by default.

And, for example, if `buf_size` is set to 4000, $0$ is sampled $3999$ times, and then $1$ is sampled once.
Thus, AM is such that $\SI{0.25}{ms}=1/\SI{4}{kHz}$ is output in the period $\SI{1}{s}$.
