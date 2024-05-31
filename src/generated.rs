use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use num_enum::UnsafeFromPrimitive;
use bitflags::bitflags;
use num_enum::FromPrimitive;
/// VLCB opcodes
#[derive(
    Debug,
    Copy,
    Clone,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
    TryFromPrimitive,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum OpCode {
    /// General Acknowledgement.
    ///
    /// Positive response to query/ request performed or report of availability on-line.
    #[doc = include_str!("../docs/opcode/ack.md")]
    GeneralAck = 0,
    /// General No Ack.
    ///
    /// Negative response to query/ request denied.
    #[doc = include_str!("../docs/opcode/nak.md")]
    GeneralNack = 1,
    /// Bus Halt.
    ///
    /// Commonly broadcasted to all nodes to indicate CBUS is not available and
    /// no further packets should be sent until a BON or ARST is received.
    #[doc = include_str!("../docs/opcode/hlt.md")]
    BusHalt = 2,
    /// Bus ON
    ///
    /// Commonly broadcasted to all nodes to indicate CBUS is available after a HLT
    /// message was transmitted.
    #[doc = include_str!("../docs/opcode/bon.md")]
    BusResume = 3,
    /// DCC track off.
    ///
    /// Commonly broadcasted to all nodes by a command station to indicate track power is off
    /// and no further command packets should be sent, except inquiries.
    #[doc = include_str!("../docs/opcode/tof.md")]
    DccTrackPoweredOff = 4,
    /// DCC track on.
    ///
    /// Commonly broadcasted to all nodes by a command station to indicate track power is on.
    #[doc = include_str!("../docs/opcode/ton.md")]
    DccTrackPoweredOn = 5,
    /// DCC emergency stop.
    ///
    /// Commonly broadcast to all nodes by a command station to indicate all engines have been emergency stopped.
    #[doc = include_str!("../docs/opcode/estop.md")]
    DccEmergencyStopEngaged = 6,
    /// Perform reboot on all nodes.
    ///
    /// Commonly broadcasted to all nodes to indicate a full system restart.
    /// Similar to NNRST which directs a single node to be restarted.
    #[doc = include_str!("../docs/opcode/arst.md")]
    RestartAllNodes = 7,
    /// DCC request track off.
    ///
    /// Sent to request change of track power state to “off”.
    #[doc = include_str!("../docs/opcode/rtof.md")]
    DccTrackPowerOff = 8,
    /// DCC request track on.
    ///
    /// Sent to request change of track power state to “on”.
    #[doc = include_str!("../docs/opcode/rton.md")]
    DccTrackPowerOn = 9,
    /// DCC request emergency stop.
    ///
    /// Sent to request an emergency stop to all trains.
    /// Does not affect accessory control.
    #[doc = include_str!("../docs/opcode/restp.md")]
    DccEmergencyStop = 10,
    /// Request Command Station Status.
    ///
    /// Sent to query the status of the command station. See description of (STAT) for the response from the command station.
    #[doc = include_str!("../docs/opcode/rstat.md")]
    DccQueryCommandStationStatus = 12,
    /// Query node number.
    ///
    /// Requests a PNN reply from each node on the bus.
    #[doc = include_str!("../docs/opcode/qnn.md")]
    QueryNodeInfo = 13,
    /// Request node parameters.
    ///
    /// Sent to a node while in Setup mode to read its parameter set.
    /// Used when initially configuring a node.
    #[doc = include_str!("../docs/opcode/rqnp.md")]
    QueryNodeParameters = 16,
    /// Request module name.
    ///
    /// Sent by a node to request the name of the type of module that is in setup mode or Learn mode.
    /// The module in setup mode or learn mode will reply with opcode NAME.
    #[doc = include_str!("../docs/opcode/rqmn.md")]
    QueryModuleName = 17,
    /// Release Engine.
    ///
    /// Sent by a CAB to the Command Station. The engine with that Session number is removed from the active engine list.
    #[doc = include_str!("../docs/opcode/kloc.md")]
    DccReleaseSession = 33,
    /// Query engine.
    ///
    /// Used to determine if the command station session is valid and to obtain information about the status of the locomotive.
    #[doc = include_str!("../docs/opcode/qloc.md")]
    DccQueryLocoStatus = 34,
    /// Session keep alive.
    ///
    /// The cab sends a keep alive at regular intervals for the active session.
    /// The interval between keep alive messages must be less than the session timeout implemented by the command station.
    #[doc = include_str!("../docs/opcode/dkeep.md")]
    DccSessionKeepAlive = 35,
    /// Debug message with one data byte.
    ///
    /// Freeform status byte for debugging during CBUS module development. Not used during normal operation.
    #[doc = include_str!("../docs/opcode/dbg1.md")]
    DebugMsg1 = 48,
    /// Extended opcode with zero additional bytes.
    /// Reserved to allow the 0 additional bytes range to be extended by a further 256 opcodes.
    #[doc = include_str!("../docs/opcode/extc.md")]
    ExtOpCode = 63,
    /// Request engine session.
    ///
    /// This command is typically sent by a cab to the command station following a change of the controlled decoder address.
    /// RLOC is exactly equivalent to GLOC with all flag bits set to zero, but command stations
    #[doc = include_str!("../docs/opcode/rloc.md")]
    DccRequestNewSession = 64,
    /// Query Consist.
    ///
    /// Allows enumeration of a consist. Command station responds with PLOC if an engine exists at the specified index, otherwise responds
    #[doc = include_str!("../docs/opcode/qcon.md")]
    DccQueryConsist = 65,
    /// Set Node Number.
    ///
    /// Sent by a configuration tool to assign a node number to a requesting node in response to a RQNN message. The target node must be in
    #[doc = include_str!("../docs/opcode/snn.md")]
    SetNodeNumber = 66,
    /// Allocate loco to activity.
    #[doc = include_str!("../docs/opcode/aloc.md")]
    DccAllocateLocoToActivity = 67,
    /// Set CAB session mode.
    ///
    /// Bits 0–1: speed mode
    /// - 00 =128 speed steps
    /// - 01 =14 speed steps
    /// - 10 =28 speed steps with interleave steps
    /// - 11 =28 speed steps
    ///
    /// Bit 2: service mode
    /// Bit 3:sound control mode
    #[doc = include_str!("../docs/opcode/stmod.md")]
    DccSetThrottleMode = 68,
    /// Consist Engine.
    ///
    /// Adds a decoder specified by Session to a consist.
    /// Consist# has bit 7 set if consist direction is reversed.
    #[doc = include_str!("../docs/opcode/pcon.md")]
    DccConsistAddLoco = 69,
    /// Removes a loco from a consist.
    #[doc = include_str!("../docs/opcode/kcon.md")]
    DccConsistRemoveLoco = 70,
    /// Set Engine Speed/Dir.
    ///
    /// Speed/dir value, where the most significant bit is direction and the 7ls bits are the unsigned speed value.
    /// Sent by a CAB or equivalent to request an engine speed/dir change.
    #[doc = include_str!("../docs/opcode/dspd.md")]
    DccSetLocoThrottle = 71,
    /// Set Engine Flags.
    ///
    /// Sent by a cab to notify the command station of a change in engine flags.
    ///
    /// Bits 0-1: Speed Mode (00 =128 speed steps, 01 =14 speed steps,10 =28 speed steps with interleave steps, 11 =28 speed steps)
    /// Bit 2:Lights On/OFF Bit 3:Engine relative direction
    /// Bits 4-5: Engine state (active =0 , consisted =1, consist master=2, inactive=3)
    /// Bits 6-7: Reserved.
    #[doc = include_str!("../docs/opcode/dflg.md")]
    DccSetLocoFlags = 72,
    /// Set Engine function on.
    ///
    /// Sent by a cab to turn on a specific loco function.
    /// This provides an alternative method to DFUN for controlling loco functions. A command
    #[doc = include_str!("../docs/opcode/dfnon.md")]
    DccLocoFunctionOn = 73,
    /// Set Engine function off.
    ///
    /// Sent by a cab to turn off a specific loco function.
    /// This provides an alternative method to DFUN for controlling loco functions. A command
    #[doc = include_str!("../docs/opcode/dfnof.md")]
    DccLocoFunctionOff = 74,
    /// Service mode status.
    ///
    /// Status returned by command station/programmer at the end of a programming operation that does not return data. Response to QCVS to indicate no data.
    #[doc = include_str!("../docs/opcode/sstat.md")]
    DccServiceModeStatus = 76,
    /// Reset to manufacturer settings.
    /// Reset a module back to manufacturer settings.
    #[doc = include_str!("../docs/opcode/nnrsm.md")]
    ResetModuleToFactory = 79,
    /// Request node number.
    ///
    /// The module is requesting that it is provided with a new node number.
    /// A configuration tool should respond with SNN to provide the requesting
    #[doc = include_str!("../docs/opcode/rqnn.md")]
    RequestNewNodeNumber = 80,
    /// Node number release.
    ///
    /// A node signals that it no longer requires a node number by sending NNREL.
    /// The module will do this upon moving from normal mode to setup mode.
    #[doc = include_str!("../docs/opcode/nnrel.md")]
    NodeNumberReleased = 81,
    /// Node number acknowledge.
    ///
    /// This message is sent in response to SNN.
    /// A node signals that it will
    #[doc = include_str!("../docs/opcode/nnack.md")]
    NodeNumberAck = 82,
    /// Set node into learn mode.
    ///
    /// Sent by a configuration tool to put a specific node into learn mode.
    #[doc = include_str!("../docs/opcode/nnlrn.md")]
    PutNodeIntoLearnMode = 83,
    /// Release node from learn mode.
    ///
    /// Sent by a configuration tool to take the module out of learn mode and revert to normal operation.
    #[doc = include_str!("../docs/opcode/nnuln.md")]
    ReleaseNodeFromLearnMode = 84,
    /// Clear all events from a node.
    ///
    /// Sent by a configuration tool to clear all events from a specific node.
    /// Must be in learn mode first to safeguard against accidental erasure of all events
    #[doc = include_str!("../docs/opcode/nnclr.md")]
    ForgetAllLearnedEvents = 85,
    /// Read the number of event slots available in a node.
    ///
    /// Sent by a configuration tool to read the number of available event slots in a node.
    #[doc = include_str!("../docs/opcode/nnevn.md")]
    QueryAvailableEventSlots = 86,
    /// Read back all stored events in a node.
    ///
    /// There MUST be no hidden events.
    /// Sent by a configuration tool to read all the stored events in a node.
    #[doc = include_str!("../docs/opcode/nerd.md")]
    QueryAllLearnedEvents = 87,
    /// Request to read number of stored events.
    ///
    /// Sent by a configuration tool to read the number of stored events in a node.
    #[doc = include_str!("../docs/opcode/rqevn.md")]
    QueryLearnedEventCount = 88,
    /// Write acknowledge.
    ///
    /// Sent by a node to indicate the completion of a write to memory operation.
    /// All nodes must issue WRACK when a write operation to node variables, events or event variables has completed.
    /// This allows for teaching nodes where the processing time may be slow.
    #[doc = include_str!("../docs/opcode/wrack.md")]
    WriteAck = 89,
    /// Request node data event.
    ///
    /// Sent by one node to read the data event from another node.(eg: RFID data).
    #[doc = include_str!("../docs/opcode/rqdat.md")]
    QueryNodeData = 90,
    /// Request device data –short mode.
    ///
    /// To request a ‘data set’ from a device using the short event method where DN is the device number.
    #[doc = include_str!("../docs/opcode/rqdds.md")]
    RequestDeviceDataShortMode = 91,
    /// Put node into bootloading mode.
    /// For modules with no NN then the NN of the command must be zero. For nodes in Normal mode the command must contain the NN of the target node. Sent by a configuration tool to prepare for loading a new program.
    #[doc = include_str!("../docs/opcode/bootm.md")]
    RebootIntoBootloader = 92,
    /// Force a self enumeration cycle for use with CAN.
    ///
    /// For nodes in Normal mode using CAN as a transport.
    /// This message will force a self-enumeration cycle for the specified node.
    /// A new CAN_ID will be allocated if needed.
    #[doc = include_str!("../docs/opcode/enum.md")]
    ForceCanEnumeration = 93,
    /// Reset module’s CPU.
    ///
    /// Reset a module’s microprocessor.
    #[doc = include_str!("../docs/opcode/nnrst.md")]
    RestartNode = 94,
    /// Extended opcode with 1 additional byte.
    ///
    /// Reserved to allow the 1 additional bytes range to be extended by a further 256 opcodes.
    #[doc = include_str!("../docs/opcode/extc1.md")]
    ExtOpCode1 = 95,
    /// Set Engine functions.
    ///
    /// <Fn1>is the function range 1 is F0(FL) to F4, 2 is F5 to F8, 3 is F9 to F12, 4 is F13 to F20, 5 is F21to F28) <Fn2> is the NMRA DCC format function byte for that range in corresponding bits. A bit set to 1 turns function “on” and a cleared bit sets function “off”. Sent by a CAB or equivalent to request an engine Fn state change.
    #[doc = include_str!("../docs/opcode/dfun.md")]
    DccSetLocoFunctions = 96,
    /// Get engine session.
    ///
    /// <Flags> contains flag bits as follows: Bit 0: Set for "Steal" mode Bit 1: Set for "Share" mode. Both bits set to 0 is exactly equivalent to an RLOC request but
    #[doc = include_str!("../docs/opcode/gloc.md")]
    DccQueryLocoSession = 97,
    /// Command Station Error report.
    ///
    /// Sent in response to an error situation by a command station.
    /// See Appendix A - DCC ERR error codes for a list of error codes.
    #[doc = include_str!("../docs/opcode/err.md")]
    DccCommandStationError = 99,
    /// Error messages from nodes during configuration.
    ///
    /// Sent by node if there is an error when a configuration command is sent. See Appendix C - CMDERR error codes for the list of supported codes.
    #[doc = include_str!("../docs/opcode/cmderr.md")]
    NodeConfigurationError = 111,
    /// Event space left reply from node.
    ///
    /// Spaces is a one byte value giving the number of available event spaces left in the node’s event table.
    /// This is the maximum number of additional events that can be stored by the module.
    #[doc = include_str!("../docs/opcode/evnlf.md")]
    AvailableEventSlots = 112,
    /// Request read of a node variable.
    ///
    /// NV# is the index for the node variable value requested.
    /// Response is NVANS.
    #[doc = include_str!("../docs/opcode/nvrd.md")]
    QueryNodeVariable = 113,
    /// Request read of stored event by event index.
    ///
    /// EN# is the index for the stored event requested.
    #[doc = include_str!("../docs/opcode/nenrd.md")]
    QueryLearnedEventByIndex = 114,
    /// Request read of a node parameter by index.
    ///
    /// Para# is the index for the parameter requested. Reading Index 0 first returns a PARAN with the number of available
    #[doc = include_str!("../docs/opcode/rqnpn.md")]
    QueryNodeParameterByIndex = 115,
    /// Number of events stored by node.
    ///
    /// Response to request RQEVN
    #[doc = include_str!("../docs/opcode/numev.md")]
    LearnedEventCount = 116,
    /// Set the CAN_ID in the node.
    ///
    /// Used to force a specified CAN_ID into a node. Value range is from 1 to 0x63 (99 decimal). This OPC must be used with care as duplicate CAN_IDs are not allowed.
    #[doc = include_str!("../docs/opcode/canid.md")]
    SetNodeCanId = 117,
    /// Request a change to a module’s operating mode.
    ///
    /// Request to change the operational mode of the module.
    #[doc = include_str!("../docs/opcode/mode.md")]
    PutNodeIntoMode = 118,
    /// Request service discovery.
    ///
    /// Request service data from a module.
    /// If the ServiceIndex is zero then the module responds with a SD
    #[doc = include_str!("../docs/opcode/rqsd.md")]
    ServiceDiscoveryQuery = 120,
    /// Extended opcode with 2 additional bytes.
    ///
    /// Reserved to allow the 2 additional bytes range to be extended by a
    /// further 256 opcodes.
    #[doc = include_str!("../docs/opcode/extc2.md")]
    ExtOpCode2 = 127,
    /// Request 3-byte DCC Packet.
    ///
    /// Allows a CAB or equivalent to request a 3 byte DCC packet to be sent to the track. The packet is sent <REP> times and is not refreshed on a regular basis. Note: a 3 byte DCC packet is the minimum allowed.
    #[doc = include_str!("../docs/opcode/rdcc3.md")]
    DccSendRawPacket3 = 128,
    /// Write CV (byte) in OPS mode.
    ///
    /// Sent to the command station to write a DCC CV byte in OPS mode to a specific loco (on the main).
    #[doc = include_str!("../docs/opcode/wcvo.md")]
    DccWriteCvByteInOpsMode = 130,
    /// Write CV (bit) in OPS mode.
    /// Sent to the command station to write a DCC CV in OPS mode to specific loco (on the main). The format for Value is that specified in RP 9.2.1 for OTM bit manipulation in a DCC packet. This is ‘111CDBBB’ where C here is always 1 as only ‘writes’ are possible OTM (unless some loco ACK scheme like RailCom is used). D is the bit value, either 0 or 1 and BBB is the bit position in the CV byte. 000 to 111 for bits 0 to 7.
    #[doc = include_str!("../docs/opcode/wcvb.md")]
    DcWriteCvBitInOpsMode = 131,
    /// Read CV.
    /// This command is used exclusively with service mode.
    /// Sent by the cab to the command station in order to read a CV value.
    #[doc = include_str!("../docs/opcode/qcvs.md")]
    DccReadCv = 132,
    /// Report CV.
    ///
    /// This command is used exclusively with service mode.
    /// Sent by the command station to report a read CV in response to QCVS.
    #[doc = include_str!("../docs/opcode/pcvs.md")]
    DccCvValue = 133,
    /// Request diagnostic data.
    ///
    /// Request diagnostic data from a module.
    /// If the requested diagnostic data is zero then a response for all diagnostic data is returned.
    #[doc = include_str!("../docs/opcode/rdgn.md")]
    QueryDiagnosticData = 135,
    /// Set a NV value with read back.
    /// Sets a NV value and additionally responds with the new value.nvset
    /// The new value may not be the value which was requested to be written.
    #[doc = include_str!("../docs/opcode/nvsetrd.md")]
    SetNodeVariable = 142,
    /// Accessory ON long event.
    ///
    /// Indicates an ‘ON’ event using the full event number of 4 bytes (long event). An event is sent by a module when it detects a change of state. Modules may consume the event and perform actions.
    #[doc = include_str!("../docs/opcode/acon.md")]
    LongEventAccessoryOn = 144,
    /// Accessory OFF long event.
    ///
    /// Indicates an ‘OFF’ event using the full event number of 4 bytes (long event). An event is sent by a module when it detects a change of state. Modules may consume the event and perform actions.
    #[doc = include_str!("../docs/opcode/acof.md")]
    LongEventAccessoryOff = 145,
    /// Accessory Request Event.
    ///
    /// Indicates a ‘request’ event using the full event number of 4 bytes (long event).
    /// A request event is used to elicit a status response from a producer when it is required to know the ‘state’ of the producer without producing an ON or OFF event.
    #[doc = include_str!("../docs/opcode/areq.md")]
    QueryLongEventAccessoryState = 146,
    /// Accessory Response Event.
    ///
    /// Indicates an ‘ON’ response. A response is a reply to a status request (AREQ) without producing an ON or OFF event.
    #[doc = include_str!("../docs/opcode/aron.md")]
    LongEventAccessoryStateOn = 147,
    /// Accessory Response Event (AROF).
    ///
    /// Indicates an ‘OFF’ response. A response is a reply to a status request (AREQ) without producing an ON or OFF event.
    #[doc = include_str!("../docs/opcode/arof.md")]
    LongEventAccessoryStateOff = 148,
    /// Unlearn an event in learn mode.
    ///
    /// Sent by a configuration tool to remove an event from a node.
    #[doc = include_str!("../docs/opcode/evuln.md")]
    ForgetLearnedEvent = 149,
    /// Set a node variable.
    ///
    /// Sent by a configuration tool to set a node variable. NV# is the NV index number.
    #[doc = include_str!("../docs/opcode/nvset.md")]
    LegacySetNodeVariable = 150,
    /// Response to a request for a node variable value.
    ///
    /// Sent by node in response to request NVRD or NVSETRD.
    #[doc = include_str!("../docs/opcode/nvans.md")]
    NodeVariableValue = 151,
    /// Accessory Short ON.
    ///
    /// Indicates an ‘ON’ event using the short event number of 2 LS bytes. An event is sent by a module when it detects a change of state. Modules may consume the event and perform actions. The NN is not used to match events, the NN normally indicates the source of the event.
    #[doc = include_str!("../docs/opcode/ason.md")]
    ShortEventAccessoryOn = 152,
    /// Accessory Short OFF.
    ///
    /// Indicates an ‘OFF’ event using the short event number of 2 LS bytes. An event is sent by a module when it detects a change of state. Modules may consume the event and perform actions. The NN is not used to match events, the NN normally indicates the source of the event.
    #[doc = include_str!("../docs/opcode/asof.md")]
    ShortEventAccessoryOff = 153,
    /// Accessory Short Request Event.
    ///
    /// Indicates a ‘request’ event using the short event number of 2 LS bytes. A request event is used to elicit a status response from a producer when it is required to know the ‘state’ of the producer without producing an ON or OFF event.
    #[doc = include_str!("../docs/opcode/asrq.md")]
    QueryShortEventAccessoryState = 154,
    /// Response to request for individual node parameter RQNPN.
    ///
    /// NN is the node number of the sending node. Para# is the index of the parameter and Para val is the parameter value. Returns a parameter value. Parameter index is the parameter number and matches that in the RQNPN request.
    #[doc = include_str!("../docs/opcode/paran.md")]
    NodeParameterValue = 155,
    /// Request for read of an event variable.
    ///
    /// This request differs from B2 (REQEV) as it doesn’t need to be in learn
    #[doc = include_str!("../docs/opcode/reval.md")]
    QueryEventVariable = 156,
    /// Accessory Short Response Event.
    ///
    /// Indicates an ‘ON’ response. A response is a reply to a status request (ASRQ) without producing an ON or OFF event.
    #[doc = include_str!("../docs/opcode/arson.md")]
    ShortEventAccessoryStateOn = 157,
    /// Accessory Short Response Event.
    ///
    /// Indicates an ‘OFF’ response. A response is a reply to a status request (ASRQ) without producing an ON or OFF event.
    #[doc = include_str!("../docs/opcode/arsof.md")]
    ShortEventAccessoryStateOff = 158,
    /// Extended opcode with 3 additional bytes.
    /// Reserved to allow the 3 additional bytes range to be extended by a further 256 opcodes.
    #[doc = include_str!("../docs/opcode/extc3.md")]
    ExtOpCode3 = 159,
    /// Request 4-byte DCC Packet.
    /// Allows a CAB or equivalent to request a 4 byte DCC packet to be sent to the track. The packet is sent <REP> times and is not refreshed on a regular basis.
    #[doc = include_str!("../docs/opcode/rdcc4.md")]
    DccSendRawPacket4 = 160,
    /// Write CV in Service mode.
    /// Sent to the command station to write a DCC CV in service mode.
    #[doc = include_str!("../docs/opcode/wcvs.md")]
    DccWriteCvInServiceMode = 162,
    /// Heartbeat from module.
    ///
    /// Heartbeat message from module indicating that the module is alive and communicating on the bus. Sent every 5 seconds by a module to confirm it is alive and connected to the network along with an indication of module status. Sequence is a count from 0 incrementing on each message transmitted and wrapping around to zero, It facilitates detection of missing frames. Status: This is a binary representation of the module’s diagnostic status as outlined in MNS Specification Section 8.3. 0x00 Shall always represent “normal“ operation. StatusBits: Reserved for future expansion, set to 0x00
    #[doc = include_str!("../docs/opcode/heartb.md")]
    Heartbeat = 171,
    /// Service discovery response.
    ///
    /// The version of a service supported by a module.
    /// Sent in response to RQSD with ServiceIndex = 0. A number of SD
    #[doc = include_str!("../docs/opcode/sd.md")]
    ServiceDiscoveryResponse = 172,
    /// Generic Response.
    ///
    /// Generic response for a configuration change request. Result byte indicates ok for success or an error code in case of failure. Indicates the module is ready for further configuration. The CMDERR codes are supported and in addition service specific
    #[doc = include_str!("../docs/opcode/grsp.md")]
    GenericResponse = 175,
    /// Accessory ON.
    ///
    /// Indicates an ‘ON’ event using the full event number of 4 bytes with one additional data byte. An event is sent by a module when it detects a change of state. Modules may consume the event and perform actions. The meaning of the additional data is dependent upon the application and must be agreed between the producer and consumer of the event.
    #[doc = include_str!("../docs/opcode/acon1.md")]
    LongEventAccessoryOn1 = 176,
    /// Accessory OFF.
    ///
    /// Indicates an ‘OFF’ event using the full event number of 4 bytes with one additional databyte. An event is sent by a module when it detects a change of state. Modules may consume the event and perform actions. The meaning of the additional data is dependent upon the application and must be agreed between the producer and consumer of the event.
    #[doc = include_str!("../docs/opcode/acof1.md")]
    LongEventAccessoryOff1 = 177,
    /// Read event variable in learn mode.
    ///
    /// Allows a configuration tool to read stored event variables from a node. EV# is the EV variable index. NN and EN identify the event and not the module. Reading EV#0 shall first return the number of EVs followed by a series of EVANS with the value for each EV.
    #[doc = include_str!("../docs/opcode/reqev.md")]
    QueryEventVariableInLearnMode = 178,
    /// Accessory Response Event.
    ///
    /// Indicates an ‘ON’ response with one additional data byte. A response is a reply to a status request (AREQ) without producing an ON or OFF
    #[doc = include_str!("../docs/opcode/aron1.md")]
    LongEventAccessoryStateOn1 = 179,
    /// Accessory Response Event.
    /// Indicates an ‘OFF’ response with one additional data byte. A response is a reply to a status request (AREQ) without producing an ON or OFF
    #[doc = include_str!("../docs/opcode/arof1.md")]
    LongEventAccessoryStateOff1 = 180,
    /// Response to request for read of EV value.
    ///
    /// This is the response to the request to read an EV - REVAL.
    #[doc = include_str!("../docs/opcode/neval.md")]
    EventVariableValue = 181,
    /// Response to Query Node - QNN.
    ///
    /// Sent in response to a QNN request.
    #[doc = include_str!("../docs/opcode/pnn.md")]
    NodeInfo = 182,
    /// Accessory Short ON.
    ///
    /// Indicates an ‘ON’ event using the short event number of 2 LS bytes with one added data byte. An event is sent by a module when it detects a change of state. Modules may consume the event and perform actions. The NN is not used to match events, the NN normally indicates the source of the event. The meaning of the additional data is dependent upon the application and must be agreed between the producer and consumer of the event.
    #[doc = include_str!("../docs/opcode/ason1.md")]
    ShortEventAccessoryOn1 = 184,
    /// Accessory Short OFF.
    ///
    /// Indicates an ‘OFF’ event using the short event number of 2 LS bytes with one added data byte. An event is sent by a module when it detects a change of state. Modules may consume the event and perform actions. The NN is not used to match events, the NN normally indicates the source of the event. The meaning of the additional data is dependent upon the application and must be agreed between the producer and consumer of the event.
    #[doc = include_str!("../docs/opcode/asof1.md")]
    ShortEventAccessoryOff1 = 185,
    /// Accessory Short Response Event.
    /// Indicates an ‘ON’ response with one added data byte. A response is a reply to a status request (ASRQ)without producing an ON or OFF
    #[doc = include_str!("../docs/opcode/arson1.md")]
    ShortEventAccessoryStateOn1 = 189,
    /// Accessory Short Response Event with one data byte.
    /// Indicates an ‘OFF’ response with one added data byte. A response is a reply to a status request (ASRQ) without producing an ON or OFF event. A response event is a reply to a status request (ASRQ)without
    #[doc = include_str!("../docs/opcode/arsof1.md")]
    ShortEventAccessoryStateOff1 = 190,
    /// Extended opcode with 4 additional bytes.
    /// Reserved to allow the 4 additional bytes range to be extended by a further 256 opcodes.
    #[doc = include_str!("../docs/opcode/extc4.md")]
    ExtOpCode4 = 191,
    /// Request 5-byte DCC Packet.
    /// Allows a CAB or equivalent to request a 5 byte DCC packet to be sent to the track. The packet is sent <REP> times and is not refreshed on a regular basis.
    #[doc = include_str!("../docs/opcode/rdcc5.md")]
    DccSendRawPacket5 = 192,
    /// Write CV (byte) in OPS mode by address.
    ///
    /// Sent to the command station to write a DCC CV byte in OPS mode to specific loco (on the main). Used by computer based ops mode programmer that does not have a valid throttle handle.
    #[doc = include_str!("../docs/opcode/wcvoa.md")]
    DccWriteCvByteInOpsModeByAddress = 193,
    /// Send data to the DCC CAB which is controlling a particular loco.
    ///
    /// addrH and addrL are the loco address in the same format as RLOC and GLOC 7 bit addresses have (addrH=0). 14 bit addresses have bits 6,7 of addrH set to 1. dataCode defines the meaning of the remaining 3 bytes. The following values for dataCod have currently been defined: ● 01 - CABSIG - Transmitted by a layout control system to send signal aspects to be displayed on a cab handset as cab signalling. Parameter data1 is used for aspect1 Parameter data2 is used for aspect2 Parameter data3 is used for speed aspect1 is signalling system independent, and is defined as follows (colours in brackets correspond to UK colour light signalling, the given aspect names may be displayed differently in other signalling systems): Bits 0-1 - 2 bit aspect code 00=danger (red), 01=caution (yellow), 10=preliminary caution (double yellow), 11=proceed (green) Bit 2 - set 1 for calling on or shunt aspect (bits 0-1 would be set to 00 for danger when calling on) Bit 3 - Set 0 to indicate upper nibble is feather location, set 1 for upper nibble is theatre type route indicator Bits 4-7 - 0 - no route indicated, 1 to 6 = feather position or 1 to 16 for theatre route indication aspect1 should be set to 0xFF if no signal information is available. This can be used, for example, to indicate leaving a cab signalling area. A cab should extinguish any currently showing aspect on receipt of this code. Note that because bits 0 and 1 should be set to zero when bit 2 is set, the code 0xFF is not otherwise a valid aspect. <aspect2> may be used as required for specific signalling systems. The meaning will vary for each signalling system. For the UK 2003 rulebook, bit 0 set indicates a flashing aspect, applicable to caution, preliminary caution or proceed. For UK semaphore signalling, where there are multiple arms for
    #[doc = include_str!("../docs/opcode/cabdat.md")]
    DccSendDataToCab = 194,
    /// Diagnostic data response.
    ///
    /// Diagnostic data value from a module. Sent in response to RDGN.
    #[doc = include_str!("../docs/opcode/dgn.md")]
    DiagnosticData = 199,
    /// Fast Clock.
    ///
    /// Used to implement a fast clock for the layout.
    #[doc = include_str!("../docs/opcode/fclk.md")]
    FastClock = 207,
    /// Accessory ON.
    ///
    /// Indicates an ‘ON’ event using the full event number of 4 bytes with two additional data bytes. An event is sent by a module when it detects a change of state. Modules may consume the event and perform actions. The meaning of the additional data is dependent upon the application and must be agreed between the producer and consumer of the event.
    #[doc = include_str!("../docs/opcode/acon2.md")]
    LongEventAccessoryOn2 = 208,
    /// Accessory OFF.
    ///
    /// Indicates an ‘OFF’ event using the full event number of 4 bytes with two additional data bytes. An event is sent by a module when it detects a change of state. Modules may consume the event and perform actions. The meaning of the additional data is dependent upon the application and must be agreed between the producer and consumer of the event.
    #[doc = include_str!("../docs/opcode/acof2.md")]
    LongEventAccessoryOff2 = 209,
    /// Teach an event in learn mode.
    ///
    /// Sent by a configuration tool to a node in learn mode to teach it an event variable. Also teaches it the associated event. This command is repeated for each EV required.
    #[doc = include_str!("../docs/opcode/evlrn.md")]
    TeachEvent = 210,
    /// Response to a request for an EV value in a node in learn mode.
    ///
    /// A node response to a request from a configuration tool for the EVs associated with an event (REQEV). For multiple EVs, there will be one
    #[doc = include_str!("../docs/opcode/evans.md")]
    EventVariableValueInLearnMode = 211,
    /// Accessory Response Event.
    ///
    /// Indicates an ‘ON’ response event with two added data bytes. A response is a reply to a status request (AREQ) without producing an
    #[doc = include_str!("../docs/opcode/aron2.md")]
    LongEventAccessoryStateOn2 = 212,
    /// Accessory Response Event.
    ///
    /// Indicates an ‘OFF’ response event with two added data bytes. A response is a reply to a status request (AREQ) without producing an
    #[doc = include_str!("../docs/opcode/arof2.md")]
    LongEventAccessoryStateOff2 = 213,
    /// Accessory Short ON.
    /// Indicates an ‘ON’ event using the short event number of 2 LS bytes with two added data bytes. An event is sent by a module when it detects a change of state. Modules may consume the event and perform actions. The NN is not used to match events, the NN normally indicates the source of the event. The meaning of the additional data is dependent upon the application and must be agreed between the producer and consumer of the event.
    #[doc = include_str!("../docs/opcode/ason2.md")]
    ShortEventAccessoryOn2 = 216,
    /// Accessory Short OFF.
    /// Indicates an ‘OFF’ event using the short event number of 2 LS bytes with two added data bytes. An event is sent by a module when it detects a change of state. Modules may consume the event and perform actions. The NN is not used to match events, the NN normally indicates the source of the event. The meaning of the additional data is dependent upon the application and must be agreed between the producer and consumer of the event.
    #[doc = include_str!("../docs/opcode/asof2.md")]
    ShortEventAccessoryOff2 = 217,
    /// Accessory Short Response Event ON with two data bytes.
    ///
    /// Indicates an ‘ON’ response event with two added data bytes. A response is a reply to a status request (ASRQ)without producing an ON
    #[doc = include_str!("../docs/opcode/arson2.md")]
    ShortEventAccessoryStateOn2 = 221,
    /// Accessory Short Response Event OFF with two data bytes.
    ///
    /// Indicates an ‘OFF’ response event with two added data bytes. A response is a reply to a status request (ASRQ) without producing an
    #[doc = include_str!("../docs/opcode/arsof2.md")]
    ShortEventAccessoryStateOff2 = 222,
    /// Extended opcode with 5 additional bytes.
    ///
    /// Reserved to allow the 5 additional bytes range to be extended by a further 256 opcodes.
    #[doc = include_str!("../docs/opcode/extc5.md")]
    ExtOpCode5 = 223,
    /// Request 6-byte DCC Packet.
    ///
    /// Allows a CAB or equivalent to request a 6 byte DCC packet to be sent to the track. The packet is sent <REP> times and is not refreshed on a regular basis.
    #[doc = include_str!("../docs/opcode/rdcc6.md")]
    DccSendRawPacket6 = 224,
    /// Engine report.
    ///
    /// A report of an engine entry sent by the command station. Sent in response to QLOC or as an acknowledgement of acquiring an engine
    #[doc = include_str!("../docs/opcode/ploc.md")]
    DccLocoReport = 225,
    /// Response to request for node name string.
    ///
    /// Returns the type name for the module in response to RQMN. Any
    #[doc = include_str!("../docs/opcode/name.md")]
    ModuleName = 226,
    /// Command Station status report.
    ///
    /// Sent by the command station in response to RSTAT.
    #[doc = include_str!("../docs/opcode/stat.md")]
    DccCommandStationStatus = 227,
    /// Event Acknowledge.
    ///
    /// Sent by a module to acknowledge the consumption of an event. Used for diagnostic purposes.
    #[doc = include_str!("../docs/opcode/enack.md")]
    EventAck = 230,
    /// Extended service discovery response.
    ///
    /// Detailed information about a service supported by a module. The data supplied is service specific. Sent in response to RQSD with ServiceIndex is not zero. A single ESD
    #[doc = include_str!("../docs/opcode/esd.md")]
    ExtendedServiceDiscoveryResponse = 231,
    /// Streaming protocol (RFC0005).
    ///
    /// Used to transport a relatively large block of data. StreamID is a unique layout wide identifier of a particular message stream. It is the responsibility of the layout installer/module installer to ensure that any StreamIDs are unique across the installation. StreamIDs 0~20 are reserved as CBUS system wide IDs. Users would not allocate these IDs to private streams. SequenceNum is a 0x00 to 0xFF identification of the frame sequence . 0x00 is used to denote a header frame , any number != 0x00 indicates a continuation frame. MessageLen is a 16 bit size in bytes of the transmitted message , However as only 254 continuation frames are possible the Message Len is limited to a count of 1275. However private protocols may be deployed to send larger messages and the full extent of these fields can be used. A Message length of Zero is supported, albeit rather pointless. 16 15 2 CRC16 is a standard implementation of CRC, ie P(x) = x +x +x +1. CRC fields are optional and set to 0x00 if not implemented. Continuation frames merely contain the StreamID and SequenceNum and 5 bytes of message data It is recommended that continuation frames are throttled at 1 over 20ms.
    #[doc = include_str!("../docs/opcode/dtxc.md")]
    StreamPacket = 233,
    /// Response to request for node parameters.
    ///
    /// Returns the first 7 parameters for the module in response to RQNP.
    #[doc = include_str!("../docs/opcode/params.md")]
    NodeParametersReport = 239,
    /// Accessory ON.
    ///
    /// Indicates an ‘ON’ event using the full event number of 4 bytes with three additional data bytes. An event is sent by a module when it detects a change of state. Modules may consume the event and perform actions. The meaning of the additional data is dependent upon the application and must be agreed between the producer and consumer of the event.
    #[doc = include_str!("../docs/opcode/acon3.md")]
    LongEventAccessoryOn3 = 240,
    /// Accessory OFF.
    ///
    /// Indicates an ‘OFF’ event using the full event number of 4 bytes with three additional data bytes. An event is sent by a module when it detects a change of state. Modules may consume the event and perform actions. The meaning of the additional data is dependent upon the application and must be agreed between the producer and consumer of the event.
    #[doc = include_str!("../docs/opcode/acof3.md")]
    LongEventAccessoryOff3 = 241,
    /// Response to request to read node events.
    ///
    /// This is a response to either NERD or NENRD.
    #[doc = include_str!("../docs/opcode/enrsp.md")]
    LearnedEventResponse = 242,
    /// Accessory Response Event.
    ///
    /// Indicates an ‘ON’ response event with three added data bytes. A response is a reply to a status request (AREQ) without producing an
    #[doc = include_str!("../docs/opcode/aron3.md")]
    LongEventAccessoryStateOn3 = 243,
    /// Accessory Response Event.
    ///
    /// Indicates an ‘OFF’ response event with three added data bytes. A response is a reply to a status request (AREQ) without producing an
    #[doc = include_str!("../docs/opcode/arof3.md")]
    LongEventAccessoryStateOff3 = 244,
    /// Teach an event in learn mode using event indexing.
    ///
    /// Sent by a configuration tool to a node in learn mode to teach it an event. The event index must be known. Also teaches it the associated event variables (EVs). This command is repeated for each EV required. Parameter EN# is ignored and this request is similar to EVLRN.
    #[doc = include_str!("../docs/opcode/evlrni.md")]
    TeachEventByIndex = 245,
    /// Accessory node data event.
    ///
    /// Indicates an event from this node with 5 bytes of data. For example, this can be used to send the 40 bits of an RFID tag. There is no event number in order to allow space for 5 bytes of data in the packet, so there can only be one data event per node. The meaning of the event is therefore dependent upon the type and use of the module.
    #[doc = include_str!("../docs/opcode/acdat.md")]
    DataEventAccessory = 246,
    /// Accessory node data Response.
    ///
    /// Indicates a node data response. A response event is a reply to a status request (RQDAT) without producing a new data event.
    #[doc = include_str!("../docs/opcode/ardat.md")]
    NodeDataEventResponse = 247,
    /// Accessory Short ON.
    ///
    /// Indicates an ‘ON’ event using the short event number of 2 LS bytes with three added data bytes. An event is sent by a module when it detects a change of state. Modules may consume the event and perform actions. The NN is not used to match events, the NN normally indicates the source of the event. The meaning of the additional data is dependent upon the application and must be agreed between the producer and consumer of the event.
    #[doc = include_str!("../docs/opcode/ason3.md")]
    ShortEventAccessoryOn3 = 248,
    /// Accessory Short OFF.
    ///
    /// Indicates an ‘OFF’ event using the short event number of 2 LS bytes with three added data bytes. An event is sent by a module when it detects a change of state. Modules may consume the event and perform actions. The NN is not used to match events, the NN normally indicates the source of the event. The meaning of the additional data is dependent upon the application and must be agreed between the producer and consumer of the event.
    #[doc = include_str!("../docs/opcode/asof3.md")]
    ShortEventAccessoryOff3 = 249,
    /// Device data event (short mode).
    ///
    /// Function is the same as ACDAT but uses device addressing so it can
    #[doc = include_str!("../docs/opcode/ddes.md")]
    DeviceDataEventShortMode = 250,
    /// Device data response (short mode).
    ///
    /// The response to a RQDDS request for data from a device.
    #[doc = include_str!("../docs/opcode/ddrs.md")]
    DeviceDataResponseShortMode = 251,
    /// Write data.
    ///
    /// Used to write data to a device such as a RFID tag. data1 ~ data5 is data to be written to the device. RC522 devices should have data1 set to 0
    #[doc = include_str!("../docs/opcode/ddws.md")]
    WriteData = 252,
    /// Accessory Short Response Event.
    ///
    /// Indicates an ‘ON’ response event with three added data bytes.A response is a reply to a status request (ASRQ)without producing an ON
    #[doc = include_str!("../docs/opcode/arson3.md")]
    ShortEventAccessoryStateOn3 = 253,
    /// Accessory Short Response Event.
    ///
    /// Indicates an ‘OFF’ response event with three added data bytes.A response is a reply to a status request (ASRQ) without producing an
    #[doc = include_str!("../docs/opcode/arsof3.md")]
    ShortEventAccessoryStateOff3 = 254,
    /// Extended opcode with 6 additional bytes.
    ///
    /// Reserved to allow the 6 additional bytes range to be extended by a further 256 opcodes.
    #[doc = include_str!("../docs/opcode/extc6.md")]
    ExtOpCode6 = 255,
}
/// VLCB Service Types
#[derive(
    Debug,
    Copy,
    Clone,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
    TryFromPrimitive,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum ServiceType {
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
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
    FromPrimitive
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum ModuleMode {
    /// Uninitialized / factory settings
    #[default]
    Uninitialized = 255,
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
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
    TryFromPrimitive,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum GenericResponseStatus {
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
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
    TryFromPrimitive,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum SysPixieModuleType {
    /// Motorised point motor driver with current sense
    CANPMSense = 1,
}
/// Modes for STMOD
#[derive(
    Debug,
    Copy,
    Clone,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
    TryFromPrimitive,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum DccThrottleMode {
    /// 128-step speed mode
    Step128 = 0,
    /// 14-step speed mode
    Step14 = 1,
    /// interleaved 28-step speed mode
    Step28Interleaved = 2,
    /// 28-step speed mode
    Step28 = 3,
}
/// Status codes for SSTAT message
#[derive(
    Debug,
    Copy,
    Clone,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
    TryFromPrimitive,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum DccServiceModeStatus {
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
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
    TryFromPrimitive,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum SprogModuleType {
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
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
    TryFromPrimitive,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum SpectrumModuleType {
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
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
    TryFromPrimitive,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum RocRailModuleType {
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
/// Parameter index numbers (readable by OPC_RQNPN, returned in OPC_PARAN)
/// Index numbers count from 1, subtract 1 for offset into parameter block
/// Note that RQNPN with index 0 returns the parameter count
#[derive(
    Debug,
    Copy,
    Clone,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
    TryFromPrimitive,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum ModuleParams {
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
bitflags! {
    #[doc = " Flags in node parameter FLAGS"] #[derive(Debug, Copy, Clone)] pub struct
    ModuleFlags : u8 { #[doc = " Module doesn't support events"] const EventsUnsupported
    = 0b00000000; #[doc = " Module is a consumer of events"] const EventConsumer =
    0b00000001; #[doc = " Module is a producer of events"] const EventProducer =
    0b00000010; #[doc = " Module is both a consumer and producer of events"] const
    EventCombi = 0b00000011; #[doc = " Module is in FLiM (CBUS)"] const FLiM =
    0b00000100; #[doc = " Module is in Normal mode (VLCB)"] const NormalMode =
    0b00000100; #[doc = " Module supports the FCU bootloader protocol"] const Bootloader
    = 0b00001000; #[doc = " Module can consume its own events"] const ConsumeOwnEvents =
    0b00010000; #[doc = " Module is in learn mode"] const LearnMode = 0b00100000; #[doc =
    " Module is VLCB compatible"] const VLCB = 0b01000000; #[doc =
    " Module supports Service Discovery (Deprecated in favour of PF_VLCB.)"] const
    ServiceDiscovery = 0b01000000; }
}
/// MERG Module types
#[derive(
    Debug,
    Copy,
    Clone,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
    TryFromPrimitive,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum MergModuleType {
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
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
    TryFromPrimitive,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum Manufacturer {
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
/// Error codes for ERR message
#[derive(
    Debug,
    Copy,
    Clone,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
    TryFromPrimitive,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum DccError {
    LocoStackIsFull = 1,
    LocoAddressIsTaken = 2,
    SessionIsNotPresent = 3,
    EmptyConsist = 4,
    LocoWasNotFound = 5,
    RxBufferOverflow = 6,
    InvalidRequest = 7,
    SessionWasCancelled = 8,
}
/// Error codes for CMDERR message
#[derive(
    Debug,
    Copy,
    Clone,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
    TryFromPrimitive,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CommandError {
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
/// Aspect codes for the first aspect byte of the CABDAT message
///
/// Colours in brackets correspond to UK colour light
/// signalling, the given aspect names may be displayed differently
/// in other signalling systems
#[derive(
    Debug,
    Copy,
    Clone,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
    TryFromPrimitive,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CabSignallingAspect1 {
    Danger = 0,
    Caution = 1,
    PreliminaryCaution = 2,
    Proceed = 3,
    /// Set bit 2 for call-on - main aspect will usually be at danger
    CallOn = 4,
    /// Set bit 3 to 0 for upper nibble is feather location, set 1 for upper nibble is theatre code
    Theatre = 8,
}
/// Defines the meaning of the remaining 3 bytes of the CABDAT message
#[derive(
    Debug,
    Copy,
    Clone,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
    TryFromPrimitive,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CabDataCode {
    /// CABSIG
    /// Transmitted by a layout control system to send
    /// signal aspects to be displayed on a cab handset as cab
    /// signalling.
    CabSignalling = 1,
}
/// BUS type that module is connected to
#[derive(
    Debug,
    Copy,
    Clone,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
    TryFromPrimitive,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum BusType {
    CAN = 1,
    Ethernet = 2,
    MiWi = 3,
    USB = 4,
}
/// Microchip Processor type codes (used by FCU to identify correct bootloader compatibility)
#[derive(
    Debug,
    Copy,
    Clone,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
    TryFromPrimitive,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum MicrochipProcessor {
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
/// ARM Processor type codes (used by FCU to identify correct bootloader compatibility)
#[derive(
    Debug,
    Copy,
    Clone,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
    TryFromPrimitive,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum ArmProcessor {
    Arm1176JzfS = 1,
    CortexA7 = 2,
    CortexA53 = 3,
}
/// Processor manufacturer codes
#[derive(
    Debug,
    Copy,
    Clone,
    UnsafeFromPrimitive,
    IntoPrimitive,
    Eq,
    PartialEq,
    TryFromPrimitive,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum ProcessorManufacturer {
    Microchip = 1,
    Atmel = 2,
    Arm = 3,
}
