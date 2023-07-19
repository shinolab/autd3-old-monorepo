# AUTD3について

AUTD3は空中触覚用超音波フェーズドアレイデバイスである.
超音波フェーズドアレイとは, 位相を個別に制御できる超音波振動子を (典型的には格子状に) 配列したものである.
超音波の位相を制御することで, 空間に任意の音場を発生させることができる.

フェーズドアレイを用いて十分に集束させた音波のエネルギーは, 音響放射圧を生じる.
この圧力を利用して, 人体の表面を非接触で押すことができる.
集束焦点の位置は, フェーズドアレイを電子的に制御することで自由に制御できる.
また, 逆問題を解くことで, 単一の焦点だけでなく, より複雑な音圧空間分布を作ることもできる.

フェーズドアレイで生成できる圧力の大きさの上限は, 現在のところ約$\SI{50}{mN/cm^2}\sim \SI{5}{gf/cm^2}$である.
また, 空間分解能は使用する波長程度までとなる (例えば, $\SI{40}{kHz}$で約$\sim\SI{8.5}{mm}$).
フェーズドアレイにはこのような制約はあるものの, 力の時空間分布を自由にデザインし, さまざまな触覚を作り出すことができる技術として注目されている.

このように非接触で触覚を刺激する技術分野を**空中触覚 (Midair Haptics)** と呼び, 我々はこの超音波空中触覚装置を**Airborne Ultrasound Tactile Display (AUTD)** と呼んでいる.
AUTDの本質的な部分は, 2008年[^1]から2010年代初頭[^2]にかけて, 東京大学によって提案・確立された.
その後, 各国の大学や企業が参入し, 活発な研究開発が行われている.
AUTD3は我々東京大学篠田牧野研究室で開発しているAUTDの3代目のバージョンである.

[研究室のホームページ](https://hapislab.org/airborne-ultrasound-tactile-display)にAUTDを使った研究の一覧が掲載されている. こちらも参照されたい.

本マニュアルはこのAUTD3を操作するための[autd3](https://github.com/shinolab/autd3)ソフトウェアライブラリについてまとめたものである.

[^1]: [Takayuki Iwamoto, Mari Tatezono, and Hiroyuki Shinoda: Non-contact Method for Producing Tactile Sensation Using Airborne Ultrasound, Haptics: Perception, Devices and Scenarios: 6th International Conference, Eurohaptics 2008 Proceedings (Lecture Notes in Computer Science), pp.504-513, 2008.](https://hapislab.org/public/hiroyuki_shinoda/research/pdf/08Eurohaptics_iwamoto.pdf)

[^2]: [Takayuki Hoshi, Masafumi Takahashi, Takayuki Iwamoto, and Hiroyuki Shinoda: Noncontact Tactile Display Based on Radiation Pressure of Airborne Ultrasound, IEEE Trans. on Haptics, Vol. 3, No. 3, pp.155-165, 2010.](https://hapislab.org/public/hiroyuki_shinoda/research/pdf/10_Trans_Haptics_Hoshi.pdf)
