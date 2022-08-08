# C API Reference

c言語向けのAPIは[capi](https://github.com/shinolab/autd3/tree/master/capi)以下で定義されている.

以下に, このAPIのリファレンスを載せる. 実際の利用方法は,
[C API Example](https://github.com/shinolab/autd3/tree/master/capi/example)や,
[C#](https://github.com/shinolab/autd3sharp)/[python](https://github.com/shinolab/pyautd)のラッパーライブラリを参照されたい.

## AUTDGetLastError (autd3capi)

最後に発生した例外のエラーメッセージを取得する.

引数にはエラーメッセージへのポインタを渡す. このポインタにエラーメッセージがコピーされる. ただし, 引数がnullptrの場合はコピーは行われない.
この関数は, null終端込みのエラーメッセージのサイズを返す.

エラーメッセージの長さは可変なので十分に大きな領域を確保しておくか, errorにnullptrを渡し必要なサイズを取得して再び呼び出すこと.

| Argument name / return | type    | in/out | description                                       |
| ---------------------- | ------- | ------ | ------------------------------------------------- |
| error                  | char*   | out    | pointer to error message                          |
| return                 | int32_t | -      | length of error message including null terminator |

## AUTDCreateController (autd3capi)

Controllerを作成する.

作成した`Controller`は最後に`AUTDFreeController`で開放する必要がある.

| Argument name / return | type   | in/out | description                      |
| ---------------------- | ------ | ------ | -------------------------------- |
| out                    | void** | out    | pointer to pointer to Controller |
| return                 | void   | -      | -                                |

## AUTDOpenController (autd3capi)

Controllerをopenする.

handleは`AUTDCreateController`で作成したものを使う. linkは各々のlinkの生成関数で作成したものを使う.

この関数は失敗した場合にfalseを返す. falseの場合には`AUTDGetLastError`でエラーメッセージを取得できる.

| Argument name / return | type  | in/out | description           |
| ---------------------- | ----- | ------ | --------------------- |
| handle                 | void* | in     | pointer to Controller |
| link                   | void* | in     | pointer to Link       |
| return                 | bool  | -      | true if success       |

## AUTDAddDevice (autd3capi)

ControllerにDeviceを追加する.

handleは`AUTDCreateController`で作成したものを使う. x, y, zは位置で, rz1, ry, rz2はZYZのオイラー角である.

この関数は追加されたDeviceのIdを返す.

| Argument name / return | type    | in/out | description                               |
| ---------------------- | ------- | ------ | ----------------------------------------- |
| handle                 | void*   | in     | pointer to Controller                     |
| x                      | double  | in     | x coordinate of position in millimeter    |
| y                      | double  | in     | y coordinate of position in millimeter    |
| z                      | double  | in     | z coordinate of position in millimeter    |
| rz1                    | double  | in     | first angle of ZYZ euler angle in radian  |
| ry                     | double  | in     | second angle of ZYZ euler angle in radian |
| rz2                    | double  | in     | third angle of ZYZ euler angle in radian  |
| return                 | int32_t | -      | Device Id                                 |

## AUTDAddDeviceQuaternion (autd3capi)

ControllerにDeviceを追加する.

handleは`AUTDCreateController`で作成したものを使う. x, y, zは位置で, qw, qx, qy,
qzは回転を表すクオータニオンである.

この関数は追加されたDeviceのIdを返す.

| Argument name / return | type    | in/out | description                            |
| ---------------------- | ------- | ------ | -------------------------------------- |
| handle                 | void*   | in     | pointer to Controller                  |
| x                      | double  | in     | x coordinate of position in millimeter |
| y                      | double  | in     | y coordinate of position in millimeter |
| z                      | double  | in     | z coordinate of position in millimeter |
| qw                     | double  | in     | w parameter of quaternion of rotation  |
| qx                     | double  | in     | x parameter of quaternion of rotation  |
| qy                     | double  | in     | y parameter of quaternion of rotation  |
| qz                     | double  | in     | z parameter of quaternion of rotation  |
| return                 | int32_t | -      | Device Id                              |

## AUTDClose (autd3capi)

Controllerをcloseする.

handleは`AUTDCreateController`で作成したものを使う.

この関数はエラーが発生した場合に0未満の値を返す. エラーが生じた場合には`AUTDGetLastError`でエラーメッセージを取得できる. また,
check ackフラグがtrue, かつ, 返り値が0より大きい場合は, データが実際のデバイスで処理されたことを保証する.

| Argument name / return | type    | in/out | description                                                                                            |
| ---------------------- | ------- | ------ | ------------------------------------------------------------------------------------------------------ |
| handle                 | void*   | in     | pointer to Controller                                                                                  |
| return                 | int32_t | -      | if $>0$ and check ack flag is true, it guarantees devices have processed data. if $<0$, error ocurred. |

## AUTDClear (autd3capi)

デバイス内の状態をclearする.

handleは`AUTDCreateController`で作成したものを使う.

この関数はエラーが発生した場合に0未満の値を返す. エラーが生じた場合には`AUTDGetLastError`でエラーメッセージを取得できる. また,
check ackフラグがtrue, かつ, 返り値が0より大きい場合は, データが実際のデバイスで処理されたことを保証する.

| Argument name / return | type    | in/out | description                                                                                            |
| ---------------------- | ------- | ------ | ------------------------------------------------------------------------------------------------------ |
| handle                 | void*   | in     | pointer to Controller                                                                                  |
| return                 | int32_t | -      | if $>0$ and check ack flag is true, it guarantees devices have processed data. if $<0$, error ocurred. |

## AUTDSynchronize (autd3capi)

デバイスを同期する.

handleは`AUTDCreateController`で作成したものを使う.

この関数はエラーが発生した場合に0未満の値を返す. エラーが生じた場合には`AUTDGetLastError`でエラーメッセージを取得できる. また,
check ackフラグがtrue, かつ, 返り値が0より大きい場合は, データが実際のデバイスで処理されたことを保証する.

| Argument name / return | type    | in/out | description                                                                                            |
| ---------------------- | ------- | ------ | ------------------------------------------------------------------------------------------------------ |
| handle                 | void*   | in     | pointer to Controller                                                                                  |
| return                 | int32_t | -      | if $>0$ and check ack flag is true, it guarantees devices have processed data. if $<0$, error ocurred. |

## AUTDCreateSilencer (autd3capi)

SilencerConfigを作成する.

作成したSilencerConfigは最後に`AUTDDeleteSilencer`で削除する必要がある.

| Argument name / return | type     | in/out | description                          |
| ---------------------- | -------- | ------ | ------------------------------------ |
| out                    | void**   | out    | pointer to pointer to SilencerConfig |
| step                   | uint16_t | in     | silencer update step                 |
| cycle                  | uint16_t | in     | silencer update cycle                |
| return                 | void     | -      | -                                    |

## AUTDDeleteSilencer (autd3capi)

SilencerConfigを削除する.

| Argument name / return | type  | in/out | description               |
| ---------------------- | ----- | ------ | ------------------------- |
| config                 | void* | in     | pointer to SilencerConfig |
| return                 | void  | -      | -                         |

## AUTDFreeController (autd3capi)

Controllerを削除する.

handleは`AUTDCreateController`で作成したものを使う.

これいこうhandleは使用できない.

| Argument name / return | type  | in/out | description           |
| ---------------------- | ----- | ------ | --------------------- |
| handle                 | void* | in     | pointer to Controller |
| return                 | void  | -      | -                     |

## AUTDIsOpen (autd3capi)

ControllerがOpenされているかどうかを返す.

handleは`AUTDCreateController`で作成したものを使う.

| Argument name / return | type  | in/out | description                |
| ---------------------- | ----- | ------ | -------------------------- |
| handle                 | void* | in     | pointer to Controller      |
| return                 | bool  | -      | true if controller is open |

## AUTDGetForceFan (autd3capi)

Force fan flagを返す.

handleは`AUTDCreateController`で作成したものを使う.

| Argument name / return | type  | in/out | description           |
| ---------------------- | ----- | ------ | --------------------- |
| handle                 | void* | in     | pointer to Controller |
| return                 | bool  | -      | Force fan flag        |

## AUTDGetReadsFPGAInfo (autd3capi)

Reads FPGA info flagを返す.

handleは`AUTDCreateController`で作成したものを使う.

| Argument name / return | type  | in/out | description           |
| ---------------------- | ----- | ------ | --------------------- |
| handle                 | void* | in     | pointer to Controller |
| return                 | bool  | -      | Reads FPGA info flag  |

## AUTDGetCheckTrials (autd3capi)

Check trialsを返す.

handleは`AUTDCreateController`で作成したものを使う.

| Argument name / return | type    | in/out | description           |
| ---------------------- | ------- | ------ | --------------------- |
| handle                 | void*   | in     | pointer to Controller |
| return                 | int32_t | -      | Check trials          |

## AUTDGetSendInterval (autd3capi)

Send intervalを返す.

handleは`AUTDCreateController`で作成したものを使う.

| Argument name / return | type    | in/out | description           |
| ---------------------- | ------- | ------ | --------------------- |
| handle                 | void*   | in     | pointer to Controller |
| return                 | int32_t | -      | Send interval         |

## AUTDSetReadsFPGAInfo (autd3capi)

Reads FPGA info flagを設定する.

handleは`AUTDCreateController`で作成したものを使う.

デバイスに実際に反映されるのはsend functionsのどれかを呼び出し後である.

| Argument name / return | type  | in/out | description           |
| ---------------------- | ----- | ------ | --------------------- |
| handle                 | void* | in     | pointer to Controller |
| reads_fpga_info        | bool  | in     | read FPGA info flag   |
| return                 | void  | -      | -                     |

## AUTDSetCheckTrials (autd3capi)

Check trialsを設定する.

handleは`AUTDCreateController`で作成したものを使う.

| Argument name / return | type    | in/out | description           |
| ---------------------- | ------- | ------ | --------------------- |
| handle                 | void*   | in     | pointer to Controller |
| trials                 | int32_t | in     | check trials          |
| return                 | void    | -      | -                     |

## AUTDSetSendInterval (autd3capi)

Send intervalを設定する.

handleは`AUTDCreateController`で作成したものを使う.

| Argument name / return | type    | in/out | description           |
| ---------------------- | ------- | ------ | --------------------- |
| handle                 | void*   | in     | pointer to Controller |
| interval               | int32_t | in     | Send interval         |
| return                 | void    | -      | -                     |

## AUTDSetForceFan (autd3capi)

Force fan flagを設定する.

handleは`AUTDCreateController`で作成したものを使う.

デバイスに実際に反映されるのはsend functionsのどれかを呼び出し後である.

| Argument name / return | type  | in/out | description           |
| ---------------------- | ----- | ------ | --------------------- |
| handle                 | void* | in     | pointer to Controller |
| force                  | bool  | in     | force fan flag        |
| return                 | void  | -      | -                     |

## AUTDGetSoundSpeed (autd3capi)

音速を返す.

handleは`AUTDCreateController`で作成したものを使う.

| Argument name / return | type   | in/out | description           |
| ---------------------- | ------ | ------ | --------------------- |
| handle                 | void*  | in     | pointer to Controller |
| return                 | double | -      | Speed of sound in m/s |

## AUTDSetSoundSpeed (autd3capi)

音速を設定する.

handleは`AUTDCreateController`で作成したものを使う.

| Argument name / return | type   | in/out | description           |
| ---------------------- | ------ | ------ | --------------------- |
| handle                 | void*  | in     | pointer to Controller |
| sound_speed            | double | in     | Speed of sound in m/s |
| return                 | void   | -      | -                     |

## AUTDGetTransFrequency (autd3capi)

指定した振動子の周波数を取得する.

handleは`AUTDCreateController`で作成したものを使う. 振動子の指定はデバイスのインデックスとローカルの振動子インデックスでおこなう.

| Argument name / return | type    | in/out | description                 |
| ---------------------- | ------- | ------ | --------------------------- |
| handle                 | void*   | in     | pointer to Controller       |
| device_idx             | int32_t | in     | device index                |
| local_trans_idx        | int32_t | in     | local transducer index      |
| return                 | double  | -      | frequency of the transducer |

## AUTDSetTransFrequency (autd3capi)

指定した振動子の周波数を設定する.

handleは`AUTDCreateController`で作成したものを使う. 振動子の指定はデバイスのインデックスとローカルの振動子インデックスでおこなう.

Legacyモードにおいては, この関数は何もしない.

| Argument name / return | type    | in/out | description                 |
| ---------------------- | ------- | ------ | --------------------------- |
| handle                 | void*   | in     | pointer to Controller       |
| device_idx             | int32_t | in     | device index                |
| local_trans_idx        | int32_t | in     | local transducer index      |
| frequency              | double  | in     | frequency of the transducer |
| return                 | void    | -      | -                           |

## AUTDGetTransCycle (autd3capi)

指定した振動子の周期を取得する.

handleは`AUTDCreateController`で作成したものを使う. 振動子の指定はデバイスのインデックスとローカルの振動子インデックスでおこなう.

| Argument name / return | type     | in/out | description             |
| ---------------------- | -------- | ------ | ----------------------- |
| handle                 | void*    | in     | pointer to Controller   |
| device_idx             | int32_t  | in     | device index            |
| local_trans_idx        | int32_t  | in     | local transducer index  |
| return                 | uint16_t | -      | cycle of the transducer |

## AUTDSetTransCycle (autd3capi)

指定した振動子の周期を設定する.

handleは`AUTDCreateController`で作成したものを使う. 振動子の指定はデバイスのインデックスとローカルの振動子インデックスでおこなう.

Legacyモードにおいては, この関数は何もしない.

| Argument name / return | type     | in/out | description             |
| ---------------------- | -------- | ------ | ----------------------- |
| handle                 | void*    | in     | pointer to Controller   |
| device_idx             | int32_t  | in     | device index            |
| local_trans_idx        | int32_t  | in     | local transducer index  |
| cycle                  | uint16_t | in     | cycle of the transducer |
| return                 | void     | -      | -                       |

## AUTDGetWavelength (autd3capi)

指定した振動子の波長を取得する.

handleは`AUTDCreateController`で作成したものを使う. 振動子の指定はデバイスのインデックスとローカルの振動子インデックスでおこなう.

| Argument name / return | type    | in/out | description                                          |
| ---------------------- | ------- | ------ | ---------------------------------------------------- |
| handle                 | void*   | in     | pointer to Controller                                |
| device_idx             | int32_t | in     | device index                                         |
| local_trans_idx        | int32_t | in     | local transducer index                               |
| return                 | double  | -      | wavelength of ultrasound emitted from the transducer |

## AUTDGetAttenuation (autd3capi)

減衰係数を返す.

handleは`AUTDCreateController`で作成したものを使う.

| Argument name / return | type   | in/out | description                      |
| ---------------------- | ------ | ------ | -------------------------------- |
| handle                 | void*  | in     | pointer to Controller            |
| return                 | double | -      | attenuation coefficient in Np/mm |

## AUTDSetAttenuation (autd3capi)

減衰係数を設定する.

handleは`AUTDCreateController`で作成したものを使う.

| Argument name / return | type   | in/out | description                      |
| ---------------------- | ------ | ------ | -------------------------------- |
| handle                 | void*  | in     | pointer to Controller            |
| attenuation            | double | in     | attenuation coefficient in Np/mm |
| return                 | void   | -      | -                                |

## AUTDGetFPGAInfo (autd3capi)

FPGAの情報を取得する.

handleは`AUTDCreateController`で作成したものを使う. outポインタが指す領域は, 接続しているデバイスと同じ長さである必要がある.

なお, FPGAの情報は下位1bitが温度センサがアサートされているかどうかを表し, 他のbitは全て0である.

この関数を呼び出す前に`AUTDSetReadsFPGAInfo`でread FPGA info flagをOnにしておく必要がある.

この関数は失敗した場合にfalseを返す. falseの場合には`AUTDGetLastError`でエラーメッセージを取得できる.

| Argument name / return | type     | in/out | description           |
| ---------------------- | -------- | ------ | --------------------- |
| handle                 | void*    | in     | pointer to Controller |
| out                    | uint8_t* | in     | FPGA information list |
| return                 | bool     | -      | true if success       |

## AUTDUpdateFlags (autd3capi)

Control flagを更新する.

send functionの一つ. force fan/reads FPGA info flagを設定した後に呼び出すと, これらの変更が実際に反映される.

handleは`AUTDCreateController`で作成したものを使う.

この関数はエラーが発生した場合に0未満の値を返す. エラーが生じた場合には`AUTDGetLastError`でエラーメッセージを取得できる. また,
check ackフラグがtrue, かつ, 返り値が0より大きい場合は, データが実際のデバイスで処理されたことを保証する.

| Argument name / return | type    | in/out | description                                                                                            |
| ---------------------- | ------- | ------ | ------------------------------------------------------------------------------------------------------ |
| handle                 | void*   | in     | pointer to Controller                                                                                  |
| return                 | int32_t | -      | if $>0$ and check ack flag is true, it guarantees devices have processed data. if $<0$, error ocurred. |

## AUTDNumDevices (autd3capi)

接続されているDeviceの数を取得する.

handleは`AUTDCreateController`で作成したものを使う.

| Argument name / return | type    | in/out | description           |
| ---------------------- | ------- | ------ | --------------------- |
| handle                 | void*   | in     | pointer to Controller |
| return                 | int32_t | -      | number of devices     |

## AUTDTransPosition (autd3capi)

指定した振動子の位置を取得する.

handleは`AUTDCreateController`で作成したものを使う. 振動子の指定はデバイスのインデックスとローカルの振動子インデックスでおこなう.

| Argument name / return | type    | in/out | description                         |
| ---------------------- | ------- | ------ | ----------------------------------- |
| handle                 | void*   | in     | pointer to Controller               |
| device_idx             | int32_t | in     | device index                        |
| local_trans_idx        | int32_t | in     | local transducer index              |
| x                      | double* | out    | x coordinate of transducer position |
| y                      | double* | out    | y coordinate of transducer position |
| z                      | double* | out    | z coordinate of transducer position |
| return                 | void    | -      | -                                   |

## AUTDTransXDirection (autd3capi)

指定した振動子のx軸方向を取得する.

handleは`AUTDCreateController`で作成したものを使う. 振動子の指定はデバイスのインデックスとローカルの振動子インデックスでおこなう.

| Argument name / return | type    | in/out | description                 |
| ---------------------- | ------- | ------ | --------------------------- |
| handle                 | void*   | in     | pointer to Controller       |
| device_idx             | int32_t | in     | device index                |
| local_trans_idx        | int32_t | in     | local transducer index      |
| x                      | double* | out    | x coordinate of x-direction |
| y                      | double* | out    | y coordinate of x-direction |
| z                      | double* | out    | z coordinate of x-direction |
| return                 | void    | -      | -                           |

## AUTDTransYDirection (autd3capi)

指定した振動子のy軸方向を取得する.

handleは`AUTDCreateController`で作成したものを使う. 振動子の指定はデバイスのインデックスとローカルの振動子インデックスでおこなう.

| Argument name / return | type    | in/out | description                 |
| ---------------------- | ------- | ------ | --------------------------- |
| handle                 | void*   | in     | pointer to Controller       |
| device_idx             | int32_t | in     | device index                |
| local_trans_idx        | int32_t | in     | local transducer index      |
| x                      | double* | out    | x coordinate of y-direction |
| y                      | double* | out    | y coordinate of y-direction |
| z                      | double* | out    | z coordinate of y-direction |
| return                 | void    | -      | -                           |

## AUTDTransZDirection (autd3capi)

指定した振動子のz軸方向を取得する.

handleは`AUTDCreateController`で作成したものを使う. 振動子の指定はデバイスのインデックスとローカルの振動子インデックスでおこなう.

| Argument name / return | type    | in/out | description                 |
| ---------------------- | ------- | ------ | --------------------------- |
| handle                 | void*   | in     | pointer to Controller       |
| device_idx             | int32_t | in     | device index                |
| local_trans_idx        | int32_t | in     | local transducer index      |
| x                      | double* | out    | x coordinate of z-direction |
| y                      | double* | out    | y coordinate of z-direction |
| z                      | double* | out    | z coordinate of z-direction |
| return                 | void    | -      | -                           |

## AUTDGetFirmwareInfoListPointer (autd3capi)

Firmware information listへのポインタを取得する.

handleは`AUTDCreateController`で作成したものを使う.

この関数で作成したlistは最後に`AUTDFreeFirmwareInfoListPointer`で開放する必要がある.

実際のFirmware informationは`AUTDGetFirmwareInfo`で取得する.

この関数はエラーが発生した場合に0未満の値を返す. エラーが生じた場合には`AUTDGetLastError`でエラーメッセージを取得できる

| Argument name / return | type    | in/out | description                                                         |
| ---------------------- | ------- | ------ | ------------------------------------------------------------------- |
| handle                 | void*   | in     | pointer to Controller                                               |
| out                    | void**  | out    | pointer to pointer to Firmware information list                     |
| return                 | int32_t | -      | if $<0$ some error occurred, else size of Firmware information list |

## AUTDGetFirmwareInfo (autd3capi)

Firmware informationを取得する.

`p_firm_info_list`は`AUTDGetFirmwareInfoListPointer`で作成したものを使う.

`info`は長さ256のバッファを渡せば十分である.

| Argument name / return | type    | in/out | description                            |
| ---------------------- | ------- | ------ | -------------------------------------- |
| p_firm_info_list       | void*   | in     | pointer to Firmware information list   |
| index                  | int32_t | in     | device index                           |
| info                   | char*   | out    | pointer to firmware information string |
| return                 | void    | -      | -                                      |

## AUTDFreeFirmwareInfoListPointer (autd3capi)

`AUTDGetFirmwareInfoListPointer`で取得したFirmware information listを開放する.

| Argument name / return | type  | in/out | description                          |
| ---------------------- | ----- | ------ | ------------------------------------ |
| p_firm_info_list       | void* | in     | pointer to Firmware information list |
| return                 | void  | -      | -                                    |

## AUTDGainNull (autd3capi)

Null gainを作成する.

作成したGainは最後に`AUTDDeleteGain`で削除する必要がある.

| Argument name / return | type   | in/out | description                     |
| ---------------------- | ------ | ------ | ------------------------------- |
| gain                   | void** | out    | pointer to pointer to Null gain |
| return                 | void   | -      | -                               |

## AUTDGainGrouped (autd3capi)

Grouped gainを作成する.

handleは`AUTDCreateController`で作成したものを使う.

作成したGainは最後に`AUTDDeleteGain`で削除する必要がある.

| Argument name / return | type   | in/out | description                        |
| ---------------------- | ------ | ------ | ---------------------------------- |
| gain                   | void** | out    | pointer to pointer to Grouped gain |
| handle                 | void*  | in     | pointer to Controller              |
| return                 | void   | -      | -                                  |

## AUTDGainGroupedAdd (autd3capi)

Grouped gainにGainを登録する.

`grouped_gain`は`AUTDGainGrouped`で作成したものを使う.

| Argument name / return | type    | in/out | description             |
| ---------------------- | ------- | ------ | ----------------------- |
| grouped_gain           | void*   | in     | pointer to Grouped gain |
| device_id              | int32_t | in     | Device Id               |
| gain                   | void*   | in     | pointer to gain         |
| return                 | void    | -      | -                       |

## AUTDGainFocus (autd3capi)

Focus gainを作成する.

作成したGainは最後に`AUTDDeleteGain`で削除する必要がある.

| Argument name / return | type   | in/out | description                 |
| ---------------------- | ------ | ------ | --------------------------- |
| gain                   | void** | out    | pointer to Focus gain       |
| x                      | double | in     | x coordinate of focal point |
| y                      | double | in     | y coordinate of focal point |
| z                      | double | in     | z coordinate of focal point |
| amp                    | double | in     | amplitude of focus          |
| return                 | void   | -      | -                           |

## AUTDGainBesselBeam (autd3capi)

Bessel beam gainを作成する.

作成したGainは最後に`AUTDDeleteGain`で削除する必要がある.

| Argument name / return | type   | in/out | description                    |
| ---------------------- | ------ | ------ | ------------------------------ |
| gain                   | void** | out    | pointer to Bessel beam gain    |
| x                      | double | in     | x coordinate of apex           |
| y                      | double | in     | y coordinate of apex           |
| z                      | double | in     | z coordinate of apex           |
| n_x                    | double | in     | x coordinate of beam direction |
| n_y                    | double | in     | y coordinate of beam direction |
| n_z                    | double | in     | z coordinate of beam direction |
| theta_z                | double | in     | tilt angle of beam             |
| amp                    | double | in     | amplitude of beam              |
| return                 | void   | -      | -                              |

## AUTDGainPlaneWave (autd3capi)

Plane wave gainを作成する.

作成したGainは最後に`AUTDDeleteGain`で削除する必要がある.

| Argument name / return | type   | in/out | description                    |
| ---------------------- | ------ | ------ | ------------------------------ |
| gain                   | void** | out    | pointer to Plane wave gain     |
| n_x                    | double | in     | x coordinate of wave direction |
| n_y                    | double | in     | y coordinate of wave direction |
| n_z                    | double | in     | z coordinate of wave direction |
| amp                    | double | in     | amplitude of wave              |
| return                 | void   | -      | -                              |

## AUTDGainCustom (autd3capi)

Custom gainを作成する.

Custom gainは位相と振幅を直接指定するGainである.

作成したGainは最後に`AUTDDeleteGain`で削除する必要がある.

| Argument name / return | type     | in/out | description           |
| ---------------------- | -------  | ------ | --------------------- |
| gain                   | void**   | out    | pointer to Focus gain |
| amp                    | double*  | in     | pointer to amplitude  |
| phase                  | double*  | in     | pointer to phase      |
| size                   | uint64_t | in     | size of amp and phase |
| return                 | void     | -      | -                     |

## AUTDDeleteGain (autd3capi)

作成したGainを削除する.

| Argument name / return | type  | in/out | description     |
| ---------------------- | ----- | ------ | --------------- |
| gain                   | void* | in     | pointer to gain |
| return                 | void  | -      | -               |

## AUTDModulationStatic (autd3capi)

Static modulationを作成する.

作成したModulationは最後に`AUTDDeleteModulation`で削除する必要がある.

| Argument name / return | type   | in/out | description                             |
| ---------------------- | ------ | ------ | --------------------------------------- |
| mod                    | void** | out    | pointer to pointer to Static modulation |
| amp                    | double | in     | amplitude of modulation                 |
| return                 | void   | -      | -                                       |

## AUTDModulationSine (autd3capi)

Sine modulationを作成する.

作成したModulationは最後に`AUTDDeleteModulation`で削除する必要がある.

| Argument name / return | type    | in/out | description                           |
| ---------------------- | ------- | ------ | ------------------------------------- |
| mod                    | void**  | out    | pointer to pointer to Sine modulation |
| freq                   | int32_t | in     | frequency of sine modulation          |
| amp                    | double  | in     | amplitude of sine modulation          |
| offset                 | double  | in     | offset of sine modulation             |
| return                 | void    | -      | -                                     |

## AUTDModulationSineSquared (autd3capi)

SineSquared modulationを作成する.

作成したModulationは最後に`AUTDDeleteModulation`で削除する必要がある.

| Argument name / return | type    | in/out | description                                  |
| ---------------------- | ------- | ------ | -------------------------------------------- |
| mod                    | void**  | out    | pointer to pointer to SineSquared modulation |
| freq                   | int32_t | in     | frequency of sine modulation                 |
| amp                    | double  | in     | amplitude of sine modulation                 |
| offset                 | double  | in     | offset of sine modulation                    |
| return                 | void    | -      | -                                            |

## AUTDModulationSineLegacy (autd3capi)

SineLegacy modulationを作成する.

作成したModulationは最後に`AUTDDeleteModulation`で削除する必要がある.

| Argument name / return | type   | in/out | description                                 |
| ---------------------- | ------ | ------ | ------------------------------------------- |
| mod                    | void** | out    | pointer to pointer to SineLegacy modulation |
| freq                   | double | in     | frequency of sine modulation                |
| amp                    | double | in     | amplitude of sine modulation                |
| offset                 | double | in     | offset of sine modulation                   |
| return                 | void   | -      | -                                           |

## AUTDModulationSquare (autd3capi)

Square modulationを作成する.

作成したModulationは最後に`AUTDDeleteModulation`で削除する必要がある.

| Argument name / return | type    | in/out | description                                  |
| ---------------------- | ------- | ------ | -------------------------------------------- |
| mod                    | void**  | out    | pointer to pointer to Square modulation      |
| freq                   | int32_t | in     | frequency of square modulation               |
| low                    | double  | in     | amplitude at low level of square modulation  |
| high                   | double  | in     | amplitude at high level of square modulation |
| duty                   | double  | in     | duty ratio of square modulation              |
| return                 | void    | -      | -                                            |

## AUTDModulationCustom (autd3capi)

Custom modulationを作成する.

Custom modulationは振幅を直接指定するModulationである.

作成したModulationは最後に`AUTDDeleteModulation`で削除する必要がある.

| Argument name / return | type     | in/out | description                             |
| ---------------------- | -------- | ------ | --------------------------------------- |
| mod                    | void**   | out    | pointer to pointer to Custom modulation |
| buffer                 | uint8_t* | in     | pointer to modulation data              |
| size                   | uint64_t | in     | size of buffer                          |
| freq_div               | uint32_t | in     | modulation sampling frequency division  |
| return                 | void     | -      | -                                       |

## AUTDModulationSamplingFrequencyDivision (autd3capi)

Modulation sampling frequency divisionを返す.

| Argument name / return | type     | in/out | description                            |
| ---------------------- | -------- | ------ | -------------------------------------- |
| mod                    | void*    | in     | pointer to modulation                  |
| return                 | uint32_t | -      | modulation sampling frequency division |

## AUTDModulationSetSamplingFrequencyDivision (autd3capi)

Modulation sampling frequency divisionを設定する.

| Argument name / return | type     | in/out | description                            |
| ---------------------- | -------- | ------ | -------------------------------------- |
| mod                    | void*    | in     | pointer to modulation                  |
| freq_div               | uint32_t | in     | modulation sampling frequency division |
| return                 | void     | -      | -                                      |

## AUTDModulationSamplingFrequency (autd3capi)

Sampling frequencyを返す.

| Argument name / return | type   | in/out | description                   |
| ---------------------- | ------ | ------ | ----------------------------- |
| mod                    | void*  | in     | pointer to modulation         |
| return                 | double | -      | modulation sampling frequency |

## AUTDDeleteModulation (autd3capi)

Modulationを削除する.

| Argument name / return | type  | in/out | description           |
| ---------------------- | ----- | ------ | --------------------- |
| mod                    | void* | in     | pointer to modulation |
| return                 | void  | -      | -                     |

## AUTDPointSTM (autd3capi)

Point STMを作成する.

作成したSTMは最後に`AUTDDeleteSTM`で削除する必要がある.

| Argument name / return | type   | in/out | description                     |
| ---------------------- | ------ | ------ | ------------------------------- |
| out                    | void** | out    | pointer to pointer to Point STM |
| return                 | void   | -      | -                               |

## AUTDGainSTM (autd3capi)

Gain STMを作成する.

handleは`AUTDCreateController`で作成したものを使う.

作成したSTMは最後に`AUTDDeleteSTM`で削除する必要がある.

| Argument name / return | type   | in/out | description                    |
| ---------------------- | ------ | ------ | ------------------------------ |
| out                    | void** | out    | pointer to pointer to Gain STM |
| handle                 | void*  | in     | pointer to Controller          |
| return                 | void   | -      | -                              |

## AUTDPointSTMAdd (autd3capi)

Point STMに焦点を追加する.

| Argument name / return | type    | in/out | description                 |
| ---------------------- | ------- | ------ | --------------------------- |
| stm                    | void*   | in     | pointer to Point STM        |
| x                      | double  | in     | x coordinate of focal point |
| y                      | double  | in     | y coordinate of focal point |
| z                      | double  | in     | z coordinate of focal point |
| shift                  | uint8_t | in     | duty shift                  |
| return                 | bool    | -      | true if success             |

## AUTDGainSTMAdd (autd3capi)

Gain STMにgainを追加する.

| Argument name / return | type  | in/out | description          |
| ---------------------- | ----- | ------ | -------------------- |
| stm                    | void* | in     | pointer to Point STM |
| gain                   | void* | in     | pointer to Gain      |
| return                 | bool  | -      | true if success      |

## AUTDSetGainSTMMode (autd3capi)

GainSTMのmodeを設定する.

| Argument name / return | type     | in/out | description                                                                   |
| ---------------------- | -------- | ------ | ----------------------------------------------------------------------------- |
| stm                    | void*    | in     | pointer to STM                                                                |
| mode                   | uint16_t | in     | GainSTM mode (0x0001 = PhaseDutyFull, 0x0002 = PhaseFull, 0x0004 = PhaseHalf) |
| return                 | void     | -      | -                                                                             |

## AUTDGetGainSTMMode (autd3capi)

GainSTMのmodeを取得する.

| Argument name / return | type     | in/out | description    |
| ---------------------- | -------- | ------ | -------------- |
| stm                    | void*    | in     | pointer to STM |
| return                 | uint16_t | -      | GainSTM mode   |

## AUTDSTMSetFrequency (autd3capi)

STMのfrequencyを設定する.

| Argument name / return | type   | in/out | description      |
| ---------------------- | ------ | ------ | ---------------- |
| stm                    | void*  | in     | pointer to STM   |
| freq                   | double | in     | frequency of STM |
| return                 | double | -      | actual frequency |

## AUTDSTMFrequency (autd3capi)

STMのfrequencyを取得する.

| Argument name / return | type   | in/out | description      |
| ---------------------- | ------ | ------ | ---------------- |
| stm                    | void*  | in     | pointer to STM   |
| return                 | double | -      | frequency of STM |

## AUTDSTMSamplingFrequency (autd3capi)

STMのsampling frequencyを取得する.

| Argument name / return | type   | in/out | description               |
| ---------------------- | ------ | ------ | ------------------------- |
| stm                    | void*  | in     | pointer to STM            |
| return                 | double | -      | sampling frequency of STM |

## AUTDSTMSamplingFrequencyDivision (autd3capi)

STMのsampling frequency divisionを取得する.

| Argument name / return | type     | in/out | description                     |
| ---------------------- | -------- | ------ | ------------------------------- |
| stm                    | void*    | in     | pointer to STM                  |
| return                 | uint32_t | in     | STM sampling frequency division |

## AUTDSTMSetSamplingFrequencyDivision (autd3capi)

STMのsampling frequency divisionを設定する.

| Argument name / return | type     | in/out | description                     |
| ---------------------- | -------- | ------ | ------------------------------- |
| stm                    | void*    | in     | pointer to STM                  |
| freq_div               | uint32_t | in     | STM sampling frequency division |
| return                 | void     | -      | -                               |

## AUTDDeleteSTM (autd3capi)

STMを削除する.

| Argument name / return | type  | in/out | description    |
| ---------------------- | ----- | ------ | -------------- |
| stm                    | void* | in     | pointer to STM |
| return                 | void  | -      | -              |

## AUTDStop (autd3capi)

AUTDの出力を停止する.

handleは`AUTDCreateController`で作成したものを使う.

この関数はエラーが発生した場合に0未満の値を返す. エラーが生じた場合には`AUTDGetLastError`でエラーメッセージを取得できる. また,
check ackフラグがtrue, かつ, 返り値が0より大きい場合は, データが実際のデバイスで処理されたことを保証する.

| Argument name / return | type    | in/out | description                                                                                            |
| ---------------------- | ------- | ------ | ------------------------------------------------------------------------------------------------------ |
| handle                 | void*   | in     | pointer to Controller                                                                                  |
| return                 | int32_t | -      | if $>0$ and check ack flag is true, it guarantees devices have processed data. if $<0$, error ocurred. |

## AUTDSend (autd3capi)

ヘッダーデータとボディーデータを送信する.

send functionの一つ.

handleは`AUTDCreateController`で作成したものを使う.

この関数はエラーが発生した場合に0未満の値を返す. エラーが生じた場合には`AUTDGetLastError`でエラーメッセージを取得できる. また,
check ackフラグがtrue, かつ, 返り値が0より大きい場合は, データが実際のデバイスで処理されたことを保証する.

| Argument name / return | type    | in/out | description                                                                                            |
| ---------------------- | ------- | ------ | ------------------------------------------------------------------------------------------------------ |
| handle                 | void*   | in     | pointer to Controller                                                                                  |
| header                 | void*   | in     | pointer to header data                                                                                 |
| body                   | void*   | in     | pointer to body data                                                                                   |
| return                 | int32_t | -      | if $>0$ and check ack flag is true, it guarantees devices have processed data. if $<0$, error ocurred. |

## AUTDSetModDelay (autd3capi)

指定した振動子のModulation Delayを設定する.

handleは`AUTDCreateController`で作成したものを使う. 振動子の指定はデバイスのインデックスとローカルの振動子インデックスでおこなう.

| Argument name / return | type     | in/out | description                        |
| ---------------------- | -------- | ------ | ---------------------------------- |
| handle                 | void*    | in     | pointer to Controller              |
| device_idx             | int32_t  | in     | device index                       |
| local_trans_idx        | int32_t  | in     | local transducer index             |
| delay                  | uint16_t | in     | modulation delay of the transducer |
| return                 | void     | -      | -                                  |

## AUTDCreateModDelayConfig (autd3capi)

ModDelayConfigを作成する.

作成したSilencerConfigは最後に`AUTDDeleteModDelayConfig`で削除する必要がある.

| Argument name / return | type   | in/out | description                          |
| ---------------------- | ------ | ------ | ------------------------------------ |
| out                    | void** | out    | pointer to pointer to ModDelayConfig |
| return                 | void   | -      | -                                    |

## AUTDDeleteModDelayConfig (autd3capi)

ModDelayConfigを削除する.

| Argument name / return | type  | in/out | description               |
| ---------------------- | ----- | ------ | ------------------------- |
| config                 | void* | in     | pointer to ModDelayConfig |
| return                 | void  | -      | -                         |

## AUTDCreateAmplitudes (autd3capi)

Amplitudesを作成する.

作成したSilencerConfigは最後に`AUTDDeleteAmplitudes`で削除する必要がある.

| Argument name / return | type   | in/out | description                      |
| ---------------------- | ------ | ------ | -------------------------------- |
| out                    | void** | out    | pointer to pointer to Amplitudes |
| amp                    | double | in     | amplitudes                       |
| return                 | void   | -      | -                                |

## AUTDDeleteAmplitudes (autd3capi)

Amplitudesを削除する.

| Argument name / return | type  | in/out | description           |
| ---------------------- | ----- | ------ | --------------------- |
| amplitudes             | void* | in     | pointer to Amplitudes |
| return                 | void  | -      | -                     |

## AUTDSetMode (autd3capi)

Legacy/Normalモードの設定を行う.

| Argument name / return | type    | in/out | description                                                 |
| ---------------------- | ------- | ------ | ----------------------------------------------------------- |
| handle                 | void*   | in     | pointer to Controller                                       |
| mode                   | uint8_t | in     | mode (0: Legacy mode, 1: Normal mode, 2: Normal Phase mode) |
| return                 | void    | -      | -                                                           |

## AUTDEigenBackend (autd3capi-gain-holo)

Eigen Backendを作成する.

作成したBackendは最後に`AUTDDeleteBackend`で削除する必要がある.

| Argument name / return | type   | in/out | description                         |
| ---------------------- | ------ | ------ | ----------------------------------- |
| out                    | void** | out    | pointer to pointer to Eigen backend |
| return                 | void   | -      | -                                   |

## AUTDDeleteBackend (autd3capi-gain-holo)

Backendを作成する.

| Argument name / return | type  | in/out | description        |
| ---------------------- | ----- | ------ | ------------------ |
| backend                | void* | in     | pointer to backend |
| return                 | void  | -      | -                  |

## AUTDGainHoloSDP (autd3capi-gain-holo)

SDP holo gainを作成する.

作成したGainは最後に`AUTDDeleteGain`で削除する必要がある.

| Argument name / return | type     | in/out | description                    |
| ---------------------- | -------- | ------ | ------------------------------ |
| gain                   | void**   | out    | pointer to pointer to SDP gain |
| backend                | void*    | in     | pointer to backend             |
| alpha                  | double   | in     | parameter                      |
| lambda                 | double   | in     | parameter                      |
| repeat                 | uint64_t | in     | parameter                      |
| return                 | void     | -      | -                              |

## AUTDGainHoloEVD (autd3capi-gain-holo)

EVD holo gainを作成する.

作成したGainは最後に`AUTDDeleteGain`で削除する必要がある.

| Argument name / return | type   | in/out | description                    |
| ---------------------- | ------ | ------ | ------------------------------ |
| gain                   | void** | out    | pointer to pointer to EVD gain |
| backend                | void*  | in     | pointer to backend             |
| gamma                  | double | in     | parameter                      |
| return                 | void   | -      | -                              |

## AUTDGainHoloNaive (autd3capi-gain-holo)

Naive holo gainを作成する.

作成したGainは最後に`AUTDDeleteGain`で削除する必要がある.

| Argument name / return | type   | in/out | description                      |
| ---------------------- | ------ | ------ | -------------------------------- |
| gain                   | void** | out    | pointer to pointer to Naive gain |
| backend                | void*  | in     | pointer to backend               |
| return                 | void   | -      | -                                |

## AUTDGainHoloGS (autd3capi-gain-holo)

GS holo gainを作成する.

作成したGainは最後に`AUTDDeleteGain`で削除する必要がある.

| Argument name / return | type     | in/out | description                   |
| ---------------------- | -------- | ------ | ----------------------------- |
| gain                   | void**   | out    | pointer to pointer to GS gain |
| backend                | void*    | in     | pointer to backend            |
| repeat                 | uint64_t | in     | parameter                     |
| return                 | void     | -      | -                             |

## AUTDGainHoloGSPAT (autd3capi-gain-holo)

GSPAT holo gainを作成する.

作成したGainは最後に`AUTDDeleteGain`で削除する必要がある.

| Argument name / return | type     | in/out | description                      |
| ---------------------- | -------- | ------ | -------------------------------- |
| gain                   | void**   | out    | pointer to pointer to GSPAT gain |
| backend                | void*    | in     | pointer to backend               |
| repeat                 | uint64_t | in     | parameter                        |
| return                 | void     | -      | -                                |

## AUTDGainHoloLM (autd3capi-gain-holo)

LM holo gainを作成する.

作成したGainは最後に`AUTDDeleteGain`で削除する必要がある.

| Argument name / return | type     | in/out | description                   |
| ---------------------- | -------- | ------ | ----------------------------- |
| gain                   | void**   | out    | pointer to pointer to LM gain |
| backend                | void*    | in     | pointer to backend            |
| eps_1                  | double   | in     | parameter                     |
| eps_2                  | double   | in     | parameter                     |
| tau                    | double   | in     | parameter                     |
| k_max                  | uint64_t | in     | parameter                     |
| initial                | double*  | in     | initial guess                 |
| initial_size           | int32_t  | in     | size of initial               |
| return                 | void     | -      | -                             |

## AUTDGainHoloGreedy (autd3capi-gain-holo)

Greedy holo gainを作成する.

作成したGainは最後に`AUTDDeleteGain`で削除する必要がある.

| Argument name / return | type    | in/out | description                       |
| ---------------------- | ------- | ------ | --------------------------------- |
| gain                   | void**  | out    | pointer to pointer to Greedy gain |
| backend                | void*   | in     | pointer to backend                |
| phase_div              | int32_t | in     | parameter                         |
| return                 | void    | -      | -                                 |


## AUTDGainHoloLSSGreedy (autd3capi-gain-holo)

LSSGreedy holo gainを作成する.

作成したGainは最後に`AUTDDeleteGain`で削除する必要がある.

| Argument name / return | type    | in/out | description                          |
| ---------------------- | ------- | ------ | ---------------------------------    |
| gain                   | void**  | out    | pointer to pointer to LSSGreedy gain |
| backend                | void*   | in     | pointer to backend                   |
| phase_div              | int32_t | in     | parameter                            |
| return                 | void    | -      | -                                    |

## AUTDGainHoloAPO (autd3capi-gain-holo)

APO holo gainを作成する.

作成したGainは最後に`AUTDDeleteGain`で削除する必要がある.

| Argument name / return | type    | in/out | description                          |
| ---------------------- | ------- | ------ | ---------------------------------    |
| gain                   | void**  | out    | pointer to pointer to APO gain       |
| backend                | void*   | in     | pointer to backend                   |
| eps                    | double  | in     | parameter                            |
| lambda                 | double  | in     | parameter                            |
| k_max                  | int32_t | in     | parameter                            |
| line_search_max        | int32_t | in     | parameter                            |
| return                 | void    | -      | -                                    |

## AUTDGainHoloAdd (autd3capi-gain-holo)

Holo gainに焦点を追加する.

| Argument name / return | type   | in/out | description               |
| ---------------------- | ------ | ------ | ------------------------- |
| gain                   | void*  | in     | pointer to holo gain      |
| x                      | double | in     | x coordinate of the focus |
| y                      | double | in     | y coordinate of the focus |
| z                      | double | in     | z coordinate of the focus |
| amp                    | double | in     | amplitude of the focus    |
| return                 | void   | -      | -                         |

## AUTDSetConstraint (autd3capi-gain-holo)

Holo gainのAmplitudeConstraintを設定する.

| Argument name / return | type    | in/out | description                                                           |
| ---------------------- | ------- | ------ | --------------------------------------------------------------------- |
| gain                   | void*   | in     | pointer to holo gain                                                  |
| type                   | int32_t | in     | AmplitudeConstraint (0: DontCare, 1: Normalize, 2: Uniform, 3: Clamp) |
| param                  | void*   | in     | pointer to additional parameter                                       |
| return                 | void    | -      | -                                                                     |

## AUTDBLASBackend (autd3capi-backend-blas)

BLAS Backendを作成する.

作成したBackendは最後に`AUTDDeleteBackend`で削除する必要がある.

| Argument name / return | type   | in/out | description                        |
| ---------------------- | ------ | ------ | ---------------------------------- |
| out                    | void** | out    | pointer to pointer to BLAS backend |
| return                 | void   | -      | -                                  |

## AUTDCUDABackend (autd3capi-backend-cuda)

CUDA Backendを作成する.

作成したBackendは最後に`AUTDDeleteBackend`で削除する必要がある.

| Argument name / return | type   | in/out | description                        |
| ---------------------- | ------ | ------ | ---------------------------------- |
| out                    | void** | out    | pointer to pointer to CUDA backend |
| return                 | void   | -      | -                                  |

## AUTDLinkEmulator (autd3capi-link-emulator)

Emulator linkを作成する.

| Argument name / return | type     | in/out | description                         |
| ---------------------- | -------- | ------ | ----------------------------------- |
| out                    | void**   | out    | pointer to pointer to Emulator link |
| port                   | uint16_t | in     | port number                         |
| cnt                    | void*    | in     | pointer to Controller               |
| return                 | void     | -      | -                                   |

## AUTDLinkRemoteTwinCAT (autd3capi-link-remote-twincat)

RemoteTwinCAT linkを作成する.

| Argument name / return | type   | in/out | description                              |
| ---------------------- | ------ | ------ | ---------------------------------------- |
| out                    | void** | out    | pointer to pointer to RemoteTwinCAT link |
| remote_ip_addr         | char*  | in     | remote ip address                        |
| remote_ams_net_id      | char*  | in     | remote ams net id                        |
| local_ams_net_id       | char*  | in     | local ams net id                         |
| return                 | void   | -      | -                                        |

## AUTDGetAdapterPointer (autd3capi-link-soem)

Adapter listへのポインタを取得する.

この関数で作成したlistは最後に`AUTDFreeAdapterPointer`で開放する必要がある.

実際のAdapterの情報は`AUTDGetAdapter`で取得する.

この関数はAdapter listのサイズを返す.

| Argument name / return | type    | in/out | description                        |
| ---------------------- | ------- | ------ | ---------------------------------- |
| out                    | void**  | out    | pointer to pointer to adapter list |
| return                 | int32_t | -      | size of adapter list               |

## AUTDGetAdapter (autd3capi-link-soem)

アダプターの名前と説明を取得する.

`p_adapter`は`AUTDGetAdapterPointer`で作成したものを使う.

`desc`, `name`はそれぞれ長さ128のバッファを渡せば十分である.

| Argument name / return | type    | in/out | description                    |
| ---------------------- | ------- | ------ | ------------------------------ |
| p_adapter              | void*   | in     | pointer to adapter list        |
| index                  | int32_t | in     | index                          |
| desc                   | char*   | out    | pointer to adapter description |
| name                   | char*   | out    | pointer to adapter name        |
| return                 | void    | -      | -                              |

## AUTDFreeAdapterPointer (autd3capi-link-soem)

Adapter listへのポインタを削除する.

| Argument name / return | type  | in/out | description             |
| ---------------------- | ----- | ------ | ----------------------- |
| p_adapter              | void* | in     | pointer to adapter list |
| return                 | void  | -      | size of adapter list    |

## AUTDLinkSOEM (autd3capi-link-soem)

| Argument name / return | type     | in/out | description                        |
| ---------------------- | -------- | ------ | ---------------------------------- |
| out                    | void**   | out    | pointer to pointer to TwinCAT link |
| ifname                 | char*    | in     | interface name                     |
| device_num             | int32_t  | in     | number of devices                  |
| sync0_cycle            | uint16_t | in     | sync0 cycle                        |
| send_cycle             | uint16_t | in     | send cycle                         |
| freerun                | bool     | in     | free run mode                      |
| on_lost                | void*    | in     | pointer to on-lost callback        |
| high_precision         | bool     | in     | high precision mode                |
| return                 | void     | -      | -                                  |

## AUTDLinkTwinCAT (autd3capi-link-twincat)

TwinCAT linkを作成する.

| Argument name / return | type   | in/out | description                        |
| ---------------------- | ------ | ------ | ---------------------------------- |
| out                    | void** | out    | pointer to pointer to TwinCAT link |
| return                 | void   | -      | -                                  |

## AUTDModulationRawPCM (autd3capi-modulation-audio-file)

RawPCM modulationを作成する.

作成したModulationは最後に`AUTDDeleteModulation`で削除する必要がある.

| Argument name / return | type     | in/out | description                            |
| ---------------------- | -------- | ------ | -------------------------------------- |
| mod                    | void**   | out    | pointer to pointer to Sine modulation  |
| filename               | char*    | in     | path to pcm file                       |
| sampling_freq          | double   | in     | pcm sampling frequency                 |
| mod_sampling_freq_div  | uint32_t | in     | modulation sampling frequency division |
| return                 | void     | -      | -                                      |

## AUTDModulationWav (autd3capi-modulation-audio-file)

Wav modulationを作成する.

作成したModulationは最後に`AUTDDeleteModulation`で削除する必要がある.

| Argument name / return | type     | in/out | description                            |
| ---------------------- | -------- | ------ | -------------------------------------- |
| mod                    | void**   | out    | pointer to pointer to Sine modulation  |
| filename               | char*    | in     | path to pcm file                       |
| mod_sampling_freq_div  | uint32_t | in     | modulation sampling frequency division |
| return                 | void     | -      | -                                      |
