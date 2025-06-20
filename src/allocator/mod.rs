use linked_list_allocator::LockedHeap;

pub mod heap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();
