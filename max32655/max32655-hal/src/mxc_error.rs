#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NoDevice,
    BadParam,
    Invalid,
    Uninitialized,
    Busy,
    BadState,
    Unknown,
    CommErr,
    NoResponse,
    Overflow,
    Underflow,
    NoneAvailable,
    Shutdown,
    Abort,
    NotSupported,
    Fail,
}
