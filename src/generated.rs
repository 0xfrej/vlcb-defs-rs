use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use num_enum::UnsafeFromPrimitive;
use bitflags::bitflags;
/// VLCB Service Types
#[derive(
    Debug,
    Copy,
    Clone,
    TryFromPrimitive,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum VlcbServiceTypes {
    /// Internal service.
    ///
    /// Can be used for implementing certain behaviors while
    /// using the service pattern for the implementation.
    ///
    /// Will not be shown in list of services requested by RQSD.
    Internal = 0,
    /// The minimum node service.
    ///
    /// All modules must implement this.
    MinimumNodeService = 1,
    /// The NV service.
    NodeVariable = 2,
    /// CAN service.
    ///
    /// Deals with CANID enumeration.
    CanBus = 3,
    /// Old (CBUS) event teaching service.
    LegacyEventTeaching = 4,
    /// Event producer service.
    EventProducer = 5,
    /// Event comsumer service.
    EventConsumer = 6,
    /// New event teaching service.
    EventTeaching = 7,
    /// Consume own events service.
    ConsumeOwnEvents = 8,
    /// Event acknowledge service.
    ///
    /// Useful for debugging event configuration.
    EventAcknowledge = 9,
    /// FCU/PIC bootloader service.
    Bootloader = 10,
    /// Streaming (Long Messages) service.
    Streaming = 17,
}
/// Parameters to the MODE op-code
///
/// Exclusive modes
#[derive(
    Debug,
    Copy,
    Clone,
    TryFromPrimitive,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum VlcbModeParams {
    /// Uninitialised / factory settings
    Uninitialised = 255,
    /// Set up mode
    InSetup = 0,
    /// Normal operation mode
    Normal = 1,
    /// Turn on learn mode
    EnableLearnMode = 8,
    /// Turn off learn mode
    DisableLearnMode = 9,
    /// Turn on event acknowledgements
    EnableEventAck = 10,
    /// Turn off event acknowledgements
    DisableEventAck = 11,
    /// Turn on heartbeat
    EnableHeartbeat = 12,
    /// Turn off heartbeat
    DisableHeartbeat = 13,
    /// PIC Boot loader mode
    Bootloader = 14,
}
/// GRSP codes
#[derive(
    Debug,
    Copy,
    Clone,
    TryFromPrimitive,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum VlcbGrspCodes {
    /// Success
    Ok = 0,
    /// Unknown non volatile memory type
    UnknownPersistentMemoryType = 254,
    /// Invalid diagnostic
    InvalidDiagnostic = 253,
    /// Invalid service
    InvalidService = 252,
    /// Invalid parameter in command
    InvalidCommandParameter = 251,
    /// Invalid Mode
    InvalidMode = 250,
}
/// SysPixie Module types (Konrad Orlowski)
#[derive(
    Debug,
    Copy,
    Clone,
    TryFromPrimitive,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CbusSysPixieModuleTypes {
    /// Motorised point motor driver with current sense
    CANPMSense = 1,
}
/// Modes for STMOD
#[derive(
    Debug,
    Copy,
    Clone,
    TryFromPrimitive,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CbusStmodModes {
    /// 128-step speed mode
    Step128 = 0,
    /// 14-step speed mode
    Step14 = 1,
    /// interleaved 28-step speed mode
    Step28Interleaved = 2,
    /// 28-step speed mode
    Step28 = 3,
}
/// Status codes for OPC_SSTAT
#[derive(
    Debug,
    Copy,
    Clone,
    TryFromPrimitive,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CbusSStats {
    NoAck = 1,
    Ovld = 2,
    WriteAck = 3,
    Busy = 4,
    CvError = 5,
}
/// Sprog Module types
#[derive(
    Debug,
    Copy,
    Clone,
    TryFromPrimitive,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CbusSprogModuleTypes {
    /// Pi-SPROG 3 programmer/command station
    CANPiSPRG3 = 1,
    /// SPROG 3 Plus programmer/command station
    CANSPROG3P = 2,
    /// CAN SPROG programmer/command station
    CANSPROG = 3,
    /// System Booster
    CANSBOOST = 4,
    /// Pi-SPROG 3 Plus programmer/command station
    CANPiSPRGP = 5,
    /// 8-channel (4-pairs) Solenoid I/O module
    CANSOLNOID = 8,
    /// 8-channel Servo I/O module
    CANSERVOIO = 50,
    /// CAN ISB Isolated CAN USB Interface
    CANISB = 100,
    /// 8-channel (4-pairs) Solenoid I/O module
    CANSOLIO = 101,
}
/// Spectrum Engineering Animated Modeller module types
#[derive(
    Debug,
    Copy,
    Clone,
    TryFromPrimitive,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CbusSpectrumModuleTypes {
    /// Animation controller (firmware derived from cancmd)
    AMCTRLR = 1,
    /// Dual cab based on cancab
    DUALCAB = 2,
}
/// Rocrail Module types
#[derive(
    Debug,
    Copy,
    Clone,
    TryFromPrimitive,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CbusRocRailModuleTypes {
    /// RS232 PC interface
    CANGC1 = 1,
    /// 16 I/O
    CANGC2 = 2,
    /// Command station (derived from cancmd)
    CANGC3 = 3,
    /// 8 channel RFID reader
    CANGC4 = 4,
    /// Cab for fixed panels (derived from cancab)
    CANGC5 = 5,
    /// 4 channel servo controller
    CANGC6 = 6,
    /// Fast clock module
    CANGC7 = 7,
    /// CAN<->Ethernet interface
    CANGC1e = 11,
}
/// Processor manufacturer codes
#[derive(
    Debug,
    Copy,
    Clone,
    TryFromPrimitive,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CbusProcessorManufacturers {
    Microchip = 1,
    Atmel = 2,
    Arm = 3,
}
/// Parameter index numbers (readable by OPC_RQNPN, returned in OPC_PARAN)
/// Index numbers count from 1, subtract 1 for offset into parameter block
/// Note that RQNPN with index 0 returns the parameter count
#[derive(
    Debug,
    Copy,
    Clone,
    TryFromPrimitive,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CbusParams {
    /// Number of parameters
    ModuleParameterCount = 0,
    /// Manufacturer id
    ModuleManufacturer = 1,
    /// Minor version (single alphabetic character)
    MinorVersion = 2,
    /// Module type code
    ModuleType = 3,
    /// Number of events supported
    MaxEventCount = 4,
    /// Event variables per event
    EventVariableCount = 5,
    /// Number of Node variables
    NodeVariableCount = 6,
    /// Major version (numeric)
    MajorVersion = 7,
    /// Node flags
    NodeFlags = 8,
    /// Processor type
    CpuId = 9,
    /// Bus type
    BusType = 10,
    /// load address, 4 bytes
    LoadAddress = 11,
    /// CPU manufacturer's id as read from the chip config space, 4 bytes (note - read from cpu at runtime, so not included in checksum)
    CpuManufacturerId = 15,
    /// CPU manufacturer code
    CpuManufacturer = 19,
    /// Beta revision (numeric), or 0 if release
    BetaVersion = 20,
}
/// Offsets to other values stored at the top of the parameter block.
/// These are not returned by opcode PARAN, but are present in the hex
/// file for FCU.
#[derive(
    Debug,
    Copy,
    Clone,
    TryFromPrimitive,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CbusParamOffsetsPic {
    /// Number of parameters implemented
    COUNT = 24,
    /// 4 byte Address of Module type name, up to 8 characters null terminated
    NAME = 26,
    /// Checksum word at end of parameters
    CKSUM = 30,
}
bitflags! {
    #[doc = " Flags in PAR_FLAGS"] #[derive(Debug, Copy, Clone)] pub struct
    CbusParamFlags : u8 { #[doc = " Module doesn't support events"] const
    EventsUnsupported = 0b00000000; #[doc = " Module is a consumer of events"] const
    EventConsumer = 0b00000001; #[doc = " Module is a producer of events"] const
    EventProducer = 0b00000010; #[doc =
    " Module is both a consumer and producer of events"] const EventCombi = 0b00000011;
    #[doc = " Module is in FLiM (CBUS)"] const FLiM = 0b00000100; #[doc =
    " Module is in Normal mode (VLCB)"] const NormalMode = 0b00000100; #[doc =
    " Module supports the FCU bootloader protocol"] const Bootloader = 0b00001000; #[doc
    = " Module can consume its own events"] const ConsumeOwnEvents = 0b00010000; #[doc =
    " Module is in learn mode"] const LearnMode = 0b00100000; #[doc =
    " Module is VLCB compatible"] const VLCB = 0b01000000; #[doc =
    " Module supports Service Discovery (Deprecated in favour of PF_VLCB.)"] const
    ServiceDiscovery = 0b01000000; }
}
/// VLCB opcodes list
/// Packets with no data bytes
#[derive(
    Debug,
    Copy,
    Clone,
    TryFromPrimitive,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CbusOpCodes {
    /// General ack
    ACK = 0,
    /// General nak
    NAK = 1,
    /// Bus Halt
    HLT = 2,
    /// Bus on
    BON = 3,
    /// Track off
    TOF = 4,
    /// Track on
    TON = 5,
    /// Track stopped
    ESTOP = 6,
    /// System reset
    ARST = 7,
    /// Request track off
    RTOF = 8,
    /// Request track on
    RTON = 9,
    /// Request emergency stop all
    RESTP = 10,
    /// Request node status
    RSTAT = 12,
    /// Query nodes
    QNN = 13,
    /// Read node parameters
    RQNP = 16,
    /// Request name of module type
    RQMN = 17,
    /// Release engine by handle
    KLOC = 33,
    /// Query engine by handle
    QLOC = 34,
    /// Keep alive for cab
    DKEEP = 35,
    /// Debug message with 1 status byte
    DBG1 = 48,
    /// Extended opcode
    EXTC = 63,
    /// Request session for loco
    RLOC = 64,
    /// Query consist
    QCON = 65,
    /// Set node number
    SNN = 66,
    /// Allocate loco (used to allocate to a shuttle in cancmd)
    ALOC = 67,
    /// Set Throttle mode
    STMOD = 68,
    /// Consist loco
    PCON = 69,
    /// De-consist loco
    KCON = 70,
    /// Loco speed/dir
    DSPD = 71,
    /// Set engine flags
    DFLG = 72,
    /// Loco function on
    DFNON = 73,
    /// Loco function off
    DFNOF = 74,
    /// Service mode status
    SSTAT = 76,
    /// Reset to manufacturer's defaults
    NNRSM = 79,
    /// Request Node number in setup mode
    RQNN = 80,
    /// Node number release
    NNREL = 81,
    /// Node number acknowledge
    NNACK = 82,
    /// Set learn mode
    NNLRN = 83,
    /// Release learn mode
    NNULN = 84,
    /// Clear all events
    NNCLR = 85,
    /// Read available event slots
    NNEVN = 86,
    /// Read all stored events
    NERD = 87,
    /// Read number of stored events
    RQEVN = 88,
    /// Write acknowledge
    WRACK = 89,
    /// Request node data event
    RQDAT = 90,
    /// Request short data frame
    RQDDS = 91,
    /// Put node into boot mode
    BOOT = 92,
    /// Force can_id self enumeration
    ENUM = 93,
    /// Reset node (as in restart)
    NNRST = 94,
    /// Extended opcode with 1 data byte
    EXTC1 = 95,
    /// Set engine functions
    DFUN = 96,
    /// Get loco (with support for steal/share)
    GLOC = 97,
    /// Command station error
    ERR = 99,
    /// Errors from nodes during config
    CMDERR = 111,
    /// Event slots left response
    EVNLF = 112,
    /// Request read of node variable
    NVRD = 113,
    /// Request read stored event by index
    NENRD = 114,
    /// Request read module parameters
    RQNPN = 115,
    /// Number of events stored response
    NUMEV = 116,
    /// Set canid
    CANID = 117,
    /// Set mode
    MODE = 118,
    /// Request service discovery
    RQSD = 120,
    /// Extended opcode with 2 data bytes
    EXTC2 = 127,
    /// 3 byte DCC packet
    RDCC3 = 128,
    /// Write CV byte Ops mode by handle
    WCVO = 130,
    /// Write CV bit Ops mode by handle
    WCVB = 131,
    /// Read CV
    QCVS = 132,
    /// Report CV
    PCVS = 133,
    /// Request diagnostics
    RDGN = 135,
    /// Set NV with Read
    NVSETRD = 142,
    /// on event
    ACON = 144,
    /// off event
    ACOF = 145,
    /// Accessory Request event
    AREQ = 146,
    /// Accessory response event on
    ARON = 147,
    /// Accessory response event off
    AROF = 148,
    /// Unlearn event
    EVULN = 149,
    /// Set a node variable
    NVSET = 150,
    /// Node variable value response
    NVANS = 151,
    /// Short event on
    ASON = 152,
    /// Short event off
    ASOF = 153,
    /// Short Request event
    ASRQ = 154,
    /// Single node parameter response
    PARAN = 155,
    /// Request read of event variable
    REVAL = 156,
    /// Accessory short response on event
    ARSON = 157,
    /// Accessory short response off event
    ARSOF = 158,
    /// Extended opcode with 3 data bytes
    EXTC3 = 159,
    /// 4 byte DCC packet
    RDCC4 = 160,
    /// Write CV service mode
    WCVS = 162,
    /// Heartbeat
    HEARTB = 171,
    /// Service discovery response
    SD = 172,
    /// General response
    GRSP = 175,
    /// On event with one data byte
    ACON1 = 176,
    /// Off event with one data byte
    ACOF1 = 177,
    /// Read event variable in learn mode
    REQEV = 178,
    /// Accessory on response (1 data byte)
    ARON1 = 179,
    /// Accessory off response (1 data byte)
    AROF1 = 180,
    /// Event variable by index read response
    NEVAL = 181,
    /// Response to QNN
    PNN = 182,
    /// Accessory short on with 1 data byte
    ASON1 = 184,
    /// Accessory short off with 1 data byte
    ASOF1 = 185,
    /// Short response event on with one data byte
    ARSON1 = 189,
    /// Short response event off with one data byte
    ARSOF1 = 190,
    /// Extended opcode with 4 data bytes
    EXTC4 = 191,
    /// 5 byte DCC packet
    RDCC5 = 192,
    /// Write CV ops mode by address
    WCVOA = 193,
    /// Cab data (cab signalling)
    CABDAT = 194,
    /// Diagnostics
    DGN = 199,
    /// Fast clock
    FCLK = 207,
    /// On event with two data bytes
    ACON2 = 208,
    /// Off event with two data bytes
    ACOF2 = 209,
    /// Teach event
    EVLRN = 210,
    /// Event variable read response in learn mode
    EVANS = 211,
    /// Accessory on response
    ARON2 = 212,
    /// Accessory off response
    AROF2 = 213,
    /// Accessory short on with 2 data bytes
    ASON2 = 216,
    /// Accessory short off with 2 data bytes
    ASOF2 = 217,
    /// Short response event on with two data bytes
    ARSON2 = 221,
    /// Short response event off with two data bytes
    ARSOF2 = 222,
    /// Extended opcode with 5 data bytes
    EXTC5 = 223,
    /// 6 byte DCC packets
    RDCC6 = 224,
    /// Loco session report
    PLOC = 225,
    /// Module name response
    NAME = 226,
    /// Command station status report
    STAT = 227,
    /// Event Acknowledge
    ENACK = 230,
    /// Extended service discovery
    ESD = 231,
    /// Long message packet
    DTXC = 233,
    /// Node parameters response
    PARAMS = 239,
    /// On event with 3 data bytes
    ACON3 = 240,
    /// Off event with 3 data bytes
    ACOF3 = 241,
    /// Read node events response
    ENRSP = 242,
    /// Accessory on response
    ARON3 = 243,
    /// Accessory off response
    AROF3 = 244,
    /// Teach event using event indexing
    EVLRNI = 245,
    /// Accessory data event: 5 bytes of node data (eg: RFID)
    ACDAT = 246,
    /// Accessory data response
    ARDAT = 247,
    /// Accessory short on with 3 data bytes
    ASON3 = 248,
    /// Accessory short off with 3 data bytes
    ASOF3 = 249,
    /// Short data frame aka device data event (device id plus 5 data bytes)
    DDES = 250,
    /// Short data frame response aka device data response
    DDRS = 251,
    /// Device Data Write Short
    DDWS = 252,
    /// Short response event on with 3 data bytes
    ARSON3 = 253,
    /// Short response event off with 3 data bytes
    ARSOF3 = 254,
    /// Extended opcode with 6 data byes
    EXTC6 = 255,
    /// Verify CV service mode - used for CV read hints
    VCVS = 164,
}
/// Microchip Processor type codes (used by FCU to identify correct bootloader compatibility)
#[derive(
    Debug,
    Copy,
    Clone,
    TryFromPrimitive,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CbusMicrochipProcessors {
    P18F2480 = 1,
    P18F4480 = 2,
    P18F2580 = 3,
    P18F4580 = 4,
    P18F2585 = 5,
    P18F4585 = 6,
    P18F2680 = 7,
    P18F4680 = 8,
    P18F2682 = 9,
    P18F4682 = 10,
    P18F2685 = 11,
    P18F4685 = 12,
    P18F25K80 = 13,
    P18F45K80 = 14,
    P18F26K80 = 15,
    P18F46K80 = 16,
    P18F65K80 = 17,
    P18F66K80 = 18,
    P18F25K83 = 19,
    P18F26K83 = 20,
    P18F27Q84 = 21,
    P18F47Q84 = 22,
    P18F27Q83 = 23,
    P18F14K22 = 25,
    P32MX534F064 = 30,
    P32MX564F064 = 31,
    P32MX564F128 = 32,
    P32MX575F256 = 33,
    P32MX575F512 = 34,
    P32MX764F128 = 35,
    P32MX775F256 = 36,
    P32MX775F512 = 37,
    P32MX795F512 = 38,
}
/// MERG Module types
#[derive(
    Debug,
    Copy,
    Clone,
    TryFromPrimitive,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CbusMergModuleTypes {
    /// default for SLiM nodes
    SLIM = 0,
    /// Solenoid point driver
    CANACC4 = 1,
    /// Motorised point driver
    CANACC5 = 2,
    /// 8 digital outputs
    CANACC8 = 3,
    /// Control panel switch/button encoder
    CANACE3 = 4,
    /// 8 digital inputs
    CANACE8C = 5,
    /// 64 led driver
    CANLED = 6,
    /// 64 led driver (multi leds per event)
    CANLED64 = 7,
    /// 12v version of CANACC4
    CANACC4_2 = 8,
    /// CANCAB hand throttle
    CANCAB = 9,
    /// CANCMD command station
    CANCMD = 10,
    /// 8 servo driver (on canacc8 or similar hardware)
    CANSERVO = 11,
    /// BC1a command station
    CANBC = 12,
    /// RPI and RFID interface
    CANRPI = 13,
    /// Turntable controller (turntable end)
    CANTTCA = 14,
    /// Turntable controller (control panel end)
    CANTTCB = 15,
    /// Handset controller for old BC1a type handsets
    CANHS = 16,
    /// Track occupancy detector
    CANTOTI = 17,
    /// 8 inputs 8 outputs
    CAN8I8O = 18,
    /// Canservo with servo position feedback
    CANSERVO8C = 19,
    /// RFID input
    CANRFID = 20,
    CANTC4 = 21,
    /// 16 inputs
    CANACE16C = 22,
    /// 8 way I/O
    CANIO8 = 23,
    CANSNDX = 24,
    /// Ethernet interface
    CANEther = 25,
    /// Multiple aspect signalling for CANLED module
    CANSIG64 = 26,
    /// Multiple aspect signalling for CANACC8 module
    CANSIG8 = 27,
    /// Conditional event generation
    CANCOND8C = 28,
    /// Control panel 32/32
    CANPAN = 29,
    /// Newer version of CANACE3 firmware
    CANACE3C = 30,
    /// Control panel 64/64
    CANPanel = 31,
    /// Multiple I/O – Universal CANMIO firmware
    CANMIO = 32,
    /// Multiple IO module 16 inputs emulating CANACE8C on CANMIO hardware
    CANACE8MIO = 33,
    /// Solenoid driver module
    CANSOL = 34,
    /// Universal CANBIP firmware - Bipolar IO module with additional 8 I/O pins (CANMIO family)
    CANBIP = 35,
    /// Solenoid driver module with additional 6 I/O pins (CANMIO family)
    CANCDU = 36,
    /// CANACC4 firmware ported to CANCDU
    CANACC4CDU = 37,
    /// CAN to MiWi base station
    CANWiBase = 38,
    /// Wireless cab using MiWi protocol
    WiCAB = 39,
    /// CAN to WiFi connection with Withrottle to CBUS protocol conversion
    CANWiFi = 40,
    /// Turntable controller configured using FLiM
    CANFTT = 41,
    /// Handset (alternative to CANCAB)
    CANHNDST = 42,
    /// Touchscreen handset
    CANTCHNDST = 43,
    /// multi-channel RFID reader
    CANRFID8 = 44,
    /// either a 2ch or 8ch RFID reader
    CANmchRFID = 45,
    /// a Raspberry Pi based module for WiFi
    CANPiWi = 46,
    /// DC train controller
    CAN4DC = 47,
    /// Nelevator controller
    CANELEV = 48,
    /// 128 switch inputs
    CANSCAN = 49,
    /// 16MHz 25k80 version of CANSERVO8c on CANMIO hardware
    CANMIO_SVO = 50,
    /// 16MHz 25k80 version of CANACE8MIO on CANMIO hardware
    CANMIO_INP = 51,
    /// 16MHz 25k80 version of CANACC8 on CANMIO hardware
    CANMIO_OUT = 52,
    /// 16MHz 25k80 version of CANACC5 on CANBIP hardware
    CANBIP_OUT = 53,
    /// DCC stop generator
    CANASTOP = 54,
    /// CANCMD with on board 3A booster
    CANCSB = 55,
    /// Magnet on Track detector
    CANMAG = 56,
    /// 16 input equivaent to CANACE8C
    CANACE16CMIO = 57,
    /// CBUS module based on Raspberry Pi
    CANPiNODE = 58,
    /// 25K80 version of CANLED64 (IHart and MB)
    CANDISP = 59,
    /// Compute Event processing engine
    CANCOMPUTE = 60,
    /// Read/Write from/to RC522 RFID tags
    CANRC522 = 61,
    /// 8 inputs module (2g version of CANACE8c) (Pete Brownlow)
    CANINP = 62,
    /// 8 outputs module (2g version of CANACC8) (Pete Brownlow)
    CANOUT = 63,
    /// Extended CANMIO (24 I/O ports) (Pete Brownlow)
    CANXIO = 64,
    /// DC cab
    CANCABDC = 65,
    /// DC Railcom detector/reader
    CANRCOM = 66,
    /// MP3 sound player in response to events (eg: station announcements) (Duncan Greenwood)
    CANMP3 = 67,
    /// Addressed RGB LED driver (Duncan Greenwood)
    CANXMAS = 68,
    /// Servo setting box (Duncan Greenwood)
    CANSVOSET = 69,
    /// DC Command station
    CANCMDDC = 70,
    /// Text message display
    CANTEXT = 71,
    /// Signal controller
    CANASIGNAL = 72,
    /// DCC cab with slider control (Dave Radcliffe)
    CANSLIDER = 73,
    /// DC ATC module (Dave Harris)
    CANDCATC = 74,
    /// Logic module using and/or gates (Phil Silver)
    CANGATE = 75,
    /// Q series PIC input module (Ian Hart)
    CANSINP = 76,
    /// Q series PIC input module (Ian Hart)
    CANSOUT = 77,
    /// Q series PIC input module (Ian Hart)
    CANSBIP = 78,
    /// Message buffer (Phil Silver)
    CANBUFFER = 79,
    /// Lever frame module (Tim Coombs)
    CANLEVER = 80,
    /// Kit 110 Arduino shield test firmware
    CANSHIELD = 81,
    /// 4 inputs 4 outputs (Arduino module)
    CAN4IN4OUT = 82,
    /// CANCMD with built in booster (Simon West)
    CANCMDB = 83,
    /// neopixel driver (Jon Denham)
    CANPIXEL = 84,
    /// Cab2 with pot or encoder (Simon West hardware, Jon Denham new C firmware)
    CANCABPE = 85,
    /// Smart train detector (Michael Smith)
    CANSMARTTD = 86,
    /// All VLCB modules have the same ID
    VLCB = 252,
    /// Software nodes
    CAN_SW = 255,
    /// Empty module, bootloader only
    EMPTY = 254,
    /// USB interface
    CANUSB = 253,
}
/// Manufacturer definitions
///
/// Here are definitions for all known MERG registered manufacturers.
#[derive(
    Debug,
    Copy,
    Clone,
    TryFromPrimitive,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CbusManufacturer {
    /// Develoment mode manufacturer
    ///
    /// For manufacturers who don't have a manufacturer id yet or
    /// for lib developers during testing.
    ///
    /// Do not use in production!
    Development = 13,
    /// https://www.merg.co.uk
    MERG = 165,
    /// https://www.sprog-dcc.co.uk/
    SPROG = 44,
    /// http://www.rocrail.net
    ROCRAIL = 70,
    /// http://animatedmodeler.com  (Spectrum Engineering)
    SPECTRUM = 80,
    /// range of MERG VLCB modules
    MERG_VLCB = 250,
    /// Konrad Orlowski
    SYSPIXIE = 249,
    /// http://rmeuk.com  (Railway Modelling Experts Limited)
    RME = 248,
}
/// Error codes for OPC_ERR
#[derive(
    Debug,
    Copy,
    Clone,
    TryFromPrimitive,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CbusErrs {
    LocoStackIsFull = 1,
    LocoAddressIsTaken = 2,
    SessionIsNotPresent = 3,
    EmptyConsist = 4,
    LocoWasNotFound = 5,
    RxBufferOverflow = 6,
    InvalidRequest = 7,
    SessionWasCancelled = 8,
}
/// Error codes for OPC_CMDERR
#[derive(
    Debug,
    Copy,
    Clone,
    TryFromPrimitive,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CbusCmdErrs {
    /// Invalid command
    InvalidCommand = 1,
    /// The mode is not currently in learn mode
    NotInLearnMode = 2,
    /// The mode is not currently in setup mode
    NotInSetupMode = 3,
    /// Too many events provisioned in module
    ///
    /// The event storage is exhausted
    TooManyEvents = 4,
    /// No Event-Variable
    NoEv = 5,
    /// Invalid EV index
    InvalidEvIndex = 6,
    /// Invalid event
    InvalidEvent = 7,
    /// Invalid event index
    InvalidEventIndex = 8,
    /// Invalid param index
    InvalidParamIndex = 9,
    /// Invalid NV index
    InvalidNvIndex = 10,
    /// Invalid EV value
    InvalidEvValue = 11,
    /// Invalid NV value
    InvalidNvValue = 12,
    /// Another module is already in learn mode
    ///
    /// Sent when module in learn mode sees NNLRN for different module (also exits learn mode)
    AnotherModuleIsInLearnMode = 13,
}
/// Aspect codes for CDAT_CABSIG
///
/// Second Aspect byte
#[derive(
    Debug,
    Copy,
    Clone,
    TryFromPrimitive,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CbusCabSigAspect2 {
    /// Set bit 0 to indicate lit
    Lit = 0,
    /// Set bit 1 for lunar indication
    Lunar = 1,
}
/// Aspect codes for CDAT_CABSIG
///
/// First aspect byte
#[derive(
    Debug,
    Copy,
    Clone,
    TryFromPrimitive,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CbusCabSigAspect1 {
    Danger = 0,
    Caution = 1,
    PreliminaryCaution = 2,
    Proceed = 3,
    /// Set bit 2 for call-on - main aspect will usually be at danger
    CallOn = 4,
    /// Set bit 3 to 0 for upper nibble is feather location, set 1 for upper nibble is theatre code
    Theatre = 8,
}
/// Sub opcodes for OPC_CABDAT
#[derive(
    Debug,
    Copy,
    Clone,
    TryFromPrimitive,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CbusCabDatOpcode {
    CABSIG = 1,
}
/// BUS type that module is connected to
#[derive(
    Debug,
    Copy,
    Clone,
    TryFromPrimitive,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CbusBusTypes {
    CAN = 1,
    Ethernet = 2,
    MiWi = 3,
    USB = 4,
}
/// ARM Processor type codes (used by FCU to identify correct bootloader compatibility)
#[derive(
    Debug,
    Copy,
    Clone,
    TryFromPrimitive,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CbusArmProcessors {
    /// As used in Raspberry Pi
    Arm1176JzfS = 1,
    /// As Used in Raspberry Pi 2
    CortexA7 = 2,
    /// As used in Raspberry Pi 3
    CortexA53 = 3,
}
