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

impl<T> Lock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            state: Cell::new(State::Avaiable),
            data: UnsafeCell::new(data),
        }
    }
    pub fn lock(&mut self) -> Result<guard::Guard<T>, error::Kind> {
        if self.state.get() == State::Poisoned {
            return Err(error::Kind::Poisoned);
        }
        while self.state.get() == State::Locked {}
        self.state.set(State::Locked);

        Ok(guard::Guard::new(&mut self.state, &mut self.data))
    }
    #[allow(dead_code)]
    pub fn try_lock(&mut self) -> Result<guard::Guard<T>, error::Kind> {
        if self.state.get() == State::Poisoned {
            return Err(error::Kind::Poisoned);
        }
        if self.state.get() == State::Avaiable {
            self.state.set(State::Locked);
            Ok(guard::Guard::new(&mut self.state, &mut self.data))
        } else {
            Err(error::Kind::AlreadyLocked)
        }
    }
    #[allow(dead_code)]
    //
    //This IS unsafe and you shouldn't call it. IF you do you should make sure all Guards have been dropped
    pub unsafe fn force_unlock(&mut self) {
        self.state.set(State::Avaiable);
    }
}

