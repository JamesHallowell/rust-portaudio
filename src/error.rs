//!
//! A module for implementing the Portaudio Error type and
//! implementing the std Error trait.
//!

#![allow(missing_docs)]

use ffi;

/// Error codes returned by PortAudio functions.
#[repr(i32)]
#[derive(
    Copy, Clone, PartialEq, PartialOrd, Debug, thiserror::Error, num_derive::FromPrimitive,
)]
pub enum Error {
    #[error("no error")]
    NoError = ffi::PaErrorCode_paNoError,

    #[error("portaudio not initialized")]
    NotInitialized = ffi::PaErrorCode_paNotInitialized,

    #[error("unanticipated error from the host")]
    UnanticipatedHostError = ffi::PaErrorCode_paUnanticipatedHostError,

    #[error("invalid channel count")]
    InvalidChannelCount = ffi::PaErrorCode_paInvalidChannelCount,

    #[error("invalid sample rate")]
    InvalidSampleRate = ffi::PaErrorCode_paInvalidSampleRate,

    #[error("invalid device")]
    InvalidDevice = ffi::PaErrorCode_paInvalidDevice,

    #[error("invalid flag")]
    InvalidFlag = ffi::PaErrorCode_paInvalidFlag,

    #[error("the sample format is not supported")]
    SampleFormatNotSupported = ffi::PaErrorCode_paSampleFormatNotSupported,

    #[error("input device not compatible with output device")]
    BadIODeviceCombination = ffi::PaErrorCode_paBadIODeviceCombination,

    #[error("memory insufficient")]
    InsufficientMemory = ffi::PaErrorCode_paInsufficientMemory,

    #[error("the buffer is too big")]
    BufferTooBig = ffi::PaErrorCode_paBufferTooBig,

    #[error("the buffer is too small")]
    BufferTooSmall = ffi::PaErrorCode_paBufferTooSmall,

    #[error("invalid callback")]
    NullCallback = ffi::PaErrorCode_paNullCallback,

    #[error("invalid Stream")]
    BadStreamPtr = ffi::PaErrorCode_paBadStreamPtr,

    #[error("time out")]
    TimedOut = ffi::PaErrorCode_paTimedOut,

    #[error("portaudio internal error")]
    InternalError = ffi::PaErrorCode_paInternalError,

    #[error("device unavailable")]
    DeviceUnavailable = ffi::PaErrorCode_paDeviceUnavailable,

    #[error("stream info not compatible with the host")]
    IncompatibleHostApiSpecificStreamInfo =
        ffi::PaErrorCode_paIncompatibleHostApiSpecificStreamInfo,

    #[error("the stream is stopped")]
    StreamIsStopped = ffi::PaErrorCode_paStreamIsStopped,

    #[error("the stream is not stopped")]
    StreamIsNotStopped = ffi::PaErrorCode_paStreamIsNotStopped,

    #[error("the input stream has overflowed")]
    InputOverflowed = ffi::PaErrorCode_paInputOverflowed,

    #[error("the output has underflowed")]
    OutputUnderflowed = ffi::PaErrorCode_paOutputUnderflowed,

    #[error("the host api is not found by portaudio")]
    HostApiNotFound = ffi::PaErrorCode_paHostApiNotFound,

    #[error("the host api is invalid")]
    InvalidHostApi = ffi::PaErrorCode_paInvalidHostApi,

    #[error("portaudio cannot read from the callback stream")]
    CanNotReadFromACallbackStream = ffi::PaErrorCode_paCanNotReadFromACallbackStream,

    #[error("portaudio cannot write to the callback stream")]
    CanNotWriteToACallbackStream = ffi::PaErrorCode_paCanNotWriteToACallbackStream,

    #[error("portaudio cannot read from an output only stream")]
    CanNotReadFromAnOutputOnlyStream = ffi::PaErrorCode_paCanNotReadFromAnOutputOnlyStream,

    #[error("portaudio cannot write to an input only stream")]
    CanNotWriteToAnInputOnlyStream = ffi::PaErrorCode_paCanNotWriteToAnInputOnlyStream,

    #[error("the stream is not compatible with the host API")]
    IncompatibleStreamHostApi = ffi::PaErrorCode_paIncompatibleStreamHostApi,

    #[error("invalid buffer")]
    BadBufferPtr = ffi::PaErrorCode_paBadBufferPtr,
}
