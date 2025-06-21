use spin::{Mutex, MutexGuard};
use crate::allocator::fixed_size_block::FixedSizeBlockAllocator;

pub mod heap;
pub mod fixed_size_block;

#[global_allocator]
static ALLOCATOR: Locked<FixedSizeBlockAllocator> = Locked::new(FixedSizeBlockAllocator::new());

pub struct Locked<T> {
    inner: Mutex<T>,
}

impl<'a, T> Locked<T> {
    pub const fn new(inner: T) -> Self {
        Self { inner: Mutex::new(inner) }
    }

    pub fn lock(&'a self) -> MutexGuard<'a, T> {
        self.inner.lock()
    }
}