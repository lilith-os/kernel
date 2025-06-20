use linked_list_allocator::LockedHeap;
use crate::allocator::bump::{BumpAllocator, Locked};

pub mod heap;
pub mod bump;

#[global_allocator]
static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());

fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}