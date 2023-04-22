use std::sync::atomic::{AtomicBool, Ordering};

/// Trait shared by [`bool`] and [`AtomicBool`] for global implementation.
pub trait CanBeTrue {
    fn is_true(&self) -> bool;
}

impl CanBeTrue for bool {
    fn is_true(&self) -> bool {
        *self
    }
}

impl CanBeTrue for AtomicBool {
    fn is_true(&self) -> bool {
        self.load(Ordering::Relaxed)
    }
}
