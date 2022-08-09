use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    inner: Kind,
}

#[derive(Debug)]
pub enum Kind {
    IO(std::io::Error),
    Spawn,
    OpenDesktop,
    SetThread,
    Os
}

impl Error {
    pub fn open_desktop_error() -> Self {
        Self {
            inner: Kind::OpenDesktop,
        }
    }

    pub fn set_thread_error() -> Self {
        Self {
            inner: Kind::SetThread,
        }
    }

    pub fn spawn_process_error() -> Self {
        Self { inner: Kind::Spawn }
    }

    pub fn os_error() -> Self{
        Self{
            inner: Kind::Os
        }
    }

}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self {
            inner: Kind::IO(err),
        }
    }
}
