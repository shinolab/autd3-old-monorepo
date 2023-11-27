/*
 * File: SamplingConfig.cs
 * Project: tests
 * Created Date: 25/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/11/2023
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
    public void SamplingConfigNewWithFreqDiv()
    {
        var config = SamplingConfiguration.NewWithFrequencyDivision(SamplingFreqDivMin);
        Assert.Equal(512u, config.FrequencyDivision);
        Assert.Equal(40e3, config.Frequency);
        Assert.Equal(TimeSpan.FromMicroseconds(25), config.Period);

        Assert.Throws<AUTDException>(() => _ = SamplingConfiguration.NewWithFrequencyDivision(SamplingFreqDivMin-1));
    }

    [Fact]
    public void SamplingConfigNewWithFreq()
    {
        var config = SamplingConfiguration.NewWithFrequency(40e3);
        Assert.Equal(512u, config.FrequencyDivision);
        Assert.Equal(40e3, config.Frequency);
        Assert.Equal(TimeSpan.FromMicroseconds(25), config.Period);

        Assert.Throws<AUTDException>(() => _ = SamplingConfiguration.NewWithFrequency(FreqMin - 0.1));
        Assert.Throws<AUTDException>(() => _ = SamplingConfiguration.NewWithFrequency(FreqMax + 0.1));
    }

    [Fact]
    public void SamplingConfigNewWithPeriod()
    {
        var config = SamplingConfiguration.NewWithPeriod(TimeSpan.FromMicroseconds(25));
        Assert.Equal(512u, config.FrequencyDivision);
        Assert.Equal(40e3, config.Frequency);
        Assert.Equal(TimeSpan.FromMicroseconds(25), config.Period);

        Assert.Throws<AUTDException>(() => _ = SamplingConfiguration.NewWithPeriod(TimeSpan.FromMicroseconds(PeriodMin/1000.0 - 1.0)));
        Assert.Throws<AUTDException>(() => _ = SamplingConfiguration.NewWithPeriod(TimeSpan.FromMicroseconds(PeriodMax / 1000.0 + 1.0)));
    }
}