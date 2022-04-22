# Implementation

FPGA内のブロック図は以下の図のようになっている.

<figure>
<img alt="AUTD3 block diagram" src="../figs/overview.svg">
<figcaption>AUTD3 block diagram</figcaption>
</figure>

Normal/STM Operatorから出力されたDuty比/位相データがMuxによって選択され, ModulatorによりDuty比に変調がかけられ, Silencerで静音化処理がかけられた後, PWMモジュールによって振動子を駆動するPWM信号が生成される.

以上のモジュールはSynchronizerから生成された同期時刻を参照し, タイミングを制御する.

また, WDTモジュールはEtherCATの動作を監視し, 動作が止まった際にPWMの出力を止める.

以上すべての設定や制御をControllerモジュールが担当する.
