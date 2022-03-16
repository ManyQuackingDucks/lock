#[derive(Debug)]
pub enum Kind {
    AlreadyLocked,
    Poisoned,
}

impl core::fmt::Display for Kind {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::AlreadyLocked => write!(f, "The state is already locked and it could not lock"),
            Self::Poisoned => write!(f, "The lock has been poisoned and the quality of the data is unknown")
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Kind {}
