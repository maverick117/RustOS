/*
 *  Main source file for the Rust kernel
 *  Author: Jianzhong Liu
 *  All Rights Reserved
 */

#![feature(lang_items,asm)]
#![no_std]


extern "C" {
    fn set_isr(interrupt_num:u64, function_address:u64);
    fn clear_console();
    fn print_char(txt:u16);
}

fn print_str(s:&str){
    let mut i:usize = 0;

}

#[no_mangle]
pub extern fn rust_start(){
    let welcome_msg:&str = "Welcome to RustOS.\nA safe and fast operating system for the future.\n";
    print_str(welcome_msg);
    
    unsafe{

    }
    loop{}
}



#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"] #[no_mangle]
pub extern fn panic_fmt()->!{loop{}}
