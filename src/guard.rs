use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};

///Guard SHOULD NEVER be shared between threads so Guard will never implement copy or clone and new will never be pub
#[derive(Debug)]
pub struct Guard<'a, T> {
    state: &'a super::lock_state::LockState,
    data: &'a UnsafeCell<T>,
}
impl<'a, T> Guard<'a, T> {
    pub fn get(&self) -> &'a T {
        //FIXME: make sure unsafe cant happen
        unsafe { &*self.data.get() }
    }
    pub fn get_mut(&mut self) -> &mut T {
        //FIXME: make sure unsafe cant happen
        unsafe { &mut *self.data.get() }
    }
    pub(super) const fn new(
        state: &'a super::lock_state::LockState,
        data: &'a UnsafeCell<T>,
    ) -> Self {
        Self { state, data }
    }
}

impl<'a, T> Drop for Guard<'a, T> {
    fn drop(&mut self) {
        //If the method holding a Guard panics then the data of the Guard is considered poisoned
        #[cfg(feature = "std")]
        if std::thread::panicking() {
            self.state.poison_lock();
        }
        self.state.state_unlock();
    }
}

impl<'a, T> Deref for Guard<'a, T> {
    type Target = T;
    fn deref(&self) -> &'a Self::Target {
        self.get()
    }
}

impl<'a, T> DerefMut for Guard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}
