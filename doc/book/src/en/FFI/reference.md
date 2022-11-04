# C API Reference

The API for the C language is defined under [capi](https://github.com/shinolab/autd3/tree/master/capi).

The following is a reference to this API. 
For actual usage, please refer to [C API Example](https://github.com/shinolab/autd3/tree/master/capi/example).

## AUTDGetLastError (autd3capi)

Get the error message of the last exception.

The argument is a pointer to the error message. 
The error message is copied to this pointer. However, if the argument is nullptr, no copying is done.
The function returns the size of the error message including null terminations.

Since the length of the error message is variable, you should reserve a large enough area or pass nullptr to error to get the required size and call the function again.

| Argument name / return | type    | in/out | description                                       |
| ---------------------- | ------- | ------ | ------------------------------------------------- |
| error                  | char*   | out    | pointer to error message                          |
| return                 | int32_t | -      | length of error message including null terminator |

## AUTDCreateController (autd3capi)

Create `Controller`.

The controller created must be freed at the end by `AUTDFreeController`.

| Argument name / return | type   | in/out | description                      |
| ---------------------- | ------ | ------ | -------------------------------- |
| out                    | void** | out    | pointer to pointer to Controller |
| return                 | void   | -      | -                                |

## AUTDOpenController (autd3capi)

Open `Controller`.

This function returns false if failure, and you can get the error message with `AUTDGetLastError`.

| Argument name / return | type  | in/out | description           |
| ---------------------- | ----- | ------ | --------------------- |
| handle                 | void* | in     | pointer to Controller |
| link                   | void* | in     | pointer to Link       |
| return                 | bool  | -      | true if success       |

## AUTDAddDevice (autd3capi)

Add a device to the Controller.

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

Add a device to the Controller.

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

Close Controller.

This function returns a value less than zero if an error occurred. If an error occurs, you can get the error message with `AUTDGetLastError`.

| Argument name / return | type    | in/out | description                                                                                            |
| ---------------------- | ------- | ------ | ------------------------------------------------------------------------------------------------------ |
| handle                 | void*   | in     | pointer to Controller                                                                                  |
| return                 | int32_t | -      | if $>0$ and check ack flag is true, it guarantees devices have processed data. if $<0$, error ocurred. |

## AUTDClear (autd3capi)

Clear devices.

This function returns a value less than zero if an error occurred. If an error occurs, you can get the error message with `AUTDGetLastError`.

| Argument name / return | type    | in/out | description                                                                                            |
| ---------------------- | ------- | ------ | ------------------------------------------------------------------------------------------------------ |
| handle                 | void*   | in     | pointer to Controller                                                                                  |
| return                 | int32_t | -      | if $>0$ and check ack flag is true, it guarantees devices have processed data. if $<0$, error ocurred. |

## AUTDSynchronize (autd3capi)

Synchronize devices.

This function returns a value less than zero if an error occurred. If an error occurs, you can get the error message with `AUTDGetLastError`.

| Argument name / return | type    | in/out | description                                                                                            |
| ---------------------- | ------- | ------ | ------------------------------------------------------------------------------------------------------ |
| handle                 | void*   | in     | pointer to Controller                                                                                  |
| return                 | int32_t | -      | if $>0$ and check ack flag is true, it guarantees devices have processed data. if $<0$, error ocurred. |

## AUTDCreateSilencer (autd3capi)

Create SilencerConfig.

The config created must be deleted at the end by `AUTDDeleteSilencer`.

| Argument name / return | type     | in/out | description                          |
| ---------------------- | -------- | ------ | ------------------------------------ |
| out                    | void**   | out    | pointer to pointer to SilencerConfig |
| step                   | uint16_t | in     | silencer update step                 |
| cycle                  | uint16_t | in     | silencer update cycle                |
| return                 | void     | -      | -                                    |

## AUTDDeleteSilencer (autd3capi)

Delete SilencerConfig.

| Argument name / return | type  | in/out | description               |
| ---------------------- | ----- | ------ | ------------------------- |
| config                 | void* | in     | pointer to SilencerConfig |
| return                 | void  | -      | -                         |

## AUTDFreeController (autd3capi)

Delete Controller.

| Argument name / return | type  | in/out | description           |
| ---------------------- | ----- | ------ | --------------------- |
| handle                 | void* | in     | pointer to Controller |
| return                 | void  | -      | -                     |

## AUTDIsOpen (autd3capi)

Return if the controller is opened.

| Argument name / return | type  | in/out | description                |
| ---------------------- | ----- | ------ | -------------------------- |
| handle                 | void* | in     | pointer to Controller      |
| return                 | bool  | -      | true if controller is open |

## AUTDGetForceFan (autd3capi)

Get Force fan flag.

| Argument name / return | type  | in/out | description           |
| ---------------------- | ----- | ------ | --------------------- |
| handle                 | void* | in     | pointer to Controller |
| return                 | bool  | -      | Force fan flag        |

## AUTDGetReadsFPGAInfo (autd3capi)

Get Reads FPGA info flag

| Argument name / return | type  | in/out | description           |
| ---------------------- | ----- | ------ | --------------------- |
| handle                 | void* | in     | pointer to Controller |
| return                 | bool  | -      | Reads FPGA info flag  |

## AUTDGetCheckTrials (autd3capi)

Get Check trials.

| Argument name / return | type    | in/out | description           |
| ---------------------- | ------- | ------ | --------------------- |
| handle                 | void*   | in     | pointer to Controller |
| return                 | int32_t | -      | Check trials          |

## AUTDGetSendInterval (autd3capi)

Get Send interval.

| Argument name / return | type    | in/out | description           |
| ---------------------- | ------- | ------ | --------------------- |
| handle                 | void*   | in     | pointer to Controller |
| return                 | int32_t | -      | Send interval         |

## AUTDSetReadsFPGAInfo (autd3capi)

Set Reads FPGA info flag.

| Argument name / return | type  | in/out | description           |
| ---------------------- | ----- | ------ | --------------------- |
| handle                 | void* | in     | pointer to Controller |
| reads_fpga_info        | bool  | in     | read FPGA info flag   |
| return                 | void  | -      | -                     |

## AUTDSetCheckTrials (autd3capi)

Set Check trials.

| Argument name / return | type    | in/out | description           |
| ---------------------- | ------- | ------ | --------------------- |
| handle                 | void*   | in     | pointer to Controller |
| trials                 | int32_t | in     | check trials          |
| return                 | void    | -      | -                     |

## AUTDSetSendInterval (autd3capi)

Set Send interval.

| Argument name / return | type    | in/out | description           |
| ---------------------- | ------- | ------ | --------------------- |
| handle                 | void*   | in     | pointer to Controller |
| interval               | int32_t | in     | Send interval         |
| return                 | void    | -      | -                     |

## AUTDSetForceFan (autd3capi)

Set Force fan flag.

| Argument name / return | type  | in/out | description           |
| ---------------------- | ----- | ------ | --------------------- |
| handle                 | void* | in     | pointer to Controller |
| force                  | bool  | in     | force fan flag        |
| return                 | void  | -      | -                     |

## AUTDGetSoundSpeed (autd3capi)

Get sound speed.

| Argument name / return | type   | in/out | description           |
| ---------------------- | ------ | ------ | --------------------- |
| handle                 | void*  | in     | pointer to Controller |
| return                 | double | -      | Speed of sound in m/s |

## AUTDSetSoundSpeed (autd3capi)

Set sound speed.

| Argument name / return | type   | in/out | description           |
| ---------------------- | ------ | ------ | --------------------- |
| handle                 | void*  | in     | pointer to Controller |
| sound_speed            | double | in     | Speed of sound in m/s |
| return                 | void   | -      | -                     |

## AUTDGetTransFrequency (autd3capi)

Get frequency of the transducer.

| Argument name / return | type    | in/out | description                 |
| ---------------------- | ------- | ------ | --------------------------- |
| handle                 | void*   | in     | pointer to Controller       |
| device_idx             | int32_t | in     | device index                |
| local_trans_idx        | int32_t | in     | local transducer index      |
| return                 | double  | -      | frequency of the transducer |

## AUTDSetTransFrequency (autd3capi)

Set frequency of the transducer.

| Argument name / return | type    | in/out | description                 |
| ---------------------- | ------- | ------ | --------------------------- |
| handle                 | void*   | in     | pointer to Controller       |
| device_idx             | int32_t | in     | device index                |
| local_trans_idx        | int32_t | in     | local transducer index      |
| frequency              | double  | in     | frequency of the transducer |
| return                 | void    | -      | -                           |

## AUTDGetTransCycle (autd3capi)

Get cycle of the transducer.

| Argument name / return | type     | in/out | description             |
| ---------------------- | -------- | ------ | ----------------------- |
| handle                 | void*    | in     | pointer to Controller   |
| device_idx             | int32_t  | in     | device index            |
| local_trans_idx        | int32_t  | in     | local transducer index  |
| return                 | uint16_t | -      | cycle of the transducer |

## AUTDSetTransCycle (autd3capi)

Set cycle of the transducer.

| Argument name / return | type     | in/out | description             |
| ---------------------- | -------- | ------ | ----------------------- |
| handle                 | void*    | in     | pointer to Controller   |
| device_idx             | int32_t  | in     | device index            |
| local_trans_idx        | int32_t  | in     | local transducer index  |
| cycle                  | uint16_t | in     | cycle of the transducer |
| return                 | void     | -      | -                       |

## AUTDGetWavelength (autd3capi)

Get wavelength of the transducer.

| Argument name / return | type    | in/out | description                                          |
| ---------------------- | ------- | ------ | ---------------------------------------------------- |
| handle                 | void*   | in     | pointer to Controller                                |
| device_idx             | int32_t | in     | device index                                         |
| local_trans_idx        | int32_t | in     | local transducer index                               |
| return                 | double  | -      | wavelength of ultrasound emitted from the transducer |

## AUTDGetAttenuation (autd3capi)

Get attenuation coefficient.

| Argument name / return | type   | in/out | description                      |
| ---------------------- | ------ | ------ | -------------------------------- |
| handle                 | void*  | in     | pointer to Controller            |
| return                 | double | -      | attenuation coefficient in Np/mm |

## AUTDSetAttenuation (autd3capi)

Set attenuation coefficient.

| Argument name / return | type   | in/out | description                      |
| ---------------------- | ------ | ------ | -------------------------------- |
| handle                 | void*  | in     | pointer to Controller            |
| attenuation            | double | in     | attenuation coefficient in Np/mm |
| return                 | void   | -      | -                                |

## AUTDGetFPGAInfo (autd3capi)

Get FPGA information.

Make sure set read FPGA info flag by `AUTDSetReadsFPGAInfo` before calling this function,

This function returns false if failure, and you can get the error message with `AUTDGetLastError`.

| Argument name / return | type     | in/out | description           |
| ---------------------- | -------- | ------ | --------------------- |
| handle                 | void*    | in     | pointer to Controller |
| out                    | uint8_t* | in     | FPGA information list |
| return                 | bool     | -      | true if success       |

## AUTDUpdateFlags (autd3capi)

Update Control flag.

This function returns a value less than zero if an error occurred. If an error occurs, you can get the error message with `AUTDGetLastError`.

| Argument name / return | type    | in/out | description                                                                                            |
| ---------------------- | ------- | ------ | ------------------------------------------------------------------------------------------------------ |
| handle                 | void*   | in     | pointer to Controller                                                                                  |
| return                 | int32_t | -      | if $>0$ and check ack flag is true, it guarantees devices have processed data. if $<0$, error ocurred. |

## AUTDNumDevices (autd3capi)

Get the number of devices.

| Argument name / return | type    | in/out | description           |
| ---------------------- | ------- | ------ | --------------------- |
| handle                 | void*   | in     | pointer to Controller |
| return                 | int32_t | -      | number of devices     |

## AUTDTransPosition (autd3capi)

Get the position of the transducer.

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

Get the x-direction of the transducer.

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

Get the y-direction of the transducer.

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

Get the z-direction of the transducer.

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

Get pointer to Firmware information list.

| Argument name / return | type    | in/out | description                                                         |
| ---------------------- | ------- | ------ | ------------------------------------------------------------------- |
| handle                 | void*   | in     | pointer to Controller                                               |
| out                    | void**  | out    | pointer to pointer to Firmware information list                     |
| return                 | int32_t | -      | if $<0$ some error occurred, else size of Firmware information list |

## AUTDGetFirmwareInfo (autd3capi)

Get Firmware information.

`info` must be a buffer of length 256.

| Argument name / return | type    | in/out | description                            |
| ---------------------- | ------- | ------ | -------------------------------------- |
| p_firm_info_list       | void*   | in     | pointer to Firmware information list   |
| index                  | int32_t | in     | device index                           |
| info                   | char*   | out    | pointer to firmware information string |
| return                 | void    | -      | -                                      |

## AUTDFreeFirmwareInfoListPointer (autd3capi)

Free Firmware information list obtained by `AUTDGetFirmwareInfoListPointer`.

| Argument name / return | type  | in/out | description                          |
| ---------------------- | ----- | ------ | ------------------------------------ |
| p_firm_info_list       | void* | in     | pointer to Firmware information list |
| return                 | void  | -      | -                                    |

## AUTDGainNull (autd3capi)

Create Null gain.

The gain created must be deleted at the end by `AUTDDeleteGain`.

| Argument name / return | type   | in/out | description                     |
| ---------------------- | ------ | ------ | ------------------------------- |
| gain                   | void** | out    | pointer to pointer to Null gain |
| return                 | void   | -      | -                               |

## AUTDGainGrouped (autd3capi)

Create Grouped gain.

The gain created must be deleted at the end by `AUTDDeleteGain`.

| Argument name / return | type   | in/out | description                        |
| ---------------------- | ------ | ------ | ---------------------------------- |
| gain                   | void** | out    | pointer to pointer to Grouped gain |
| handle                 | void*  | in     | pointer to Controller              |
| return                 | void   | -      | -                                  |

## AUTDGainGroupedAdd (autd3capi)

Add Gain to Grouped gain.

| Argument name / return | type    | in/out | description             |
| ---------------------- | ------- | ------ | ----------------------- |
| grouped_gain           | void*   | in     | pointer to Grouped gain |
| device_id              | int32_t | in     | Device Id               |
| gain                   | void*   | in     | pointer to gain         |
| return                 | void    | -      | -                       |

## AUTDGainFocus (autd3capi)

Create Focus gain.

The gain created must be deleted at the end by `AUTDDeleteGain`.

| Argument name / return | type   | in/out | description                 |
| ---------------------- | ------ | ------ | --------------------------- |
| gain                   | void** | out    | pointer to Focus gain       |
| x                      | double | in     | x coordinate of focal point |
| y                      | double | in     | y coordinate of focal point |
| z                      | double | in     | z coordinate of focal point |
| amp                    | double | in     | amplitude of focus          |
| return                 | void   | -      | -                           |

## AUTDGainBesselBeam (autd3capi)

Create Bessel beam gain.

The gain created must be deleted at the end by `AUTDDeleteGain`.

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

Create Plane wave gain.

The gain created must be deleted at the end by `AUTDDeleteGain`.

| Argument name / return | type   | in/out | description                    |
| ---------------------- | ------ | ------ | ------------------------------ |
| gain                   | void** | out    | pointer to Plane wave gain     |
| n_x                    | double | in     | x coordinate of wave direction |
| n_y                    | double | in     | y coordinate of wave direction |
| n_z                    | double | in     | z coordinate of wave direction |
| amp                    | double | in     | amplitude of wave              |
| return                 | void   | -      | -                              |


## AUTDGainTransducerTest (autd3capi)

Create TransducerTest gain.

The gain created must be deleted at the end by `AUTDDeleteGain`.

| Argument name / return | type     | in/out | description                     |
| ---------------------- | -------  | ------ | ---------------------           |
| gain                   | void**   | out    | pointer to TransducerTest gain  |
| dev_idx                | int32_t  | in     | device index                    |
| tr_idx                 | int32_t  | in     | local transducer index          |
| amp                    | double   | in     | amplitude                       |
| phase                  | double   | in     | phase                           |
| return                 | void     | -      | -                               |

## AUTDGainCustom (autd3capi)

Create Custom gain.

The gain created must be deleted at the end by `AUTDDeleteGain`.

| Argument name / return | type     | in/out | description            |
| ---------------------- | -------  | ------ | ---------------------  |
| gain                   | void**   | out    | pointer to Custom gain |
| amp                    | double*  | in     | pointer to amplitude   |
| phase                  | double*  | in     | pointer to phase       |
| size                   | uint64_t | in     | size of amp and phase  |
| return                 | void     | -      | -                      |

## AUTDDeleteGain (autd3capi)

Delete Gain.

| Argument name / return | type  | in/out | description     |
| ---------------------- | ----- | ------ | --------------- |
| gain                   | void* | in     | pointer to gain |
| return                 | void  | -      | -               |

## AUTDModulationStatic (autd3capi)

Create Static modulation.

The created Modulation must be deleted at the end by `AUTDDeleteModulation`.

| Argument name / return | type   | in/out | description                             |
| ---------------------- | ------ | ------ | --------------------------------------- |
| mod                    | void** | out    | pointer to pointer to Static modulation |
| amp                    | double | in     | amplitude of modulation                 |
| return                 | void   | -      | -                                       |

## AUTDModulationSine (autd3capi)

Create Sine modulation.

The created Modulation must be deleted at the end by `AUTDDeleteModulation`.

| Argument name / return | type    | in/out | description                           |
| ---------------------- | ------- | ------ | ------------------------------------- |
| mod                    | void**  | out    | pointer to pointer to Sine modulation |
| freq                   | int32_t | in     | frequency of sine modulation          |
| amp                    | double  | in     | amplitude of sine modulation          |
| offset                 | double  | in     | offset of sine modulation             |
| return                 | void    | -      | -                                     |

## AUTDModulationSineSquared (autd3capi)

Create SineSquared modulation.

The created Modulation must be deleted at the end by `AUTDDeleteModulation`.

| Argument name / return | type    | in/out | description                                  |
| ---------------------- | ------- | ------ | -------------------------------------------- |
| mod                    | void**  | out    | pointer to pointer to SineSquared modulation |
| freq                   | int32_t | in     | frequency of sine modulation                 |
| amp                    | double  | in     | amplitude of sine modulation                 |
| offset                 | double  | in     | offset of sine modulation                    |
| return                 | void    | -      | -                                            |

## AUTDModulationSineLegacy (autd3capi)

Create SineLegacy modulation.

The created Modulation must be deleted at the end by `AUTDDeleteModulation`.

| Argument name / return | type   | in/out | description                                 |
| ---------------------- | ------ | ------ | ------------------------------------------- |
| mod                    | void** | out    | pointer to pointer to SineLegacy modulation |
| freq                   | double | in     | frequency of sine modulation                |
| amp                    | double | in     | amplitude of sine modulation                |
| offset                 | double | in     | offset of sine modulation                   |
| return                 | void   | -      | -                                           |

## AUTDModulationSquare (autd3capi)

Create Square modulation.

The created Modulation must be deleted at the end by `AUTDDeleteModulation`.

| Argument name / return | type    | in/out | description                                  |
| ---------------------- | ------- | ------ | -------------------------------------------- |
| mod                    | void**  | out    | pointer to pointer to Square modulation      |
| freq                   | int32_t | in     | frequency of square modulation               |
| low                    | double  | in     | amplitude at low level of square modulation  |
| high                   | double  | in     | amplitude at high level of square modulation |
| duty                   | double  | in     | duty ratio of square modulation              |
| return                 | void    | -      | -                                            |

## AUTDModulationLPF (autd3capi)

Create LPF modulation.

The created Modulation must be deleted at the end by `AUTDDeleteModulation`.

| Argument name / return | type    | in/out | description                                  |
| ---------------------- | ------- | ------ | -------------------------------------------- |
| mod                    | void**  | out    | pointer to pointer to LPF modulation         |
| mod_in                 | void*   | in     | original modulation                          |
| return                 | void    | -      | -                                            |

## AUTDModulationCustom (autd3capi)

Create Custom modulation.

The created Modulation must be deleted at the end by `AUTDDeleteModulation`.

| Argument name / return | type     | in/out | description                             |
| ---------------------- | -------- | ------ | --------------------------------------- |
| mod                    | void**   | out    | pointer to pointer to Custom modulation |
| buffer                 | uint8_t* | in     | pointer to modulation data              |
| size                   | uint64_t | in     | size of buffer                          |
| freq_div               | uint32_t | in     | modulation sampling frequency division  |
| return                 | void     | -      | -                                       |

## AUTDModulationSamplingFrequencyDivision (autd3capi)

Get sampling frequency division of Modulation.

| Argument name / return | type     | in/out | description                            |
| ---------------------- | -------- | ------ | -------------------------------------- |
| mod                    | void*    | in     | pointer to modulation                  |
| return                 | uint32_t | -      | modulation sampling frequency division |

## AUTDModulationSetSamplingFrequencyDivision (autd3capi)

Set sampling frequency division of Modulation.

| Argument name / return | type     | in/out | description                            |
| ---------------------- | -------- | ------ | -------------------------------------- |
| mod                    | void*    | in     | pointer to modulation                  |
| freq_div               | uint32_t | in     | modulation sampling frequency division |
| return                 | void     | -      | -                                      |

## AUTDModulationSamplingFrequency (autd3capi)

Get Sampling frequency.

| Argument name / return | type   | in/out | description                   |
| ---------------------- | ------ | ------ | ----------------------------- |
| mod                    | void*  | in     | pointer to modulation         |
| return                 | double | -      | modulation sampling frequency |

## AUTDDeleteModulation (autd3capi)

Delete Modulation.

| Argument name / return | type  | in/out | description           |
| ---------------------- | ----- | ------ | --------------------- |
| mod                    | void* | in     | pointer to modulation |
| return                 | void  | -      | -                     |

## AUTDPointSTM (autd3capi)

Create Point STM.

The stm created must be deleted at the end by `AUTDDeleteSTM`.

| Argument name / return | type   | in/out | description                     |
| ---------------------- | ------ | ------ | ------------------------------- |
| out                    | void** | out    | pointer to pointer to Point STM |
| return                 | void   | -      | -                               |

## AUTDGainSTM (autd3capi)

Create Gain STM.

The stm created must be deleted at the end by `AUTDDeleteSTM`.

| Argument name / return | type   | in/out | description                    |
| ---------------------- | ------ | ------ | ------------------------------ |
| out                    | void** | out    | pointer to pointer to Gain STM |
| handle                 | void*  | in     | pointer to Controller          |
| return                 | void   | -      | -                              |

## AUTDPointSTMAdd (autd3capi)

Add focus to PointSTM.

| Argument name / return | type    | in/out | description                 |
| ---------------------- | ------- | ------ | --------------------------- |
| stm                    | void*   | in     | pointer to Point STM        |
| x                      | double  | in     | x coordinate of focal point |
| y                      | double  | in     | y coordinate of focal point |
| z                      | double  | in     | z coordinate of focal point |
| shift                  | uint8_t | in     | duty shift                  |
| return                 | bool    | -      | true if success             |

## AUTDGainSTMAdd (autd3capi)

Add gain to GainSTM.

| Argument name / return | type  | in/out | description          |
| ---------------------- | ----- | ------ | -------------------- |
| stm                    | void* | in     | pointer to Point STM |
| gain                   | void* | in     | pointer to Gain      |
| return                 | bool  | -      | true if success      |

## AUTDSetGainSTMMode (autd3capi)

Set GainSTM mode.

| Argument name / return | type     | in/out | description                                                                   |
| ---------------------- | -------- | ------ | ----------------------------------------------------------------------------- |
| stm                    | void*    | in     | pointer to STM                                                                |
| mode                   | uint16_t | in     | GainSTM mode (0x0001 = PhaseDutyFull, 0x0002 = PhaseFull, 0x0004 = PhaseHalf) |
| return                 | void     | -      | -                                                                             |

## AUTDGetGainSTMMode (autd3capi)

Get GainSTM mode.

| Argument name / return | type     | in/out | description    |
| ---------------------- | -------- | ------ | -------------- |
| stm                    | void*    | in     | pointer to STM |
| return                 | uint16_t | -      | GainSTM mode   |

## AUTDSTMSetFrequency (autd3capi)

Set frequency of STM.

| Argument name / return | type   | in/out | description      |
| ---------------------- | ------ | ------ | ---------------- |
| stm                    | void*  | in     | pointer to STM   |
| freq                   | double | in     | frequency of STM |
| return                 | double | -      | actual frequency |

## AUTDSTMFrequency (autd3capi)

Get frequency of STM.

| Argument name / return | type   | in/out | description      |
| ---------------------- | ------ | ------ | ---------------- |
| stm                    | void*  | in     | pointer to STM   |
| return                 | double | -      | frequency of STM |

## AUTDSTMSamplingFrequency (autd3capi)

Get sampling frequency of STM.

| Argument name / return | type   | in/out | description               |
| ---------------------- | ------ | ------ | ------------------------- |
| stm                    | void*  | in     | pointer to STM            |
| return                 | double | -      | sampling frequency of STM |

## AUTDSTMSamplingFrequencyDivision (autd3capi)

Get sampling frequency division of STM.

| Argument name / return | type     | in/out | description                     |
| ---------------------- | -------- | ------ | ------------------------------- |
| stm                    | void*    | in     | pointer to STM                  |
| return                 | uint32_t | in     | STM sampling frequency division |

## AUTDSTMSetSamplingFrequencyDivision (autd3capi)

Set sampling frequency division of STM.

| Argument name / return | type     | in/out | description                     |
| ---------------------- | -------- | ------ | ------------------------------- |
| stm                    | void*    | in     | pointer to STM                  |
| freq_div               | uint32_t | in     | STM sampling frequency division |
| return                 | void     | -      | -                               |

## AUTDDeleteSTM (autd3capi)

Delete STM.

| Argument name / return | type  | in/out | description    |
| ---------------------- | ----- | ------ | -------------- |
| stm                    | void* | in     | pointer to STM |
| return                 | void  | -      | -              |

## AUTDStop (autd3capi)

Stop AUTD3 outputting.

This function returns a value less than zero if an error occurred. If an error occurs, you can get the error message with `AUTDGetLastError`.

| Argument name / return | type    | in/out | description                                                                                            |
| ---------------------- | ------- | ------ | ------------------------------------------------------------------------------------------------------ |
| handle                 | void*   | in     | pointer to Controller                                                                                  |
| return                 | int32_t | -      | if $>0$ and check ack flag is true, it guarantees devices have processed data. if $<0$, error ocurred. |

## AUTDSend (autd3capi)

Send header and body data.

This function returns a value less than zero if an error occurred. If an error occurs, you can get the error message with `AUTDGetLastError`.

| Argument name / return | type    | in/out | description                                                                                            |
| ---------------------- | ------- | ------ | ------------------------------------------------------------------------------------------------------ |
| handle                 | void*   | in     | pointer to Controller                                                                                  |
| header                 | void*   | in     | pointer to header data                                                                                 |
| body                   | void*   | in     | pointer to body data                                                                                   |
| return                 | int32_t | -      | if $>0$ and check ack flag is true, it guarantees devices have processed data. if $<0$, error ocurred. |

## AUTDSetModDelay (autd3capi)

Configure Modulation Delay of the transducer.

| Argument name / return | type     | in/out | description                        |
| ---------------------- | -------- | ------ | ---------------------------------- |
| handle                 | void*    | in     | pointer to Controller              |
| device_idx             | int32_t  | in     | device index                       |
| local_trans_idx        | int32_t  | in     | local transducer index             |
| delay                  | uint16_t | in     | modulation delay of the transducer |
| return                 | void     | -      | -                                  |

## AUTDGetModDelay (autd3capi)

Get Modulation Delay of the transducer.

| Argument name / return | type     | in/out | description                        |
| ---------------------- | -------- | ------ | ---------------------------------- |
| handle                 | void*    | in     | pointer to Controller              |
| device_idx             | int32_t  | in     | device index                       |
| local_trans_idx        | int32_t  | in     | local transducer index             |
| return                 | uint16_t | -      | modulation delay of the transducer |

## AUTDCreateModDelayConfig (autd3capi)

Create ModDelayConfig.

The config created must be deleted at the end by `AUTDDeleteModDelayConfig`.

| Argument name / return | type   | in/out | description                          |
| ---------------------- | ------ | ------ | ------------------------------------ |
| out                    | void** | out    | pointer to pointer to ModDelayConfig |
| return                 | void   | -      | -                                    |

## AUTDDeleteModDelayConfig (autd3capi)

Delete ModDelayConfig.

| Argument name / return | type  | in/out | description               |
| ---------------------- | ----- | ------ | ------------------------- |
| config                 | void* | in     | pointer to ModDelayConfig |
| return                 | void  | -      | -                         |

## AUTDCreateAmplitudes (autd3capi)

Create Amplitudes.

The amplitude created must be deleted at the end by `AUTDDeleteAmplitudes`.

| Argument name / return | type   | in/out | description                      |
| ---------------------- | ------ | ------ | -------------------------------- |
| out                    | void** | out    | pointer to pointer to Amplitudes |
| amp                    | double | in     | amplitudes                       |
| return                 | void   | -      | -                                |

## AUTDDeleteAmplitudes (autd3capi)

Delete Amplitudes.

| Argument name / return | type  | in/out | description           |
| ---------------------- | ----- | ------ | --------------------- |
| amplitudes             | void* | in     | pointer to Amplitudes |
| return                 | void  | -      | -                     |

## AUTDSoftwareSTM (autd3capi)

Create SoftwareSTM.

| Argument name / return | type  | in/out | description                       |
| ---------------------- | ----- | ------ | --------------------------------- |
| out                    | void**| out    | pointer to pointer to SoftwareSTM |
| return                 | void  | -      | -                                 |

## AUTDSoftwareSTMSetStrategy (autd3capi)

Configure TimerStrategy of SoftwareSTM.

| Argument name / return | type    | in/out | description                       |
| ---------------------- | -----   | ------ | --------------------------------- |
| stm                    | void*   | in     | pointer to SoftwareSTM            |
| strategy               | uint8_t | in     | TimerStrategy                     |
| return                 | void    | -      | -                                 |

## AUTDSoftwareSTMAdd (autd3capi)

Add Gain to SoftwareSTM.

| Argument name / return | type    | in/out | description                       |
| ---------------------- | -----   | ------ | --------------------------------- |
| stm                    | void*   | in     | pointer to SoftwareSTM            |
| gain                   | void*   | in     | pointer to Gain                   |
| return                 | void    | -      | -                                 |

## AUTDSoftwareSTMStart (autd3capi)

Start SoftwareSTM.

After calling this function, the use of Controller is forbidden.

| Argument name / return | type    | in/out | description                                   |
| ---------------------- | -----   | ------ | --------------------------------------------- |
| handle                 | void**  | out    | pointer to pointer to SoftwareSTMThreadHandle |
| stm                    | void*   | in     | pointer to SoftwareSTM                        |
| cnt                    | void*   | in     | pointer to Controller                         |
| return                 | void    | -      | -                                             |

## AUTDSoftwareSTMFinish (autd3capi)

| Argument name / return | type    | in/out | description                        |
| ---------------------- | -----   | ------ | -----------------------------------|
| cnt                    | void**  | out    | pointer to pointer to Controller   |
| handle                 | void*   | in     | pointer to SoftwareSTMThreadHandle |
| return                 | void    | -      | -                                  |


## AUTDSoftwareSTMSetFrequency (autd3capi)

Set SoftwareSTM frequency.

| Argument name / return | type    | in/out | description            |
| ---------------------- | -----   | ------ | -----------------------|
| stm                    | void*   | in     | pointer to SoftwareSTM |
| freq                   | double  | in     | frequency              |
| return                 | double  | -      | actual frequency       |

## AUTDSoftwareSTMFrequency (autd3capi)

Get SoftwareSTM frequency.

| Argument name / return | type    | in/out | description            |
| ---------------------- | -----   | ------ | -----------------------|
| stm                    | void*   | in     | pointer to SoftwareSTM |
| return                 | double  | -      | frequency              |

## AUTDSoftwareSTMPeriod (autd3capi)

Get SoftwareSTM period in ns.

| Argument name / return | type     | in/out | description            |
| ---------------------- | -----    | ------ | -----------------------|
| stm                    | void*    | in     | pointer to SoftwareSTM |
| return                 | uint64_t | -      | period                 |

## AUTDSoftwareSTMSamplingFrequency (autd3capi)

Get SoftwareSTM sampling frequency.

| Argument name / return | type    | in/out | description            |
| ---------------------- | -----   | ------ | -----------------------|
| stm                    | void*   | in     | pointer to SoftwareSTM |
| return                 | double  | -      | sampling frequency     |

## AUTDSoftwareSTMSamplingPeriod (autd3capi)

Get SoftwareSTM sampling period in ns.

| Argument name / return | type     | in/out | description            |
| ---------------------- | -----    | ------ | -----------------------|
| stm                    | void*    | in     | pointer to SoftwareSTM |
| return                 | uint64_t | -      | sampling period        |

## AUTDSoftwareSTMSetSamplingPeriod (autd3capi)

Set SoftwareSTM sampling period in ns.

| Argument name / return | type     | in/out | description            |
| ---------------------- | -----    | ------ | -----------------------|
| stm                    | void*    | in     | pointer to SoftwareSTM |
| period                 | uint64_t | in     | sampling period        |
| return                 | void     | -      | -                      |

## AUTDDeleteSoftwareSTM (autd3capi)

Delete SoftwareSTM.

| Argument name / return | type  | in/out | description            |
| ---------------------- | ----- | ------ | ---------------------- |
| stm                    | void* | in     | pointer to SoftwareSTM |
| return                 | void  | -      | -                      |

## AUTDSetMode (autd3capi)

Set Legacy/Normal mode.

| Argument name / return | type    | in/out | description                                                 |
| ---------------------- | ------- | ------ | ----------------------------------------------------------- |
| handle                 | void*   | in     | pointer to Controller                                       |
| mode                   | uint8_t | in     | mode (0: Legacy mode, 1: Normal mode, 2: Normal Phase mode) |
| return                 | void    | -      | -                                                           |

## AUTDEigenBackend (autd3capi-gain-holo)

Create Eigen Backend.

The backend created must be deleted at the end by `AUTDDeleteBackend`.

| Argument name / return | type   | in/out | description                         |
| ---------------------- | ------ | ------ | ----------------------------------- |
| out                    | void** | out    | pointer to pointer to Eigen backend |
| return                 | void   | -      | -                                   |

## AUTDDeleteBackend (autd3capi-gain-holo)

Create Backend.

| Argument name / return | type  | in/out | description        |
| ---------------------- | ----- | ------ | ------------------ |
| backend                | void* | in     | pointer to backend |
| return                 | void  | -      | -                  |

## AUTDGainHoloSDP (autd3capi-gain-holo)

Create SDP holo gain.

The gain created must be deleted at the end by `AUTDDeleteGain`.

| Argument name / return | type     | in/out | description                    |
| ---------------------- | -------- | ------ | ------------------------------ |
| gain                   | void**   | out    | pointer to pointer to SDP gain |
| backend                | void*    | in     | pointer to backend             |
| alpha                  | double   | in     | parameter                      |
| lambda                 | double   | in     | parameter                      |
| repeat                 | uint64_t | in     | parameter                      |
| return                 | void     | -      | -                              |

## AUTDGainHoloEVD (autd3capi-gain-holo)

Create EVD holo gain.

The gain created must be deleted at the end by `AUTDDeleteGain`.

| Argument name / return | type   | in/out | description                    |
| ---------------------- | ------ | ------ | ------------------------------ |
| gain                   | void** | out    | pointer to pointer to EVD gain |
| backend                | void*  | in     | pointer to backend             |
| gamma                  | double | in     | parameter                      |
| return                 | void   | -      | -                              |

## AUTDGainHoloNaive (autd3capi-gain-holo)

Create Naive holo gain.

The gain created must be deleted at the end by `AUTDDeleteGain`.

| Argument name / return | type   | in/out | description                      |
| ---------------------- | ------ | ------ | -------------------------------- |
| gain                   | void** | out    | pointer to pointer to Naive gain |
| backend                | void*  | in     | pointer to backend               |
| return                 | void   | -      | -                                |

## AUTDGainHoloGS (autd3capi-gain-holo)

Create GS holo gain.

The gain created must be deleted at the end by `AUTDDeleteGain`.

| Argument name / return | type     | in/out | description                   |
| ---------------------- | -------- | ------ | ----------------------------- |
| gain                   | void**   | out    | pointer to pointer to GS gain |
| backend                | void*    | in     | pointer to backend            |
| repeat                 | uint64_t | in     | parameter                     |
| return                 | void     | -      | -                             |

## AUTDGainHoloGSPAT (autd3capi-gain-holo)

Create GSPAT holo gain.

The gain created must be deleted at the end by `AUTDDeleteGain`.

| Argument name / return | type     | in/out | description                      |
| ---------------------- | -------- | ------ | -------------------------------- |
| gain                   | void**   | out    | pointer to pointer to GSPAT gain |
| backend                | void*    | in     | pointer to backend               |
| repeat                 | uint64_t | in     | parameter                        |
| return                 | void     | -      | -                                |

## AUTDGainHoloLM (autd3capi-gain-holo)

Create LM holo gain.

The gain created must be deleted at the end by `AUTDDeleteGain`.

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

Create Greedy holo gain.

The gain created must be deleted at the end by `AUTDDeleteGain`.

| Argument name / return | type    | in/out | description                       |
| ---------------------- | ------- | ------ | --------------------------------- |
| gain                   | void**  | out    | pointer to pointer to Greedy gain |
| backend                | void*   | in     | pointer to backend                |
| phase_div              | int32_t | in     | parameter                         |
| return                 | void    | -      | -                                 |


## AUTDGainHoloLSSGreedy (autd3capi-gain-holo)

Create LSSGreedy holo gain.

The gain created must be deleted at the end by `AUTDDeleteGain`.

| Argument name / return | type    | in/out | description                          |
| ---------------------- | ------- | ------ | ---------------------------------    |
| gain                   | void**  | out    | pointer to pointer to LSSGreedy gain |
| backend                | void*   | in     | pointer to backend                   |
| phase_div              | int32_t | in     | parameter                            |
| return                 | void    | -      | -                                    |

## AUTDGainHoloAPO (autd3capi-gain-holo)

Create APO holo gain.

The gain created must be deleted at the end by `AUTDDeleteGain`.

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

Add focus to Holo gain.

| Argument name / return | type   | in/out | description               |
| ---------------------- | ------ | ------ | ------------------------- |
| gain                   | void*  | in     | pointer to holo gain      |
| x                      | double | in     | x coordinate of the focus |
| y                      | double | in     | y coordinate of the focus |
| z                      | double | in     | z coordinate of the focus |
| amp                    | double | in     | amplitude of the focus    |
| return                 | void   | -      | -                         |

## AUTDConstraintDontCare (autd3capi-gain-holo)

Create DontCare AmplitudeConstraint.

| Argument name / return | type    | in/out | description                                                           |
| ---------------------- | ------- | ------ | --------------------------------------------------------------------- |
| constraint             | void**  | out    | pointer to pointer to constraint                                      |
| return                 | void    | -      | -                                                                     |

## AUTDConstraintNormalize (autd3capi-gain-holo)

Create Normalize AmplitudeConstraint.

| Argument name / return | type    | in/out | description                                                           |
| ---------------------- | ------- | ------ | --------------------------------------------------------------------- |
| constraint             | void**  | out    | pointer to pointer to constraint                                      |
| return                 | void    | -      | -                                                                     |

## AUTDConstraintUniform (autd3capi-gain-holo)

Create Uniform AmplitudeConstraint.

| Argument name / return | type    | in/out | description                                                           |
| ---------------------- | ------- | ------ | --------------------------------------------------------------------- |
| constraint             | void**  | out    | pointer to pointer to constraint                                      |
| value                  | double  | in     | uniform amplitude                                                     |
| return                 | void    | -      | -                                                                     |

## AUTDConstraintClamp (autd3capi-gain-holo)

Create Clamp AmplitudeConstraint.

| Argument name / return | type    | in/out | description                                                           |
| ---------------------- | ------- | ------ | --------------------------------------------------------------------- |
| constraint             | void**  | out    | pointer to pointer to constraint                                      |
| return                 | void    | -      | -                                                                     |

## AUTDSetConstraint (autd3capi-gain-holo)

Configure AmplitudeConstraint of Holo Gain.

| Argument name / return | type    | in/out | description                                                           |
| ---------------------- | ------- | ------ | --------------------------------------------------------------------- |
| gain                   | void*   | in     | pointer to holo gain                                                  |
| constraint             | void*   | in     | pointer to AmplitudeConstraint                                        |
| return                 | void    | -      | -                                                                     |

## AUTDBLASBackend (autd3capi-backend-blas)

Create BLAS Backend.

The backend created must be deleted at the end by `AUTDDeleteBackend`.

| Argument name / return | type   | in/out | description                        |
| ---------------------- | ------ | ------ | ---------------------------------- |
| out                    | void** | out    | pointer to pointer to BLAS backend |
| return                 | void   | -      | -                                  |

## AUTDCUDABackend (autd3capi-backend-cuda)

Create CUDA Backend.

The backend created must be deleted at the end by `AUTDDeleteBackend`.

| Argument name / return | type   | in/out | description                        |
| ---------------------- | ------ | ------ | ---------------------------------- |
| out                    | void** | out    | pointer to pointer to CUDA backend |
| return                 | void   | -      | -                                  |

## AUTDLinkDebug (autd3capi-link-debug)

Create Debug link.

| Argument name / return | type     | in/out | description                         |
| ---------------------- | -------- | ------ | ----------------------------------- |
| out                    | void**   | out    | pointer to pointer to Debug link    |
| return                 | void     | -      | -                                   |

## AUTDLinkDebugSetLevel (autd3capi-link-debug)

Configure log level.

| Argument name / return | type     | in/out | description                         |
| ---------------------- | -------- | ------ | ----------------------------------- |
| level                  | int32_t  | in     | debug level                         |
| return                 | void     | -      | -                                   |

## AUTDLinkSimulator (autd3capi-link-simulator)

Create Simulator link.

| Argument name / return | type     | in/out | description                         |
| ---------------------- | -------- | ------ | ----------------------------------- |
| out                    | void**   | out    | pointer to pointer to Simulator link |
| return                 | void     | -      | -                                   |

## AUTDLinkRemoteTwinCAT (autd3capi-link-remote-twincat)

Create RemoteTwinCAT link.

| Argument name / return | type   | in/out | description                              |
| ---------------------- | ------ | ------ | ---------------------------------------- |
| out                    | void** | out    | pointer to pointer to RemoteTwinCAT link |
| server_ip_addr         | char*  | in     | server ip address                        |
| server_ams_net_id      | char*  | in     | server ams net id                        |
| client_ams_net_id      | char*  | in     | client ams net id                        |
| return                 | void   | -      | -                                        |

## AUTDGetAdapterPointer (autd3capi-link-soem)

Get pointer to Adapter list.

Delete the list created this function must be freed at the end by `AUTDFreeAdapterPointer`.

The adapter information can be obtained by `AUTDGetAdapter`.

This function returns the size of adapter list.

| Argument name / return | type    | in/out | description                        |
| ---------------------- | ------- | ------ | ---------------------------------- |
| out                    | void**  | out    | pointer to pointer to adapter list |
| return                 | int32_t | -      | size of adapter list               |

## AUTDGetAdapter (autd3capi-link-soem)

Get the adapter name and description.

`p_adapter` must be created by `AUTDGetAdapterPointer`.

Pass buffer of length 128 to `desc`, `name`.

| Argument name / return | type    | in/out | description                    |
| ---------------------- | ------- | ------ | ------------------------------ |
| p_adapter              | void*   | in     | pointer to adapter list        |
| index                  | int32_t | in     | index                          |
| desc                   | char*   | out    | pointer to adapter description |
| name                   | char*   | out    | pointer to adapter name        |
| return                 | void    | -      | -                              |

## AUTDFreeAdapterPointer (autd3capi-link-soem)

Delete pointer to Adapter list.

| Argument name / return | type  | in/out | description             |
| ---------------------- | ----- | ------ | ----------------------- |
| p_adapter              | void* | in     | pointer to adapter list |
| return                 | void  | -      | size of adapter list    |

## AUTDLinkSOEM (autd3capi-link-soem)

| Argument name / return | type     | in/out | description                        |
| ---------------------- | -------- | ------ | ---------------------------------- |
| out                    | void**   | out    | pointer to pointer to TwinCAT link |
| ifname                 | char*    | in     | interface name                     |
| sync0_cycle            | uint16_t | in     | sync0 cycle                        |
| send_cycle             | uint16_t | in     | send cycle                         |
| freerun                | bool     | in     | free run mode                      |
| on_lost                | void*    | in     | pointer to on-lost callback        |
| high_precision         | bool     | in     | high precision mode                |
| state_check_interval   | uint64_t | in     | state check interval in ms         |
| return                 | void     | -      | -                                  |

## AUTDLinkSOEMSetLogLevel (autd3capi-link-soem)

| Argument name / return | type     | in/out | description                        |
| ---------------------- | -------- | ------ | ---------------------------------- |
| level                  | int32_t  | in     | log level                          |
| return                 | void     | -      | -                                  |

## AUTDLinkSOEMSetDefaultLogger (autd3capi-link-soem)

| Argument name / return | type     | in/out | description                        |
| ---------------------- | -------- | ------ | ---------------------------------- |
| out                    | void*    | in     | output callback                    |
| flush                  | void*    | in     | flush callback                     |
| return                 | void     | -      | -                                  |

## AUTDLinkRemoteSOEM (autd3capi-link-remote-soem)

Create RemoteSOEM link.

| Argument name / return | type    | in/out | description                              |
| ---------------------- | ------  | ------ | ---------------------------------------- |
| out                    | void**  | out    | pointer to pointer to RemoteSOEM link    |
| ip                     | char*   | in     | server ip address                        |
| port                   | uint16_t| in     | port                                     |
| return                 | void    | -      | -                                        |

## AUTDLinkTwinCAT (autd3capi-link-twincat)

Create TwinCAT link.

| Argument name / return | type   | in/out | description                        |
| ---------------------- | ------ | ------ | ---------------------------------- |
| out                    | void** | out    | pointer to pointer to TwinCAT link |
| return                 | void   | -      | -                                  |

## AUTDModulationRawPCM (autd3capi-modulation-audio-file)

Create RawPCM modulation.

The created Modulation must be deleted at the end by `AUTDDeleteModulation`.

| Argument name / return | type     | in/out | description                            |
| ---------------------- | -------- | ------ | -------------------------------------- |
| mod                    | void**   | out    | pointer to pointer to Sine modulation  |
| filename               | char*    | in     | path to pcm file                       |
| sampling_freq          | double   | in     | pcm sampling frequency                 |
| mod_sampling_freq_div  | uint32_t | in     | modulation sampling frequency division |
| return                 | void     | -      | -                                      |

## AUTDModulationWav (autd3capi-modulation-audio-file)

Create Wav modulation.

The created Modulation must be deleted at the end by `AUTDDeleteModulation`.

| Argument name / return | type     | in/out | description                            |
| ---------------------- | -------- | ------ | -------------------------------------- |
| mod                    | void**   | out    | pointer to pointer to Sine modulation  |
| filename               | char*    | in     | path to pcm file                       |
| mod_sampling_freq_div  | uint32_t | in     | modulation sampling frequency division |
| return                 | void     | -      | -                                      |

## AUTDExtraGeometryViewer (autd3capi-extra-geometry-viewer)

Run Geometry Viewer.

| Argument name / return | type    | in/out | description                        |
| ---------------------- | ------- | ------ | ---------------------------------- |
| cnt                    | void*   | in     | pointer to Controller              |
| width                  | int32_t | in     | window width                       |
| height                 | int32_t | in     | window height                      |
| vsync                  | bool    | in     | vsync                              |
| gpu_idx                | int32_t | in     | GPU index                          |
| return                 | void    | -      | -                                  |

## AUTDExtraSimulator (autd3capi-extra-simulator)

Run Simulator.

If a configuration file exists in `settings_path`, the parameters (`vsync`, and `gpu_idx`) are overridden by the contents of the configuration file.

| Argument name / return | type    | in/out | description                        |
| ---------------------- | ------- | ------ | ---------------------------------- |
| settings_path          | char*   | in     | path to setting file               |
| vsync                  | bool    | in     | vsync                              |
| gpu_idx                | int32_t | in     | GPU index                          |
| return                 | void    | -      | -                                  |
