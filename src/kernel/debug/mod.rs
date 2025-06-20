#[macro_export] macro_rules! debug_call {
    ($ident:expr) => {
        $crate::print!("[{}]... ", core::any::type_name_of_val(&$ident));
        $ident();
        $crate::println!("[ok]")
    };
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}