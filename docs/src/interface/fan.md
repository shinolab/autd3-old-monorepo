# FAN control

## Jumper switch

AUTD3のファン制御はAuto, Off, Onの3つのモードが有る.
Autoモードでは温度監視ICがICの温度を監視し, 一定温度以上になった際に自動的にファンを起動する.
また, AutoモードではFORCE_FAN出力をアサートすることによってもファンを起動できる.
Offモードではファンは常時オフであり, Onモードでは常時オンになる.

モードの切替は, ファン横のジャンパスイッチで行う.
少しわかりにくいが, 下図のようにファン側をショートするとAuto, 真ん中でOff, 右側でOnとなる.

<figure>
<img alt="Fan jumper switch" src="../figs/interface/fan.jpg">
<figcaption>Fan jumper switch</figcaption>
</figure>
