pub const ERROR_BIT: usize = 1 << (core::mem::size_of::<usize>() * 8 - 1);

#[repr(usize)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Status {
    Success,

    WarnUnknownGlyph,
    WarnDeleteFailure,
    WarnWriteFailure,
    WarnBufferTooSmall,
    WarnStaleData,
    WarnFileSystem,
    WarnResetRequired,

    LoadError = ERROR_BIT | 1,
    InvalidParameter,
    Unsupported,
    BadBufferSize,
    BufferTooSmall,
    NotReady,
    DeviceError,
    OutOfResources,
    WriteProtected,
    VolumeCorrupted,
    VolumeFull,
    NoMedia,
    MediaChanged,
    NotFound,
    AccessDenied,
    NoResponse,
    NoMapping,
    Timeout,
    NotStarted,
    AlreadyStarted,
    Aborted,
    ICMPError,
    TFTPError,
    ProtocolError,
    IncompatibleVersion,
    SecurityViolation,
    CRCError,
    EndOfMedia,
    EndOfFile,
    InvalidLanguage,
    CompromisedData,
    IPAddressConflict,
    HTTPError,
    Unknown,
}

impl From<usize> for Status {
    fn from(n: usize) -> Self {
        use Status::*;
        if n & ERROR_BIT == 0 {
            match n {
                0 => Success,
                1 => WarnUnknownGlyph,
                2 => WarnDeleteFailure,
                3 => WarnWriteFailure,
                4 => WarnBufferTooSmall,
                5 => WarnStaleData,
                6 => WarnFileSystem,
                7 => WarnResetRequired,
                _ => Unknown
            }
        } else {
            let n = n & !ERROR_BIT;
            match n {
                0 => LoadError,
                1 => InvalidParameter,
                2 => Unsupported,
                3 => BadBufferSize,
                4 => BufferTooSmall,
                5 => NotReady,
                6 => DeviceError,
                7 => OutOfResources,
                8 => WriteProtected,
                9 => VolumeCorrupted,
                10 => VolumeFull,
                11 => NoMedia,
                12 => MediaChanged,
                13 => NotFound,
                14 => AccessDenied,
                15 => NoResponse,
                16 => NoMapping,
                17 => Timeout,
                18 => NotStarted,
                19 => AlreadyStarted,
                20 => Aborted,
                21 => ICMPError,
                22 => TFTPError,
                23 => ProtocolError,
                24 => IncompatibleVersion,
                25 => SecurityViolation,
                26 => CRCError,
                27 => EndOfMedia,
                28 => EndOfFile,
                29 => InvalidLanguage,
                30 => CompromisedData,
                31 => IPAddressConflict,
                32 => HTTPError,
                _ => Unknown
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(usize)]
pub enum Error {
    LoadError,
    InvalidParameter,
    Unsupported,
    BadBufferSize,
    BufferTooSmall,
    NotReady,
    DeviceError,
    OutOfResources,
    WriteProtected,
    VolumeCorrupted,
    VolumeFull,
    NoMedia,
    MediaChanged,
    NotFound,
    AccessDenied,
    NoResponse,
    NoMapping,
    Timeout,
    NotStarted,
    AlreadyStarted,
    Aborted,
    ICMPError,
    TFTPError,
    ProtocolError,
    IncompatibleVersion,
    SecurityViolation,
    CRCError,
    EndOfMedia,
    EndOfFile,
    InvalidLanguage,
    CompromisedData,
    IPAddressConflict,
    HTTPError,
    Unknown
}

impl From<usize> for Error {
    fn from(n: usize) -> Self {
        use Error::*;
        match n {
            0 => LoadError,
            1 => InvalidParameter,
            2 => Unsupported,
            3 => BadBufferSize,
            4 => BufferTooSmall,
            5 => NotReady,
            6 => DeviceError,
            7 => OutOfResources,
            8 => WriteProtected,
            9 => VolumeCorrupted,
            10 => VolumeFull,
            11 => NoMedia,
            12 => MediaChanged,
            13 => NotFound,
            14 => AccessDenied,
            15 => NoResponse,
            16 => NoMapping,
            17 => Timeout,
            18 => NotStarted,
            19 => AlreadyStarted,
            20 => Aborted,
            21 => ICMPError,
            22 => TFTPError,
            23 => ProtocolError,
            24 => IncompatibleVersion,
            25 => SecurityViolation,
            26 => CRCError,
            27 => EndOfMedia,
            28 => EndOfFile,
            29 => InvalidLanguage,
            30 => CompromisedData,
            31 => IPAddressConflict,
            32 => HTTPError,
            _ => Unknown
        }
    }
}

impl Status {
    pub fn into_result<T>(self, value: T) -> Result<T, Error> {
        if self as usize & ERROR_BIT == 0 {
            Ok(value)
        }else {
            Err(Error::from(self as usize & !ERROR_BIT))
        }
    }
}

