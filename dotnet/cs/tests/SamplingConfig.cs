/*
 * File: SamplingConfig.cs
 * Project: tests
 * Created Date: 25/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */


namespace tests;

public class SamplingConfigTest
{
    private const uint SamplingFreqDivMin = 512;
    private const uint SamplingFreqDivMax = 0xFFFFFFFF;
    private const double FreqMin = (double)AUTD3.FPGAClkFreq / SamplingFreqDivMax;
    private const double FreqMax = (double)AUTD3.FPGAClkFreq / SamplingFreqDivMin;
    private const ulong PeriodMin = (ulong)(1000000000.0 / AUTD3.FPGAClkFreq * SamplingFreqDivMin);
    private const ulong PeriodMax = (ulong)(1000000000.0 / AUTD3.FPGAClkFreq * SamplingFreqDivMax);

    [Fact]
    public void SamplingConfigFromFreqDiv()
    {
        var config = SamplingConfiguration.FromFrequencyDivision(SamplingFreqDivMin);
        Assert.Equal(512u, config.FrequencyDivision);
        Assert.Equal(40e3, config.Frequency);
        Assert.Equal(TimeSpan.FromMicroseconds(25), config.Period);

        Assert.Throws<AUTDException>(() => _ = SamplingConfiguration.FromFrequencyDivision(SamplingFreqDivMin - 1));
    }

    [Fact]
    public void SamplingConfigFromFreq()
    {
        var config = SamplingConfiguration.FromFrequency(40e3);
        Assert.Equal(512u, config.FrequencyDivision);
        Assert.Equal(40e3, config.Frequency);
        Assert.Equal(TimeSpan.FromMicroseconds(25), config.Period);

        Assert.Throws<AUTDException>(() => _ = SamplingConfiguration.FromFrequency(FreqMin - 0.1));
        Assert.Throws<AUTDException>(() => _ = SamplingConfiguration.FromFrequency(FreqMax + 0.1));
    }

    [Fact]
    public void SamplingConfigFromPeriod()
    {
        var config = SamplingConfiguration.FromPeriod(TimeSpan.FromMicroseconds(25));
        Assert.Equal(512u, config.FrequencyDivision);
        Assert.Equal(40e3, config.Frequency);
        Assert.Equal(TimeSpan.FromMicroseconds(25), config.Period);

        Assert.Throws<AUTDException>(() => _ = SamplingConfiguration.FromPeriod(TimeSpan.FromMicroseconds(PeriodMin / 1000.0 - 1.0)));
        Assert.Throws<AUTDException>(() => _ = SamplingConfiguration.FromPeriod(TimeSpan.FromMicroseconds(PeriodMax / 1000.0 + 1.0)));
    }
}