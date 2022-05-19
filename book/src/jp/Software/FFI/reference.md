# C API Reference

c言語向けのAPIは[capi](https://github.com/shinolab/autd3/tree/master/capi)以下で定義されている.

以下に, このAPIのリファレンスを載せる. 
実際の利用方法は, [C API Example](https://github.com/shinolab/autd3/tree/master/capi/example)や, [C#](https://github.com/shinolab/autd3sharp)/[python](https://github.com/shinolab/pyautd)/[Julia](https://github.com/shinolab/AUTD3.jl)のラッパーライブラリを参照されたい.

また, ダイナミックライブラリの末尾にlegacyとつく物がある場合, legacyがついている方はLegacyモード専用で, ついていない方はNormalモード専用である.
末尾にlegacyとつく物がない場合, 両モード兼用である.

##  AUTDGetLastError (autd3capi/autd3capi-legacy)

最後に発生した例外のエラーメッセージを取得する.

引数にはエラーメッセージへのポインタを渡す.
このポインタにエラーメッセージがコピーされる. 
ただし, 引数がnullptrの場合はコピーは行われない. 
この関数は, null終端込みのエラーメッセージのサイズを返す.

エラーメッセージの長さは可変なので十分に大きな領域を確保しておくか, errorにnullptrを渡し必要なサイズを取得して再び呼び出すこと.

| Argument name / return | type             | in/out | description                                                                             |
|------------------------|------------------|--------|-----------------------------------------------------------------------------------------|
| error                  | char*            | out    | pointer to error message                                                                |
| return                 | int32_t          | -      | length of error message including null terminator                                       |

##  AUTDCreateController (autd3capi/autd3capi-legacy)

Controllerを作成する.

作成した`Controller`は最後に`AUTDFreeController`で開放する必要がある.

| Argument name / return | type             | in/out | description                                                  |
|------------------------|------------------|--------|--------------------------------------------------------------|
| out                    | void**           | out    | pointer to pointer to Controller                             |
| return                 | void             | -      | -                                                            |

##  AUTDOpenController (autd3capi/autd3capi-legacy)

Controllerをopenする. 

handleは`AUTDCreateController`で作成したものを使う.
linkは各々のlinkの生成関数で作成したものを使う.

この関数は失敗した場合にfalseを返す.
falseの場合には`AUTDGetLastError`でエラーメッセージを取得できる.

| Argument name / return | type             | in/out | description                                                  |
|------------------------|------------      |--------|--------------------------------------------------------------|
| handle                 | void*            | in     | pointer to Controller                                        |
| link                   | void*            | in     | pointer to Link                                              |
| return                 | bool             | -      | true if success                                              |

##  AUTDAddDevice (autd3capi/autd3capi-legacy)

ControllerにDeviceを追加する.

handleは`AUTDCreateController`で作成したものを使う. 
x, y, zは位置で, rz1, ry, rz2はZYZのオイラー角である.

この関数は追加されたDeviceのIdを返す.

| Argument name / return | type             | in/out | description                                                  |
|------------------------|------------------|--------|--------------------------------------------------------------|
| handle                 | void*            | in     | ControllerPtr                                                |
| x                      | double           | in     | x coordinate of position in millimeter                       |
| y                      | double           | in     | y coordinate of position in millimeter                       |
| z                      | double           | in     | z coordinate of position in millimeter                       |
| rz1                    | double           | in     | first angle of ZYZ euler angle in radian                     |
| ry                     | double           | in     | second angle of ZYZ euler angle in radian                    |
| rz2                    | double           | in     | third angle of ZYZ euler angle in radian                     |
| return                 | int32_t          | -      | Device Id                                                    |

##  AUTDAddDeviceQuaternion (autd3capi/autd3capi-legacy)

ControllerにDeviceを追加する.

handleは`AUTDCreateController`で作成したものを使う. 
x, y, zは位置で, qw, qx, qy, qzは回転を表すクオータニオンである.

この関数は追加されたDeviceのIdを返す.

| Argument name / return | type             | in/out | description                                                  |
|------------------------|------------------|--------|--------------------------------------------------------------|
| handle                 | void*            | in     | ControllerPtr                                                |
| x                      | double           | in     | x coordinate of position in millimeter                       |
| y                      | double           | in     | y coordinate of position in millimeter                       |
| z                      | double           | in     | z coordinate of position in millimeter                       |
| qw                     | double           | in     | w parameter of quaternion of rotation                        |
| qx                     | double           | in     | x parameter of quaternion of rotation                        |
| qy                     | double           | in     | y parameter of quaternion of rotation                        |
| qz                     | double           | in     | z parameter of quaternion of rotation                        |
| return                 | int32_t          | -      | Device Id                                                    |


##  AUTDClose (autd3capi/autd3capi-legacy)

Controllerをcloseする.

handleは`AUTDCreateController`で作成したものを使う.

この関数はエラーが発生した場合に0未満の値を返す.
エラーが生じた場合には`AUTDGetLastError`でエラーメッセージを取得できる.
また, check ackフラグがtrue, かつ, 返り値が0より大きい場合は, データが実際のデバイスで処理されたことを保証する.

| Argument name / return | type             | in/out | description                                                                                                        |
|------------------------|------------------|--------|--------------------------------------------------------------------------------------------------------------------|
| handle                 | void*            | in     | ControllerPtr                                                                                                      |
| return                 | int32_t          | -      | if $>0$ and check ack flag is true, it guarantees devices have processed data. if $<0$, error ocurred.             |

##  AUTDClear (autd3capi/autd3capi-legacy)

デバイス内の状態をclearする.

handleは`AUTDCreateController`で作成したものを使う.

この関数はエラーが発生した場合に0未満の値を返す.
エラーが生じた場合には`AUTDGetLastError`でエラーメッセージを取得できる.
また, check ackフラグがtrue, かつ, 返り値が0より大きい場合は, データが実際のデバイスで処理されたことを保証する.

| Argument name / return | type             | in/out | description                                                                                                        |
|------------------------|------------------|--------|-----------------------------------------------------------------------------------------                           |
| handle                 | void*            | in     | ControllerPtr                                                                                                      |
| return                 | int32_t          | -      | if $>0$ and check ack flag is true, it guarantees devices have processed data. if $<0$, error ocurred.             |

##  AUTDSynchronize (autd3capi/autd3capi-legacy)

デバイスを同期する.

handleは`AUTDCreateController`で作成したものを使う.

この関数はエラーが発生した場合に0未満の値を返す.
エラーが生じた場合には`AUTDGetLastError`でエラーメッセージを取得できる.
また, check ackフラグがtrue, かつ, 返り値が0より大きい場合は, データが実際のデバイスで処理されたことを保証する.

| Argument name / return | type             | in/out | description                                                                                                        |
|------------------------|------------------|--------|-----------------------------------------------------------------------------------------                           |
| handle                 | void*            | in     | ControllerPtr                                                                                                      |
| return                 | int32_t          | -      | if $>0$ and check ack flag is true, it guarantees devices have processed data. if $<0$, error ocurred.             |

##  AUTDSetSilencer (autd3capi/autd3capi-legacy)

Silencerを設定する.

handleは`AUTDCreateController`で作成したものを使う.

この関数はエラーが発生した場合に0未満の値を返す.
エラーが生じた場合には`AUTDGetLastError`でエラーメッセージを取得できる.
また, check ackフラグがtrue, かつ, 返り値が0より大きい場合は, データが実際のデバイスで処理されたことを保証する.

| Argument name / return | type             | in/out | description                                                                                                        |
|------------------------|------------------|--------|-----------------------------------------------------------------------------------------                           |
| handle                 | void*            | in     | ControllerPtr                                                                                                      |
| step                   | uint16_t         | in     | silencer update step                                                                                               |
| cycle                  | uint16_t         | in     | silencer update cycle                                                                                              |
| return                 | int32_t          | -      | if $>0$ and check ack flag is true, it guarantees devices have processed data. if $<0$, error ocurred.             |

##  AUTDFreeController (autd3capi/autd3capi-legacy)

Controllerを削除する.

handleは`AUTDCreateController`で作成したものを使う.

これいこうhandleは使用できない.

| Argument name / return | type             | in/out | description                                                  |
|------------------------|------------------|--------|--------------------------------------------------------------|
| handle                 | void*            | in     | pointer to Controller                                        |
| return                 | void             | -      | -                                                            |

##  AUTDIsOpen (autd3capi/autd3capi-legacy)

ControllerがOpenされているかどうかを返す.

handleは`AUTDCreateController`で作成したものを使う.

| Argument name / return | type             | in/out | description                                                                             |
|------------------------|------------------|--------|-----------------------------------------------------------------------------------------|
| handle                 | void*            | in     | ControllerPtr                                                                           |
| return                 | bool             | -      | true if controller is open                                                              |

##  AUTDGetForceFan (autd3capi/autd3capi-legacy)

Force fan flagを返す. 

handleは`AUTDCreateController`で作成したものを使う.

| Argument name / return | type             | in/out | description                                                                             |
|------------------------|------------------|--------|-----------------------------------------------------------------------------------------|
| handle                 | void*            | in     | ControllerPtr                                                                           |
| return                 | bool             | -      | Force fan flag                                                                          |

##  AUTDGetReadsFPGAInfo (autd3capi/autd3capi-legacy)

Reads FPGA info flagを返す. 

handleは`AUTDCreateController`で作成したものを使う.

| Argument name / return | type             | in/out | description                                                                             |
|------------------------|------------------|--------|-----------------------------------------------------------------------------------------|
| handle                 | void*            | in     | ControllerPtr                                                                           |
| return                 | bool             | -      | Reads FPGA info flag                                                                    |

##  AUTDGetCheckAck (autd3capi/autd3capi-legacy)

Check ack flagを返す. 

handleは`AUTDCreateController`で作成したものを使う.

| Argument name / return | type             | in/out | description                                                                             |
|------------------------|------------------|--------|-----------------------------------------------------------------------------------------|
| handle                 | void*            | in     | ControllerPtr                                                                           |
| return                 | bool             | -      | Check ack flag                                                                          |

##  AUTDSetReadsFPGAInfo (autd3capi/autd3capi-legacy)

Reads FPGA info flagを設定する. 

handleは`AUTDCreateController`で作成したものを使う.

デバイスに実際に反映されるのはsend functionsのどれかを呼び出し後である.

| Argument name / return | type             | in/out | description                                                                             |
|------------------------|------------------|--------|-----------------------------------------------------------------------------------------|
| handle                 | void*            | in     | ControllerPtr                                                                           |
| reads_fpga_info        | bool             | in     | read FPGA info flag                                                                     |
| return                 | void             | -      | -                                                                                       |

##  AUTDSetCheckAck (autd3capi/autd3capi-legacy)

Check ack flagを設定する. 

handleは`AUTDCreateController`で作成したものを使う.

| Argument name / return | type             | in/out | description                                                                             |
|------------------------|------------------|--------|-----------------------------------------------------------------------------------------|
| handle                 | void*            | in     | ControllerPtr                                                                           |
| check ack              | bool             | in     | check ack flag                                                                          |
| return                 | void             | -      | -                                                                                       |

##  AUTDSetForceFan (autd3capi/autd3capi-legacy)

Force fan flagを設定する. 

handleは`AUTDCreateController`で作成したものを使う.

デバイスに実際に反映されるのはsend functionsのどれかを呼び出し後である.

| Argument name / return | type             | in/out | description                                                                             |
|------------------------|------------------|--------|-----------------------------------------------------------------------------------------|
| handle                 | void*            | in     | ControllerPtr                                                                           |
| force_fan              | bool             | in     | force fan      flag                                                                     |
| return                 | void             | -      | -                                                                                       |

##  AUTDGetSoundSpeed (autd3capi/autd3capi-legacy)

音速を返す. 

handleは`AUTDCreateController`で作成したものを使う.

| Argument name / return | type             | in/out | description                                                                             |
|------------------------|------------------|--------|-----------------------------------------------------------------------------------------|
| handle                 | void*            | in     | ControllerPtr                                                                           |
| return                 | double           | -      | Speed of sound in m/s                                                                   |

##  AUTDSetSoundSpeed (autd3capi/autd3capi-legacy)

音速を設定する. 

handleは`AUTDCreateController`で作成したものを使う.

| Argument name / return | type             | in/out | description                                                                             |
|------------------------|------------------|--------|-----------------------------------------------------------------------------------------|
| handle                 | void*            | in     | ControllerPtr                                                                           |
| sound_speed            | double           | in     | Speed of sound in m/s                                                                   |
| return                 | void             | -      | -                                                                                       |

##  AUTDGetTransFrequency (autd3capi/autd3capi-legacy)

指定した振動子の周波数を取得する.

handleは`AUTDCreateController`で作成したものを使う.
振動子の指定はデバイスのインデックスとローカルの振動子インデックスでおこなう.

| Argument name / return       | type             | in/out | description                                                                             |
|------------------------------|------------------|--------|-----------------------------------------------------------------------------------------|
| handle                       | void*            | in     | ControllerPtr                                                                           |
| device_idx                   | int32_t          | in     | device index                                                                            |
| local_trans_idx              | int32_t          | in     | local transducer index                                                                  |
| return                       | double           | -      | frequency of the transducer                                                             |

##  AUTDSetTransFrequency (autd3capi/autd3capi-legacy)

指定した振動子の周波数を設定する.

handleは`AUTDCreateController`で作成したものを使う.
振動子の指定はデバイスのインデックスとローカルの振動子インデックスでおこなう.

autd3capi-legacyにおいては, この関数は何もしない.

| Argument name / return       | type             | in/out | description                                                                             |
|------------------------------|------------------|--------|-----------------------------------------------------------------------------------------|
| handle                       | void*            | in     | ControllerPtr                                                                           |
| device_idx                   | int32_t          | in     | device index                                                                            |
| local_trans_idx              | int32_t          | in     | local transducer index                                                                  |
| frequency                    | double           | -      | frequency of the transducer                                                             |
| return                       | void             | -      | -                                                                                       |

##  AUTDGetTransCycle (autd3capi/autd3capi-legacy)

指定した振動子の周期を取得する.

handleは`AUTDCreateController`で作成したものを使う.
振動子の指定はデバイスのインデックスとローカルの振動子インデックスでおこなう.

| Argument name / return       | type             | in/out | description                                                                             |
|------------------------------|------------------|--------|-----------------------------------------------------------------------------------------|
| handle                       | void*            | in     | ControllerPtr                                                                           |
| device_idx                   | int32_t          | in     | device index                                                                            |
| local_trans_idx              | int32_t          | in     | local transducer index                                                                  |
| return                       | uint16_t         | -      | cycle of the transducer                                                                 |

##  AUTDSetTransCycle (autd3capi/autd3capi-legacy)

指定した振動子の周期を設定する.

handleは`AUTDCreateController`で作成したものを使う.
振動子の指定はデバイスのインデックスとローカルの振動子インデックスでおこなう.

autd3capi-legacyにおいては, この関数は何もしない.

| Argument name / return       | type             | in/out | description                                                                             |
|------------------------------|------------------|--------|-----------------------------------------------------------------------------------------|
| handle                       | void*            | in     | ControllerPtr                                                                           |
| device_idx                   | int32_t          | in     | device index                                                                            |
| local_trans_idx              | int32_t          | in     | local transducer index                                                                  |
| frequency                    | uint16_t         | -      | cycle of the transducer                                                                 |
| return                       | void             | -      | -                                                                                       |

##  AUTDGetWavelength (autd3capi/autd3capi-legacy)

指定した振動子の波長を取得する.

handleは`AUTDCreateController`で作成したものを使う.
振動子の指定はデバイスのインデックスとローカルの振動子インデックスでおこなう.

| Argument name / return       | type             | in/out | description                                                                             |
|------------------------------|------------------|--------|-----------------------------------------------------------------------------------------|
| handle                       | void*            | in     | ControllerPtr                                                                           |
| device_idx                   | int32_t          | in     | device index                                                                            |
| local_trans_idx              | int32_t          | in     | local transducer index                                                                  |
| sound_speed                  | double           | in     | Speed of sound in m/s                                                                   |
| return                       | double           | -      | wavelength of ultrasound emitted from the transducer                                    |

##  AUTDGetAttenuation (autd3capi/autd3capi-legacy)

減衰係数を返す. 

handleは`AUTDCreateController`で作成したものを使う.

| Argument name / return | type             | in/out | description                                                                             |
|------------------------|------------------|--------|-----------------------------------------------------------------------------------------|
| handle                 | void*            | in     | ControllerPtr                                                                           |
| return                 | double           | -      | attenuation coefficient in Np/mm                                                        |

##  AUTDSetAttenuation (autd3capi/autd3capi-legacy)

減衰係数を設定する. 

handleは`AUTDCreateController`で作成したものを使う.

| Argument name / return | type             | in/out | description                                                                             |
|------------------------|------------------|--------|-----------------------------------------------------------------------------------------|
| handle                 | void*            | in     | ControllerPtr                                                                           |
| attenuation            | double           | in     | attenuation coefficient in Np/mm                                                        |
| return                 | void             | -      | -                                                                                       |

##  AUTDGetFPGAInfo (autd3capi/autd3capi-legacy)

FPGAの情報を取得する. 

handleは`AUTDCreateController`で作成したものを使う.
outポインタが指す領域は, 接続しているデバイスと同じ長さである必要がある.

なお, FPGAの情報は下位1bitが温度センサがアサートされているかどうかを表し, 他のbitは全て0である.

この関数を呼び出す前に`AUTDSetReadsFPGAInfo`でread FPGA info flagをOnにしておく必要がある.

この関数は失敗した場合にfalseを返す.
falseの場合には`AUTDGetLastError`でエラーメッセージを取得できる.

| Argument name / return | type             | in/out | description                                                                             |
|------------------------|------------------|--------|-----------------------------------------------------------------------------------------|
| handle                 | void*            | in     | ControllerPtr                                                                           |
| out                    | uint8_t*         | out    | FPGA information list                                                                   |
| return                 | bool             | -      | true if success                                                                         |

##  AUTDUpdateFlags (autd3capi/autd3capi-legacy)

Control flagを更新する.

send functionの一つ. 
force fan/reads FPGA info flagを設定した後に呼び出すと, これらの変更が実際に反映される.

handleは`AUTDCreateController`で作成したものを使う.

この関数はエラーが発生した場合に0未満の値を返す.
エラーが生じた場合には`AUTDGetLastError`でエラーメッセージを取得できる.
また, check ackフラグがtrue, かつ, 返り値が0より大きい場合は, データが実際のデバイスで処理されたことを保証する.

| Argument name / return | type             | in/out | description                                                                                                        |
|------------------------|------------------|--------|--------------------------------------------------------------------------------------------------------------------|
| handle                 | void*            | in     | ControllerPtr                                                                                                      |
| return                 | int32_t          | -      | if $>0$ and check ack flag is true, it guarantees devices have processed data. if $<0$, error ocurred.             |

##  AUTDNumDevices (autd3capi/autd3capi-legacy)

接続されているDeviceの数を取得する.

handleは`AUTDCreateController`で作成したものを使う.

| Argument name / return | type             | in/out | description                                                                             |
|----------------------- |------------------|--------|-----------------------------------------------------------------------------------------|
| handle                 | void*            | in     | ControllerPtr                                                                           |
| return                 | int32_t          | -      | number of devices                                                                       |

##  AUTDTransPosition (autd3capi/autd3capi-legacy)

指定した振動子の位置を取得する.

handleは`AUTDCreateController`で作成したものを使う.
振動子の指定はデバイスのインデックスとローカルの振動子インデックスでおこなう.

| Argument name / return | type             | in/out | description                                                                             |
|------------------------|------------------|--------|-----------------------------------------------------------------------------------------|
| handle                 | void*            | in     | ControllerPtr                                                                           |
| device_idx             | int32_t          | in     | device index                                                                            |
| local_trans_idx        | int32_t          | in     | local transducer index                                                                  |
| x                      | double*          | out    | x coordinate of transducer position                                                     |
| y                      | double*          | out    | y coordinate of transducer position                                                     |
| z                      | double*          | out    | z coordinate of transducer position                                                     |
| return                 | void             | -      | -                                                                                       |

##  AUTDTransXDirection (autd3capi/autd3capi-legacy)

指定した振動子のx軸方向を取得する.

handleは`AUTDCreateController`で作成したものを使う.
振動子の指定はデバイスのインデックスとローカルの振動子インデックスでおこなう.

| Argument name / return | type             | in/out | description                                                                             |
|------------------------|------------------|--------|-----------------------------------------------------------------------------------------|
| handle                 | void*            | in     | ControllerPtr                                                                           |
| device_idx             | int32_t          | in     | device index                                                                            |
| local_trans_idx        | int32_t          | in     | local transducer index                                                                  |
| x                      | double*          | out    | x coordinate of x-direction                                                             |
| y                      | double*          | out    | y coordinate of x-direction                                                             |
| z                      | double*          | out    | z coordinate of x-direction                                                             |
| return                 | void             | -      | -                                                                                       |

##  AUTDTransYDirection (autd3capi/autd3capi-legacy)

指定した振動子のy軸方向を取得する.

handleは`AUTDCreateController`で作成したものを使う.
振動子の指定はデバイスのインデックスとローカルの振動子インデックスでおこなう.

| Argument name / return | type             | in/out | description                                                                             |
|------------------------|------------------|--------|-----------------------------------------------------------------------------------------|
| handle                 | void*            | in     | ControllerPtr                                                                           |
| device_idx             | int32_t          | in     | device index                                                                            |
| local_trans_idx        | int32_t          | in     | local transducer index                                                                  |
| x                      | double*          | out    | x coordinate of y-direction                                                             |
| y                      | double*          | out    | y coordinate of y-direction                                                             |
| z                      | double*          | out    | z coordinate of y-direction                                                             |
| return                 | void             | -      | -                                                                                       |

##  AUTDTransZDirection (autd3capi/autd3capi-legacy)

指定した振動子のz軸方向を取得する.

handleは`AUTDCreateController`で作成したものを使う.
振動子の指定はデバイスのインデックスとローカルの振動子インデックスでおこなう.

| Argument name / return | type             | in/out | description                                                                             |
|------------------------|------------------|--------|-----------------------------------------------------------------------------------------|
| handle                 | void*            | in     | ControllerPtr                                                                           |
| device_idx             | int32_t          | in     | device index                                                                            |
| local_trans_idx        | int32_t          | in     | local transducer index                                                                  |
| x                      | double*          | out    | x coordinate of z-direction                                                             |
| y                      | double*          | out    | y coordinate of z-direction                                                             |
| z                      | double*          | out    | z coordinate of z-direction                                                             |
| return                 | void             | -      | -                                                                                       |

##  AUTDGetFirmwareInfoListPointer (autd3capi/autd3capi-legacy)

Firmware information listへのポインタを取得する.

handleは`AUTDCreateController`で作成したものを使う.

この関数で作成したlistは最後に`AUTDFreeFirmwareInfoListPointer`で開放する必要がある.

実際のFirmware informationは`AUTDGetFirmwareInfo`で取得する.

この関数はエラーが発生した場合に0未満の値を返す.
エラーが生じた場合には`AUTDGetLastError`でエラーメッセージを取得できる

| Argument name / return | type             | in/out | description                                                                             |
|------------------------|------------------|--------|-----------------------------------------------------------------------------------------|
| handle                 | void*            | in     | ControllerPtr                                                                           |
| out                    | void**           | out    | pointer to pointer to Firmware information list                                         |
| return                 | int32_t          | -      | if $<0$ some error occurred, else size of Firmware information list                     |

##  AUTDGetFirmwareInfo (autd3capi/autd3capi-legacy)

Firmware informationを取得する.

`p_firm_info_list`は`AUTDGetFirmwareInfoListPointer`で作成したものを使う.

`info`は長さ256のバッファを渡せば十分である.

| Argument name / return | type             | in/out | description                                                                             |
|----------------------- |------------------|--------|-----------------------------------------------------------------------------------------|
| p_firm_info_list       | void*            | in     | pointer to Firmware information list                                                    |
| index                  | int32_t          | in     | device index                                                                            |
| info                   | char*            | out    | pointer to firmware information string                                                  |
| return                 | void             | -      | -                                                                                       |

##  AUTDFreeFirmwareInfoListPointer (autd3capi/autd3capi-legacy)

`AUTDGetFirmwareInfoListPointer`で取得したFirmware information listを開放する.

| Argument name / return | type             | in/out | description                                                                             |
|----------------------- |------------------|--------|-----------------------------------------------------------------------------------------|
| p_firm_info_list       | void*            | in     | pointer to Firmware information list                                                    |
| return                 | void             | -      | -                                                                                       |

##  AUTDGainNull (autd3capi/autd3capi-legacy)

Null gainを作成する.

作成したGainは最後に`AUTDDeleteGain`で削除する必要がある.

| Argument name / return | type             | in/out | description                                                                             |
|------------------------|------------------|--------|-----------------------------------------------------------------------------------------|
| gain                   | void**           | out    | pointer to pointer to Null gain                                                         |
| return                 | void             | -      | -                                                                                       |

##  AUTDGainGrouped (autd3capi/autd3capi-legacy)

Grouped gainを作成する.

handleは`AUTDCreateController`で作成したものを使う.

作成したGainは最後に`AUTDDeleteGain`で削除する必要がある.

| Argument name / return | type             | in/out | description                                                                             |
|------------------------|------------------|--------|-----------------------------------------------------------------------------------------|
| gain                   | void**           | out    | pointer to pointer to Grouped gain                                                      |
| handle                 | void*            | in     | ControllerPtr                                                                           |
| return                 | void             | -      | -                                                                                       |

##  AUTDGainGroupedAdd (autd3capi/autd3capi-legacy)

Grouped gainにGainを登録する.

`grouped_gain`は`AUTDGainGrouped`で作成したものを使う. 

| Argument name / return | type             | in/out | description                                                                             |
|------------------------|------------------|--------|-----------------------------------------------------------------------------------------|
| grouped_gain           | void*            | in     | pointer to Grouped gain                                                                 |
| device_id              | int32_t          | in     | Device Id                                                                               |
| gain                   | void*            | in     | pointer to gain                                                                         |
| return                 | void             | -      | -                                                                                       |

##  AUTDGainFocus (autd3capi/autd3capi-legacy)

Focus gainを作成する.

作成したGainは最後に`AUTDDeleteGain`で削除する必要がある.

| Argument name / return | type             | in/out | description                                                                             |
|----------------------- |------------------|--------|-----------------------------------------------------------------------------------------|
| gain                   | void**           | out    | pointer to Focus gain                                                                   |
| x                      | double           | in     | x coordinate of focal point                                                             |
| y                      | double           | in     | y coordinate of focal point                                                             |
| z                      | double           | in     | z coordinate of focal point                                                             |
| amp                    | double           | in     | amplitude of focus                                                                      |
| return                 | void             | -      | -                                                                                       |

##  AUTDGainBesselBeam (autd3capi/autd3capi-legacy)

Bessel beam gainを作成する.

作成したGainは最後に`AUTDDeleteGain`で削除する必要がある.

| Argument name / return | type             | in/out | description                                                                             |
|----------------------- |------------------|--------|-----------------------------------------------------------------------------------------|
| gain                   | void**           | out    | pointer to Bessel beam gain                                                             |
| x                      | double           | in     | x coordinate of apex                                                                    |
| y                      | double           | in     | y coordinate of apex                                                                    |
| z                      | double           | in     | z coordinate of apex                                                                    |
| n_x                    | double           | in     | x coordinate of beam direction                                                          |
| n_y                    | double           | in     | y coordinate of beam direction                                                          |
| n_z                    | double           | in     | z coordinate of beam direction                                                          |
| theta_z                | double           | in     | tilt angle of beam                                                                      |
| amp                    | double           | in     | amplitude of beam                                                                       |
| return                 | void             | -      | -                                                                                       |

##  AUTDGainPlaneWave (autd3capi/autd3capi-legacy)

Plane wave gainを作成する.

作成したGainは最後に`AUTDDeleteGain`で削除する必要がある.

| Argument name / return | type             | in/out | description                                                                             |
|----------------------- |------------------|--------|-----------------------------------------------------------------------------------------|
| gain                   | void**           | out    | pointer to Plane wave gain                                                              |
| n_x                    | double           | in     | x coordinate of wave direction                                                          |
| n_y                    | double           | in     | y coordinate of wave direction                                                          |
| n_z                    | double           | in     | z coordinate of wave direction                                                          |
| amp                    | double           | in     | amplitude of wave                                                                       |
| return                 | void             | -      | -                                                                                       |

##  AUTDGainCustom (autd3capi/autd3capi-legacy)

Custom gainを作成する.

Custom gainは位相と振幅を直接指定するGainである.

作成したGainは最後に`AUTDDeleteGain`で削除する必要がある.

| Argument name / return | type             | in/out | description                                                                             |
|----------------------- |------------------|--------|-----------------------------------------------------------------------------------------|
| gain                   | void**           | out    | pointer to Focus gain                                                                   |
| amp                    | double*          | in     | pointer to amplitude                                                                    |
| phase                  | double*          | in     | pointer to phase                                                                        |
| return                 | void             | -      | -                                                                                       |

##  AUTDDeleteGain (autd3capi/autd3capi-legacy)

作成したGainを削除する.

| Argument name / return | type             | in/out | description                                                                             |
|------------------------|------------------|--------|-----------------------------------------------------------------------------------------|
| gain                   | void*            | in     | pointer to gain                                                                         |
| return                 | void             | -      | -                                                                                       |

##  AUTDModulationStatic (autd3capi/autd3capi-legacy)

Static modulationを作成する.

作成したModulationは最後に`AUTDDeleteModulation`で削除する必要がある.

| Argument name / return | type             | in/out | description                                                                             |
|----------------------- |------------------|--------|-----------------------------------------------------------------------------------------|
| mod                    | void**           | out    | pointer to pointer to Static modulation                                                 |
| amp                    | double           | in     | amplitude of modulation                                                                 |
| return                 | void             | -      | -                                                                                       |

##  AUTDModulationSine (autd3capi/autd3capi-legacy)

Sine modulationを作成する.

作成したModulationは最後に`AUTDDeleteModulation`で削除する必要がある.

| Argument name / return | type             | in/out | description                                                                             |
|----------------------- |------------------|--------|-----------------------------------------------------------------------------------------|
| mod                    | void**           | out    | pointer to pointer to Sine modulation                                                   |
| freq                   | int32_t          | in     | frequency of sine modulation                                                            |
| amp                    | double           | in     | amplitude of sine modulation                                                            |
| offset                 | double           | in     | offset of sine modulation                                                               |
| return                 | void             | -      | -                                                                                       |

##  AUTDModulationSineSquared (autd3capi/autd3capi-legacy)

SineSquared modulationを作成する.

作成したModulationは最後に`AUTDDeleteModulation`で削除する必要がある.

| Argument name / return | type             | in/out | description                                                                             |
|----------------------- |------------------|--------|-----------------------------------------------------------------------------------------|
| mod                    | void**           | out    | pointer to pointer to SineSquared modulation                                            |
| freq                   | int32_t          | in     | frequency of sine modulation                                                            |
| amp                    | double           | in     | amplitude of sine modulation                                                            |
| offset                 | double           | in     | offset of sine modulation                                                               |
| return                 | void             | -      | -                                                                                       |

##  AUTDModulationSineLegacy (autd3capi/autd3capi-legacy)

SineLegacy modulationを作成する.

作成したModulationは最後に`AUTDDeleteModulation`で削除する必要がある.

| Argument name / return | type             | in/out | description                                                                             |
|----------------------- |------------------|--------|-----------------------------------------------------------------------------------------|
| mod                    | void**           | out    | pointer to pointer to SineLegacy modulation                                             |
| freq                   | double           | in     | frequency of sine modulation                                                            |
| amp                    | double           | in     | amplitude of sine modulation                                                            |
| offset                 | double           | in     | offset of sine modulation                                                               |
| return                 | void             | -      | -                                                                                       |

##  AUTDModulationSquare (autd3capi/autd3capi-legacy)

Square modulationを作成する.

作成したModulationは最後に`AUTDDeleteModulation`で削除する必要がある.

| Argument name / return | type             | in/out | description                                                                             |
|----------------------- |------------------|--------|-----------------------------------------------------------------------------------------|
| mod                    | void**           | out    | pointer to pointer to Square modulation                                                 |
| freq                   | int32_t          | in     | frequency of square modulation                                                          |
| low                    | double           | in     | amplitude at low level of square modulation                                             |
| high                   | double           | in     | amplitude at high level of square modulation                                            |
| duty                   | double           | in     | duty ratio of square modulation                                                         |
| return                 | void             | -      | -                                                                                       |


##  AUTDModulationCustom (autd3capi/autd3capi-legacy)

Custom modulationを作成する.

Custom modulationは振幅を直接指定するModulationである.

作成したModulationは最後に`AUTDDeleteModulation`で削除する必要がある.

| Argument name / return | type             | in/out | description                                                                             |
|----------------------- |------------------|--------|-----------------------------------------------------------------------------------------|
| mod                    | void**           | out    | pointer to pointer to Custom modulation                                                 |
| buffer                 | uint8_t*         | in     | pointer to modulation data                                                              |
| size                   | uint64_t         | in     | size of buffer                                                                          |
| freq_div               | uint32_t         | in     | modulation sampling frequency division                                                  |
| return                 | void             | -      | -                                                                                       |

##  AUTDModulationSamplingFrequencyDivision (autd3capi/autd3capi-legacy)

Modulation sampling frequency divisionを返す.

| Argument name / return | type             | in/out | description                                                                             |
|----------------------- |------------------|--------|-----------------------------------------------------------------------------------------|
| mod                    | void*            | in     | pointer to modulation                                                                   |
| return                 | uint32_t         | -      | modulation sampling frequency division                                                  |

##  AUTDModulationSetSamplingFrequencyDivision (autd3capi/autd3capi-legacy)

Modulation sampling frequency divisionを設定する.

| Argument name / return | type             | in/out | description                                                                             |
|----------------------- |------------------|--------|-----------------------------------------------------------------------------------------|
| mod                    | void*            | in     | pointer to modulation                                                                   |
| freq_div               | uint32_t         | in     | modulation sampling frequency division                                                  |
| return                 | void             | -      | -                                                                                       |

##  AUTDModulationSamplingFrequency (autd3capi/autd3capi-legacy)

Sampling frequencyを返す.

| Argument name / return | type             | in/out | description                                                                             |
|----------------------- |------------------|--------|-----------------------------------------------------------------------------------------|
| mod                    | void*            | in     | pointer to modulation                                                                   |
| return                 | double           | -      | modulation sampling frequency                                                           |

##  AUTDDeleteModulation (autd3capi/autd3capi-legacy)

Modulationを削除する.

| Argument name / return | type             | in/out | description                                                                             |
|----------------------- |------------------|--------|-----------------------------------------------------------------------------------------|
| mod                    | void*            | in     | pointer to modulation                                                                   |
| return                 | void             | -      | -                                                                                       |

##  AUTDPointSTM (autd3capi/autd3capi-legacy)

Point STMを作成する.

作成したModulationは最後に`AUTDDeleteSTM`で削除する必要がある. 

| Argument name / return | type             | in/out | description                                                                             |
|----------------------- |------------------|--------|-----------------------------------------------------------------------------------------|
| out                    | void**           | out    | pointer to pointer to Point STM                                                         |
| return                 | void             | -      | -                                                                                       |

##  AUTDGainSTM (autd3capi/autd3capi-legacy)

Gain STMを作成する.

作成したModulationは最後に`AUTDDeleteSTM`で削除する必要がある. 

| Argument name / return | type             | in/out | description                                                                             |
|----------------------- |------------------|--------|-----------------------------------------------------------------------------------------|
| out                    | void**           | out    | pointer to pointer to Gain STM                                                         |
| return                 | void             | -      | -                                                                                       |

##  AUTDPointSTMAdd (autd3capi/autd3capi-legacy)

Point STMに焦点を追加する.

| Argument name / return | type             | in/out | description                                                                             |
|----------------------- |------------------|--------|-----------------------------------------------------------------------------------------|
| stm                    | void*            | in     | pointer to Point STM                                                                    |
| x                      | double           | in     | x coordinate of focal point                                                             |
| y                      | double           | in     | y coordinate of focal point                                                             |
| z                      | double           | in     | z coordinate of focal point                                                             |
| shift                  | uint8_t          | in     | duty shift                                                                              |
| return                 | void             | -      | -                                                                                       |

##  AUTDGainSTMAdd (autd3capi/autd3capi-legacy)

Gain STMにgainを追加する.

| Argument name / return | type             | in/out | description                                                                             |
|----------------------- |------------------|--------|-----------------------------------------------------------------------------------------|
| stm                    | void*            | in     | pointer to Point STM                                                                    |
| gain                   | void*            | in     | pointer to Gain                                                                         |
| return                 | void             | -      | -                                                                                       |

##  AUTDSTMSetFrequency (autd3capi/autd3capi-legacy)

STMのfrequencyを設定する.

| Argument name / return | type             | in/out | description                                                                             |
|----------------------- |------------------|--------|-----------------------------------------------------------------------------------------|
| stm                    | void*            | in     | pointer to STM                                                                          |
| freq                   | double           | in     | frequency of STM                                                                        |
| return                 | void             | -      | -                                                                                       |

##  AUTDSTMFrequency (autd3capi/autd3capi-legacy)

STMのfrequencyを取得する.

| Argument name / return | type             | in/out | description                                                                             |
|----------------------- |------------------|--------|-----------------------------------------------------------------------------------------|
| stm                    | void*            | in     | pointer to STM                                                                          |
| return                 | double           | -      | frequency of STM                                                                        |

##  AUTDSTMSamplingFrequency (autd3capi/autd3capi-legacy)

STMのsampling frequencyを取得する.

| Argument name / return | type             | in/out | description                                                                             |
|----------------------- |------------------|--------|-----------------------------------------------------------------------------------------|
| stm                    | void*            | in     | pointer to STM                                                                          |
| return                 | double           | -      | sampling frequency of STM                                                               |

##  AUTDSTMSamplingFrequencyDivision (autd3capi/autd3capi-legacy)

STMのsampling frequency divisionを取得する.

| Argument name / return | type             | in/out | description                                                                             |
|----------------------- |------------------|--------|-----------------------------------------------------------------------------------------|
| stm                    | void*            | in     | pointer to STM                                                                          |
| return                 | uint32_t         | in     | STM sampling frequency division                                                         |

##  AUTDSTMSetSamplingFrequencyDivision (autd3capi/autd3capi-legacy)

STMのsampling frequency divisionを設定する.

| Argument name / return | type             | in/out | description                                                                             |
|----------------------- |------------------|--------|-----------------------------------------------------------------------------------------|
| stm                    | void*            | in     | pointer to STM                                                                          |
| freq_div               | uint32_t         | in     | STM sampling frequency division                                                         |
| return                 | void             | -      | -                                                                                       |

##  AUTDDeleteSTM (autd3capi/autd3capi-legacy)

STMを削除する.

| Argument name / return | type             | in/out | description                                                                             |
|----------------------- |------------------|--------|-----------------------------------------------------------------------------------------|
| stm                    | void*            | in     | pointer to STM                                                                          |
| return                 | void             | -      | -                                                                                       |

##  AUTDStop (autd3capi/autd3capi-legacy)

AUTDの出力を停止する.

handleは`AUTDCreateController`で作成したものを使う.

この関数はエラーが発生した場合に0未満の値を返す.
エラーが生じた場合には`AUTDGetLastError`でエラーメッセージを取得できる.
また, check ackフラグがtrue, かつ, 返り値が0より大きい場合は, データが実際のデバイスで処理されたことを保証する.

| Argument name / return | type             | in/out | description                                                                                                        |
|------------------------|------------------|--------|--------------------------------------------------------------------------------------------------------------------|
| handle                 | void*            | in     | ControllerPtr                                                                                                      |
| return                 | int32_t          | -      | if $>0$ and check ack flag is true, it guarantees devices have processed data. if $<0$, error ocurred.             |

##  AUTDSendHeader (autd3capi/autd3capi-legacy)

ヘッダーデータを送信する.

send functionの一つ. 

handleは`AUTDCreateController`で作成したものを使う.

この関数はエラーが発生した場合に0未満の値を返す.
エラーが生じた場合には`AUTDGetLastError`でエラーメッセージを取得できる.
また, check ackフラグがtrue, かつ, 返り値が0より大きい場合は, データが実際のデバイスで処理されたことを保証する.

| Argument name / return | type             | in/out | description                                                                                                        |
|------------------------|------------------|--------|--------------------------------------------------------------------------------------------------------------------|
| handle                 | void*            | in     | ControllerPtr                                                                                                      |
| header                 | void*            | in     | pointer to header data                                                                                             |
| return                 | int32_t          | -      | if $>0$ and check ack flag is true, it guarantees devices have processed data. if $<0$, error ocurred.             |

##  AUTDSendBody (autd3capi/autd3capi-legacy)

ボディーデータを送信する.

send functionの一つ. 

handleは`AUTDCreateController`で作成したものを使う.

この関数はエラーが発生した場合に0未満の値を返す.
エラーが生じた場合には`AUTDGetLastError`でエラーメッセージを取得できる.
また, check ackフラグがtrue, かつ, 返り値が0より大きい場合は, データが実際のデバイスで処理されたことを保証する.

| Argument name / return | type             | in/out | description                                                                                                        |
|------------------------|------------------|--------|--------------------------------------------------------------------------------------------------------------------|
| handle                 | void*            | in     | ControllerPtr                                                                                                      |
| body                   | void*            | in     | pointer to body data                                                                                               |
| return                 | int32_t          | -      | if $>0$ and check ack flag is true, it guarantees devices have processed data. if $<0$, error ocurred.             |

##  AUTDSendHeaderBody (autd3capi/autd3capi-legacy)

ヘッダーデータとボディーデータを送信する.

send functionの一つ. 

handleは`AUTDCreateController`で作成したものを使う.

この関数はエラーが発生した場合に0未満の値を返す.
エラーが生じた場合には`AUTDGetLastError`でエラーメッセージを取得できる.
また, check ackフラグがtrue, かつ, 返り値が0より大きい場合は, データが実際のデバイスで処理されたことを保証する.

| Argument name / return | type             | in/out | description                                                                                                        |
|------------------------|------------------|--------|--------------------------------------------------------------------------------------------------------------------|
| handle                 | void*            | in     | ControllerPtr                                                                                                      |
| header                 | void*            | in     | pointer to header data                                                                                             |
| body                   | void*            | in     | pointer to body data                                                                                               |
| return                 | int32_t          | -      | if $>0$ and check ack flag is true, it guarantees devices have processed data. if $<0$, error ocurred.             |
