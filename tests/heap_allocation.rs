#![no_std]
#![no_main]
extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use qemu_bindings::exit::{exit_qemu, QemuExitCode};
use kernel_lib::{init_test_entry, test_main};
use kernel_lib::allocator::heap::HEAP_SIZE;

init_test_entry!();

test_main!(
    test_allocation,
    large_vec,
    many_boxes,
    long_lived_many_boxes
);

fn test_allocation() {
    let heap_value = Box::new(41);
    let heap_value2 = Box::new(42);
    assert_eq!(*heap_value, 41);
    assert_eq!(*heap_value2, 42);
}

fn large_vec() {
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }

    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

fn many_boxes() {
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}

fn long_lived_many_boxes() {
    let long_lived = Box::new(5);
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
    assert_eq!(*long_lived, 5);
}