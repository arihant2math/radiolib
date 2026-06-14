use core::fmt;

/// Type used for radio-agnostic IRQ flags.
pub type RadioLibIrqFlags = u32;

/// Data shaping filter values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum DataShaping {
    /// No shaping.
    #[default]
    None = 0x00,
    /// Gaussian shaping filter, BT = 0.3.
    GaussianBt03 = 0x01,
    /// Gaussian shaping filter, BT = 0.5.
    GaussianBt05 = 0x02,
    /// Gaussian shaping filter, BT = 0.7.
    GaussianBt07 = 0x03,
    /// Gaussian shaping filter, BT = 1.0.
    GaussianBt10 = 0x04,
}

impl DataShaping {
    /// Returns the raw RadioLib value.
    pub const fn as_raw(self) -> u8 {
        self as u8
    }

    /// Parses a raw RadioLib value.
    pub const fn from_raw(value: u8) -> Option<Self> {
        match value {
            0x00 => Some(Self::None),
            0x01 => Some(Self::GaussianBt03),
            0x02 => Some(Self::GaussianBt05),
            0x03 => Some(Self::GaussianBt07),
            0x04 => Some(Self::GaussianBt10),
            _ => None,
        }
    }
}

/// Line encoding values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum Encoding {
    /// Non-return-to-zero; no encoding.
    #[default]
    Nrz = 0x00,
    /// Manchester encoding.
    Manchester = 0x01,
    /// Whitening.
    Whitening = 0x02,
    /// Inverted Manchester encoding.
    InvertedManchester = 0x03,
}

impl Encoding {
    /// Returns the raw RadioLib value.
    pub const fn as_raw(self) -> u8 {
        self as u8
    }

    /// Parses a raw RadioLib value.
    pub const fn from_raw(value: u8) -> Option<Self> {
        match value {
            0x00 => Some(Self::Nrz),
            0x01 => Some(Self::Manchester),
            0x02 => Some(Self::Whitening),
            0x03 => Some(Self::InvertedManchester),
            _ => None,
        }
    }
}

/// Standby mode selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum StandbyMode {
    /// Module-specific default standby mode.
    #[default]
    Default = 0x00,
    /// Warm standby, e.g. crystal left running.
    Warm = 0x01,
    /// Cold standby, e.g. only internal RC oscillator running.
    Cold = 0x02,
}

impl StandbyMode {
    /// Returns the raw RadioLib value.
    pub const fn as_raw(self) -> u8 {
        self as u8
    }

    /// Parses a raw RadioLib value.
    pub const fn from_raw(value: u8) -> Option<Self> {
        match value {
            0x00 => Some(Self::Default),
            0x01 => Some(Self::Warm),
            0x02 => Some(Self::Cold),
            _ => None,
        }
    }
}

/// Meaningful non-error RadioLib event code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Event {
    /// LoRa preamble was detected during channel activity detection.
    PreambleDetected,
    /// No LoRa preambles were detected during channel activity detection.
    ChannelFree,
    /// LoRa transmission detected while scanning the channel.
    LoraDetected,
    /// The LoRaWAN session was successfully restored.
    LorawanSessionRestored,
    /// A new LoRaWAN session was started.
    LorawanNewSession,
}

impl Event {
    /// Returns the raw RadioLib value.
    pub const fn as_raw(self) -> i16 {
        match self {
            Self::PreambleDetected => -14,
            Self::ChannelFree => -15,
            Self::LoraDetected => -702,
            Self::LorawanSessionRestored => -1117,
            Self::LorawanNewSession => -1118,
        }
    }

    /// Parses a raw RadioLib event code.
    pub const fn from_raw(code: i16) -> Option<Self> {
        match code {
            -14 => Some(Self::PreambleDetected),
            -15 => Some(Self::ChannelFree),
            -702 => Some(Self::LoraDetected),
            -1117 => Some(Self::LorawanSessionRestored),
            -1118 => Some(Self::LorawanNewSession),
            _ => None,
        }
    }

    /// Returns the symbolic event name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::PreambleDetected => "PREAMBLE_DETECTED",
            Self::ChannelFree => "CHANNEL_FREE",
            Self::LoraDetected => "LORA_DETECTED",
            Self::LorawanSessionRestored => "LORAWAN_SESSION_RESTORED",
            Self::LorawanNewSession => "LORAWAN_NEW_SESSION",
        }
    }
}

impl From<Event> for i16 {
    fn from(value: Event) -> Self {
        value.as_raw()
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name(), self.as_raw())
    }
}

/// RadioLib error code.
///
/// Most upstream values are fixed numeric constants, but the LR11x0 GNSS error families are
/// parameterized and therefore map naturally to payload-bearing variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error {
    Unknown,
    ChipNotFound,
    MemoryAllocationFailed,
    PacketTooLong,
    TxTimeout,
    RxTimeout,
    CrcMismatch,
    InvalidBandwidth,
    InvalidSpreadingFactor,
    InvalidCodingRate,
    InvalidBitRange,
    InvalidFrequency,
    InvalidOutputPower,
    SpiWriteFailed,
    InvalidCurrentLimit,
    InvalidPreambleLength,
    InvalidGain,
    WrongModem,
    InvalidNumSamples,
    InvalidRssiOffset,
    InvalidEncoding,
    LoraHeaderDamaged,
    Unsupported,
    InvalidDioPin,
    InvalidRssiThreshold,
    NullPointer,
    InvalidIrq,
    PacketTooShort,
    InvalidBitRate,
    InvalidFrequencyDeviation,
    InvalidBitRateBwRatio,
    InvalidRxBandwidth,
    InvalidSyncWord,
    InvalidDataShaping,
    InvalidModulation,
    InvalidOokRssiPeakType,
    InvalidBitRateToleranceValue,
    InvalidSymbol,
    InvalidMicETelemetry,
    InvalidMicETelemetryLength,
    MicETelemetryStatus,
    InvalidSsdvMode,
    InvalidImageSize,
    InvalidImageQuality,
    InvalidSubsampling,
    InvalidRttyShift,
    UnsupportedEncoding,
    InvalidDataRate,
    InvalidAddressWidth,
    InvalidPipeNumber,
    AckNotReceived,
    InvalidNumBroadAddrs,
    InvalidCrcConfiguration,
    InvalidTcxoVoltage,
    InvalidModulationParameters,
    SpiCmdTimeout,
    SpiCmdInvalid,
    SpiCmdFailed,
    InvalidSleepPeriod,
    InvalidRxPeriod,
    InvalidCallsign,
    InvalidNumRepeaters,
    InvalidRepeaterCallsign,
    RangingTimeout,
    InvalidPayload,
    AddressNotFound,
    InvalidFunction,
    NetworkNotJoined,
    DownlinkMalformed,
    InvalidRevision,
    InvalidPort,
    NoRxWindow,
    NoChannelAvailable,
    InvalidCid,
    UplinkUnavailable,
    CommandQueueFull,
    CommandQueueItemNotFound,
    JoinNonceInvalid,
    MicMismatch,
    MulticastFcntInvalid,
    DwellTimeExceeded,
    ChecksumMismatch,
    NoJoinAccept,
    NoncesDiscarded,
    SessionDiscarded,
    InvalidMode,
    InvalidMulticastGroup,
    InvalidWifiType,
    GnssSubframeNotAvailable,
    /// LR11x0 GNSS demodulator error.
    GnssDemod(i16),
    /// LR11x0 GNSS solver error.
    GnssSolver(i16),
    FrontendCalibrationFailed,
    InvalidSideDetect,
    AdsbInvalidMsgType,
    AdsbInvalidCategory,
    /// A future or otherwise unknown error code.
    Other(i16),
}

impl Error {
    /// Base offset used by RadioLib's `RADIOLIB_ERR_GNSS_DEMOD(X)` macro.
    pub const GNSS_DEMOD_OFFSET: i16 = -1210;

    /// Base offset used by RadioLib's `RADIOLIB_ERR_GNSS_SOLVER(X)` macro.
    pub const GNSS_SOLVER_OFFSET: i16 = -1230;

    /// Returns the raw RadioLib value.
    pub const fn as_raw(self) -> i16 {
        match self {
            Self::Unknown => -1,
            Self::ChipNotFound => -2,
            Self::MemoryAllocationFailed => -3,
            Self::PacketTooLong => -4,
            Self::TxTimeout => -5,
            Self::RxTimeout => -6,
            Self::CrcMismatch => -7,
            Self::InvalidBandwidth => -8,
            Self::InvalidSpreadingFactor => -9,
            Self::InvalidCodingRate => -10,
            Self::InvalidBitRange => -11,
            Self::InvalidFrequency => -12,
            Self::InvalidOutputPower => -13,
            Self::SpiWriteFailed => -16,
            Self::InvalidCurrentLimit => -17,
            Self::InvalidPreambleLength => -18,
            Self::InvalidGain => -19,
            Self::WrongModem => -20,
            Self::InvalidNumSamples => -21,
            Self::InvalidRssiOffset => -22,
            Self::InvalidEncoding => -23,
            Self::LoraHeaderDamaged => -24,
            Self::Unsupported => -25,
            Self::InvalidDioPin => -26,
            Self::InvalidRssiThreshold => -27,
            Self::NullPointer => -28,
            Self::InvalidIrq => -29,
            Self::PacketTooShort => -30,
            Self::InvalidBitRate => -101,
            Self::InvalidFrequencyDeviation => -102,
            Self::InvalidBitRateBwRatio => -103,
            Self::InvalidRxBandwidth => -104,
            Self::InvalidSyncWord => -105,
            Self::InvalidDataShaping => -106,
            Self::InvalidModulation => -107,
            Self::InvalidOokRssiPeakType => -108,
            Self::InvalidBitRateToleranceValue => -109,
            Self::InvalidSymbol => -201,
            Self::InvalidMicETelemetry => -202,
            Self::InvalidMicETelemetryLength => -203,
            Self::MicETelemetryStatus => -204,
            Self::InvalidSsdvMode => -301,
            Self::InvalidImageSize => -302,
            Self::InvalidImageQuality => -303,
            Self::InvalidSubsampling => -304,
            Self::InvalidRttyShift => -401,
            Self::UnsupportedEncoding => -402,
            Self::InvalidDataRate => -501,
            Self::InvalidAddressWidth => -502,
            Self::InvalidPipeNumber => -503,
            Self::AckNotReceived => -504,
            Self::InvalidNumBroadAddrs => -601,
            Self::InvalidCrcConfiguration => -701,
            Self::InvalidTcxoVoltage => -703,
            Self::InvalidModulationParameters => -704,
            Self::SpiCmdTimeout => -705,
            Self::SpiCmdInvalid => -706,
            Self::SpiCmdFailed => -707,
            Self::InvalidSleepPeriod => -708,
            Self::InvalidRxPeriod => -709,
            Self::InvalidCallsign => -801,
            Self::InvalidNumRepeaters => -802,
            Self::InvalidRepeaterCallsign => -803,
            Self::RangingTimeout => -901,
            Self::InvalidPayload => -1001,
            Self::AddressNotFound => -1002,
            Self::InvalidFunction => -1003,
            Self::NetworkNotJoined => -1101,
            Self::DownlinkMalformed => -1102,
            Self::InvalidRevision => -1103,
            Self::InvalidPort => -1104,
            Self::NoRxWindow => -1105,
            Self::NoChannelAvailable => -1106,
            Self::InvalidCid => -1107,
            Self::UplinkUnavailable => -1108,
            Self::CommandQueueFull => -1109,
            Self::CommandQueueItemNotFound => -1110,
            Self::JoinNonceInvalid => -1111,
            Self::MicMismatch => -1112,
            Self::MulticastFcntInvalid => -1113,
            Self::DwellTimeExceeded => -1114,
            Self::ChecksumMismatch => -1115,
            Self::NoJoinAccept => -1116,
            Self::NoncesDiscarded => -1119,
            Self::SessionDiscarded => -1120,
            Self::InvalidMode => -1121,
            Self::InvalidMulticastGroup => -1122,
            Self::InvalidWifiType => -1200,
            Self::GnssSubframeNotAvailable => -1201,
            Self::GnssDemod(error) => Self::GNSS_DEMOD_OFFSET + error,
            Self::GnssSolver(error) => Self::GNSS_SOLVER_OFFSET - error,
            Self::FrontendCalibrationFailed => -1300,
            Self::InvalidSideDetect => -1301,
            Self::AdsbInvalidMsgType => -1400,
            Self::AdsbInvalidCategory => -1401,
            Self::Other(code) => code,
        }
    }

    /// Parses a raw RadioLib error code.
    ///
    /// Returns `None` for success and for known non-error event codes.
    pub const fn from_raw(code: i16) -> Option<Self> {
        match code {
            0 => None,
            -14 | -15 | -702 | -1117 | -1118 => None,
            -1 => Some(Self::Unknown),
            -2 => Some(Self::ChipNotFound),
            -3 => Some(Self::MemoryAllocationFailed),
            -4 => Some(Self::PacketTooLong),
            -5 => Some(Self::TxTimeout),
            -6 => Some(Self::RxTimeout),
            -7 => Some(Self::CrcMismatch),
            -8 => Some(Self::InvalidBandwidth),
            -9 => Some(Self::InvalidSpreadingFactor),
            -10 => Some(Self::InvalidCodingRate),
            -11 => Some(Self::InvalidBitRange),
            -12 => Some(Self::InvalidFrequency),
            -13 => Some(Self::InvalidOutputPower),
            -16 => Some(Self::SpiWriteFailed),
            -17 => Some(Self::InvalidCurrentLimit),
            -18 => Some(Self::InvalidPreambleLength),
            -19 => Some(Self::InvalidGain),
            -20 => Some(Self::WrongModem),
            -21 => Some(Self::InvalidNumSamples),
            -22 => Some(Self::InvalidRssiOffset),
            -23 => Some(Self::InvalidEncoding),
            -24 => Some(Self::LoraHeaderDamaged),
            -25 => Some(Self::Unsupported),
            -26 => Some(Self::InvalidDioPin),
            -27 => Some(Self::InvalidRssiThreshold),
            -28 => Some(Self::NullPointer),
            -29 => Some(Self::InvalidIrq),
            -30 => Some(Self::PacketTooShort),
            -101 => Some(Self::InvalidBitRate),
            -102 => Some(Self::InvalidFrequencyDeviation),
            -103 => Some(Self::InvalidBitRateBwRatio),
            -104 => Some(Self::InvalidRxBandwidth),
            -105 => Some(Self::InvalidSyncWord),
            -106 => Some(Self::InvalidDataShaping),
            -107 => Some(Self::InvalidModulation),
            -108 => Some(Self::InvalidOokRssiPeakType),
            -109 => Some(Self::InvalidBitRateToleranceValue),
            -201 => Some(Self::InvalidSymbol),
            -202 => Some(Self::InvalidMicETelemetry),
            -203 => Some(Self::InvalidMicETelemetryLength),
            -204 => Some(Self::MicETelemetryStatus),
            -301 => Some(Self::InvalidSsdvMode),
            -302 => Some(Self::InvalidImageSize),
            -303 => Some(Self::InvalidImageQuality),
            -304 => Some(Self::InvalidSubsampling),
            -401 => Some(Self::InvalidRttyShift),
            -402 => Some(Self::UnsupportedEncoding),
            -501 => Some(Self::InvalidDataRate),
            -502 => Some(Self::InvalidAddressWidth),
            -503 => Some(Self::InvalidPipeNumber),
            -504 => Some(Self::AckNotReceived),
            -601 => Some(Self::InvalidNumBroadAddrs),
            -701 => Some(Self::InvalidCrcConfiguration),
            -703 => Some(Self::InvalidTcxoVoltage),
            -704 => Some(Self::InvalidModulationParameters),
            -705 => Some(Self::SpiCmdTimeout),
            -706 => Some(Self::SpiCmdInvalid),
            -707 => Some(Self::SpiCmdFailed),
            -708 => Some(Self::InvalidSleepPeriod),
            -709 => Some(Self::InvalidRxPeriod),
            -801 => Some(Self::InvalidCallsign),
            -802 => Some(Self::InvalidNumRepeaters),
            -803 => Some(Self::InvalidRepeaterCallsign),
            -901 => Some(Self::RangingTimeout),
            -1001 => Some(Self::InvalidPayload),
            -1002 => Some(Self::AddressNotFound),
            -1003 => Some(Self::InvalidFunction),
            -1101 => Some(Self::NetworkNotJoined),
            -1102 => Some(Self::DownlinkMalformed),
            -1103 => Some(Self::InvalidRevision),
            -1104 => Some(Self::InvalidPort),
            -1105 => Some(Self::NoRxWindow),
            -1106 => Some(Self::NoChannelAvailable),
            -1107 => Some(Self::InvalidCid),
            -1108 => Some(Self::UplinkUnavailable),
            -1109 => Some(Self::CommandQueueFull),
            -1110 => Some(Self::CommandQueueItemNotFound),
            -1111 => Some(Self::JoinNonceInvalid),
            -1112 => Some(Self::MicMismatch),
            -1113 => Some(Self::MulticastFcntInvalid),
            -1114 => Some(Self::DwellTimeExceeded),
            -1115 => Some(Self::ChecksumMismatch),
            -1116 => Some(Self::NoJoinAccept),
            -1119 => Some(Self::NoncesDiscarded),
            -1120 => Some(Self::SessionDiscarded),
            -1121 => Some(Self::InvalidMode),
            -1122 => Some(Self::InvalidMulticastGroup),
            -1200 => Some(Self::InvalidWifiType),
            -1201 => Some(Self::GnssSubframeNotAvailable),
            -1300 => Some(Self::FrontendCalibrationFailed),
            -1301 => Some(Self::InvalidSideDetect),
            -1400 => Some(Self::AdsbInvalidMsgType),
            -1401 => Some(Self::AdsbInvalidCategory),
            _ => {
                if code <= Self::GNSS_DEMOD_OFFSET && code > Self::GNSS_SOLVER_OFFSET {
                    Some(Self::GnssDemod(code - Self::GNSS_DEMOD_OFFSET))
                } else if code <= Self::GNSS_SOLVER_OFFSET && code > -1300 {
                    Some(Self::GnssSolver(-(code - Self::GNSS_SOLVER_OFFSET)))
                } else {
                    Some(Self::Other(code))
                }
            }
        }
    }

    /// Returns the symbolic error name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Unknown => "UNKNOWN",
            Self::ChipNotFound => "CHIP_NOT_FOUND",
            Self::MemoryAllocationFailed => "MEMORY_ALLOCATION_FAILED",
            Self::PacketTooLong => "PACKET_TOO_LONG",
            Self::TxTimeout => "TX_TIMEOUT",
            Self::RxTimeout => "RX_TIMEOUT",
            Self::CrcMismatch => "CRC_MISMATCH",
            Self::InvalidBandwidth => "INVALID_BANDWIDTH",
            Self::InvalidSpreadingFactor => "INVALID_SPREADING_FACTOR",
            Self::InvalidCodingRate => "INVALID_CODING_RATE",
            Self::InvalidBitRange => "INVALID_BIT_RANGE",
            Self::InvalidFrequency => "INVALID_FREQUENCY",
            Self::InvalidOutputPower => "INVALID_OUTPUT_POWER",
            Self::SpiWriteFailed => "SPI_WRITE_FAILED",
            Self::InvalidCurrentLimit => "INVALID_CURRENT_LIMIT",
            Self::InvalidPreambleLength => "INVALID_PREAMBLE_LENGTH",
            Self::InvalidGain => "INVALID_GAIN",
            Self::WrongModem => "WRONG_MODEM",
            Self::InvalidNumSamples => "INVALID_NUM_SAMPLES",
            Self::InvalidRssiOffset => "INVALID_RSSI_OFFSET",
            Self::InvalidEncoding => "INVALID_ENCODING",
            Self::LoraHeaderDamaged => "LORA_HEADER_DAMAGED",
            Self::Unsupported => "UNSUPPORTED",
            Self::InvalidDioPin => "INVALID_DIO_PIN",
            Self::InvalidRssiThreshold => "INVALID_RSSI_THRESHOLD",
            Self::NullPointer => "NULL_POINTER",
            Self::InvalidIrq => "INVALID_IRQ",
            Self::PacketTooShort => "PACKET_TOO_SHORT",
            Self::InvalidBitRate => "INVALID_BIT_RATE",
            Self::InvalidFrequencyDeviation => "INVALID_FREQUENCY_DEVIATION",
            Self::InvalidBitRateBwRatio => "INVALID_BIT_RATE_BW_RATIO",
            Self::InvalidRxBandwidth => "INVALID_RX_BANDWIDTH",
            Self::InvalidSyncWord => "INVALID_SYNC_WORD",
            Self::InvalidDataShaping => "INVALID_DATA_SHAPING",
            Self::InvalidModulation => "INVALID_MODULATION",
            Self::InvalidOokRssiPeakType => "INVALID_OOK_RSSI_PEAK_TYPE",
            Self::InvalidBitRateToleranceValue => "INVALID_BIT_RATE_TOLERANCE_VALUE",
            Self::InvalidSymbol => "INVALID_SYMBOL",
            Self::InvalidMicETelemetry => "INVALID_MIC_E_TELEMETRY",
            Self::InvalidMicETelemetryLength => "INVALID_MIC_E_TELEMETRY_LENGTH",
            Self::MicETelemetryStatus => "MIC_E_TELEMETRY_STATUS",
            Self::InvalidSsdvMode => "INVALID_SSDV_MODE",
            Self::InvalidImageSize => "INVALID_IMAGE_SIZE",
            Self::InvalidImageQuality => "INVALID_IMAGE_QUALITY",
            Self::InvalidSubsampling => "INVALID_SUBSAMPLING",
            Self::InvalidRttyShift => "INVALID_RTTY_SHIFT",
            Self::UnsupportedEncoding => "UNSUPPORTED_ENCODING",
            Self::InvalidDataRate => "INVALID_DATA_RATE",
            Self::InvalidAddressWidth => "INVALID_ADDRESS_WIDTH",
            Self::InvalidPipeNumber => "INVALID_PIPE_NUMBER",
            Self::AckNotReceived => "ACK_NOT_RECEIVED",
            Self::InvalidNumBroadAddrs => "INVALID_NUM_BROAD_ADDRS",
            Self::InvalidCrcConfiguration => "INVALID_CRC_CONFIGURATION",
            Self::InvalidTcxoVoltage => "INVALID_TCXO_VOLTAGE",
            Self::InvalidModulationParameters => "INVALID_MODULATION_PARAMETERS",
            Self::SpiCmdTimeout => "SPI_CMD_TIMEOUT",
            Self::SpiCmdInvalid => "SPI_CMD_INVALID",
            Self::SpiCmdFailed => "SPI_CMD_FAILED",
            Self::InvalidSleepPeriod => "INVALID_SLEEP_PERIOD",
            Self::InvalidRxPeriod => "INVALID_RX_PERIOD",
            Self::InvalidCallsign => "INVALID_CALLSIGN",
            Self::InvalidNumRepeaters => "INVALID_NUM_REPEATERS",
            Self::InvalidRepeaterCallsign => "INVALID_REPEATER_CALLSIGN",
            Self::RangingTimeout => "RANGING_TIMEOUT",
            Self::InvalidPayload => "INVALID_PAYLOAD",
            Self::AddressNotFound => "ADDRESS_NOT_FOUND",
            Self::InvalidFunction => "INVALID_FUNCTION",
            Self::NetworkNotJoined => "NETWORK_NOT_JOINED",
            Self::DownlinkMalformed => "DOWNLINK_MALFORMED",
            Self::InvalidRevision => "INVALID_REVISION",
            Self::InvalidPort => "INVALID_PORT",
            Self::NoRxWindow => "NO_RX_WINDOW",
            Self::NoChannelAvailable => "NO_CHANNEL_AVAILABLE",
            Self::InvalidCid => "INVALID_CID",
            Self::UplinkUnavailable => "UPLINK_UNAVAILABLE",
            Self::CommandQueueFull => "COMMAND_QUEUE_FULL",
            Self::CommandQueueItemNotFound => "COMMAND_QUEUE_ITEM_NOT_FOUND",
            Self::JoinNonceInvalid => "JOIN_NONCE_INVALID",
            Self::MicMismatch => "MIC_MISMATCH",
            Self::MulticastFcntInvalid => "MULTICAST_FCNT_INVALID",
            Self::DwellTimeExceeded => "DWELL_TIME_EXCEEDED",
            Self::ChecksumMismatch => "CHECKSUM_MISMATCH",
            Self::NoJoinAccept => "NO_JOIN_ACCEPT",
            Self::NoncesDiscarded => "NONCES_DISCARDED",
            Self::SessionDiscarded => "SESSION_DISCARDED",
            Self::InvalidMode => "INVALID_MODE",
            Self::InvalidMulticastGroup => "INVALID_MULTICAST_GROUP",
            Self::InvalidWifiType => "INVALID_WIFI_TYPE",
            Self::GnssSubframeNotAvailable => "GNSS_SUBFRAME_NOT_AVAILABLE",
            Self::GnssDemod(_) => "GNSS_DEMOD",
            Self::GnssSolver(_) => "GNSS_SOLVER",
            Self::FrontendCalibrationFailed => "FRONTEND_CALIBRATION_FAILED",
            Self::InvalidSideDetect => "INVALID_SIDE_DETECT",
            Self::AdsbInvalidMsgType => "ADSB_INVALID_MSG_TYPE",
            Self::AdsbInvalidCategory => "ADSB_INVALID_CATEGORY",
            Self::Other(_) => "OTHER",
        }
    }

    /// Returns the LR11x0 GNSS demodulator sub-error when applicable.
    pub const fn gnss_demod_error(self) -> Option<i16> {
        match self {
            Self::GnssDemod(error) => Some(error),
            _ => None,
        }
    }

    /// Returns the LR11x0 GNSS solver sub-error when applicable.
    pub const fn gnss_solver_error(self) -> Option<i16> {
        match self {
            Self::GnssSolver(error) => Some(error),
            _ => None,
        }
    }
}

impl From<Error> for i16 {
    fn from(value: Error) -> Self {
        value.as_raw()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name(), self.as_raw())
    }
}

/// High-level classification for a [`Status`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatusKind {
    /// Operation completed successfully.
    Success,
    /// Operation completed with a meaningful non-error status/event code.
    Event,
    /// Operation failed.
    Error,
}

/// Top-level RadioLib status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Status {
    /// Operation completed successfully.
    #[default]
    Ok,
    /// Operation completed with a meaningful non-error event.
    Event(Event),
    /// Operation failed.
    Error(Error),
}

impl Status {
    /// Creates a status from its raw RadioLib code.
    pub const fn from_raw(code: i16) -> Self {
        if code == 0 {
            Self::Ok
        } else if let Some(event) = Event::from_raw(code) {
            Self::Event(event)
        } else if let Some(error) = Error::from_raw(code) {
            Self::Error(error)
        } else {
            Self::Error(Error::Other(code))
        }
    }

    /// Returns the raw RadioLib code.
    pub const fn as_raw(self) -> i16 {
        match self {
            Self::Ok => 0,
            Self::Event(event) => event.as_raw(),
            Self::Error(error) => error.as_raw(),
        }
    }

    /// Returns `true` when this status represents success.
    pub const fn is_ok(self) -> bool {
        matches!(self, Self::Ok)
    }

    /// Returns `true` when this status represents an event.
    pub const fn is_event(self) -> bool {
        matches!(self, Self::Event(_))
    }

    /// Returns `true` when this status represents an error.
    pub const fn is_error(self) -> bool {
        matches!(self, Self::Error(_))
    }

    /// Returns the coarse-grained status classification.
    pub const fn kind(self) -> StatusKind {
        match self {
            Self::Ok => StatusKind::Success,
            Self::Event(_) => StatusKind::Event,
            Self::Error(_) => StatusKind::Error,
        }
    }

    /// Returns the contained event, if any.
    pub const fn event(self) -> Option<Event> {
        match self {
            Self::Event(event) => Some(event),
            _ => None,
        }
    }

    /// Returns the contained error, if any.
    pub const fn error(self) -> Option<Error> {
        match self {
            Self::Error(error) => Some(error),
            _ => None,
        }
    }

    /// Converts the status into a standard Rust-style result.
    pub const fn into_result(self) -> Result<Option<Event>, Error> {
        match self {
            Self::Ok => Ok(None),
            Self::Event(event) => Ok(Some(event)),
            Self::Error(error) => Err(error),
        }
    }

    /// Returns the symbolic status name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::Event(event) => event.name(),
            Self::Error(error) => error.name(),
        }
    }
}

impl From<i16> for Status {
    fn from(value: i16) -> Self {
        Self::from_raw(value)
    }
}

impl From<Status> for i16 {
    fn from(value: Status) -> Self {
        value.as_raw()
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ok => write!(f, "OK (0)"),
            Self::Event(event) => event.fmt(f),
            Self::Error(error) => error.fmt(f),
        }
    }
}
