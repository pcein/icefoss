/* automatically generated by rust-bindgen */

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]
pub const NUM_PINS: ::std::os::raw::c_uchar = 17;
pub const WPI_MODE_PINS: ::std::os::raw::c_uchar = 0;
pub const WPI_MODE_GPIO: ::std::os::raw::c_uchar = 1;
pub const WPI_MODE_GPIO_SYS: ::std::os::raw::c_uchar = 2;
pub const WPI_MODE_PHYS: ::std::os::raw::c_uchar = 3;
pub const WPI_MODE_PIFACE: ::std::os::raw::c_uchar = 4;
pub const WPI_MODE_UNINITIALISED: ::std::os::raw::c_schar = -1;
pub const INPUT: ::std::os::raw::c_uchar = 0;
pub const OUTPUT: ::std::os::raw::c_uchar = 1;
pub const PWM_OUTPUT: ::std::os::raw::c_uchar = 2;
pub const GPIO_CLOCK: ::std::os::raw::c_uchar = 3;
pub const SOFT_PWM_OUTPUT: ::std::os::raw::c_uchar = 4;
pub const SOFT_TONE_OUTPUT: ::std::os::raw::c_uchar = 5;
pub const PWM_TONE_OUTPUT: ::std::os::raw::c_uchar = 6;
pub const LOW: ::std::os::raw::c_uchar = 0;
pub const HIGH: ::std::os::raw::c_uchar = 1;
pub const PUD_OFF: ::std::os::raw::c_uchar = 0;
pub const PUD_DOWN: ::std::os::raw::c_uchar = 1;
pub const PUD_UP: ::std::os::raw::c_uchar = 2;
pub const PWM_MODE_MS: ::std::os::raw::c_uchar = 0;
pub const PWM_MODE_BAL: ::std::os::raw::c_uchar = 1;
pub const INT_EDGE_SETUP: ::std::os::raw::c_uchar = 0;
pub const INT_EDGE_FALLING: ::std::os::raw::c_uchar = 1;
pub const INT_EDGE_RISING: ::std::os::raw::c_uchar = 2;
pub const INT_EDGE_BOTH: ::std::os::raw::c_uchar = 3;
pub const PI_MODEL_UNKNOWN: ::std::os::raw::c_uchar = 0;
pub const PI_VERSION_UNKNOWN: ::std::os::raw::c_uchar = 0;
pub const PI_VERSION_1: ::std::os::raw::c_uchar = 1;
pub const PI_VERSION_1_1: ::std::os::raw::c_uchar = 2;
pub const PI_VERSION_1_2: ::std::os::raw::c_uchar = 3;
pub const PI_VERSION_2: ::std::os::raw::c_uchar = 4;
pub const PI_MAKER_UNKNOWN: ::std::os::raw::c_uchar = 0;
pub const PI_MAKER_EGOMAN: ::std::os::raw::c_uchar = 1;
pub const PI_MAKER_SONY: ::std::os::raw::c_uchar = 2;
pub const PI_MAKER_QISDA: ::std::os::raw::c_uchar = 3;
pub const PI_MAKER_LEMAKER: ::std::os::raw::c_uchar = 4;
pub const PI_MAKER_FRIENDLYELEC: ::std::os::raw::c_uchar = 5;
pub const BPRVER: ::std::os::raw::c_uchar = 3;
pub const S3C6410_COMMON: ::std::os::raw::c_ushort = 6410;
pub const S5PV210_COMMON: ::std::os::raw::c_uchar = 210;
pub const S5P4412_COMMON: ::std::os::raw::c_ushort = 4412;
pub const S5P4418_BASE: ::std::os::raw::c_ushort = 4418;
pub const NanoPi2: ::std::os::raw::c_ushort = 4418;
pub const NanoPC_T2: ::std::os::raw::c_ushort = 4419;
pub const NanoPi_S2: ::std::os::raw::c_ushort = 4420;
pub const Smart4418: ::std::os::raw::c_ushort = 4421;
pub const NanoPi2_Fire: ::std::os::raw::c_ushort = 4422;
pub const NanoPi_M2: ::std::os::raw::c_ushort = 4423;
pub const NanoPi_M2A: ::std::os::raw::c_ushort = 4425;
pub const Smart4418SDK: ::std::os::raw::c_ushort = 4677;
pub const S5P4418_MAX: ::std::os::raw::c_ushort = 4677;
pub const S5P6818_BASE: ::std::os::raw::c_ushort = 6818;
pub const NanoPC_T3: ::std::os::raw::c_ushort = 6819;
pub const NanoPi_S3: ::std::os::raw::c_ushort = 6820;
pub const Smart6818: ::std::os::raw::c_ushort = 6821;
pub const NanoPC_T3T: ::std::os::raw::c_ushort = 6822;
pub const NanoPi_M3: ::std::os::raw::c_ushort = 6825;
pub const S5P6818_MAX: ::std::os::raw::c_ushort = 6825;
pub const S3C2451_BASE: ::std::os::raw::c_ushort = 2451;
pub const S3C2451_COMMON: ::std::os::raw::c_ushort = 2451;
pub const ALLWINNER_BASE: ::std::os::raw::c_ushort = 7000;
pub const NanoPi_M1: ::std::os::raw::c_ushort = 7001;
pub const NanoPi_NEO: ::std::os::raw::c_ushort = 7002;
pub const NanoPi_NEO_Air: ::std::os::raw::c_ushort = 7003;
pub const NanoPi_M1_Plus: ::std::os::raw::c_ushort = 7004;
pub const NanoPi_A64: ::std::os::raw::c_ushort = 7005;
pub const NanoPi_NEO2: ::std::os::raw::c_ushort = 7006;
pub const NanoPi_M1_Plus2: ::std::os::raw::c_ushort = 7007;
pub const NanoPi_NEO_Plus2: ::std::os::raw::c_ushort = 7008;
pub const NanoPi_NEO_Core: ::std::os::raw::c_ushort = 7009;
pub const NanoPi_Duo: ::std::os::raw::c_ushort = 7010;
pub const ALLWINNER_MAX: ::std::os::raw::c_ushort = 7010;
pub const AMLOGIC_BASE: ::std::os::raw::c_ushort = 8000;
pub const NanoPi_K2: ::std::os::raw::c_ushort = 8001;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct wiringPiNodeStruct {
    pub pinBase: ::std::os::raw::c_int,
    pub pinMax: ::std::os::raw::c_int,
    pub fd: ::std::os::raw::c_int,
    pub data0: ::std::os::raw::c_uint,
    pub data1: ::std::os::raw::c_uint,
    pub data2: ::std::os::raw::c_uint,
    pub data3: ::std::os::raw::c_uint,
    pub pinMode: ::std::option::Option<unsafe extern "C" fn(node:
                                                                *mut wiringPiNodeStruct,
                                                            pin:
                                                                ::std::os::raw::c_int,
                                                            mode:
                                                                ::std::os::raw::c_int)>,
    pub pullUpDnControl: ::std::option::Option<unsafe extern "C" fn(node:
                                                                        *mut wiringPiNodeStruct,
                                                                    pin:
                                                                        ::std::os::raw::c_int,
                                                                    mode:
                                                                        ::std::os::raw::c_int)>,
    pub digitalRead: ::std::option::Option<unsafe extern "C" fn(node:
                                                                    *mut wiringPiNodeStruct,
                                                                pin:
                                                                    ::std::os::raw::c_int)
                                               -> ::std::os::raw::c_int>,
    pub digitalWrite: ::std::option::Option<unsafe extern "C" fn(node:
                                                                     *mut wiringPiNodeStruct,
                                                                 pin:
                                                                     ::std::os::raw::c_int,
                                                                 value:
                                                                     ::std::os::raw::c_int)>,
    pub pwmWrite: ::std::option::Option<unsafe extern "C" fn(node:
                                                                 *mut wiringPiNodeStruct,
                                                             pin:
                                                                 ::std::os::raw::c_int,
                                                             value:
                                                                 ::std::os::raw::c_int)>,
    pub analogRead: ::std::option::Option<unsafe extern "C" fn(node:
                                                                   *mut wiringPiNodeStruct,
                                                               pin:
                                                                   ::std::os::raw::c_int)
                                              -> ::std::os::raw::c_int>,
    pub analogWrite: ::std::option::Option<unsafe extern "C" fn(node:
                                                                    *mut wiringPiNodeStruct,
                                                                pin:
                                                                    ::std::os::raw::c_int,
                                                                value:
                                                                    ::std::os::raw::c_int)>,
    pub next: *mut wiringPiNodeStruct,
}
impl ::std::default::Default for wiringPiNodeStruct {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct BoardHardwareInfo {
    pub kernelHardware: [::std::os::raw::c_uchar; 255usize],
    pub kernelRevision: ::std::os::raw::c_int,
    pub boardTypeId: ::std::os::raw::c_int,
    pub boardDisplayName: [::std::os::raw::c_uchar; 255usize],
    pub allwinnerBoardID: [::std::os::raw::c_uchar; 255usize],
}
impl ::std::clone::Clone for BoardHardwareInfo {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for BoardHardwareInfo {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
extern "C" {
    pub static mut wiringPiNodes: *mut wiringPiNodeStruct;
}
extern "C" {
    pub fn wiringPiFindNode(pin: ::std::os::raw::c_int)
     -> *mut wiringPiNodeStruct;
    pub fn wiringPiNewNode(pinBase: ::std::os::raw::c_int,
                           numPins: ::std::os::raw::c_int)
     -> *mut wiringPiNodeStruct;
    pub fn wiringPiSetup() -> ::std::os::raw::c_int;
    pub fn wiringPiSetupSys() -> ::std::os::raw::c_int;
    pub fn wiringPiSetupGpio() -> ::std::os::raw::c_int;
    pub fn wiringPiSetupPhys() -> ::std::os::raw::c_int;
    pub fn pinModeAlt(pin: ::std::os::raw::c_int,
                      mode: ::std::os::raw::c_int);
    pub fn pinMode(pin: ::std::os::raw::c_int, mode: ::std::os::raw::c_int);
    pub fn pullUpDnControl(pin: ::std::os::raw::c_int,
                           pud: ::std::os::raw::c_int);
    pub fn digitalRead(pin: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn digitalWrite(pin: ::std::os::raw::c_int,
                        value: ::std::os::raw::c_int);
    pub fn pwmWrite(pin: ::std::os::raw::c_int, value: ::std::os::raw::c_int);
    pub fn analogRead(pin: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn analogWrite(pin: ::std::os::raw::c_int,
                       value: ::std::os::raw::c_int);
    pub fn wiringPiSetupPiFace() -> ::std::os::raw::c_int;
    pub fn wiringPiSetupPiFaceForGpioProg() -> ::std::os::raw::c_int;
    pub fn piBoardRev() -> ::std::os::raw::c_int;
    pub fn piBoardId(model: *mut ::std::os::raw::c_int,
                     rev: *mut ::std::os::raw::c_int,
                     mem: *mut ::std::os::raw::c_int,
                     maker: *mut ::std::os::raw::c_int,
                     overVolted: *mut ::std::os::raw::c_int);
    pub fn wpiPinToGpio(wpiPin: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn physPinToGpio(physPin: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn physPinToPin(physPin: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn setPadDrive(group: ::std::os::raw::c_int,
                       value: ::std::os::raw::c_int);
    pub fn getAlt(pin: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn pwmToneWrite(pin: ::std::os::raw::c_int,
                        freq: ::std::os::raw::c_int);
    pub fn digitalWriteByte(value: ::std::os::raw::c_int);
    pub fn pwmSetMode(mode: ::std::os::raw::c_int);
    pub fn pwmSetRange(range: ::std::os::raw::c_uint);
    pub fn pwmSetClock(divisor: ::std::os::raw::c_int);
    pub fn gpioClockSet(pin: ::std::os::raw::c_int,
                        freq: ::std::os::raw::c_int);
    pub fn waitForInterrupt(pin: ::std::os::raw::c_int,
                            mS: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
    pub fn wiringPiISR(pin: ::std::os::raw::c_int,
                       mode: ::std::os::raw::c_int,
                       function: ::std::option::Option<extern "C" fn()>)
     -> ::std::os::raw::c_int;
    pub fn piThreadCreate(fn_:
                              ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                             *mut ::std::os::raw::c_void)
                                                        ->
                                                            *mut ::std::os::raw::c_void>)
     -> ::std::os::raw::c_int;
    pub fn piLock(key: ::std::os::raw::c_int);
    pub fn piUnlock(key: ::std::os::raw::c_int);
    pub fn piHiPri(pri: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn delay(howLong: ::std::os::raw::c_uint);
    pub fn delayMicroseconds(howLong: ::std::os::raw::c_uint);
    pub fn millis() -> ::std::os::raw::c_uint;
    pub fn micros() -> ::std::os::raw::c_uint;
    pub fn getBoardType(retBoardInfo: *mut *mut BoardHardwareInfo)
     -> ::std::os::raw::c_int;
}
