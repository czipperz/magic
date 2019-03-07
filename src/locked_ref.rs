use std::ops::Deref;
use std::sync::MutexGuard;

pub struct LockedRef<'a, T> {
    guard: MutexGuard<'a, T>,
}

impl<'a, T> From<MutexGuard<'a, T>> for LockedRef<'a, T> {
    fn from(guard: MutexGuard<'a, T>) -> Self {
        LockedRef { guard }
    }
}

impl<'a, T> Deref for LockedRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.guard
    }
}
