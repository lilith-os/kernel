#![no_std]
#![no_main]

use limine::BaseRevision;
use limine::request::{RequestsEndMarker, RequestsStartMarker};

#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests_start_marker")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".requests_end_marker")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

#[unsafe(no_mangle)]
unsafe extern "C" fn _main() -> ! {
    assert!(BASE_REVISION.is_supported());
    hlt_loop()
}

fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    hlt_loop()
}