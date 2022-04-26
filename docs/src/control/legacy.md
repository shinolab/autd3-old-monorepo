# Legacy mode

Legacyモードは旧ver1.x系と同様に$\SI{8}{bit}$で位相とDuty比を指定するモードである.

Legacyモードでは, $\SI{8}{bit}$で位相$P$とDuty比$D$はそれぞれ,
$$
 P \leftarrow P \ll 5\\
 D \leftarrow (D \ll 3) + 8
$$
のように変換される.

Legacyモードを指定するには, FPGA_CTL_REGのLEGACY_MODE bitをセットする.
