#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![no_std]
#[cfg(feature = "std")]
extern crate std;

use core::cell::UnsafeCell;
pub mod error;
mod guard;
pub mod lock_state;

pub struct Lock<T> {
    state: lock_state::LockState,
    data: UnsafeCell<T>,
}
//Is safe because state and data cannot be moddifed when poisoned or locked EXCEPT if force unlock is ran.
unsafe impl<T> Sync for Lock<T> {}
impl<T> Lock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            state: lock_state::LockState::new(),
            data: UnsafeCell::new(data),
        }
    }
    pub fn lock(&self) -> Result<guard::Guard<T>, error::Kind> {
        if self.is_poisoned() {
            return Err(error::Kind::Poisoned);
        }
        while !self.is_avaiable() {}
        self.state.state_lock();

        Ok(guard::Guard::new(&self.state, &self.data))
    }
    #[allow(dead_code)]
    pub fn try_lock(&self) -> Result<guard::Guard<T>, error::Kind> {
        if self.is_poisoned() {
            return Err(error::Kind::Poisoned);
        }
        if self.is_avaiable() {
            self.state.state_lock();
            Ok(guard::Guard::new(&self.state, &self.data))
        } else {
            Err(error::Kind::AlreadyLocked)
        }
    }
    /// # Safety
    /// Make sure all guards have been droped BEFORE calling.
    pub unsafe fn force_unlock(&self) {
        self.state.poison_bypass();
        self.state.state_unlock();
    }

    pub fn is_poisoned(&self) -> bool {
        self.state.is_poisoned()
    }

    pub fn is_avaiable(&self) -> bool {
        !self.state.is_locked()
    }

    pub fn unpoison(&self) {
        if self.is_poisoned() {
            self.state.poison_bypass();
        }
    }
}
