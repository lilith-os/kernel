#[macro_export] macro_rules! debug_call {
    ($ident:expr) => {
        $crate::print!("[{}]... ", core::any::type_name_of_val(&$ident));
        $ident();
        $crate::println!("[ok]")
    };
}
