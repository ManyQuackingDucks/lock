use core::sync::atomic::{AtomicBool, Ordering};
#[derive(Debug)]
pub struct LockState {
    lock: AtomicBool,
    poison: AtomicBool,
}

impl LockState {
    pub(super) const fn new() -> Self {
        Self {
            lock: AtomicBool::new(false),
            poison: AtomicBool::new(false),
        }
    }

    pub(super) fn poison_lock(&self) {
        self.poison.store(true, Ordering::Relaxed)
    }

    pub(super) fn poison_bypass(&self) {
        self.poison.store(false, Ordering::Relaxed)
    }
    pub(super) fn is_poisoned(&self) -> bool {
        self.poison.load(Ordering::Relaxed)
    }
    pub(super) fn state_lock(&self) {
        self.lock.store(true, Ordering::Relaxed)
    }

    pub(super) fn state_unlock(&self) {
        self.lock.store(false, Ordering::Relaxed)
    }
    pub(super) fn is_locked(&self) -> bool {
        self.lock.load(Ordering::Relaxed)
    }
}
