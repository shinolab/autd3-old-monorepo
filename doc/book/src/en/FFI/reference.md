# C API Reference

The API for the C language is defined under [capi](https://github.com/shinolab/autd3/tree/master/capi).

The following is a reference to this API. 
For actual usage, please refer to [C API Example](https://github.com/shinolab/autd3/tree/master/capi/example).

## AUTDSetLogLevel (autd3capi)

| Argument name / return | type     | in/out | description                        |
| ---------------------- | -------- | ------ | ---------------------------------- |
| level                  | int32_t  | in     | log level                          |
| return                 | void     | -      | -                                  |

## AUTDSetDefaultLogger (autd3capi)

| Argument name / return | type     | in/out | description                        |
| ---------------------- | -------- | ------ | ---------------------------------- |
| out                    | void*    | in     | output callback                    |
| flush                  | void*    | in     | flush callback                     |
| return                 | void     | -      | -                                  |


## AUTDCreateGeometryBuilder (autd3capi)

| Argument name / return | type    | in/out | description                            |
| ---------------------- | ------- | ------ | -------------------------------------- |
| out                    | void**  | out     | pointer to pointer to Geometry::Builder                  |
| return                 | void    | -      | -                              |

## AUTDAddDevice (autd3capi)

| Argument name / return | type    | in/out | description                               |
| ---------------------- | ------- | ------ | ----------------------------------------- |
| geometry_builder                | void*   | in     | pointer to Geometry::Builder                     |
| x                      | double  | in     | x coordinate of position in millimeter    |
| y                      | double  | in     | y coordinate of position in millimeter    |
| z                      | double  | in     | z coordinate of position in millimeter    |
| rz1                    | double  | in     | first angle of ZYZ euler angle in radian  |
| ry                     | double  | in     | second angle of ZYZ euler angle in radian |
| rz2                    | double  | in     | third angle of ZYZ euler angle in radian  |
| return                 | bool | -      | -                                 |

## AUTDAddDeviceQuaternion (autd3capi)

| Argument name / return | type    | in/out | description                            |
| ---------------------- | ------- | ------ | -------------------------------------- |
| geometry_builder                 | void*   | in     | pointer to Geometry::Builder                  |
| x                      | double  | in     | x coordinate of position in millimeter |
| y                      | double  | in     | y coordinate of position in millimeter |
| z                      | double  | in     | z coordinate of position in millimeter |
| qw                     | double  | in     | w parameter of quaternion of rotation  |
| qx                     | double  | in     | x parameter of quaternion of rotation  |
| qy                     | double  | in     | y parameter of quaternion of rotation  |
| qz                     | double  | in     | z parameter of quaternion of rotation  |
| return                 | bool    | -      | -                              |

## AUTDSetMode (autd3capi)

Set Legacy/Advanced mode.

| Argument name / return | type    | in/out | description                                                 |
| ---------------------- | ------- | ------ | ----------------------------------------------------------- |
| geometry_builder                 | void*   | in     | pointer to GeometryBuilder                                       |
| mode                   | uint8_t | in     | mode (0: Legacy mode, 1: Advanced mode, 2: Advanced Phase mode) |
| return                 | void    | -      | -                                                           |

## AUTDBuildGeometry (autd3capi)

| Argument name / return | type    | in/out | description                            |
| ---------------------- | ------- | ------ | -------------------------------------- |
| out                    | void**  | out     | pointer to pointer to Geometry                  |
| geometry_builder       | void*   | in     | pointer to Geometry::Builder                  |
| return                 | void    | -      | -                              |

## AUTDGetGeometry (autd3capi)

| Argument name / return | type    | in/out | description                            |
| ---------------------- | ------- | ------ | -------------------------------------- |
| geometry               | void**  | out    | pointer to pointer to Geometry         |
| cnt                    | void*   | in     | pointer to Controller                  |
| return                 | void    | -      | -                              |


## AUTDOpenController (autd3capi)

| Argument name / return | type  | in/out | description                      |
| ---------------------- | ----- | ------ | -------------------------------- |
| out                    | void**| out    | pointer to pointer to Controller |
| geometry               | void* | in     | pointer to Geometry              |
| link                   | void* | in     | pointer to Link                  |
| return                 | bool  | -      | true if success                  |

## AUTDClose (autd3capi)

Close Controller.

| Argument name / return | type    | in/out | description                                                                                            |
| ---------------------- | ------- | ------ | ------------------------------------------------------------------------------------------------------ |
| handle                 | void*   | in     | pointer to Controller                                                                                  |
| return                 | bool    | -      | true if successful                                                                                     |

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


## AUTDSetReadsFPGAInfo (autd3capi)

Set Reads FPGA info flag.

| Argument name / return | type  | in/out | description           |
| ---------------------- | ----- | ------ | --------------------- |
| handle                 | void* | in     | pointer to Controller |
| reads_fpga_info        | bool  | in     | read FPGA info flag   |
| return                 | void  | -      | -                     |

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
| geometry                 | void*   | in     | pointer to Geometry                  |
| return                 | double | -      | Speed of sound in mm/s |

## AUTDSetSoundSpeed (autd3capi)

Set sound speed.

| Argument name / return | type   | in/out | description           |
| ---------------------- | ------ | ------ | --------------------- |
| geometry                 | void*   | in     | pointer to Geometry                  |
| sound_speed            | double | in     | Speed of sound in mm/s |
| return                 | void   | -      | -                     |

## AUTDSetSoundSpeedFromTemp (autd3capi)

Set sound speed from temperature.

| Argument name / return | type   | in/out | description           |
| ---------------------- | ------ | ------ | --------------------- |
| geometry                 | void*  | in     | pointer to Geometry |
| temp                   | double | in     | temperature in Celsius degree |
| k                   | double | in     | Heat capacity ratio           |
| r                   | double | in     |  Gas constant [J K^-1 mol^-1]           |
| m                   | double | in     | Molar mass [kg mod^-1]           |
| return                 | void   | -      | -                     |

## AUTDGetTransFrequency (autd3capi)

Get frequency of the transducer.

| Argument name / return | type    | in/out | description                 |
| ---------------------- | ------- | ------ | --------------------------- |
| geometry                 | void*   | in     | pointer to Geometry                  |
| trans_idx        | int32_t | in     | transducer index      |
| return                 | double  | -      | frequency of the transducer |

## AUTDSetTransFrequency (autd3capi)

Set frequency of the transducer.

| Argument name / return | type    | in/out | description                 |
| ---------------------- | ------- | ------ | --------------------------- |
| geometry                 | void*   | in     | pointer to Geometry                  |
| trans_idx        | int32_t | in     | transducer index      |
| frequency              | double  | in     | frequency of the transducer |
| return                 | void    | -      | -                           |

## AUTDGetTransCycle (autd3capi)

Get cycle of the transducer.

| Argument name / return | type     | in/out | description             |
| ---------------------- | -------- | ------ | ----------------------- |
| geometry                 | void*   | in     | pointer to Geometry                  |
| trans_idx        | int32_t | in     | transducer index      |
| return                 | uint16_t | -      | cycle of the transducer |

## AUTDSetTransCycle (autd3capi)

Set cycle of the transducer.

| Argument name / return | type     | in/out | description             |
| ---------------------- | -------- | ------ | ----------------------- |
| geometry                 | void*   | in     | pointer to Geometry                  |
| trans_idx        | int32_t | in     | transducer index      |
| cycle                  | uint16_t | in     | cycle of the transducer |
| return                 | void     | -      | -                       |

## AUTDGetWavelength (autd3capi)

Get wavelength of the transducer.

| Argument name / return | type    | in/out | description                                          |
| ---------------------- | ------- | ------ | ---------------------------------------------------- |
| geometry                 | void*   | in     | pointer to Geometry                  |
| trans_idx        | int32_t | in     | transducer index      |
| return                 | double  | -      | wavelength of ultrasound emitted from the transducer |

## AUTDGetAttenuation (autd3capi)

Get attenuation coefficient.

| Argument name / return | type   | in/out | description                      |
| ---------------------- | ------ | ------ | -------------------------------- |
| geometry                 | void*   | in     | pointer to Geometry                  |
| return                 | double | -      | attenuation coefficient in Np/mm |

## AUTDSetAttenuation (autd3capi)

Set attenuation coefficient.

| Argument name / return | type   | in/out | description                      |
| ---------------------- | ------ | ------ | -------------------------------- |
| geometry                 | void*   | in     | pointer to Geometry                  |
| attenuation            | double | in     | attenuation coefficient in Np/mm |
| return                 | void   | -      | -                                |

## AUTDGetFPGAInfo (autd3capi)

Get FPGA information.

Make sure set read FPGA info flag by `AUTDSetReadsFPGAInfo` before calling this function,

| Argument name / return | type     | in/out | description           |
| ---------------------- | -------- | ------ | --------------------- |
| handle                 | void*    | in     | pointer to Controller |
| out                    | uint8_t* | in     | FPGA information list |
| return                 | bool     | -      | true if success       |

## AUTDNumTransducers (autd3capi)

Get the number of transducers.

| Argument name / return | type    | in/out | description           |
| ---------------------- | ------- | ------ | --------------------- |
| geometry                 | void*   | in     | pointer to Geometry                  |
| return                 | int32_t | -      | number of transducers |

## AUTDNumDevices (autd3capi)

Get the number of devices.

| Argument name / return | type    | in/out | description           |
| ---------------------- | ------- | ------ | --------------------- |
| geometry                 | void*   | in     | pointer to Geometry                  |
| return                 | int32_t | -      | number of devices     |

## AUTDGeometryCenter (autd3capi)

Get the geometry center.

| Argument name / return | type    | in/out | description                         |
| ---------------------- | ------- | ------ | ----------------------------------- |
| geometry                 | void*   | in     | pointer to Geometry                  |
| x                      | double* | out    | x coordinate of center              |
| y                      | double* | out    | y coordinate of center              |
| z                      | double* | out    | z coordinate of center              |
| return                 | void    | -      | -                                   |

## AUTDGeometryCenterOf (autd3capi)

Get the device center.

| Argument name / return | type    | in/out | description                         |
| ---------------------- | ------- | ------ | ----------------------------------- |
| geometry                 | void*   | in     | pointer to Geometry                  |
| dev_idx                | int32_t | in     | device index                        |
| x                      | double* | out    | x coordinate of center              |
| y                      | double* | out    | y coordinate of center              |
| z                      | double* | out    | z coordinate of center              |
| return                 | void    | -      | -                                   |

## AUTDTransPosition (autd3capi)

Get the position of the transducer.

| Argument name / return | type    | in/out | description                         |
| ---------------------- | ------- | ------ | ----------------------------------- |
| geometry                 | void*   | in     | pointer to Geometry                  |
| trans_idx        | int32_t | in     | transducer index      |
| x                      | double* | out    | x coordinate of transducer position |
| y                      | double* | out    | y coordinate of transducer position |
| z                      | double* | out    | z coordinate of transducer position |
| return                 | void    | -      | -                                   |

## AUTDTransXDirection (autd3capi)

Get the x-direction of the transducer.

| Argument name / return | type    | in/out | description                 |
| ---------------------- | ------- | ------ | --------------------------- |
| geometry                 | void*   | in     | pointer to Geometry                  |
| trans_idx        | int32_t | in     | transducer index      |
| x                      | double* | out    | x coordinate of x-direction |
| y                      | double* | out    | y coordinate of x-direction |
| z                      | double* | out    | z coordinate of x-direction |
| return                 | void    | -      | -                           |

## AUTDTransYDirection (autd3capi)

Get the y-direction of the transducer.

| Argument name / return | type    | in/out | description                 |
| ---------------------- | ------- | ------ | --------------------------- |
| geometry                 | void*   | in     | pointer to Geometry                  |
| trans_idx        | int32_t | in     | transducer index      |
| x                      | double* | out    | x coordinate of y-direction |
| y                      | double* | out    | y coordinate of y-direction |
| z                      | double* | out    | z coordinate of y-direction |
| return                 | void    | -      | -                           |

## AUTDTransZDirection (autd3capi)

Get the z-direction of the transducer.

| Argument name / return | type    | in/out | description                 |
| ---------------------- | ------- | ------ | --------------------------- |
| geometry                 | void*   | in     | pointer to Geometry                  |
| trans_idx        | int32_t | in     | transducer index      |
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
| matches_version        | bool*   | out    | matches version                        |
| is_supported              | bool*   | out    | is latest                              |
| return                 | void    | -      | -                                      |

## AUTDFreeFirmwareInfoListPointer (autd3capi)

Free Firmware information list obtained by `AUTDGetFirmwareInfoListPointer`.

| Argument name / return | type  | in/out | description                          |
| ---------------------- | ----- | ------ | ------------------------------------ |
| p_firm_info_list       | void* | in     | pointer to Firmware information list |
| return                 | void  | -      | -                                    |

## AUTDGetLatestFirmware (autd3capi)

| Argument name / return | type  | in/out | description                          |
| ---------------------- | ----- | ------ | ------------------------------------ |
| latest_version         | char* | out    | pointer to latest firmware version   |
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
| return                 | void     | -      | -                               |

## AUTDGainTransducerTestSet (autd3capi)

Set amp and phase of TransducerTest gain.

| Argument name / return | type     | in/out | description                     |
| ---------------------- | -------  | ------ | ---------------------           |
| gain                   | void*    | in     | pointer to TransducerTest gain  |
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
| buffer                 | double*  | in     | pointer to modulation data              |
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

## AUTDFocusSTM (autd3capi)

Create Focus STM.

The stm created must be deleted at the end by `AUTDDeleteSTM`.

| Argument name / return | type   | in/out | description                     |
| ---------------------- | ------ | ------ | ------------------------------- |
| out                    | void** | out    | pointer to pointer to Focus STM |
| return                 | void   | -      | -                               |

## AUTDGainSTM (autd3capi)

Create Gain STM.

The stm created must be deleted at the end by `AUTDDeleteSTM`.

| Argument name / return | type   | in/out | description                    |
| ---------------------- | ------ | ------ | ------------------------------ |
| out                    | void** | out    | pointer to pointer to Gain STM |
| mode                   | uint16_t | in     | GainSTM mode (0x0001 = PhaseDutyFull, 0x0002 = PhaseFull, 0x0004 = PhaseHalf) |
| return                 | void   | -      | -                              |

## AUTDFocusSTMAdd (autd3capi)

Add focus to FocusSTM.

| Argument name / return | type    | in/out | description                 |
| ---------------------- | ------- | ------ | --------------------------- |
| stm                    | void*   | in     | pointer to Focus STM        |
| x                      | double  | in     | x coordinate of focal point |
| y                      | double  | in     | y coordinate of focal point |
| z                      | double  | in     | z coordinate of focal point |
| shift                  | uint8_t | in     | duty shift                  |
| return                 | void    | -      | -             |

## AUTDGainSTMAdd (autd3capi)

Add gain to GainSTM.

| Argument name / return | type  | in/out | description          |
| ---------------------- | ----- | ------ | -------------------- |
| stm                    | void* | in     | pointer to Focus STM |
| gain                   | void* | in     | pointer to Gain      |
| return                 | void  | -      | -      |

## AUTDSTMGetStartIdx (autd3capi)

Get start index of STM.

| Argument name / return | type   | in/out | description      |
| ---------------------- | ------ | ------ | ---------------- |
| stm                    | void*  | in     | pointer to STM   |
| return                 | int32_t | -      | start index (if < 0, nullopt) |

## AUTDSTMGetFinishIdx (autd3capi)

Get finish index of STM.

| Argument name / return | type   | in/out | description      |
| ---------------------- | ------ | ------ | ---------------- |
| stm                    | void*  | in     | pointer to STM   |
| return                 | int32_t | -      | finish index (if < 0, nullopt) |

## AUTDSTMSetStartIdx (autd3capi)

Set start index of STM.

| Argument name / return | type   | in/out | description      |
| ---------------------- | ------ | ------ | ---------------- |
| stm                    | void*  | in     | pointer to STM   |
| start_idx              | int32_t | in      | start index (if < 0, nullopt) |
| return                 | void | -      | - |

## AUTDSTMSetFinishIdx (autd3capi)

Set finish index of STM.

| Argument name / return | type   | in/out | description      |
| ---------------------- | ------ | ------ | ---------------- |
| stm                    | void*  | in     | pointer to STM   |
| finish_idx              | int32_t | in     | start index (if < 0, nullopt) |
| return                 | void | -      | - |

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

## AUTDUpdateFlags (autd3capi)

Create `UpdateFlag`.

The `UpdateFlag` created must be deleted at the end by `AUTDDeleteSpecialData`.

| Argument name / return | type   | in/out | description                    |
| ---------------------- | ------ | ------ | ------------------------------ |
| out                    | void** | out    | pointer to pointer to UpdateFlag |
| return                 | void   | -      | -                              |

## AUTDSynchronize (autd3capi)

Create `Synchronize`.

The `Synchronize` created must be deleted at the end by `AUTDDeleteSpecialData`.

| Argument name / return | type   | in/out | description                    |
| ---------------------- | ------ | ------ | ------------------------------ |
| out                    | void** | out    | pointer to pointer to Synchronize |
| return                 | void   | -      | -                              |

## AUTDStop (autd3capi)

Create `Stop`.

The `Stop` created must be deleted at the end by `AUTDDeleteSpecialData`.

| Argument name / return | type   | in/out | description                    |
| ---------------------- | ------ | ------ | ------------------------------ |
| out                    | void** | out    | pointer to pointer to Stop     |
| return                 | void   | -      | -                              |

## AUTDClear (autd3capi)

Create `Clear`.

The `Clear` created must be deleted at the end by `AUTDDeleteSpecialData`.

| Argument name / return | type   | in/out | description                    |
| ---------------------- | ------ | ------ | ------------------------------ |
| out                    | void** | out    | pointer to pointer to Clear    |
| return                 | void   | -      | -                              |

## AUTDModDelayConfig (autd3capi)

Create `ModDelayConfig`.

The `ModDelayConfig` created must be deleted at the end by `AUTDDeleteSpecialData`.

| Argument name / return | type   | in/out | description                    |
| ---------------------- | ------ | ------ | ------------------------------ |
| out                    | void** | out    | pointer to pointer to ModDelayConfig    |
| return                 | void   | -      | -                              |

## AUTDDeleteSpecialData (autd3capi)

Delete special data.

| Argument name / return | type  | in/out | description    |
| ---------------------- | ----- | ------ | -------------- |
| data                   | void* | in     | pointer to special data |
| return                 | void  | -      | -              |

## AUTDSend (autd3capi)

Send header and body data.

| Argument name / return | type    | in/out | description                                                                                            |
| ---------------------- | ------- | ------ | ------------------------------------------------------------------------------------------------------ |
| handle                 | void*   | in     | pointer to Controller                                                                                  |
| header                 | void*   | in     | pointer to header data                                                                                 |
| body                   | void*   | in     | pointer to body data                                                                                   |
| timeout_ns            |  int64_t| in     | timeout in ns (null if $<0$)                                                                                |
| return                 | bool    | -      | true if successful                                                                                     |

## AUTDSendSpecial (autd3capi)

Send special data.

| Argument name / return | type    | in/out | description                                                                                            |
| ---------------------- | ------- | ------ | ------------------------------------------------------------------------------------------------------ |
| handle                 | void*   | in     | pointer to Controller                                                                                  |
| special                | void*   | in     | pointer to special data                                                                                |
| timeout_ns            |  int64_t| in     | timeout in ns (null if $<0$)                                                                                |
| return                 | bool    | -      | true if successful                                                                                     |

## AUTDSetTransModDelay (autd3capi)

Configure Modulation Delay of the transducer.

| Argument name / return | type     | in/out | description                        |
| ---------------------- | -------- | ------ | ---------------------------------- |
| geometry                 | void*   | in     | pointer to Geometry                  |
| trans_idx        | int32_t | in     | transducer index      |
| delay                  | uint16_t | in     | modulation delay of the transducer |
| return                 | void     | -      | -                                  |

## AUTDGetTransModDelay (autd3capi)

Get Modulation Delay of the transducer.

| Argument name / return | type     | in/out | description                        |
| ---------------------- | -------- | ------ | ---------------------------------- |
| geometry                 | void*   | in     | pointer to Geometry                  |
| trans_idx        | int32_t | in     | transducer index      |
| return                 | uint16_t | -      | modulation delay of the transducer |

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
| strategy               | uint8_t | in     | TimerStrategy                   |
| return                 | void  | -      | -                                 |

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
| return                 | uint32_t | -      | sampling period        |

## AUTDSoftwareSTMSetSamplingPeriod (autd3capi)

Set SoftwareSTM sampling period in ns.

| Argument name / return | type     | in/out | description            |
| ---------------------- | -----    | ------ | -----------------------|
| stm                    | void*    | in     | pointer to SoftwareSTM |
| period                 | uint32_t | in     | sampling period        |
| return                 | void     | -      | -                      |

## AUTDDeleteSoftwareSTM (autd3capi)

Delete SoftwareSTM.

| Argument name / return | type  | in/out | description            |
| ---------------------- | ----- | ------ | ---------------------- |
| stm                    | void* | in     | pointer to SoftwareSTM |
| return                 | void  | -      | -                      |

## AUTDLinkDebug (autd3capi)

Create Debug link builder.

| Argument name / return | type     | in/out | description                         |
| ---------------------- | -------- | ------ | ----------------------------------- |
| out                    | void**   | out    | pointer to pointer to Debug link builder    |
| return                 | void     | -      | -                                   |

## AUTDLinkDebugLogLevel (autd3capi)

Set log level of Debug link.

| Argument name / return | type     | in/out | description                         |
| ---------------------- | -------- | ------ | ----------------------------------- |
| debug                    | void*   | in    | pointer to Debug link builder |
| level                  | int32_t  | in     | log level                          |
| return                 | void     | -      | -                                   |

## AUTDLinkDebugLogFunc (autd3capi)

Set log func of Debug link.

| Argument name / return | type     | in/out | description                         |
| ---------------------- | -------- | ------ | ----------------------------------- |
| debug                    | void*   | in    | pointer to Debug link builder |
| out_func               | void*    | in     | output callback                    |
| flush_func             | void*    | in     | flush callback                     |
| return                 | void     | -      | -                                   |

## AUTDLinkDebugTimeout (autd3capi)

Set timeout of Debug link.

| Argument name / return | type     | in/out | description                         |
| ---------------------- | -------- | ------ | ----------------------------------- |
| debug                    | void*   | in    | pointer to Debug link builder |
| timeout_ns                  | uint64_t  | in     | timeout in ns                          |
| return                 | void     | -      | -                                   |

## AUTDLinkDebugBuild (autd3capi)

Create Debug link.

| Argument name / return | type     | in/out | description                         |
| ---------------------- | -------- | ------ | ----------------------------------- |
| out                    | void**   | out    | pointer to Debug link |
| debug                    | void*   | in    | pointer to Debug link builder |
| return                 | void     | -      | -                                   |


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

## AUTDGainHoloEVP (autd3capi-gain-holo)

Create EVP holo gain.

The gain created must be deleted at the end by `AUTDDeleteGain`.

| Argument name / return | type   | in/out | description                    |
| ---------------------- | ------ | ------ | ------------------------------ |
| gain                   | void** | out    | pointer to pointer to EVP gain |
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

## AUTDLinkSimulator (autd3capi-link-simulator)

Create Simulator link builder.

| Argument name / return | type     | in/out | description                         |
| ---------------------- | -------- | ------ | ----------------------------------- |
| out                    | void**   | out    | pointer to pointer to Simulator link builder |
| return                 | void     | -      | -                                   |

## AUTDLinkSimulatorLogLevel (autd3capi-link-simulator)

Set log level of Simulator link.

| Argument name / return | type     | in/out | description                         |
| ---------------------- | -------- | ------ | ----------------------------------- |
| simulator                    | void*   | in    | pointer to Simulator link builder |
| level                  | int32_t  | in     | log level                          |
| return                 | void     | -      | -                                   |

## AUTDLinkSimulatorLogFunc (autd3capi-link-simulator)

Set log func of Simulator link.

| Argument name / return | type     | in/out | description                         |
| ---------------------- | -------- | ------ | ----------------------------------- |
| simulator                    | void*   | in    | pointer to Simulator link builder |
| out_func               | void*    | in     | output callback                    |
| flush_func             | void*    | in     | flush callback                     |
| return                 | void     | -      | -                                   |

## AUTDLinkSimulatorTimeout (autd3capi-link-simulator)

Set timeout of Simulator link.

| Argument name / return | type     | in/out | description                         |
| ---------------------- | -------- | ------ | ----------------------------------- |
| simulator                    | void*   | in    | pointer to Simulator link builder |
| timeout_ns                  | uint64_t  | in     | timeout in ns                          |
| return                 | void     | -      | -                                   |

## AUTDLinkSimulatorBuild (autd3capi-link-simulator)

Create Simulator link.

| Argument name / return | type     | in/out | description                         |
| ---------------------- | -------- | ------ | ----------------------------------- |
| out                    | void**   | out    | pointer to Simulator link |
| simulator                    | void*   | in    | pointer to Simulator link builder |
| return                 | void     | -      | -                                   |

## AUTDLinkRemoteTwinCAT (autd3capi-link-remote-twincat)

Create RemoteTwinCAT link builder.

| Argument name / return | type   | in/out | description                              |
| ---------------------- | ------ | ------ | ---------------------------------------- |
| out                    | void** | out    | pointer to pointer to RemoteTwinCAT link builder |
| server_ams_net_id      | char*  | in     | server ams net id                        |
| return                 | void   | -      | -                                        |


## AUTDLinkRemoteTwinCATServerIpAddr (autd3capi-link-remote-twincat)

Set server ip address of RemoteTwinCAT link.

| Argument name / return | type     | in/out | description                         |
| ---------------------- | -------- | ------ | ----------------------------------- |
| remote_twincat                    | void*   | in    | pointer to RemoteTwinCAT link builder |
| server_ip_addr         | char*  | in     | server ip address                        |
| return                 | void     | -      | -                                   |

## AUTDLinkRemoteTwinCATClientAmsNetId (autd3capi-link-remote-twincat)

Set client ams net id of RemoteTwinCAT link.

| Argument name / return | type     | in/out | description                         |
| ---------------------- | -------- | ------ | ----------------------------------- |
| remote_twincat                    | void*   | in    | pointer to RemoteTwinCAT link builder |
| client_ams_net_id      | char*  | in     | client ams net id                        |
| return                 | void     | -      | -                                   |

## AUTDLinkRemoteTwinCATLogLevel (autd3capi-link-remote-twincat)

Set log level of RemoteTwinCAT link.

| Argument name / return | type     | in/out | description                         |
| ---------------------- | -------- | ------ | ----------------------------------- |
| remote_twincat                    | void*   | in    | pointer to RemoteTwinCAT link builder |
| level                  | int32_t  | in     | log level                          |
| return                 | void     | -      | -                                   |

## AUTDLinkRemoteTwinCATLogFunc (autd3capi-link-remote-twincat)

Set log func of RemoteTwinCAT link.

| Argument name / return | type     | in/out | description                         |
| ---------------------- | -------- | ------ | ----------------------------------- |
| remote_twincat                    | void*   | in    | pointer to RemoteTwinCAT link builder |
| out_func               | void*    | in     | output callback                    |
| flush_func             | void*    | in     | flush callback                     |
| return                 | void     | -      | -                                   |

## AUTDLinkRemoteTwinCATTimeout (autd3capi-link-remote-twincat)

Set timeout of RemoteTwinCAT link.

| Argument name / return | type     | in/out | description                         |
| ---------------------- | -------- | ------ | ----------------------------------- |
| remote_twincat                    | void*   | in    | pointer to RemoteTwinCAT link builder |
| timeout_ns                  | uint64_t  | in     | timeout in ns                          |
| return                 | void     | -      | -                                   |

## AUTDLinkRemoteTwinCATBuild (autd3capi-link-remote-twincat)

Create RemoteTwinCAT link.

| Argument name / return | type     | in/out | description                         |
| ---------------------- | -------- | ------ | ----------------------------------- |
| out                    | void**   | out    | pointer to RemoteTwinCAT link |
| remote_twincat                    | void*   | in    | pointer to RemoteTwinCAT link builder |
| return                 | void     | -      | -                                   |


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
| out                    | void**   | out    | pointer to pointer to SOEM link builder |
| return                 | void     | -      | -                                  |

## AUTDLinkSOEMBufSize (autd3capi-link-soem)

| Argument name / return | type     | in/out | description                        |
| ---------------------- | -------- | ------ | ---------------------------------- |
| soem                    | void*   | in    | pointer to SOEM link builder |
| buf_size               | uint64_t | in     | buf size (unlimited if 0)          |
| return                 | void     | -      | -                                  |

## AUTDLinkSOEMIfname (autd3capi-link-soem)

| Argument name / return | type     | in/out | description                        |
| ---------------------- | -------- | ------ | ---------------------------------- |
| soem                    | void*   | in    | pointer to SOEM link builder |
| ifname                 | char*    | in     | interface name                     |
| return                 | void     | -      | -                                  |

## AUTDLinkSOEMSync0Cycle (autd3capi-link-soem)

| Argument name / return | type     | in/out | description                        |
| ---------------------- | -------- | ------ | ---------------------------------- |
| soem                    | void*   | in    | pointer to SOEM link builder |
| sync0_cycle            | uint16_t | in     | sync0 cycle                        |
| return                 | void     | -      | -                                  |

## AUTDLinkSOEMSendCycle (autd3capi-link-soem)

| Argument name / return | type     | in/out | description                        |
| ---------------------- | -------- | ------ | ---------------------------------- |
| soem                    | void*   | in    | pointer to SOEM link builder |
| send_cycle             | uint16_t | in     | send cycle                         |
| return                 | void     | -      | -                                  |

## AUTDLinkSOEMFreerun (autd3capi-link-soem)

| Argument name / return | type     | in/out | description                        |
| ---------------------- | -------- | ------ | ---------------------------------- |
| soem                    | void*   | in    | pointer to SOEM link builder |
| freerun                | bool     | in     | free run mode                      |
| return                 | void     | -      | -                                  |

## AUTDLinkSOEMOnLost (autd3capi-link-soem)

| Argument name / return | type     | in/out | description                        |
| ---------------------- | -------- | ------ | ---------------------------------- |
| soem                    | void*   | in    | pointer to SOEM link builder |
| on_lost                | void*    | in     | pointer to on-lost callback        |
| return                 | void     | -      | -                                  |

## AUTDLinkSOEMTimerStrategy (autd3capi-link-soem)

| Argument name / return | type     | in/out | description                        |
| ---------------------- | -------- | ------ | ---------------------------------- |
| soem                    | void*   | in    | pointer to SOEM link builder |
| timer_strategy         | uint8_t  | in     | TimerStrategy                      |
| return                 | void     | -      | -                                  |

## AUTDLinkSOEMStateCheckInterval (autd3capi-link-soem)

| Argument name / return | type     | in/out | description                        |
| ---------------------- | -------- | ------ | ---------------------------------- |
| soem                    | void*   | in    | pointer to SOEM link builder |
| state_check_interval   | uint64_t | in     | state check interval in ms         |
| return                 | void     | -      | -                                  |

## AUTDLinkSOEMLogLevel (autd3capi-link-soem)

| Argument name / return | type     | in/out | description                        |
| ---------------------- | -------- | ------ | ---------------------------------- |
| soem                    | void*   | in    | pointer to SOEM link builder |
| level                  | int32_t  | in     | log level                          |
| return                 | void     | -      | -                                  |

## AUTDLinkSOEMLogFunc (autd3capi-link-soem)

| Argument name / return | type     | in/out | description                        |
| ---------------------- | -------- | ------ | ---------------------------------- |
| soem                    | void*   | in    | pointer to SOEM link builder |
| out_func               | void*    | in     | output callback                    |
| flush_func             | void*    | in     | flush callback                     |
| return                 | void     | -      | -                                  |

## AUTDLinkSOEMTimeout (autd3capi-link-soem)

| Argument name / return | type     | in/out | description                        |
| ---------------------- | -------- | ------ | ---------------------------------- |
| soem                    | void*   | in    | pointer to SOEM link builder |
| timeout_ns                  | uint64_t  | in     | timeout in ns                          |
| return                 | void     | -      | -                                  |

## AUTDLinkSOEMBuild (autd3capi-link-soem)

| Argument name / return | type     | in/out | description                        |
| ---------------------- | -------- | ------ | ---------------------------------- |
| out                    | void**   | out    | pointer to pointer to SOEM link |
| soem                    | void*  | in    | pointer to SOEM link builder |
| return                 | void     | -      | -                                  |

## AUTDLinkRemoteSOEM (autd3capi-link-remote-soem)

Create RemoteSOEM link builder.

| Argument name / return | type    | in/out | description                              |
| ---------------------- | ------  | ------ | ---------------------------------------- |
| out                    | void**  | out    | pointer to pointer to RemoteSOEM link    |
| ip                     | char*   | in     | server ip address                        |
| port                   | uint16_t| in     | port                                     |
| return                 | void    | -      | -                                        |

## AUTDLinkRemoteSOEMLogLevel (autd3capi-link-remote-soem)

Set log level of RemoteSOEM link.

| Argument name / return | type     | in/out | description                         |
| ---------------------- | -------- | ------ | ----------------------------------- |
| remote_soem                    | void*   | in    | pointer to RemoteSOEM link builder |
| level                  | int32_t  | in     | log level                          |
| return                 | void     | -      | -                                   |

## AUTDLinkRemoteSOEMLogFunc (autd3capi-link-remote-soem)

Set log func of RemoteSOEM link.

| Argument name / return | type     | in/out | description                         |
| ---------------------- | -------- | ------ | ----------------------------------- |
| remote_soem                    | void*   | in    | pointer to RemoteSOEM link builder |
| out_func               | void*    | in     | output callback                    |
| flush_func             | void*    | in     | flush callback                     |
| return                 | void     | -      | -                                   |

## AUTDLinkRemoteSOEMTimeout (autd3capi-link-remote-soem)

Set timeout of RemoteSOEM link.

| Argument name / return | type     | in/out | description                         |
| ---------------------- | -------- | ------ | ----------------------------------- |
| remote_soem                    | void*   | in    | pointer to RemoteSOEM link builder |
| timeout_ns                  | uint64_t  | in     | timeout in ns                          |
| return                 | void     | -      | -                                   |

## AUTDLinkRemoteSOEMBuild (autd3capi-link-remote-soem)

Create RemoteSOEM link.

| Argument name / return | type     | in/out | description                         |
| ---------------------- | -------- | ------ | ----------------------------------- |
| out                    | void**   | out    | pointer to RemoteSOEM link |
| remote_soem                    | void*   | in    | pointer to RemoteSOEM link builder |
| return                 | void     | -      | -                                   |

## AUTDLinkTwinCAT (autd3capi-link-twincat)

Create TwinCAT link buider.

| Argument name / return | type   | in/out | description                        |
| ---------------------- | ------ | ------ | ---------------------------------- |
| out                    | void** | out    | pointer to pointer to TwinCAT link builder |
| return                 | void   | -      | -                                  |


## AUTDLinkTwinCATLogLevel (autd3capi-link-twincat)

Set log level of TwinCAT link.

| Argument name / return | type     | in/out | description                         |
| ---------------------- | -------- | ------ | ----------------------------------- |
| twincat                    | void*   | in    | pointer to TwinCAT link builder |
| level                  | int32_t  | in     | log level                          |
| return                 | void     | -      | -                                   |

## AUTDLinkTwinCATLogFunc (autd3capi-link-twincat)

Set log func of TwinCAT link.

| Argument name / return | type     | in/out | description                         |
| ---------------------- | -------- | ------ | ----------------------------------- |
| twincat                    | void*   | in    | pointer to TwinCAT link builder |
| out_func               | void*    | in     | output callback                    |
| flush_func             | void*    | in     | flush callback                     |
| return                 | void     | -      | -                                   |

## AUTDLinkTwinCATTimeout (autd3capi-link-twincat)

Set timeout of TwinCAT link.

| Argument name / return | type     | in/out | description                         |
| ---------------------- | -------- | ------ | ----------------------------------- |
| twincat                    | void*   | in    | pointer to TwinCAT link builder |
| timeout_ns                  | uint64_t  | in     | timeout in ns                          |
| return                 | void     | -      | -                                   |

## AUTDLinkTwinCATBuild (autd3capi-link-twincat)

Create TwinCAT link.

| Argument name / return | type     | in/out | description                         |
| ---------------------- | -------- | ------ | ----------------------------------- |
| out                    | void**   | out    | pointer to TwinCAT link |
| twincat                    | void*   | in    | pointer to TwinCAT link builder |
| return                 | void     | -      | -                                   |


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
| geometry                 | void*   | in     | pointer to Geometry                  |
| width                  | int32_t | in     | window width                       |
| height                 | int32_t | in     | window height                      |
| vsync                  | bool    | in     | vsync                              |
| gpu_idx                | int32_t | in     | GPU index                          |
| return                 | bool    | -      | true if successful                                                                                     |

## AUTDExtraSimulator (autd3capi-extra-simulator)

Run Simulator.

If a configuration file exists in `settings_path`, the parameters (`vsync`, and `gpu_idx`) are overridden by the contents of the configuration file.

| Argument name / return | type    | in/out | description                        |
| ---------------------- | ------- | ------ | ---------------------------------- |
| settings_path          | char*   | in     | path to setting file               |
| vsync                  | bool    | in     | vsync                              |
| gpu_idx                | int32_t | in     | GPU index                          |
| return                 | bool    | -      | true if successful                                                                                     |
