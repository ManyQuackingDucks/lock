#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
use std::cell::{Cell, UnsafeCell};
pub mod error;
mod guard;
#[derive(Clone, Copy, PartialEq)]
pub enum State {
    Avaiable,
    Locked,
    Poisoned,
}

pub struct Lock<T> {
    state: Cell<State>,
    data: UnsafeCell<T>,
}
//Is safe because state and data cannot be moddifed when poisoned or locked EXCEPT if force unlock is ran.
unsafe impl<T> Sync for Lock<T> {}
impl<T> Lock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            state: Cell::new(State::Avaiable),
            data: UnsafeCell::new(data),
        }
    }
    pub fn lock(&self) -> Result<guard::Guard<T>, error::Kind> {
        if self.state.get() == State::Poisoned {
            return Err(error::Kind::Poisoned);
        }
        while self.state.get() == State::Locked {}
        self.state.set(State::Locked);

        Ok(guard::Guard::new(&self.state, &self.data))
    }
    #[allow(dead_code)]
    pub fn try_lock(&self) -> Result<guard::Guard<T>, error::Kind> {
        if self.state.get() == State::Poisoned {
            return Err(error::Kind::Poisoned);
        }
        if self.state.get() == State::Avaiable {
            self.state.set(State::Locked);
            Ok(guard::Guard::new(&self.state, &self.data))
        } else {
            Err(error::Kind::AlreadyLocked)
        }
    }
    /// # Safety
    /// Make sure all guards have been droped BEFORE calling.
    pub unsafe fn force_unlock(&self) {
        self.state.set(State::Avaiable);
    }
}

