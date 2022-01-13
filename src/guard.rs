use std::cell::{Cell, UnsafeCell};
use std::ops::{Deref, DerefMut};
use std::thread;

///Guard SHOULD NEVER be shared between threads so Guard will never implement copy or clone and new will never be pub
pub struct Guard<'a, T> {
    state: &'a mut Cell<super::State>,
    data: &'a mut UnsafeCell<T>,
}
impl<'a, T> Guard<'a, T> {
    #[allow(clippy::missing_const_for_fn)] //Not stable in rust yet
    pub fn get(&self) -> &'a T {
        unsafe { &*self.data.get() }
    }
    pub fn get_mut(&mut self) -> &mut T {
        self.data.get_mut()
    }
    pub(super) fn new(state: &'a mut Cell<super::State>, data: &'a mut UnsafeCell<T>) -> Self {
        Self { state, data }
    }
}

impl<'a, T> Drop for Guard<'a, T> {
    fn drop(&mut self) {
        //If the method containg Guard panics then the data of the Guard is considered poisoned
        if thread::panicking() {
            self.state.set(super::State::Poisoned);
        } else {
            self.state.set(super::State::Avaiable);
        }
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
