#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Success,
    NoDevice,
    BadParam,
    Invalid,
    Uninitialized,
    Busy,
    BadState,
    Unknown,
    CommErr,
    Timeout,
    NoResponse,
    Overflow,
    Underflow,
    NoneAvailable,
    Shutdown,
    Abort,
    NotSupported,
    Fail,
}
