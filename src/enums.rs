use flags::*;

#[derive(Clone, Copy, Debug)]
pub enum JackErr {
    CallbackRegistrationError,
    CallbackDeregistrationError,
    ClientActivationError,
    ClientDeactivationError,
    ClientError(ClientStatus),
    FreewheelError,
    InvalidDeactivation,
    PortAliasError,
    PortConnectionError,
    PortDisconnectionError,
    PortNamingError,
    PortMonitorError,
    PortNotFound,
    PortRegistrationError,
    SetBufferSizeError,
    TimeError,
    UnknownError,
}

/// Used by `JackHandler::latency()`.
#[derive(Clone, Copy, Debug)]
pub enum LatencyType {
    Capture,
    Playback,
}

#[derive(Clone, Copy, Debug)]
pub enum JackControl {
    Continue,
    Quit,
}

impl JackControl {
    pub fn to_ffi(self) -> i32 {
        match self {
            JackControl::Continue => 0,
            JackControl::Quit     => -1,
        }
    }
}
